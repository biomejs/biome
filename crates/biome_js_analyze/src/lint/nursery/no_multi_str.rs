use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsStringLiteralExpression;
use biome_rowan::AstNode;
use biome_rule_options::no_multi_str::NoMultiStrOptions;

declare_lint_rule! {
    /// Disallow creating multiline strings by escaping newlines.
    ///
    /// Escaping newlines to create multiline strings is discouraged because it
    /// can lead to subtle errors caused by unexpected whitespace after the
    /// backslash.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo =
    ///     "Line 1\n\
    /// Line 2";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = "Line 1\nLine 2";
    /// ```
    ///
    /// ```js
    /// const bar = `Line 1
    /// Line 2`;
    /// ```
    pub NoMultiStr {
        version: "2.3.8",
        name: "noMultiStr",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-multi-str").same()],
        severity: Severity::Error,
    }
}

impl Rule for NoMultiStr {
    type Query = Ast<JsStringLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoMultiStrOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let text = node.value_token().ok()?.token_text_trimmed();

        if text.contains("\\\n") || text.contains("\\\r\n") {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Escaping newlines to create multiline strings is disallowed."
            },
        ))
    }
}
