use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsModuleItem, AnyJsStatement, AnyTsType, JsLanguage,
    JsModule, JsReferenceIdentifier,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, TextRange, WalkEvent};
use biome_rule_options::use_vue_valid_define_options::UseVueValidDefineOptionsOptions;

declare_lint_rule! {
    /// Enforce valid usage of Vue's `defineOptions()` macro.
    ///
    /// In `<script setup>`, `defineOptions()` sets component options, such as the component's
    /// `name` or `inheritAttrs`. Vue reads the call when it compiles the component, so the call
    /// must follow these rules:
    ///
    /// - It is called at most once.
    /// - It receives the options as an argument.
    /// - It doesn't receive type arguments, such as `defineOptions<{ name: "Foo" }>()`.
    /// - It doesn't declare `props`, `emits`, `expose`, or `slots`. `<script setup>` has dedicated
    ///   macros for these: `defineProps()`, `defineEmits()`, `defineExpose()`, and `defineSlots()`.
    /// - It doesn't use variables or functions declared in `<script setup>`. Vue moves the options
    ///   out of `<script setup>` when it compiles the component, so these declarations aren't
    ///   available to them. Imports, and constants whose value is a string, number, or other
    ///   literal value, can be used.
    ///
    /// Breaking one of these rules makes the Vue compiler report an error, or makes the call
    /// have no effect.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const options = { name: "Foo" };
    /// defineOptions(options);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineOptions({ name: "Foo" });
    /// defineOptions({ inheritAttrs: false });
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineOptions();
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineOptions<{ name: "Foo" }>({});
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineOptions({ props: { msg: String } });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { componentName } from "./names";
    ///
    /// const inheritAttrs = false;
    ///
    /// defineOptions({ name: componentName, inheritAttrs });
    /// defineProps({ msg: String });
    /// </script>
    /// ```
    ///
    pub UseVueValidDefineOptions {
        version: "2.6.5",
        name: "useVueValidDefineOptions",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-define-options").same()],
    }
}

/// A problem with a `defineOptions()` call.
pub enum InvalidDefineOptions {
    /// A call that follows an earlier call in the same block.
    Repeated {
        call: TextRange,
        /// The range of the first call in the block.
        first_call: TextRange,
    },
    /// A call without arguments.
    MissingOptions(TextRange),
    /// The range of the type arguments of a call.
    TypeArguments(TextRange),
    /// An option that a dedicated macro declares instead.
    MacroOption {
        member: TextRange,
        option: MacroOption,
    },
    /// A use of a variable that is declared in `<script setup>`.
    LocalReference {
        reference: JsReferenceIdentifier,
        /// The range of the variable's declared name.
        declaration: TextRange,
    },
}

impl Rule for UseVueValidDefineOptions {
    type Query = Semantic<JsModule>;
    type State = InvalidDefineOptions;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidDefineOptionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_vue_setup()
        {
            return Box::default();
        }

        let model = ctx.model();
        let mut signals = Vec::new();
        let mut first_call = None;
        for call in ctx
            .query()
            .items()
            .iter()
            .flat_map(top_level_expressions)
            .filter_map(|expression| match expression.omit_parentheses() {
                AnyJsExpression::JsCallExpression(call) => Some(call),
                _ => None,
            })
            .filter(|call| is_vue_compiler_macro_call(call, model, "defineOptions"))
        {
            let call_range = call.range();
            match first_call {
                Some(first_call) => signals.push(InvalidDefineOptions::Repeated {
                    call: call_range,
                    first_call,
                }),
                None => first_call = Some(call_range),
            }

            let argument = call
                .arguments()
                .ok()
                .and_then(|arguments| arguments.args().iter().next());
            match argument {
                Some(Ok(argument)) => {
                    if let Some(AnyJsExpression::JsObjectExpression(object)) = argument
                        .as_any_js_expression()
                        .map(|expression| expression.clone().omit_parentheses())
                    {
                        signals.extend(object.members().iter().flatten().filter_map(|member| {
                            let option = MacroOption::from_name(member.name()?.text())?;
                            Some(InvalidDefineOptions::MacroOption {
                                member: member.range(),
                                option,
                            })
                        }));
                    }
                    collect_local_references(&argument, model, &mut signals);
                }
                Some(Err(_)) => {}
                None => signals.push(InvalidDefineOptions::MissingOptions(call_range)),
            }

            if let Some(type_arguments) = call.type_arguments() {
                signals.push(InvalidDefineOptions::TypeArguments(type_arguments.range()));
            }
        }
        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            InvalidDefineOptions::Repeated { call, first_call } => RuleDiagnostic::new(
                rule_category!(),
                call,
                markup! {
                    <Emphasis>"defineOptions()"</Emphasis>" is called more than once."
                },
            )
            .detail(
                first_call,
                markup! { "It is first called here." },
            )
            .note(markup! {
                "Vue accepts only one "<Emphasis>"defineOptions()"</Emphasis>" call per component, and reports an error when it compiles this one."
            })
            .note(markup! {
                "Move these options into the first call."
            }),
            InvalidDefineOptions::MissingOptions(call) => RuleDiagnostic::new(
                rule_category!(),
                call,
                markup! {
                    <Emphasis>"defineOptions()"</Emphasis>" is called without options."
                },
            )
            .note(markup! {
                "Without options, the call doesn't change the component."
            })
            .note(markup! {
                "Pass an object with the component options, or remove the call."
            }),
            InvalidDefineOptions::TypeArguments(type_arguments) => RuleDiagnostic::new(
                rule_category!(),
                type_arguments,
                markup! {
                    <Emphasis>"defineOptions()"</Emphasis>" doesn't accept type arguments."
                },
            )
            .note(markup! {
                "Vue reads the options only from the object passed in parentheses, and reports an error when it compiles a call with type arguments."
            })
            .note(markup! {
                "Remove the type arguments, and pass the options as an object."
            }),
            InvalidDefineOptions::MacroOption { member, option } => RuleDiagnostic::new(
                rule_category!(),
                member,
                markup! {
                    <Emphasis>"defineOptions()"</Emphasis>" can't declare "<Emphasis>{option}</Emphasis>"."
                },
            )
            .note(markup! {
                "Vue reports an error when it compiles this option, because "<Emphasis>"<script setup>"</Emphasis>" has a dedicated macro for it."
            })
            .note(markup! {
                "Declare "<Emphasis>{option}</Emphasis>" with "<Emphasis>{option.macro_name()}</Emphasis>" instead."
            }),
            InvalidDefineOptions::LocalReference {
                reference,
                declaration,
            } => {
                let name = reference.value_token().ok()?;
                RuleDiagnostic::new(
                    rule_category!(),
                    reference.range(),
                    markup! {
                        <Emphasis>"defineOptions()"</Emphasis>" uses "<Emphasis>{name.text_trimmed()}</Emphasis>", which is declared in "<Emphasis>"<script setup>"</Emphasis>"."
                    },
                )
                .detail(
                    declaration,
                    markup! { <Emphasis>{name.text_trimmed()}</Emphasis>" is declared here." },
                )
                .note(markup! {
                    "Vue moves the options out of "<Emphasis>"<script setup>"</Emphasis>" when it compiles the component, so variables declared there aren't available to them."
                })
                .note(markup! {
                    "Move the declaration to a separate "<Emphasis>"<script>"</Emphasis>" block, or import it from another file."
                })
            }
        };
        Some(diagnostic)
    }
}

/// An option that `defineOptions()` can't declare, because `<script setup>` has a dedicated
/// macro for it.
#[derive(Clone, Copy)]
pub enum MacroOption {
    Props,
    Emits,
    Expose,
    Slots,
}

impl MacroOption {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "props" => Some(Self::Props),
            "emits" => Some(Self::Emits),
            "expose" => Some(Self::Expose),
            "slots" => Some(Self::Slots),
            _ => None,
        }
    }

    /// The macro that declares this option in `<script setup>`.
    const fn macro_name(self) -> &'static str {
        match self {
            Self::Props => "defineProps()",
            Self::Emits => "defineEmits()",
            Self::Expose => "defineExpose()",
            Self::Slots => "defineSlots()",
        }
    }
}

impl biome_console::fmt::Display for MacroOption {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Props => "props",
            Self::Emits => "emits",
            Self::Expose => "expose",
            Self::Slots => "slots",
        })
    }
}

/// Returns the expression that a top-level statement consists of, or the initializers of the
/// variables that a top-level statement declares.
///
/// Vue reads compiler macros only from these positions.
fn top_level_expressions(item: AnyJsModuleItem) -> impl Iterator<Item = AnyJsExpression> {
    let (expression, declarators) = match item {
        AnyJsModuleItem::AnyJsStatement(AnyJsStatement::JsExpressionStatement(statement)) => {
            (statement.expression().ok(), None)
        }
        AnyJsModuleItem::AnyJsStatement(AnyJsStatement::JsVariableStatement(statement)) => (
            None,
            statement
                .declaration()
                .ok()
                .map(|declaration| declaration.declarators()),
        ),
        _ => (None, None),
    };
    expression.into_iter().chain(
        declarators
            .into_iter()
            .flat_map(|declarators| declarators.iter())
            .filter_map(|declarator| declarator.ok()?.initializer()?.expression().ok()),
    )
}

/// Adds a signal for each use, in `argument`, of a variable that is declared in `<script setup>`
/// and isn't available to the component options.
///
/// Uses inside TypeScript types are ignored, because types are removed when the component is
/// compiled.
fn collect_local_references(
    argument: &AnyJsCallArgument,
    model: &SemanticModel,
    signals: &mut Vec<InvalidDefineOptions>,
) {
    let argument_range = argument.range();
    let mut preorder = argument.syntax().preorder_of_kinds(REFERENCE_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        let Some(reference) = JsReferenceIdentifier::cast(node) else {
            preorder.skip_subtree();
            continue;
        };
        // An undeclared name is a global, or comes from the plain `<script>` block.
        let Some(binding) = model.binding(&reference) else {
            continue;
        };
        let declaration = binding.range();
        if binding.is_imported()
            || argument_range.contains_range(declaration)
            || is_literal_constant(&binding.tree())
        {
            continue;
        }
        signals.push(InvalidDefineOptions::LocalReference {
            reference,
            declaration,
        });
    }
}

/// Returns `true` when `binding` is declared by a `const` declaration whose value is a literal,
/// such as `const name = "Foo"`.
///
/// Vue keeps these declarations available to the component options.
fn is_literal_constant(binding: &AnyJsIdentifierBinding) -> bool {
    let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) = binding.declaration()
    else {
        return false;
    };
    declarator
        .declaration()
        .is_some_and(|declaration| declaration.is_const())
        && declarator
            .initializer()
            .and_then(|initializer| initializer.expression().ok())
            .is_some_and(|expression| {
                matches!(
                    expression.omit_parentheses(),
                    AnyJsExpression::AnyJsLiteralExpression(_)
                )
            })
}

/// The nodes the search for variable uses stops at: variable names, and TypeScript types, whose
/// contents are skipped.
const REFERENCE_SEARCH_KINDS: SyntaxKindSet<JsLanguage> =
    JsReferenceIdentifier::KIND_SET.union(AnyTsType::KIND_SET);
