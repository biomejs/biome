use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsBinaryExpression, JsSyntaxKind, JsSyntaxToken};
use biome_rowan::BatchMutationExt;
use biome_rule_options::no_xor_as_exponentiation::NoXorAsExponentiationOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

declare_lint_rule! {
    /// Disallow the bitwise XOR operator where exponentiation was likely intended.
    ///
    /// In JavaScript, `^` is the bitwise XOR operator, not exponentiation.
    /// Developers coming from languages like Lua, Julia, R, or MATLAB, or from
    /// math notation, often expect `^` to mean "to the power of", so `2 ^ 32`
    /// silently evaluates to `34` instead of `4294967296`. The actual
    /// exponentiation operator is `**`.
    ///
    /// This rule flags `^` between two decimal integer literals, which is
    /// almost always this mistake. Hexadecimal, octal, and binary literals
    /// (such as `0xFF ^ 8`) and any non-literal operands (such as
    /// `flags ^ MASK`) are ignored, since those are far more likely to be
    /// intentional bitwise XOR.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const kibibyte = 2 ^ 10; // 8, not 1024
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const cube = 3 ^ 3; // 0, not 27
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const kibibyte = 2 ** 10;
    /// const cube = 3 ** 3;
    /// const masked = flags ^ MASK;
    /// const bits = 0xFF ^ 8;
    /// ```
    ///
    pub NoXorAsExponentiation {
        version: "next",
        name: "noXorAsExponentiation",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-xor-as-exponentiation").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoXorAsExponentiation {
    type Query = Ast<JsBinaryExpression>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoXorAsExponentiationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let operator = node.operator_token().ok()?;
        if operator.kind() != JsSyntaxKind::CARET {
            return None;
        }
        let left = is_decimal_integer_literal(node.left().ok()?)?;
        let right = is_decimal_integer_literal(node.right().ok()?)?;
        (left && right).then_some(operator)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, operator: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                operator.text_trimmed_range(),
                markup! {
                    "This bitwise XOR operator "<Emphasis>"^"</Emphasis>" is used between two integer literals."
                },
            )
            .note(markup! {
                "In JavaScript, "<Emphasis>"^"</Emphasis>" is the bitwise XOR operator, not exponentiation. "
                "The exponentiation operator is "<Emphasis>"**"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, operator: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.replace_token(operator.clone(), make::token(JsSyntaxKind::STAR2));
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>"^"</Emphasis>" with "<Emphasis>"**"</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Returns whether a number literal is a decimal integer, ignoring any
/// surrounding parentheses. Returns `None` for anything that is not a number
/// literal.
fn is_decimal_integer_literal(expression: AnyJsExpression) -> Option<bool> {
    let expression = expression.omit_parentheses();
    let token = expression
        .as_any_js_literal_expression()?
        .as_js_number_literal_expression()?
        .value_token()
        .ok()?;
    Some(is_decimal_integer(token.text_trimmed()))
}

/// Returns `true` for plain decimal integers such as `2`, `10`, or `1_000`.
///
/// Hex, binary, and octal literals are how people write bit masks, so `^`
/// next to them is intentional. Floats, exponents, and `BigInt`s rarely show
/// up in the `2 ^ 10` mistake, so they are skipped too.
fn is_decimal_integer(text: &str) -> bool {
    let mut bytes = text.bytes();
    match bytes.next().map(lookup_byte) {
        Some(Dispatch::ZER) => bytes.next().is_none(),
        Some(Dispatch::DIG) => bytes
            .all(|byte| byte == b'_' || matches!(lookup_byte(byte), Dispatch::ZER | Dispatch::DIG)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::is_decimal_integer;

    #[test]
    fn decimal_integers() {
        for text in ["0", "2", "32", "10", "1_000", "1_0_0"] {
            assert!(
                is_decimal_integer(text),
                "{text} should be a decimal integer"
            );
        }
    }

    #[test]
    fn not_decimal_integers() {
        for text in [
            "", "00", "07", "08", "0777", "0xFF", "0b100", "0o20", "2.5", "2e3", "2n", "_1", ".5",
            "0.5",
        ] {
            assert!(
                !is_decimal_integer(text),
                "{text} should not be a decimal integer"
            );
        }
    }
}
