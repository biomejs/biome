use std::sync::LazyLock;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::{MarkupBuf, markup};
use biome_deserialize::json::unescape_json_string;
use biome_diagnostics::Severity;
use biome_json_syntax::{JsonNumberValue, JsonStringValue};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_unsafe_values::NoUnsafeValuesOptions;
use regex::Regex;

declare_lint_rule! {
    /// Disallow unsafe JSON values that may cause interoperability issues.
    ///
    /// Some JSON values can break when parsed by different tools or languages as parser implementations & data types could differ.
    /// For example, a very large number might become `Infinity` in JavaScript,
    /// or a string with an incomplete Unicode pair might fail to decode properly.
    ///
    /// The common unsafe values are:
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
    /// 2e308
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// -2e308
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// "\ud83d"
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// 1e-400
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// 9007199254740992
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// 2.2250738585072009e-308
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsonc
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
        recommended: true,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintJson("no-unsafe-values").same()],
    }
}

static MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;
static MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991;

static NUMBER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^[+-]?(?<int>0|([1-9]\d*))?(?:\.(?<frac>\d*))?(?:e[+-]?\d+)?$"#).unwrap()
});
static NON_ZERO_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[1-9]"#).unwrap());

impl Rule for NoUnsafeValues {
    type Query = Ast<AnyNoUnsafeValuesQuery>;
    type State = NoUnsafeValuesIssueKind;
    type Signals = Option<Self::State>;
    type Options = NoUnsafeValuesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            AnyNoUnsafeValuesQuery::JsonNumberValue(number) => {
                let number_str = number.to_trimmed_string();
                let nmbr = number_str.parse::<f64>().ok()?;

                if nmbr.is_infinite() {
                    return Some(NoUnsafeValuesIssueKind::UnsafeNumber);
                }

                if nmbr == 0.0 {
                    let caps = NUMBER_REGEX.captures(&number_str)?;
                    let int = caps.name("int");
                    let frac = caps.name("frac");

                    if int.is_some_and(|i| NON_ZERO_REGEX.is_match(i.as_str()))
                        || frac.is_some_and(|f| NON_ZERO_REGEX.is_match(f.as_str()))
                    {
                        return Some(NoUnsafeValuesIssueKind::UnsafeZero);
                    }
                } else if !number_str.contains('.')
                    && !number_str
                        .as_bytes()
                        .iter()
                        .any(|byte| byte.eq_ignore_ascii_case(&b'e'))
                    && (nmbr > MAX_SAFE_INTEGER as f64 || nmbr < MIN_SAFE_INTEGER as f64)
                {
                    return Some(NoUnsafeValuesIssueKind::UnsafeInteger);
                } else if nmbr != 0.0 && nmbr.abs() < f64::MIN_POSITIVE {
                    return Some(NoUnsafeValuesIssueKind::Subnormal);
                }

                None
            }
            AnyNoUnsafeValuesQuery::JsonStringValue(string) => {
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
    pub AnyNoUnsafeValuesQuery =
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

/// Parses a `\uXXXX` escape starting at `i`.
///
/// Returns `None` if `i` doesn't point to a valid escape sequence, if the
/// escape is itself escaped (for example `\\uXXXX`), or if the hex digits are
/// invalid.
fn parse_unicode_escape(bytes: &[u8], i: usize) -> Option<u16> {
    let next = i.checked_add(1)?;
    let hex_start = i.checked_add(2)?;
    let end = i.checked_add(6)?;
    if end > bytes.len() {
        return None;
    }

    if bytes.get(i) != Some(&b'\\') || bytes.get(next) != Some(&b'u') {
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

    std::str::from_utf8(bytes.get(hex_start..end)?)
        .ok()
        .and_then(|s| u16::from_str_radix(s, 16).ok())
}

/// Returns `true` when the input contains any lone UTF-16 surrogate escape.
///
/// A high surrogate (`\uD800`-`\uDBFF`) must be immediately followed by a low
/// surrogate (`\uDC00`-`\uDFFF`). Any unmatched high or low surrogate is
/// considered unsafe.
fn has_lone_surrogate_escape(bytes: &[u8]) -> bool {
    let mut i = 0;
    while i < bytes.len() {
        if let Some(val) = parse_unicode_escape(bytes, i) {
            if is_high_surrogate(&val) {
                let is_paired = i
                    .checked_add(6)
                    .and_then(|next_i| parse_unicode_escape(bytes, next_i))
                    .is_some_and(|next| is_low_surrogate(&next));
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

#[cfg(test)]
mod tests {
    use super::{has_lone_surrogate_escape, parse_unicode_escape};

    #[test]
    fn parse_unicode_escape_parses_valid_escape() {
        assert_eq!(parse_unicode_escape(br"\uD83D", 0), Some(0xD83D));
    }

    #[test]
    fn parse_unicode_escape_returns_none_for_escaped_backslash() {
        assert_eq!(parse_unicode_escape(br"\\uD83D", 1), None);
    }

    #[test]
    fn parse_unicode_escape_returns_none_for_invalid_hex() {
        assert_eq!(parse_unicode_escape(br"\uZZZZ", 0), None);
    }

    #[test]
    fn parse_unicode_escape_handles_large_index_safely() {
        assert_eq!(parse_unicode_escape(br"\uD83D", usize::MAX), None);
        assert_eq!(parse_unicode_escape(br"\uD83D", 5), None);
    }

    #[test]
    fn parse_unicode_escape_handles_long_backslash_runs() {
        let mut even_run = vec![b'\\'; 20];
        even_run.extend_from_slice(b"uD83D");
        assert_eq!(parse_unicode_escape(&even_run, 19), None);

        let mut odd_run = vec![b'\\'; 21];
        odd_run.extend_from_slice(b"uD83D");
        assert_eq!(parse_unicode_escape(&odd_run, 20), Some(0xD83D));
    }

    #[test]
    fn has_lone_surrogate_escape_detects_lone_high_surrogate() {
        assert!(has_lone_surrogate_escape(br"\uD83D"));
    }

    #[test]
    fn has_lone_surrogate_escape_detects_lone_low_surrogate() {
        assert!(has_lone_surrogate_escape(br"\uDD25"));
    }

    #[test]
    fn has_lone_surrogate_escape_ignores_valid_pair() {
        assert!(!has_lone_surrogate_escape(br"\uD83D\uDD25"));
    }

    #[test]
    fn has_lone_surrogate_escape_ignores_escaped_sequence() {
        assert!(!has_lone_surrogate_escape(br"\\uD83D"));
    }

    #[test]
    fn has_lone_surrogate_escape_detects_unpaired_high_surrogate() {
        assert!(has_lone_surrogate_escape(br"\uD83D\u0041"));
    }

    #[test]
    fn has_lone_surrogate_escape_handles_long_backslash_runs() {
        let mut escaped = vec![b'\\'; 20];
        escaped.extend_from_slice(b"uD83D");
        assert!(!has_lone_surrogate_escape(&escaped));

        let mut unescaped = vec![b'\\'; 21];
        unescaped.extend_from_slice(b"uD83D");
        assert!(has_lone_surrogate_escape(&unescaped));
    }
}
