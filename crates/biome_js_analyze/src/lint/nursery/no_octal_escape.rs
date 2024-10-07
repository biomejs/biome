use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::JsStringLiteralExpression;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow octal escape sequences in string literals
    ///
    /// As of the ECMAScript 5 specification, octal escape sequences in string literals are deprecated and should not be used.
    /// Unicode escape sequences should be used instead.
    ///
    /// ### Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var foo = "Copyright \251";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var foo = "Copyright \u00A9";   // unicode
    ///
    /// var foo = "Copyright \xA9";     // hexadecimal
    /// ```
    ///
    pub NoOctalEscape {
        version: "1.9.3",
        name: "noOctalEscape",
        language: "js",
        sources: &[RuleSource::Eslint("no-octal-escape")],
        recommended: false,
    }
}

impl Rule for NoOctalEscape {
    type Query = Ast<JsStringLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let text = token.text();

        let bytes = text.as_bytes();
        let mut byte_it = bytes.iter();
        while let Some(&byte) = byte_it.next() {
            if byte == b'\\' {
                if let Some(&next_byte) = byte_it.next() {
                    if (b'0'..=b'7').contains(&next_byte) {
                        return Some(());
                    }
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let text = token.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Don't use "<Emphasis>"octal"</Emphasis>
                },
            )
            .note(markup! {
                "Don't use octal escape sequences: " {text}
            })
            .note(markup! {
                "Use unicode or hexidecimal escape sequences instead."
            }),
        )
    }
}
