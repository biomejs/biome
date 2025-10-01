use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsTemplateElement, JsLiteralMemberName, JsStringLiteralExpression, JsSyntaxKind,
    JsSyntaxToken, JsTemplateExpression,
};
use biome_rowan::{BatchMutationExt, TextRange, declare_node_union};
use biome_rule_options::no_useless_escape_in_string::NoUselessEscapeInStringOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unnecessary escapes in string literals.
    ///
    /// Escaping non-special characters in string literals doesn't have any effect.
    /// Hence, they may confuse a reader.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const s = "\a";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const o = {
    ///     "\a": 0,
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const s = `${0}\a`;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const s = "\n";
    /// ```
    ///
    /// In template literals, `\${` and `$\{` are valid escapes:
    /// ```js
    /// const s = `\${0}`;
    /// ```
    ///
    /// Tagged string templates are ignored:
    ///
    /// ```js
    /// const s = tagged`\a`;
    /// ```
    ///
    /// JSX strings are ignored:
    ///
    /// ```jsx
    /// <div attr="str\a"/>;
    /// ```
    ///
    pub NoUselessEscapeInString {
        version: "2.0.0",
        name: "noUselessEscapeInString",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessEscapeInString {
    type Query = Ast<AnyJsString>;
    type State = (JsSyntaxToken, usize);
    type Signals = Option<Self::State>;
    type Options = NoUselessEscapeInStringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            AnyJsString::JsStringLiteralExpression(literal) => {
                let token = literal.value_token().ok()?;
                let text = token.text_trimmed();
                next_useless_escape(text, text.bytes().next()?).map(|index| (token, index))
            }
            AnyJsString::JsTemplateExpression(template) => {
                if template.tag().is_some() {
                    return None;
                }
                for element in template.elements() {
                    match element {
                        AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                            let Ok(chunk) = chunk.template_chunk_token() else {
                                continue;
                            };
                            if let Some(index) = next_useless_escape(chunk.text_trimmed(), b'`') {
                                return Some((chunk, index));
                            }
                        }
                        AnyJsTemplateElement::JsTemplateElement(_) => {}
                    }
                }
                None
            }
            AnyJsString::JsLiteralMemberName(member_name) => {
                let Ok(token) = member_name.value() else {
                    return None;
                };
                if token.kind() == JsSyntaxKind::JS_STRING_LITERAL {
                    let text = token.text_trimmed();
                    next_useless_escape(text, text.bytes().next()?).map(|index| (token, index))
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, (token, index): &Self::State) -> Option<RuleDiagnostic> {
        let escape_start = token
            .text_trimmed_range()
            .start()
            .checked_add((*index as u32 + 1).into())?;
        let escaped_char = token.text_trimmed()[(1 + index)..].chars().next()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                TextRange::at(escape_start, (escaped_char.len_utf8() as u32).into()),
                "The character doesn't need to be escaped.",
            )
            .note("Only quotes that enclose the string and special characters need to be escaped."),
        )
    }

    fn action(ctx: &RuleContext<Self>, (token, index): &Self::State) -> Option<JsRuleAction> {
        let mut new_text = token.text_trimmed().to_string();
        new_text.remove(*index);
        let new_token = JsSyntaxToken::new_detached(token.kind(), &new_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), new_token);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Unescape the character." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    /// Any string literal excluding JsxString.
    pub AnyJsString = JsStringLiteralExpression | JsTemplateExpression | JsLiteralMemberName
}

/// Returns the index in `str` of the first useless escape.
fn next_useless_escape(str: &str, quote: u8) -> Option<usize> {
    let mut it = str.bytes().enumerate();
    while let Some((i, c)) = it.next() {
        if c == b'\\'
            && let Some((_, c)) = it.next()
        {
            match c {
                // Meaningful escaped character
                b'^'
                | b'\r'
                | b'\n'
                | b'0'..=b'7'
                | b'\\'
                | b'b'
                | b'f'
                | b'n'
                | b'r'
                | b't'
                | b'u'
                | b'v'
                | b'x' => {}
                // In template literals, \${ is a valid escape for producing a literal ${
                b'$' => {
                    // Clone iterator to peek ahead without advancing, so other escapes like \${\a aren't missed
                    if !(quote == b'`' && (matches!(it.clone().next(), Some((_, b'{'))))) {
                        return Some(i);
                    }
                }
                // Check the \{ sequence. This \ is only a valid escape in template literals
                // when the preceding character is $ (i.e., `$\{`)
                b'{' => {
                    // Check for template literals and look backward for $
                    if !(quote == b'`' && i > 0 && str.as_bytes()[i - 1] == b'$') {
                        return Some(i);
                    }
                }
                // Preserve escaping of Unicode characters U+2028 and U+2029
                0xE2 => {
                    if !(matches!(it.next(), Some((_, 0x80)))
                        && matches!(it.next(), Some((_, 0xA8 | 0xA9))))
                    {
                        return Some(i);
                    }
                }
                _ => {
                    // The quote can be escaped
                    if c != quote {
                        return Some(i);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_useless_escape() {
        assert_eq!(next_useless_escape(r"\n", b'"'), None);
        assert_eq!(next_useless_escape(r"\'", b'"'), Some(0));

        assert_eq!(next_useless_escape("\\\u{2027}", b'"'), Some(0));
        assert_eq!(next_useless_escape("\\\u{2028}", b'"'), None);
        assert_eq!(next_useless_escape("\\\u{2029}", b'"'), None);
        assert_eq!(next_useless_escape("\\\u{2030}", b'"'), Some(0));
    }
}
