use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_syntax::{CssString, CssSyntaxToken};
use biome_diagnostics::Severity;
use biome_rowan::{BatchMutationExt, TextRange};
use biome_rule_options::no_useless_escape_in_string::NoUselessEscapeInStringOptions;

use crate::CssRuleAction;

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
    /// ```css,expect_diagnostic
    /// a::after {
    ///   content: "\a"
    /// }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a::after {
    ///   content: "\'"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a::after {
    ///   content: "\""
    /// }
    /// ```
    ///
    /// ```css
    /// a::after {
    ///   content: "\n"
    /// }
    /// ```
    ///
    pub NoUselessEscapeInString {
        version: "2.0.0",
        name: "noUselessEscapeInString",
        language: "css",
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessEscapeInString {
    type Query = Ast<CssString>;
    type State = (CssSyntaxToken, usize);
    type Signals = Option<Self::State>;
    type Options = NoUselessEscapeInStringOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let text = token.text_trimmed();
        next_useless_escape(text, text.bytes().next()?).map(|index| (token, index))
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

    fn action(ctx: &RuleContext<Self>, (token, index): &Self::State) -> Option<CssRuleAction> {
        let mut new_text = token.text_trimmed().to_string();
        new_text.remove(*index);
        let new_token = CssSyntaxToken::new_detached(token.kind(), &new_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), new_token);
        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Unescape the character." }.to_owned(),
            mutation,
        ))
    }
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
