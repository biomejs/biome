use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsIdentifierReference, AnyJsModuleItem, AnyJsStatement,
    AnyTsType, JsCallExpression, JsModule,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, TextRange, WalkEvent};
use biome_rule_options::use_vue_valid_define_emits::UseVueValidDefineEmitsOptions;

use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce valid usage of Vue's `defineEmits` macro.
    ///
    /// `defineEmits` declares the events that a component can send to its parent.
    /// Vue reads the call when it compiles the component, so the call must follow a
    /// few rules. This rule reports a `defineEmits` call in `<script setup>` when:
    ///
    /// - It declares the events with both a type and an argument. Vue accepts only
    ///   one of the two, and fails to compile the component when both are present.
    /// - Its argument uses a variable, function, or class declared in the same
    ///   `<script setup>` block. Vue evaluates the argument before the code in
    ///   `<script setup>` runs, so these names do not exist yet. Imported names and
    ///   names declared in a separate `<script>` block can be used.
    /// - It comes after another `defineEmits` call. Vue allows a single call per
    ///   component, and fails to compile the component when there are more.
    /// - It has neither a type nor an argument, so it declares no events.
    ///
    /// This rule only checks the `<script setup>` block. It does not read an `emits`
    /// option declared in a separate `<script>` block of the same file.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const emit = defineEmits<(e: 'notify') => void>(['submit'])
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const events = ['notify']
    /// const emit = defineEmits(events)
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineEmits(['notify'])
    /// defineEmits(['submit'])
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const emit = defineEmits()
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { events } from './events'
    ///
    /// const emit = defineEmits(events)
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// const emit = defineEmits<(e: 'notify') => void>()
    /// </script>
    /// ```
    ///
    pub UseVueValidDefineEmits {
        version: "next",
        name: "useVueValidDefineEmits",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-define-emits").same()],
    }
}

impl Rule for UseVueValidDefineEmits {
    type Query = Semantic<JsModule>;
    type State = InvalidDefineEmits;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidDefineEmitsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_vue_setup()
        {
            return Box::default();
        }

        let model = ctx.model();
        let calls = find_define_emits_calls(ctx.query(), model);
        let is_called_multiple_times = calls.len() > 1;
        let first_call = calls.first().map(AstNode::range);

        let mut states = Vec::new();
        for call in calls {
            let has_type = call
                .type_arguments()
                .is_some_and(|type_arguments| !type_arguments.ts_type_argument_list().is_empty());
            let argument = call
                .arguments()
                .ok()
                .and_then(|arguments| arguments.args().first())
                .and_then(Result::ok);

            match argument {
                Some(_) if has_type => {
                    states.push(InvalidDefineEmits::TypeAndArgument { call: call.range() });
                }
                Some(argument) => collect_local_references(&argument, model, &mut states),
                None if !has_type && !is_called_multiple_times => {
                    states.push(InvalidDefineEmits::NoEvents { call: call.range() });
                }
                None => {}
            }

            if let Some(first_call) = first_call
                && first_call != call.range()
            {
                states.push(InvalidDefineEmits::Multiple {
                    call: call.range(),
                    first_call,
                });
            }
        }

        states.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            InvalidDefineEmits::TypeAndArgument { call } => RuleDiagnostic::new(
                rule_category!(),
                call,
                markup! {
                    "This "<Emphasis>"defineEmits"</Emphasis>" call declares events with both a type and an argument."
                },
            )
            .note(markup! {
                "Vue accepts only one of the two, and fails to compile the component when both are present."
            })
            .note(markup! {
                "Remove either the type between the angle brackets or the argument."
            }),
            InvalidDefineEmits::LocalReference {
                reference,
                declaration,
            } => RuleDiagnostic::new(
                rule_category!(),
                reference,
                markup! {
                    "This "<Emphasis>"defineEmits"</Emphasis>" argument uses a name that is declared in the same "<Emphasis>"<script setup>"</Emphasis>" block."
                },
            )
            .detail(declaration, markup! { "The name is declared here." })
            .note(markup! {
                "Vue evaluates the "<Emphasis>"defineEmits"</Emphasis>" argument before the code in "<Emphasis>"<script setup>"</Emphasis>" runs, so this name does not exist yet."
            })
            .note(markup! {
                "Write the value directly in the argument, import it from another file, or declare it in a separate "<Emphasis>"<script>"</Emphasis>" block."
            }),
            InvalidDefineEmits::Multiple { call, first_call } => RuleDiagnostic::new(
                rule_category!(),
                call,
                markup! {
                    <Emphasis>"defineEmits"</Emphasis>" is called more than once in this "<Emphasis>"<script setup>"</Emphasis>" block."
                },
            )
            .detail(first_call, markup! { "The first call is here." })
            .note(markup! {
                "Vue allows a single "<Emphasis>"defineEmits"</Emphasis>" call per component, and fails to compile the component when there are more."
            })
            .note(markup! {
                "Declare all the events of the component in one call."
            }),
            InvalidDefineEmits::NoEvents { call } => RuleDiagnostic::new(
                rule_category!(),
                call,
                markup! {
                    "This "<Emphasis>"defineEmits"</Emphasis>" call does not declare any events."
                },
            )
            .note(markup! {
                "Vue does not know which events the component sends, so it treats listeners for them as plain attributes and passes them to the root element of the component."
            })
            .note(markup! {
                "Declare the events with an array of names, an object, or a type."
            }),
        };

        Some(diagnostic)
    }
}

/// A problem found in a `defineEmits` call.
pub enum InvalidDefineEmits {
    /// The call has both type arguments and a runtime argument.
    TypeAndArgument { call: TextRange },
    /// The runtime argument refers to a name declared in the `<script setup>` block.
    LocalReference {
        reference: TextRange,
        declaration: TextRange,
    },
    /// The call comes after another call in the same `<script setup>` block.
    Multiple {
        call: TextRange,
        first_call: TextRange,
    },
    /// The call has neither type arguments nor a runtime argument.
    NoEvents { call: TextRange },
}

/// Returns the `defineEmits` calls that Vue compiles as macros.
///
/// Vue only recognizes the macro when the call is a statement of its own or the
/// initializer of a variable, directly inside `<script setup>`.
fn find_define_emits_calls(module: &JsModule, model: &SemanticModel) -> Vec<JsCallExpression> {
    let mut calls = Vec::new();
    let mut push_macro_call = |expression: AnyJsExpression| {
        if let Some(call) = unwrap_call(expression)
            && is_vue_compiler_macro_call(&call, model, "defineEmits")
        {
            calls.push(call);
        }
    };

    for item in module.items() {
        match item {
            AnyJsModuleItem::AnyJsStatement(AnyJsStatement::JsExpressionStatement(statement)) => {
                if let Ok(expression) = statement.expression() {
                    push_macro_call(expression);
                }
            }
            AnyJsModuleItem::AnyJsStatement(AnyJsStatement::JsVariableStatement(statement)) => {
                let Ok(declaration) = statement.declaration() else {
                    continue;
                };
                for declarator in declaration.declarators().iter().flatten() {
                    if let Some(Ok(expression)) = declarator
                        .initializer()
                        .map(|initializer| initializer.expression())
                    {
                        push_macro_call(expression);
                    }
                }
            }
            _ => {}
        }
    }

    calls
}

/// Returns the call that `expression` consists of, ignoring the parentheses and
/// the TypeScript-only wrappers around it, such as `as` casts.
///
/// A call at the end of a comma-separated sequence is not returned, because Vue
/// does not compile it as a macro.
fn unwrap_call(mut expression: AnyJsExpression) -> Option<JsCallExpression> {
    loop {
        expression = match expression {
            AnyJsExpression::JsCallExpression(call) => return Some(call),
            AnyJsExpression::JsParenthesizedExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsAsExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsSatisfiesExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsNonNullAssertionExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsTypeAssertionExpression(wrapper) => wrapper.expression().ok()?,
            AnyJsExpression::TsInstantiationExpression(wrapper) => wrapper.expression().ok()?,
            _ => return None,
        };
    }
}

/// Appends an [`InvalidDefineEmits::LocalReference`] to `states` for every name
/// in `argument` that is declared in the same script block, outside of `argument`.
///
/// Imports are skipped, because Vue keeps them available to the hoisted argument.
/// So are names inside type annotations, because those are never evaluated.
fn collect_local_references(
    argument: &AnyJsCallArgument,
    model: &SemanticModel,
    states: &mut Vec<InvalidDefineEmits>,
) {
    let argument_range = argument.range();
    let mut preorder = argument.syntax().preorder();
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if AnyTsType::KIND_SET.matches(node.kind()) {
            preorder.skip_subtree();
            continue;
        }

        let Some(reference) = AnyJsIdentifierReference::cast(node) else {
            continue;
        };
        let Some(binding) = model.binding(&reference) else {
            continue;
        };
        let declaration = binding.range();
        if binding.is_imported() || argument_range.contains_range(declaration) {
            continue;
        }

        states.push(InvalidDefineEmits::LocalReference {
            reference: reference.range(),
            declaration,
        });
    }
}
