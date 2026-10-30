use biome_analyze::RuleSource;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsRoot, JsSyntaxNode};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_irregular_whitespace::NoIrregularWhitespaceOptions;

declare_lint_rule! {
    /// Disallows the use of irregular whitespace characters.
    ///
    /// Invalid or irregular whitespace causes issues with various parsers and also makes code harder to debug.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// letcount;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let foo;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const count = 1;
    /// ```
    ///
    /// ```js
    /// const foo = '';
    /// ```
    ///
    pub NoIrregularWhitespace {
        version: "1.9.0",
        name: "noIrregularWhitespace",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-irregular-whitespace").same(),
            RuleSource::EslintVueJs("no-irregular-whitespace").same(),
            RuleSource::DenoLint("no-irregular-whitespace").same(),
        ],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<AnyJsRoot>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoIrregularWhitespaceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        get_irregular_whitespace(ctx.query().syntax()).into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Irregular whitespaces found."
                },
            )
            .note(markup!{
                "Irregular whitespaces can cause issues to other parsers, and make the code harder to debug."
            })
            .note(markup! {
                "Replace the irregular whitespaces with normal whitespaces or tabs."
            }),
        )
    }
}

fn is_irregular_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{000B}'
            | '\u{000C}'
            | '\u{0085}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{180E}'
            | '\u{2000}'..='\u{200B}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
            | '\u{FEFF}'
    )
}

/// Returns `true` for every byte that starts the UTF-8 encoding of a character
/// matched by [is_irregular_whitespace], so text without such a byte contains
/// no irregular whitespace.
///
/// Other characters start with the same bytes, such as `©` and Japanese kana.
/// A match only means that the text has to be decoded to find out.
fn is_irregular_whitespace_lead_byte(byte: u8) -> bool {
    matches!(byte, 0x0B | 0x0C | 0xC2 | 0xE1..=0xE3 | 0xEF)
}

fn get_irregular_whitespace(syntax: &JsSyntaxNode) -> Vec<TextRange> {
    let mut results = vec![];
    for token in syntax.borrowed_tokens() {
        // Nearly every token is rejected here, before looking at its trivia.
        if !token.text().bytes().any(is_irregular_whitespace_lead_byte) {
            continue;
        }

        for trivia in token
            .leading_trivia_pieces()
            .chain(token.trailing_trivia_pieces())
        {
            if trivia.kind().is_whitespace() && trivia.text().chars().any(is_irregular_whitespace) {
                results.push(trivia.text_range());
            }
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::{is_irregular_whitespace, is_irregular_whitespace_lead_byte};

    #[test]
    fn lead_bytes_cover_every_irregular_whitespace() {
        let mut buffer = [0; 4];
        let mut count = 0;
        for c in (char::MIN..=char::MAX).filter(|c| is_irregular_whitespace(*c)) {
            let lead_byte = c.encode_utf8(&mut buffer).as_bytes()[0];
            assert!(is_irregular_whitespace_lead_byte(lead_byte), "{c:?}");
            count += 1;
        }
        assert_eq!(count, 22);
    }

    #[test]
    fn lead_bytes_reject_regular_ascii_and_continuation_bytes() {
        for byte in (0x00..=0xBF).filter(|byte| !matches!(byte, 0x0B | 0x0C)) {
            assert!(!is_irregular_whitespace_lead_byte(byte), "{byte:#04x}");
        }
    }
}
