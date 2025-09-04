use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsSyntaxToken;
use biome_rowan::{BatchMutationExt, TextRange};
use biome_rule_options::no_octal_escape::NoOctalEscapeOptions;

use crate::{JsRuleAction, lint::correctness::no_nonoctal_decimal_escape::AnyJsStringLiteral};

declare_lint_rule! {
    /// Disallow octal escape sequences in string literals
    ///
    /// As of the ECMAScript 5 specification, octal escape sequences in string literals are deprecated and should not be used.
    /// Unicode escape sequences should be used instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = "Copyright \251";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = "Copyright \u00A9"; // unicode escape
    /// const bar = "Copyright \xA9"; // hexadecimal escape
    /// ```
    ///
    pub NoOctalEscape {
        version: "1.9.3",
        name: "noOctalEscape",
        language: "js",
        sources: &[RuleSource::Eslint("no-octal-escape").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoOctalEscape {
    type Query = Ast<AnyJsStringLiteral>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoOctalEscapeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = ctx.query().string_literal_token()?;
        let mut it = token.text_trimmed().bytes().enumerate();
        while let Some((index, byte)) = it.next() {
            if byte == b'\\'
                && let Some((_, byte)) = it.next()
                && matches!(byte, b'0'..=b'7')
            {
                let len = 2 + it
                    .clone()
                    .take(5)
                    .take_while(|(_, byte)| matches!(byte, b'0'..=b'7'))
                    .count();
                // Ignore the non-deprecated `\0`
                if byte != b'0' || len > 2 {
                    return Some(RuleState { index, len });
                }
            }
        }
        None
    }

    fn diagnostic(
        ctx: &RuleContext<Self>,
        RuleState { index, len }: &Self::State,
    ) -> Option<RuleDiagnostic> {
        let token = ctx.query().string_literal_token()?;
        let escape_start = token
            .text_trimmed_range()
            .start()
            .checked_add((*index as u32).into())?;
        Some(RuleDiagnostic::new(
            rule_category!(),
            TextRange::at(escape_start, (*len as u32).into()),
            markup! {
                "Don't use deprecated "<Emphasis>"octal escape sequences"</Emphasis>"."
            },
        ))
    }

    fn action(
        ctx: &RuleContext<Self>,
        RuleState { index, len }: &Self::State,
    ) -> Option<JsRuleAction> {
        let token = ctx.query().string_literal_token()?;
        let text = token.text_trimmed();
        let octal = &text[(index + 1)..(index + len)];
        let codepoint = u32::from_str_radix(octal, 8).ok()?;
        let before_octal = &text[..(index + 1)];
        let after_octal = &text[(index + len)..];
        let (new_text, unicode_or_hexa) = if codepoint <= 0xff {
            (
                format!("{before_octal}x{codepoint:02x}{after_octal}"),
                "hexadecimal",
            )
        } else {
            (
                format!("{before_octal}u{codepoint:04x}{after_octal}"),
                "unicode",
            )
        };
        let new_token = JsSyntaxToken::new_detached(token.kind(), &new_text, [], []);
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(token.clone(), new_token);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "{unicode_or_hexa}" escape sequences instead." }.to_owned(),
            mutation,
        ))
    }
}

pub struct RuleState {
    // Index of the escape sequence (starts with `\`)
    index: usize,
    // Length of the escape sequence
    len: usize,
}
