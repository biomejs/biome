use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::JsStringLiteralExpression;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow template literal placeholder syntax in regular strings.
    ///
    /// ECMAScript 6 allows programmers to create strings containing variable or expressions using template literals, 
    /// instead of string concatenation, by writing expressions like ${variable} between two backtick quotes (`). 
    /// It can be easy to use the wrong quotes when wanting to use template literals, by writing "${variable}", 
    /// and end up with the literal value "${variable}" instead of a string containing the value of the injected expressions.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// "Hello ${name}!";
    /// 'Hello ${name}!';
    /// "Time: ${12 * 60 * 60 * 1000}";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// `Hello ${name}!`;
    /// `Time: ${12 * 60 * 60 * 1000}`;
    ///
    /// templateFunction`Hello ${name}`;
    /// ```
    ///
    pub NoTemplateCurlyInString {
        version: "next",
        name: "noTemplateCurlyInString",
        language: "js",
        sources: &[RuleSource::Eslint("no-template-curly-in-string")],
        recommended: false,
    }
}

impl Rule for NoTemplateCurlyInString {
    type Query = Ast<JsStringLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let text = token.text();

        let bytes = text.as_bytes();
        let mut byte_iter = bytes.iter();

        while let Some(&byte) = byte_iter.next() {
            if byte == b'$' {
                if let Some(&next_byte) = byte_iter.next() {
                    if next_byte == b'{' {
                        for &inner_byte in byte_iter.by_ref() {
                            if inner_byte == b'}' {
                                return Some(());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected template string expression."
                },
            ),
        )
    }
}
