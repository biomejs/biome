use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsIdentifierReference, AnyJsModuleItem, AnyJsRoot, AnyJsStatement, AnyTsType,
    JsCallExpression, JsLanguage,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, TextRange, WalkEvent};
use biome_rule_options::use_vue_valid_define_props::UseVueValidDefinePropsOptions;
use biome_vue_semantic::{ComponentKind, MacroOption};

declare_lint_rule! {
    /// Enforce valid `defineProps()` calls in Vue components.
    ///
    /// In `<script setup>`, a component declares its props with `defineProps()`. Vue reads this
    /// call when it compiles the component, so the call has to follow some rules. This rule
    /// reports a `defineProps()` call when:
    ///
    /// - its argument uses a variable declared in `<script setup>`. Vue moves the argument out of
    ///   the setup code, where these variables don't exist;
    /// - it declares the props both with a type, as in `defineProps<{ msg: string }>()`, and with
    ///   an argument, as in `defineProps({ msg: String })`;
    /// - the component calls it more than once;
    /// - the default export of the plain `<script>` block also declares props with the `props`
    ///   option;
    /// - it declares no props, and the plain `<script>` block doesn't declare any either.
    ///
    /// The rule checks the `defineProps()` calls at the top level of `<script setup>`: a call
    /// written as a statement of its own or as the value of a variable, also when it's wrapped in
    /// `withDefaults()`.
    ///
    /// :::note
    /// The last two checks read the plain `<script>` block of the file, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`. Without that setting, the rule skips them.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const definition = { msg: String };
    /// defineProps(definition);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineProps<{ msg?: string }>({ msg: String });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineProps({ msg: String });
    /// defineProps({ count: Number });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script>
    /// export default {
    ///   props: { msg: String },
    /// };
    /// </script>
    /// <script setup>
    /// defineProps({ count: Number });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script setup>
    /// defineProps();
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { definition } from "./definition";
    ///
    /// defineProps(definition);
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// defineProps<{ msg?: string }>();
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script>
    /// const definition = { msg: String };
    /// </script>
    /// <script setup>
    /// defineProps(definition);
    /// </script>
    /// ```
    ///
    pub UseVueValidDefineProps {
        version: "next",
        name: "useVueValidDefineProps",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-define-props").inspired()],
    }
}

impl Rule for UseVueValidDefineProps {
    type Query = VueComponent;
    type State = InvalidDefineProps;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidDefinePropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        if component.kind() != ComponentKind::Setup {
            return Box::default();
        }
        let model = ctx.model();
        let AnyJsRoot::JsModule(module) = ctx.root() else {
            return Box::default();
        };
        let mut calls = Vec::new();
        for item in module.items() {
            let AnyJsModuleItem::AnyJsStatement(statement) = item else {
                continue;
            };
            match statement {
                AnyJsStatement::JsExpressionStatement(statement) => calls.extend(
                    statement
                        .expression()
                        .ok()
                        .and_then(|expression| define_props_call(&expression, model)),
                ),
                AnyJsStatement::JsVariableStatement(statement) => {
                    let Ok(declaration) = statement.declaration() else {
                        continue;
                    };
                    calls.extend(declaration.declarators().iter().filter_map(|declarator| {
                        let expression = declarator.ok()?.initializer()?.expression().ok()?;
                        define_props_call(&expression, model)
                    }));
                }
                _ => {}
            }
        }

        let mut signals = Vec::new();
        let mut has_empty_call = false;
        for call in &calls {
            let argument = call
                .arguments()
                .ok()
                .and_then(|arguments| arguments.args().first()?.ok());
            let has_type_argument = call
                .type_arguments()
                .is_some_and(|arguments| !arguments.ts_type_argument_list().is_empty());
            match argument {
                Some(_) if has_type_argument => {
                    signals.push(InvalidDefineProps::TypeAndArgument(call.clone()));
                }
                Some(argument) => {
                    if let Some(argument) = argument.as_any_js_expression() {
                        collect_local_references(argument, model, &mut signals);
                    }
                }
                None if !has_type_argument => has_empty_call = true,
                None => {}
            }
        }

        let Some((call, duplicates)) = calls.split_first() else {
            return signals.into_boxed_slice();
        };
        if !duplicates.is_empty() {
            signals.extend(
                duplicates
                    .iter()
                    .map(|duplicate| InvalidDefineProps::Multiple {
                        call: duplicate.clone(),
                        first: call.range(),
                    }),
            );
        } else if component.snippet().is_some() {
            // Without a snippet, the model was built from this script block
            // alone, and doesn't know what the plain `<script>` block declares.
            let has_props_option = component.macro_options().contains(MacroOption::Props);
            if has_empty_call && !has_props_option {
                signals.push(InvalidDefineProps::NotDefined(call.clone()));
            } else if !has_empty_call && has_props_option {
                signals.push(InvalidDefineProps::DefinedInBoth(call.clone()));
            }
        }
        signals.into_boxed_slice()
    }

    fn text_range(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<TextRange> {
        Some(state.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(match state {
            InvalidDefineProps::ReferencesLocal(reference) => {
                let name = reference.value_token().ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    state.range(),
                    markup! {
                        <Emphasis>"defineProps()"</Emphasis>" uses "<Emphasis>{name.text_trimmed()}</Emphasis>", which is declared in "<Emphasis>"<script setup>"</Emphasis>"."
                    },
                )
                .note(markup! {
                    "Vue moves the argument of "<Emphasis>"defineProps()"</Emphasis>" out of the setup code when it compiles the component, so the argument can't use variables declared in "<Emphasis>"<script setup>"</Emphasis>"."
                })
                .note(markup! {
                    "Import the value from another file, or declare it in a plain "<Emphasis>"<script>"</Emphasis>" block."
                })
            }
            InvalidDefineProps::TypeAndArgument(_) => RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    <Emphasis>"defineProps()"</Emphasis>" declares the props both with a type and with an argument."
                },
            )
            .note(markup! {
                "Vue can't use both at the same time, and fails to compile the component."
            })
            .note(markup! {
                "Declare the props either with a type between "<Emphasis>"<>"</Emphasis>" or with an argument between "<Emphasis>"()"</Emphasis>"."
            }),
            InvalidDefineProps::Multiple { first, .. } => RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    <Emphasis>"defineProps()"</Emphasis>" is called more than once."
                },
            )
            .detail(first, markup! {
                "It's first called here."
            })
            .note(markup! {
                "A component declares its props once, and Vue fails to compile a component that calls "<Emphasis>"defineProps()"</Emphasis>" several times."
            })
            .note(markup! {
                "Declare all the props in a single "<Emphasis>"defineProps()"</Emphasis>" call."
            }),
            InvalidDefineProps::NotDefined(_) => RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    <Emphasis>"defineProps()"</Emphasis>" is called without declaring any props."
                },
            )
            .note(markup! {
                "Without a type between "<Emphasis>"<>"</Emphasis>" or an argument between "<Emphasis>"()"</Emphasis>", the component accepts no props."
            })
            .note(markup! {
                "Declare the props with a type or an argument, or remove the call."
            }),
            InvalidDefineProps::DefinedInBoth(_) => RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Props are declared both with "<Emphasis>"defineProps()"</Emphasis>" and with the "<Emphasis>"props"</Emphasis>" option of the default export."
                },
            )
            .note(markup! {
                "With two declarations, it's hard to tell which props the component accepts."
            })
            .note(markup! {
                "Declare all the props in "<Emphasis>"defineProps()"</Emphasis>", or all of them in the "<Emphasis>"props"</Emphasis>" option."
            }),
        })
    }
}

/// A problem with a `defineProps()` call.
pub enum InvalidDefineProps {
    /// The argument uses a variable declared in `<script setup>`.
    ReferencesLocal(AnyJsIdentifierReference),
    /// The call has both a type argument and an argument.
    TypeAndArgument(JsCallExpression),
    /// A call after the first one of the component.
    Multiple {
        call: JsCallExpression,
        /// The range of the first call.
        first: TextRange,
    },
    /// The call declares no props, and neither does the plain `<script>` block.
    NotDefined(JsCallExpression),
    /// The call declares props, and so does the plain `<script>` block.
    DefinedInBoth(JsCallExpression),
}

impl InvalidDefineProps {
    fn range(&self) -> TextRange {
        match self {
            Self::ReferencesLocal(reference) => reference.range(),
            Self::TypeAndArgument(call)
            | Self::Multiple { call, .. }
            | Self::NotDefined(call)
            | Self::DefinedInBoth(call) => call.range(),
        }
    }
}

/// Returns the `defineProps()` call that `expression` is, or that it wraps in
/// `withDefaults()`.
fn define_props_call(expression: &AnyJsExpression, model: &SemanticModel) -> Option<JsCallExpression> {
    let call = expression.inner_expression()?.as_js_call_expression()?.clone();
    if is_vue_compiler_macro_call(&call, model, "defineProps") {
        return Some(call);
    }
    if !is_vue_compiler_macro_call(&call, model, "withDefaults") {
        return None;
    }
    let inner = call
        .arguments()
        .ok()?
        .args()
        .first()?
        .ok()?
        .as_any_js_expression()?
        .inner_expression()?
        .as_js_call_expression()?
        .clone();
    is_vue_compiler_macro_call(&inner, model, "defineProps").then_some(inner)
}

/// Pushes a signal for each name in `argument` that refers to a variable
/// declared in the analyzed `<script setup>` block, outside of `argument`.
///
/// Names used as types are skipped, because Vue doesn't move types.
fn collect_local_references(
    argument: &AnyJsExpression,
    model: &SemanticModel,
    signals: &mut Vec<InvalidDefineProps>,
) {
    let argument_range = argument.range();
    let mut preorder = argument.syntax().preorder_of_kinds(REFERENCE_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        let Some(reference) = AnyJsIdentifierReference::cast(node) else {
            preorder.skip_subtree();
            continue;
        };
        if model.binding(&reference).is_some_and(|binding| {
            !binding.is_imported() && !argument_range.contains_range(binding.syntax().text_trimmed_range())
        }) {
            signals.push(InvalidDefineProps::ReferencesLocal(reference));
        }
    }
}

/// The nodes the reference search stops at: names, and types, whose names
/// are skipped.
const REFERENCE_SEARCH_KINDS: SyntaxKindSet<JsLanguage> =
    AnyJsIdentifierReference::KIND_SET.union(AnyTsType::KIND_SET);
