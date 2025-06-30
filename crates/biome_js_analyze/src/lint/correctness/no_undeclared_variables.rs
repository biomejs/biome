use crate::globals::{is_js_global, is_ts_global};
use crate::services::semantic::SemanticServices;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsFunction, JsFileSource, Language, TextRange, TsAsExpression, TsReferenceType,
};
use biome_rowan::AstNode;
use biome_rule_options::no_undeclared_variables::NoUndeclaredVariablesOptions;

declare_lint_rule! {
    /// Prevents the usage of variables that haven't been declared inside the document.
    ///
    /// If you need to allow-list some global bindings, you can use the [`javascript.globals`](/reference/configuration/#javascriptglobals) configuration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foobar;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // throw diagnostic for JavaScript files
    /// PromiseLike;
    /// ```
    /// ### Valid
    ///
    /// ```ts
    /// type B<T> = PromiseLike<T>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `checkTypes`
    ///
    /// When set to `true`, it checks for undeclared types too.
    /// The option defaults to `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "checkTypes": true
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// type A = number extends infer T ? never : T;
    /// ```
    pub NoUndeclaredVariables {
        version: "1.0.0",
        name: "noUndeclaredVariables",
        language: "js",
        sources: &[RuleSource::Eslint("no-undef").same()],
        recommended: false,
        severity: Severity::Error,
    }
}

impl Rule for NoUndeclaredVariables {
    type Query = SemanticServices;
    type State = (TextRange, Box<str>);
    type Signals = Box<[Self::State]>;
    type Options = NoUndeclaredVariablesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query()
            .all_unresolved_references()
            .filter_map(|reference| {
                let identifier = reference.tree();
                let under_as_expression = identifier
                    .parent::<TsReferenceType>()
                    .and_then(|ty| ty.parent::<TsAsExpression>())
                    .is_some();

                let token = identifier.value_token().ok()?;
                let text = token.text_trimmed();

                let source_type = ctx.source_type::<JsFileSource>();

                if ctx.is_global(text) {
                    return None;
                }

                // Typescript Const Assertion
                if text == "const" && under_as_expression {
                    return None;
                }

                // arguments object within non-arrow functions
                if text == "arguments" {
                    let is_in_non_arrow_function =
                        identifier.syntax().ancestors().any(|ancestor| {
                            !matches!(
                                AnyJsFunction::cast(ancestor),
                                None | Some(AnyJsFunction::JsArrowFunctionExpression(_))
                            )
                        });
                    if is_in_non_arrow_function {
                        return None;
                    }
                }

                if is_global(text, source_type) {
                    return None;
                }

                if !ctx.options().check_types && identifier.is_only_type() {
                    return None;
                }

                let span = token.text_trimmed_range();
                let text = text.into();
                Some((span, text))
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (span, name): &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            *span,
            markup! {
                "The "<Emphasis>{name.as_ref()}</Emphasis>" variable is undeclared."
            },
        ).note(markup! {
            "By default, Biome recognizes browser and Node.js globals.\nYou can ignore more globals using the "<Hyperlink href="https://biomejs.dev/reference/configuration/#javascriptglobals">"javascript.globals"</Hyperlink>" configuration."
        }))
    }
}

fn is_global(reference_name: &str, source_type: &JsFileSource) -> bool {
    match source_type.language() {
        Language::JavaScript => is_js_global(reference_name),
        Language::TypeScript { .. } => is_js_global(reference_name) || is_ts_global(reference_name),
    }
}
