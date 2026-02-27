use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;

use biome_js_syntax::JsNumberLiteralExpression;
use biome_js_syntax::numbers::split_into_radix_and_number;
use biome_rowan::AstNode;
use biome_rule_options::no_precision_loss::NoPrecisionLossOptions;

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
            RuleSource::Eslint("no-loss-of-precision").same(),
            RuleSource::EslintTypeScript("no-loss-of-precision").same(),
            RuleSource::Clippy("lossy_float_literal").same(),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoPrecisionLoss {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoPrecisionLossOptions;

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
    // radix is passed down from split_into_radix_and_number which guarantees
    // that radix is 2, 8, 16. We've already filtered out the 10 case.
    let bits_per_digit = match radix {
        16 => 4,
        8 => 3,
        2 => 1,
        // Shouldn't ever happen
        _ => return false,
    };

    // We want to find the positions of the last set bit and the first set bit.
    // The distance between them (max - min + 1) is the number of significant bits.
    // If this distance > 53, the number cannot be exactly represented in an f64 (which has 53 bits of significand).
    let mut min_bit_index: Option<u32> = None;
    let mut current_bit_index: u32 = 0;

    // Iterate over digits in reverse order (from last to first digit)
    for c in num.chars().rev() {
        let digit = match c.to_digit(radix as u32) {
            Some(d) => d,
            None => return false,
        };

        if digit != 0 {
            if min_bit_index.is_none() {
                // Found the first non-zero digit (contains the first set bit of the number)
                let trailing_zeros = digit.trailing_zeros();
                min_bit_index = Some(current_bit_index + trailing_zeros);
            }

            // Calculate the last set bit for the current digit
            let last_bit_in_digit = (u32::BITS - digit.leading_zeros()) - 1;
            let max_bit_index = current_bit_index + last_bit_in_digit;

            // Check for overflow (exponent > 1023)
            // In IEEE 754 double precision:
            // - The exponent bias is 1023.
            // - The maximum valid exponent is 1023 (representing 2^1023).
            // - 2^1024 overflows to Infinity.
            // Thus, if the last set bit is at index 1024 or greater, the number overflows.
            if max_bit_index >= 1024 {
                return true;
            }

            // Check for precision loss
            // In IEEE 754 double precision:
            // - The significand (mantissa) has 53 bits of precision (52 stored bits + 1 implicit leading bit).
            // - If the distance between the last set bit and the first set bit
            //   exceeds 53 bits, the number cannot be exactly represented, as the lower bits would be truncated.
            // Span = max - min + 1
            // We know min_bit_index is Some because we set it above if it was None
            if let Some(min) = min_bit_index
                && max_bit_index - min + 1 > 53
            {
                return true;
            }
        }

        current_bit_index += bits_per_digit;
    }

    false
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
