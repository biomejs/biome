use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsVariableStatement};
use biome_rowan::AstNode;
use biome_rule_options::use_qwik_valid_lexical_scope::UseQwikValidLexicalScopeOptions;

declare_lint_rule! {
    /// Disallow unserializable expressions in Qwik dollar ($) scopes.
    ///
    /// Ensures all captured values in Qwik components can be properly serialized for resumability.
    /// See [Qwik Optimizer: Lexical Scope](https://qwik.dev/docs/advanced/optimizer/#lexical-scope) for proper usage patterns.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // Arrow function assigned without wrapping it in $(...)
    /// const handleClick = () => {
    ///   console.log("clicked");
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const handleClick = $(() => {
    ///   // Valid: only using serializable variables or props
    ///   console.log("clicked");
    /// });
    /// ```
    ///
 pub UseQwikValidLexicalScope {
        version: "2.2.6",
        name: "useQwikValidLexicalScope",
        language: "jsx",
        sources: &[RuleSource::EslintQwik("valid-lexical-scope").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for UseQwikValidLexicalScope {
    type Query = Ast<JsVariableStatement>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = UseQwikValidLexicalScopeOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let var_stmt = ctx.query();
        for declarator in var_stmt.declaration().ok()?.declarators() {
            let expr_opt = declarator
                .as_ref()
                .ok()
                .and_then(|d| d.initializer())
                .and_then(|init| init.expression().ok());

            if let Some(expr) = expr_opt.filter(|expr| {
                matches!(expr, AnyJsExpression::JsArrowFunctionExpression(_))
                    && !is_wrapped_with_dollar(expr)
            }) {
                return Some(expr.range());
            }
        }
        None
    }

    fn diagnostic(
        _: &biome_analyze::context::RuleContext<Self>,
        range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Non-serializable expression must be wrapped with "<Emphasis>"$(...)"</Emphasis>
            },
        )
        .note(markup! {
            "Qwik requires serializable closures for:"
            "\n- Resumability (pausing/resuming execution)"
            "\n- Code splitting (lazy loading components)"
            "\n- Optimized rehydration (client-side continuation)"
        })
        .note(markup! {
            "Wrap the expression with "<Emphasis>"$(...)"</Emphasis>" to make it serializable. "
            "Learn more: " <Hyperlink href="https://qwik.dev/docs/components/state/#use-methods">"Qwik documentation"</Hyperlink>"."
        })
        .into()
    }
}

fn is_wrapped_with_dollar(expr: &AnyJsExpression) -> bool {
    expr.syntax()
        .ancestors()
        .find_map(JsCallExpression::cast)
        .and_then(|call| call.callee().ok())
        .and_then(|callee| callee.as_js_reference_identifier())
        .and_then(|ident| ident.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == "$")
}
