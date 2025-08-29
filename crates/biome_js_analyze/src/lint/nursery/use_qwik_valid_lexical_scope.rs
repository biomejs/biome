use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsVariableStatement};
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
    /// const print = (msg: string) => {
    ///   console.log(msg);
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const print = $((msg) => {
    ///   console.log(msg);
    /// });
    /// ```
    ///
 pub UseQwikValidLexicalScope {
        version: "next",
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
            declarator
                .ok()
                .and_then(|d| d.initializer())
                .and_then(|init| init.expression().ok())
                .filter(|expr| matches!(expr, AnyJsExpression::JsArrowFunctionExpression(_)))
                .filter(|expr| !is_wrapped_with_dollar(expr))
                .map(|expr| expr.range());
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
    match expr {
        AnyJsExpression::JsCallExpression(call) => call
            .callee()
            .ok()
            .and_then(|callee| callee.as_js_identifier_expression().cloned())
            .and_then(|ident| ident.name().ok())
            .and_then(|name| name.value_token().ok())
            .is_some_and(|token| token.text() == "$"),
        _ => false,
    }
}
