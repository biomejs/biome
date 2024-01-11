use crate::utils::escape_string;
use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, JsCallArguments, JsCallExpression, JsNewExpression, JsRegexLiteralExpression,
    JsStringLiteralExpression,
};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList};
use std::{iter::Peekable, str::Chars};

declare_rule! {
    /// Prevents from having control characters and some escape sequences that match control characters in regular expressions.
    ///
    /// Control characters are hidden special characters that are numbered from 0 to 31 in the ASCII system.
    /// They're not commonly used in JavaScript text. So, if you see them in a pattern (called a regular expression), it's probably a mistake.
    ///
    /// The following elements of regular expression patterns are considered possible errors in typing and are therefore disallowed by this rule:
    ///
    /// - Hexadecimal character escapes from `\x00` to `\x1F`
    /// - Unicode character escapes from `\u0000` to `\u001F`
    /// - Unicode code point escapes from `\u{0}` to `\u{1F}`
    /// - Unescaped raw characters from U+0000 to U+001F
    ///
    /// Control escapes such as `\t` and `\n` are allowed by this rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    /// ```js,expect_diagnostic
    ///  var pattern1 = /\x00/;
    /// ```
    /// ```js,expect_diagnostic
    ///  var pattern2 = /\x0C/;
    /// ```
    /// ```js,expect_diagnostic
    ///  var pattern3 = /\x1F/;
    /// ```
    /// ```js,expect_diagnostic
    ///  var pattern4 = /\u000C/;
    /// ```
    /// ```js,expect_diagnostic
    ///  var pattern5 = /\u{C}/u;
    /// ```
    /// ```js,expect_diagnostic
    ///  var pattern7 = new RegExp("\x0C");
    /// ```
    /// ```js,expect_diagnostic
    ///  var pattern7 = new RegExp("\\x0C");
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// var pattern1 = /\x20/;
    /// var pattern2 = /\u0020/;
    /// var pattern3 = /\u{20}/u;
    /// var pattern4 = /\t/;
    /// var pattern5 = /\n/;
    /// var pattern6 = new RegExp("\x20");
    /// ```
    ///
    pub(crate) NoControlCharactersInRegex {
        version: "1.0.0",
        name: "noControlCharactersInRegex",
        source: RuleSource::Eslint("no-control-regex"),
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) RegexExpressionLike = JsNewExpression | JsCallExpression | JsRegexLiteralExpression
}

fn decode_hex_character_to_code_point(iter: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let first = iter.next()?;
    let second = iter.next()?;
    let digits = format!("{first}{second}");
    let code_point = i64::from_str_radix(&digits, 16).ok()?;
    Some((digits, code_point))
}

fn decode_unicode_escape_to_code_point(iter: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let mut digits = String::new();
    // Loop 4 times as unicode escape sequence has exactly 4 hexadecimal digits
    for _ in 0..4 {
        if let Some(&c) = iter.peek() {
            match c {
                '0'..='9' | 'a'..='f' | 'A'..='F' => digits.push(iter.next()?),
                _ => continue,
            }
        }
    }
    let code_point = i64::from_str_radix(digits.as_str(), 16).ok()?;
    Some((digits, code_point))
}

fn decode_escaped_code_point_to_code_point(iter: &mut Peekable<Chars>) -> Option<(String, i64)> {
    let mut digits = String::new();
    if iter.peek() == Some(&'{') {
        iter.next();
        while let Some(&c) = iter.peek() {
            if c == '}' {
                iter.next();
                let code_point = i64::from_str_radix(&digits, 16).ok()?;
                return Some((format!("{{{}}}", digits), code_point));
            } else {
                digits.push(iter.next()?);
            }
        }
    }
    None
}

fn add_control_character_to_vec(
    prefix: &str,
    iter: &mut Peekable<Chars>,
    control_characters: &mut Vec<String>,
    decode: fn(&mut Peekable<Chars>) -> Option<(String, i64)>,
) {
    if let Some((s, code_point)) = decode(iter) {
        // ASCII control characters are represented by code points from 0 to 31
        if (0..=31).contains(&code_point) {
            control_characters.push(format!("{prefix}{s}"));
        }
    }
}

/// Collecting control characters for regex. The following characters in regular expression patterns are considered as control characters:
/// - Hexadecimal character escapes from `\x00` to `\x1F`.
/// - Unicode character escapes from `\u0000` to `\u001F`.
/// - Unicode code point escapes range from `\u{0}` to `\u{1F}`.
///     - The Unicode flag must be set as true in order for these Unicode code point escapes to work: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/unicode.
/// - Unescaped raw characters from U+0000 to U+001F.
fn collect_control_characters(pattern: &str, flags: &str) -> Option<Vec<String>> {
    let mut control_characters: Vec<String> = Vec::new();
    let is_unicode_flag_set = flags.contains('u');
    let mut iter = pattern.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            '\\' => match iter.next() {
                Some('x') => add_control_character_to_vec(
                    "\\x",
                    &mut iter,
                    &mut control_characters,
                    decode_hex_character_to_code_point,
                ),
                Some('u') if is_unicode_flag_set => add_control_character_to_vec(
                    "\\u",
                    &mut iter,
                    &mut control_characters,
                    decode_escaped_code_point_to_code_point,
                ),
                Some('u') => add_control_character_to_vec(
                    "\\u",
                    &mut iter,
                    &mut control_characters,
                    decode_unicode_escape_to_code_point,
                ),
                Some('\\') => continue,
                _ => break,
            },
            _ => continue,
        }
    }

    if !control_characters.is_empty() {
        Some(control_characters)
    } else {
        None
    }
}

fn collect_control_characters_from_expression(
    callee: &AnyJsExpression,
    js_call_arguments: &JsCallArguments,
) -> Option<Vec<String>> {
    if callee.as_js_reference_identifier()?.has_name("RegExp") {
        let mut args = js_call_arguments.args().iter();
        let raw_pattern = args
            .next()
            .and_then(|arg| arg.ok())
            .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
            .and_then(|js_string_literal| js_string_literal.inner_string_text().ok())?
            .to_string();

        let pattern = escape_string(&raw_pattern).unwrap_or(raw_pattern);

        let regexp_flags = args
            .next()
            .and_then(|arg| arg.ok())
            .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
            .map(|js_string_literal| js_string_literal.text())
            .unwrap_or_default();

        return collect_control_characters(&pattern, &regexp_flags);
    }
    None
}

impl Rule for NoControlCharactersInRegex {
    type Query = Ast<RegexExpressionLike>;
    type State = Vec<String>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            RegexExpressionLike::JsNewExpression(js_new_expression) => {
                collect_control_characters_from_expression(
                    &js_new_expression.callee().ok()?,
                    &js_new_expression.arguments()?,
                )
            }
            RegexExpressionLike::JsCallExpression(js_call_expression) => {
                collect_control_characters_from_expression(
                    &js_call_expression.callee().ok()?,
                    &js_call_expression.arguments().ok()?,
                )
            }
            RegexExpressionLike::JsRegexLiteralExpression(js_regex_literal_expression) => {
                let (pattern, flags) = js_regex_literal_expression.decompose().ok()?;
                collect_control_characters(pattern.text(), flags.text())
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Unexpected control character(s) in regular expression: "<Emphasis>{state.join(", ")}</Emphasis>""
            },
        ).note(
            markup! {
                "Control characters are unusual and potentially incorrect inputs, so they are disallowed."
            }
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_control_characters() {
        assert_eq!(
            collect_control_characters("\\x00\\x0F\\u0010\\u001F", ""),
            Some(vec![
                String::from("\\x00"),
                String::from("\\x0F"),
                String::from("\\u0010"),
                String::from("\\u001F")
            ])
        );
        assert_eq!(
            collect_control_characters("\\u{0}\\u{1F}", "u"),
            Some(vec![String::from("\\u{0}"), String::from("\\u{1F}")])
        );
        assert_eq!(
            collect_control_characters("\\x20\\u0020\\u{20}\\t\\n", ""),
            None
        );
    }
}
