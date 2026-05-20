use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsStringLiteralExpression;
use biome_rowan::{TextRange, TextSize};
use biome_rule_options::no_template_curly_in_string::NoTemplateCurlyInStringOptions;

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
    /// GitHub Actions expressions using double curly braces are also valid:
    ///
    /// ```js
    /// const a = "${{ inputs.abc }}";
    /// ```
    ///
    pub NoTemplateCurlyInString {
        version: "1.9.3",
        name: "noTemplateCurlyInString",
        language: "js",
        sources: &[RuleSource::Eslint("no-template-curly-in-string").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoTemplateCurlyInString {
    type Query = Ast<JsStringLiteralExpression>;
    type State = (u32, u32);
    type Signals = Option<Self::State>;
    type Options = NoTemplateCurlyInStringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let text = token.text_trimmed();

        let mut iter = text.bytes().enumerate().peekable();

        while let Some((i, byte)) = iter.next() {
            if byte != b'$' {
                continue;
            }
            if iter.next_if(|(_, b)| *b == b'{').is_none() {
                continue;
            }

            // Check for GitHub Actions syntax: ${{ ... }}
            if iter.next_if(|(_, b)| *b == b'{').is_some() {
                // Scan for closing }} sequence, tracking first } position
                let mut first_close_pos = None;
                let mut prev_was_close = false;
                let mut found_double_close = false;
                for (j, b) in iter.by_ref() {
                    if b == b'}' {
                        if prev_was_close {
                            // Found }}, valid GitHub Actions expression
                            found_double_close = true;
                            break;
                        }
                        first_close_pos.get_or_insert(j);
                        prev_was_close = true;
                    } else {
                        prev_was_close = false;
                    }
                }
                if found_double_close {
                    continue;
                }
                // No }} found - if we saw any }, flag as regular template
                if let Some(j) = first_close_pos {
                    return Some((i as u32, (j + 1) as u32));
                }
                return None;
            }

            // Regular ${...} pattern - find closing brace
            for (j, inner_byte) in iter.by_ref() {
                if inner_byte == b'}' {
                    return Some((i as u32, (j + 1) as u32));
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
