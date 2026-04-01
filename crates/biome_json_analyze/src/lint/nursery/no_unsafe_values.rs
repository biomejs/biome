use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{MarkupBuf, markup};
use biome_deserialize::json::unescape_json_string;
use biome_json_syntax::{JsonNumberValue, JsonStringValue};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_unsafe_values::NoUnsafeValuesOptions;
use biome_string_case::StrOnlyExtension;
use regex::Regex;

declare_lint_rule! {
    /// Disallow JSON values that are unsafe for interchange
    ///
    /// JSON is widely used for data interchange between systems, but certain values can cause interoperability issues when transferred between different parsers and environments. The common unsafe values are:
    ///
    /// - *Lone surrogates in strings*: Incomplete Unicode character pairs that can cause encoding/decoding failures
    /// - *Numbers that evaluate to Infinity*: Values like `1e400` that exceed JavaScript's number range
    /// - *Unintentional zeros*: Very small numbers (e.g., `1e-400`) that silently evaluate to zero due to precision limitations
    /// - *Unsafe integers*: Numbers outside JavaScript's safe integer range (`±2^53-1`) that lose precision
    /// - *Subnormal numbers*: Very small floating point values that may be handled differently across systems
    ///
    /// These issues can lead to data corruption, silent failures, or inconsistent behavior across different platforms and languages.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// [
    /// 	2e308, // Number evaluating to Infinity
    ///
    /// 	-2e308, // Number evaluating to -Infinity
    ///
    /// 	"\ud83d", // String with lone surrogate
    ///
    /// 	1e-400, // Unsafe zero (too small, will evaluate to 0)
    ///
    /// 	9007199254740992, // Unsafe integer (outside safe integer range)
    ///
    /// 	2.2250738585072009e-308, // Subnormal number
    /// ]
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// [
    /// 	123,
    /// 	1234,
    /// 	12345, // Regular numbers within safe range
    ///
    /// 	"🔥", // Properly formed Unicode character (fire emoji)
    ///
    /// 	"\ud83d\udd25", // Same character with proper surrogate pair
    ///
    /// 	0.00000,
    /// 	0e0000000,
    /// 	0.00000e0000 // Zero represented in different valid ways
    /// ]
    /// ```
    ///
    pub NoUnsafeValues {
        version: "next",
        name: "noUnsafeValues",
        language: "json",
        recommended: false,
        sources: &[RuleSource::EslintJson("no-unsafe-values").same()],
    }
}

static MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_990;
static MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_990;

impl Rule for NoUnsafeValues {
    type Query = Ast<NoUnsafeValuesQuery>;
    type State = NoUnsafeValuesIssueKind;
    type Signals = Option<Self::State>;
    type Options = NoUnsafeValuesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            NoUnsafeValuesQuery::JsonNumberValue(number) => {
                let number_str = number.to_trimmed_string();
                let nmbr = number_str.parse::<f64>().ok()?;

                if nmbr.is_infinite() {
                    return Some(NoUnsafeValuesIssueKind::UnsafeNumber);
                }

                if nmbr == 0.0 {
                    let number_regex = Regex::new(
                        r"(?i)^[+-]?(?<int>0|([1-9]\d*))?(?:\.(?<frac>\d*))?(?:e[+-]?\d+)?$",
                    )
                    .unwrap();
                    let caps = number_regex.captures(&number_str)?;
                    let int = caps.name("int");
                    let frac = caps.name("frac");

                    let non_zero_regex = Regex::new(r"[1-9]").unwrap();

                    if int.is_some_and(|i| non_zero_regex.is_match(i.as_str()))
                        || frac.is_some_and(|f| non_zero_regex.is_match(f.as_str()))
                    {
                        return Some(NoUnsafeValuesIssueKind::UnsafeZero);
                    }
                } else if !number_str.contains('.')
                    && !number_str.to_lowercase_cow().contains('e')
                    && (nmbr > MAX_SAFE_INTEGER as f64 || nmbr < MIN_SAFE_INTEGER as f64)
                {
                    return Some(NoUnsafeValuesIssueKind::UnsafeInteger);
                } else if nmbr != 0.0 && nmbr.abs() < f64::MIN_POSITIVE {
                    return Some(NoUnsafeValuesIssueKind::Subnormal);
                }

                None
            }
            NoUnsafeValuesQuery::JsonStringValue(string) => {
                let value = string.inner_string_text().ok()?;
                let raw = value.text();
                if has_lone_surrogate_escape(raw.as_bytes()) {
                    return Some(NoUnsafeValuesIssueKind::LoneSurrogate);
                }

                let unescaped = unescape_json_string(value);
                if has_lone_surrogate_escape(unescaped.as_bytes()) {
                    return Some(NoUnsafeValuesIssueKind::LoneSurrogate);
                }

                None
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(rule_category!(), span, state.message())
                .note(markup! {
                    "Certain values can cause interoperability issues between different parsers and environments. Replace this value with a safe alternative."
                }),
        )
    }
}

declare_node_union! {
    pub NoUnsafeValuesQuery =
        JsonNumberValue
        | JsonStringValue
}

pub enum NoUnsafeValuesIssueKind {
    UnsafeNumber,
    UnsafeInteger,
    UnsafeZero,
    Subnormal,
    LoneSurrogate,
}

impl NoUnsafeValuesIssueKind {
    pub fn message(&self) -> MarkupBuf {
        match self {
            Self::UnsafeNumber => markup! {"The number will evaluate to Infinity."}.to_owned(),
            Self::UnsafeInteger => {
                markup! {"The integer is outside the safe integer range."}.to_owned()
            }
            Self::UnsafeZero => markup! {"The number will evaluate to zero."}.to_owned(),
            Self::Subnormal => {
                markup! {"Unexpected subnormal number found, which may cause interoperability issues."}.to_owned()
            }
            Self::LoneSurrogate => {
                markup! {"Lone surrogate found."}.to_owned()
            }
        }
    }
}

fn is_high_surrogate(u: &u16) -> bool {
    (0xD800..=0xDBFF).contains(u)
}

fn is_low_surrogate(u: &u16) -> bool {
    (0xDC00..=0xDFFF).contains(u)
}

fn parse_unicode_escape(bytes: &[u8], i: usize) -> Option<u16> {
    if i + 6 > bytes.len() || bytes[i] != b'\\' || bytes[i + 1] != b'u' {
        return None;
    }

    // `\u` is an escape only when this backslash is not itself escaped.
    let mut slash_run = 1;
    let mut j = i;
    while j > 0 && bytes[j - 1] == b'\\' {
        slash_run += 1;
        j -= 1;
    }
    if slash_run % 2 == 0 {
        return None;
    }

    std::str::from_utf8(&bytes[i + 2..i + 6])
        .ok()
        .and_then(|s| u16::from_str_radix(s, 16).ok())
}

fn has_lone_surrogate_escape(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        if let Some(val) = parse_unicode_escape(bytes, i) {
            if is_high_surrogate(&val) {
                let is_paired =
                    parse_unicode_escape(bytes, i + 6).is_some_and(|next| is_low_surrogate(&next));
                if !is_paired {
                    return true;
                }
                i += 12;
                continue;
            }

            if is_low_surrogate(&val) {
                return true;
            }

            i += 6;
            continue;
        }

        i += 1;
    }

    false
}
