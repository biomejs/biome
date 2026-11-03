use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_factory::make;
use biome_css_syntax::{
    AnyCssDimension, AnyCssExpression, AnyCssFunctionName, AnyCssValue, CssFunction, CssNumber,
    CssPercentage, CssSyntaxKind::CSS_NUMBER_LITERAL, CssSyntaxToken, T,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, declare_node_union};
use biome_rule_options::use_consistent_lightness_notation::{
    LightnessNotation, UseConsistentLightnessNotationOptions,
};

use crate::CssRuleAction;

declare_lint_rule! {
    /// Enforce a consistent notation for the lightness of colors.
    ///
    /// The `lab()`, `lch()`, `oklab()`, and `oklch()` color functions accept their first value,
    /// the lightness, either as a percentage or as a number. Both notations describe the same
    /// color: `oklch(85% 0.17 88)` and `oklch(0.85 0.17 88)` are equal. Writing lightness the same
    /// way everywhere makes colors easier to read and compare.
    ///
    /// For `oklab()` and `oklch()`, the number `1` equals `100%`. For `lab()` and `lch()`, the
    /// number `100` equals `100%`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a { color: oklch(0.85 0.17 88); }
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// a { color: lab(86 -40 30); }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// a { color: oklch(85% 0.17 88); }
    /// a { color: oklab(86% -0.2 0.15); }
    /// a { color: lch(85% 60 88); }
    /// a { color: lab(86% -40 30); }
    /// a { color: oklch(from red 50% c h); }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `notation`
    ///
    /// Default: `"percentage"`
    ///
    /// Whether lightness is written as a percentage (`"percentage"`) or as a number (`"number"`).
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "notation": "number"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```css,expect_diagnostic,use_options
    /// a { color: oklch(85% 0.17 88); }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// a { color: oklch(0.85 0.17 88); }
    /// a { color: lab(86 -40 30); }
    /// ```
    ///
    pub UseConsistentLightnessNotation {
        version: "2.6.3",
        name: "useConsistentLightnessNotation",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("lightness-notation").same()],
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub AnyCssLightnessValue = CssNumber | CssPercentage
}

/// The range of numbers a color function accepts for lightness, which decides how a number
/// converts to and from a percentage.
#[derive(Clone, Copy, Debug)]
pub enum LightnessScale {
    /// `oklab()` and `oklch()`: the number `1` equals `100%`.
    ZeroToOne,
    /// `lab()` and `lch()`: the number `100` equals `100%`.
    ZeroToHundred,
}

impl Rule for UseConsistentLightnessNotation {
    type Query = Ast<CssFunction>;
    type State = (AnyCssLightnessValue, LightnessScale);
    type Signals = Option<Self::State>;
    type Options = UseConsistentLightnessNotationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();
        let AnyCssFunctionName::CssIdentifier(name) = function.name().ok()? else {
            return None;
        };
        let scale = LightnessScale::from_function_name(name.value_token().ok()?.text_trimmed())?;
        let AnyCssExpression::CssListOfComponentValuesExpression(expression) =
            function.items().first()?.ok()?
        else {
            return None;
        };

        let mut values = expression.css_component_value_list().iter();
        let first = values.next()?;
        // In relative color syntax, such as `oklch(from red l c h)`, the lightness follows the
        // `from` keyword and the origin color.
        let lightness = if is_from_keyword(&first) {
            values.nth(1)?
        } else {
            first
        };

        let lightness = match (ctx.options().notation(), lightness) {
            (LightnessNotation::Percentage, AnyCssValue::CssNumber(number)) => {
                AnyCssLightnessValue::CssNumber(number)
            }
            (
                LightnessNotation::Number,
                AnyCssValue::AnyCssDimension(AnyCssDimension::CssPercentage(percentage)),
            ) => AnyCssLightnessValue::CssPercentage(percentage),
            _ => return None,
        };
        Some((lightness, scale))
    }

    fn diagnostic(_: &RuleContext<Self>, (lightness, _): &Self::State) -> Option<RuleDiagnostic> {
        let message = match lightness {
            AnyCssLightnessValue::CssNumber(_) => markup! {
                "This lightness is written as a number instead of a percentage."
            },
            AnyCssLightnessValue::CssPercentage(_) => markup! {
                "This lightness is written as a percentage instead of a number."
            },
        };
        Some(
            RuleDiagnostic::new(rule_category!(), lightness.range(), message).note(markup! {
                "Writing lightness the same way in every color makes colors easier to read and compare."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, (lightness, scale): &Self::State) -> Option<CssRuleAction> {
        let mut mutation = ctx.root().begin();
        let replacement = match lightness {
            AnyCssLightnessValue::CssNumber(number) => {
                let literal = number.value_token().ok()?;
                let value = match scale {
                    LightnessScale::ZeroToOne => shift_decimal_point(literal.text_trimmed(), 2)?,
                    LightnessScale::ZeroToHundred => literal.text_trimmed().to_string(),
                };
                let percentage = make::css_percentage(
                    CssSyntaxToken::new_detached(CSS_NUMBER_LITERAL, &value, [], []),
                    CssSyntaxToken::new_detached(T![%], "%", [], []),
                );
                mutation.replace_node(
                    AnyCssValue::CssNumber(number.clone()),
                    AnyCssValue::AnyCssDimension(AnyCssDimension::CssPercentage(percentage)),
                );
                format!("{value}%")
            }
            AnyCssLightnessValue::CssPercentage(percentage) => {
                let literal = percentage.value_token().ok()?;
                let value = match scale {
                    LightnessScale::ZeroToOne => shift_decimal_point(literal.text_trimmed(), -2)?,
                    LightnessScale::ZeroToHundred => literal.text_trimmed().to_string(),
                };
                mutation.replace_node(
                    AnyCssValue::AnyCssDimension(AnyCssDimension::CssPercentage(
                        percentage.clone(),
                    )),
                    AnyCssValue::CssNumber(make::css_number(CssSyntaxToken::new_detached(
                        CSS_NUMBER_LITERAL,
                        &value,
                        [],
                        [],
                    ))),
                );
                value
            }
        };
        Some(CssRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{replacement}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

impl LightnessScale {
    fn from_function_name(name: &str) -> Option<Self> {
        if name.eq_ignore_ascii_case("oklch") || name.eq_ignore_ascii_case("oklab") {
            Some(Self::ZeroToOne)
        } else if name.eq_ignore_ascii_case("lch") || name.eq_ignore_ascii_case("lab") {
            Some(Self::ZeroToHundred)
        } else {
            None
        }
    }
}

fn is_from_keyword(value: &AnyCssValue) -> bool {
    let AnyCssValue::CssIdentifier(identifier) = value else {
        return false;
    };
    identifier
        .value_token()
        .is_ok_and(|token| token.text_trimmed().eq_ignore_ascii_case("from"))
}

/// Moves the decimal point of a number literal `shift` places to the right, or to the left when
/// `shift` is negative, without going through floating-point arithmetic.
///
/// The result has no redundant leading or trailing zeros, for example `0.85` shifted by `2` is
/// `85`, and `5` shifted by `-2` is `0.05`. Returns `None` for literals written with an exponent,
/// such as `1e-1`.
fn shift_decimal_point(literal: &str, shift: isize) -> Option<String> {
    if literal.contains(['e', 'E']) {
        return None;
    }
    let (sign, unsigned) = match literal.as_bytes().first()? {
        b'+' | b'-' => literal.split_at(1),
        _ => ("", literal),
    };
    let (integer, fraction) = unsigned.split_once('.').unwrap_or((unsigned, ""));

    let point = integer.len() as isize + shift;
    let leading_zeros = (-point).max(0) as usize;
    let trailing_zeros = (point - (integer.len() + fraction.len()) as isize).max(0) as usize;
    let digits = format!(
        "{}{integer}{fraction}{}",
        "0".repeat(leading_zeros),
        "0".repeat(trailing_zeros)
    );
    let (integer, fraction) = digits.split_at(point.max(0) as usize);
    let integer = integer.trim_start_matches('0');
    let fraction = fraction.trim_end_matches('0');

    let mut shifted = String::from(sign);
    shifted.push_str(if integer.is_empty() { "0" } else { integer });
    if !fraction.is_empty() {
        shifted.push('.');
        shifted.push_str(fraction);
    }
    Some(shifted)
}

#[cfg(test)]
mod tests {
    use super::shift_decimal_point;

    #[test]
    fn shifts_decimal_point() {
        let cases = [
            ("0.85", 2, "85"),
            ("0.855", 2, "85.5"),
            ("1", 2, "100"),
            ("0", 2, "0"),
            (".5", 2, "50"),
            ("0.005", 2, "0.5"),
            ("-0.1", 2, "-10"),
            ("+0.1", 2, "+10"),
            ("85", -2, "0.85"),
            ("100", -2, "1"),
            ("5", -2, "0.05"),
            ("8.5", -2, "0.085"),
            ("0", -2, "0"),
            ("85.50", -2, "0.855"),
        ];
        for (literal, shift, expected) in cases {
            assert_eq!(
                shift_decimal_point(literal, shift).as_deref(),
                Some(expected),
                "shifting {literal} by {shift}"
            );
        }
        assert_eq!(shift_decimal_point("1e-1", 2), None);
    }
}
