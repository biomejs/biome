use std::num::IntErrorKind;
use std::ops::RangeInclusive;

use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;

use biome_js_syntax::numbers::split_into_radix_and_number;
use biome_js_syntax::JsNumberLiteralExpression;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow literal numbers that lose precision
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const x = 9007199254740993
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const x = 5.123000000000000000000000000001
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const x = 0x20000000000001
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const x = 0x2_000000000_0001;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const x = 12345
    /// const x = 123.456
    /// const x = 123e34
    /// const x = 12300000000000000000000000
    /// const x = 0x1FFFFFFFFFFFFF
    /// const x = 9007199254740991
    /// const x = 9007_1992547409_91
    /// ```
    ///
    pub NoPrecisionLoss {
        version: "1.0.0",
        name: "noPrecisionLoss",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-loss-of-precision"),
            RuleSource::EslintTypeScript("no-loss-of-precision"),
            RuleSource::Clippy("lossy_float_literal")
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoPrecisionLoss {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if is_precision_lost(node)? {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let value = node.as_number()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! { "This number literal will lose precision at runtime." },
            )
            .note(markup! { "The value at runtime will be "<Emphasis>{ value.to_string() }</Emphasis> }),
        )
    }
}

fn is_precision_lost(node: &JsNumberLiteralExpression) -> Option<bool> {
    let token = node.value_token().ok()?;
    let num = token.text_trimmed();

    let (radix, num) = split_into_radix_and_number(num);
    if radix == 10 {
        is_precision_lost_in_base_10(&num)
    } else {
        Some(is_precision_lost_in_base_other(&num, radix))
    }
}

fn is_precision_lost_in_base_10(num: &str) -> Option<bool> {
    const MAX_SIGNIFICANT_DIGITS_BASE10: u32 = 17;
    let normalized = NormalizedNumber::new(num);
    let precision = normalized.precision();
    if precision == 0 {
        return Some(false);
    }
    if precision > MAX_SIGNIFICANT_DIGITS_BASE10 as usize {
        return Some(true);
    }
    let parsed = num.parse::<f64>().ok()?;
    let stored_num = format!("{:.*e}", precision - 1, parsed);
    Some(stored_num != normalized.to_scientific())
}

fn is_precision_lost_in_base_other(num: &str, radix: u8) -> bool {
    let parsed = match i64::from_str_radix(num, radix as u32) {
        Ok(x) => x,
        Err(e) => {
            return matches!(
                e.kind(),
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow
            )
        }
    };

    const MAX_SAFE_INTEGER: i64 = 2_i64.pow(53) - 1;
    const MIN_SAFE_INTEGER: i64 = -MAX_SAFE_INTEGER;
    const SAFE_RANGE: RangeInclusive<i64> = MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER;

    !SAFE_RANGE.contains(&parsed)
}

fn remove_leading_zeros(num: &str) -> &str {
    num.trim_start_matches('0')
}

fn remove_trailing_zeros(num: &str) -> &str {
    num.trim_end_matches('0')
}

#[derive(Debug)]
/// Normalized number in the form `0.{digits}{digits_rest}e{exponent}`
struct NormalizedNumber<'a> {
    digits: &'a str,
    digits_rest: &'a str,
    exponent: isize,
}

impl NormalizedNumber<'_> {
    fn new(num: &str) -> NormalizedNumber<'_> {
        let num = remove_leading_zeros(num);
        let (mantissa, exponent) = num
            .split_once(['e', 'E'])
            .and_then(|(mantissa, exponent)| Some((mantissa, exponent.parse::<isize>().ok()?)))
            .unwrap_or((num, 0));
        match mantissa.split_once(['.']) {
            None => NormalizedNumber {
                digits: remove_trailing_zeros(mantissa),
                digits_rest: "",
                exponent: exponent + mantissa.len() as isize,
            },
            Some(("", fraction)) => {
                let digits = remove_leading_zeros(fraction);
                NormalizedNumber {
                    digits: remove_trailing_zeros(digits),
                    digits_rest: "",
                    exponent: digits.len() as isize - fraction.len() as isize + exponent,
                }
            }
            Some((integer, fraction)) => {
                let fraction = remove_trailing_zeros(fraction);
                let digits = if fraction.is_empty() {
                    remove_trailing_zeros(integer)
                } else {
                    integer
                };
                NormalizedNumber {
                    digits,
                    digits_rest: fraction,
                    exponent: exponent + integer.len() as isize,
                }
            }
        }
    }

    fn to_scientific(&self) -> String {
        let fraction = &self.digits[1..];
        if fraction.is_empty() && self.digits_rest.is_empty() {
            format!("{}e{}", self.digits, self.exponent - 1)
        } else {
            format!(
                "{}.{}{}e{}",
                &self.digits[..1],
                fraction,
                self.digits_rest,
                self.exponent - 1
            )
        }
    }

    fn precision(&self) -> usize {
        self.digits.len() + self.digits_rest.len()
    }
}
