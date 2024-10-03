use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::JsStringLiteralExpression;
use biome_rowan::{TextRange, TextSize};

declare_lint_rule! {
    /// Disallow template literal placeholder syntax in regular strings.
    ///
    /// ECMAScript 6 allows programmers to create strings containing variable or expressions using template literals,
    /// instead of string concatenation, by writing expressions like `${variable}` between two backtick quotes (\`).
    /// It can be easy to use the wrong quotes when wanting to use template literals, by writing `"${variable}"`,
    /// and end up with the literal value `"${variable}"` instead of a string containing the value of the injected expressions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = "Hello ${name}!";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 'Hello ${name}!';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = "Time: ${12 * 60 * 60 * 1000}";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = `Hello ${name}!`;
    /// const a = `Time: ${12 * 60 * 60 * 1000}`;
    ///
    /// const a = templateFunction`Hello ${name}`;
    /// ```
    ///
    pub NoTemplateCurlyInString {
        version: "1.9.3",
        name: "noTemplateCurlyInString",
        language: "js",
        sources: &[RuleSource::Eslint("no-template-curly-in-string")],
        recommended: false,
    }
}

impl Rule for NoTemplateCurlyInString {
    type Query = Ast<JsStringLiteralExpression>;
    type State = (u32, u32);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let text = token.text();

        let mut byte_iter = text.bytes().enumerate();
        while let Some((i, byte)) = byte_iter.next() {
            if byte == b'$' {
                if let Some((_, b'{')) = byte_iter.next() {
                    for (j, inner_byte) in byte_iter.by_ref() {
                        if inner_byte == b'}' {
                            return Some((i as u32, (j + 1) as u32));
                        }
                    }
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let value_token = ctx.query().value_token().ok()?;
        let value_token_range = value_token.text_trimmed_range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                TextRange::new(
                    value_token_range.start() + TextSize::from(range.0),
                    value_token_range.start() + TextSize::from(range.1),
                ),
                markup! {
                    "Unexpected template string placeholder."
                },
            )
            .note(markup! {
                "Turn the string into a template string."
            }),
        )
    }
}
