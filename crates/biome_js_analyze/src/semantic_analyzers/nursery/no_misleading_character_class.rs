use crate::{semantic_services::Semantic, JsRuleAction};
use biome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{
    JsCallExpression, JsNewExpression, JsRegexLiteralExpression, JsSyntaxKind, JsSyntaxToken,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, TextRange};
declare_rule! {
    /// Disallow characters which are made with multiple code points in character class syntax
    ///
    /// Unicode includes the characters which are made with multiple code points.
    /// RegExp character class syntax (/[abc]/) cannot handle characters which are made by multiple code points as a character; those characters will be dissolved to each code point. For example, ❇️ is made by ❇ (U+2747) and VARIATION SELECTOR-16 (U+FE0F). If this character is in RegExp character class, it will match to either ❇ (U+2747) or VARIATION SELECTOR-16 (U+FE0F) rather than ❇️.
    /// This rule reports the regular expressions which include multiple code point characters in character class syntax. This rule considers the following characters as multiple code point characters.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-misleading-character-class/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /^[Á]$/u;
    /// /^[❇️]$/u;
    /// /^[👶🏻]$/u;
    /// /^[🇯🇵]$/u;
    /// /^[👨‍👩‍👦]$/u;
    /// /^[👍]$/;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// /^[abc]$/;
    /// /^[👍]$/u;
    /// /^[\q{👶🏻}]$/v;
    /// ```
    ///

    pub(crate) NoMisleadingCharacterClass {
        version: "1.3.0",
        name: "noMisleadingCharacterClass",
        recommended: false,
    }
}

declare_node_union! {
  pub(crate) AnyRegexExpression = JsNewExpression | JsCallExpression | JsRegexLiteralExpression
}

pub enum Message {
    SurrogatePairWithoutUFlag,
    EmojiModifier,
    RegionalIndicatorSymbol,
    CombiningClassOrVs16,
    JoinedCharSequence,
}

impl Message {
    fn as_str(&self) -> &str {
        match self {
            Self::CombiningClassOrVs16 => "Unexpected combined character in character class.",
            Self::SurrogatePairWithoutUFlag => {
                "Unexpected surrogate pair in character class. Use 'u' flag."
            }
            Self::EmojiModifier => "Unexpected modified Emoji in character class. ",
            Self::RegionalIndicatorSymbol => {
                "Regional indicator symbol characters should not be used in character class."
            }
            Self::JoinedCharSequence => "Unexpected joined character sequence in character class.",
        }
    }
}

pub struct RuleState {
    range: TextRange,
    message: Message,
}

impl Rule for NoMisleadingCharacterClass {
    type Query = Semantic<AnyRegexExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex = ctx.query();

        match regex {
            AnyRegexExpression::JsRegexLiteralExpression(expr) => {
                let Ok((pattern, flags)) = expr.decompose() else {
                    return None;
                };
                let regex_literal = replace_escaped_unicode(pattern.text());
                let has_u_flag = flags.text().contains('u');
                let mut is_in_character_class = false;
                let mut escape_next = false;
                let char_iter = regex_literal.chars().peekable();
                for (i, ch) in char_iter.enumerate() {
                    if escape_next {
                        escape_next = false;
                        continue;
                    }
                    match ch {
                        '\\' => escape_next = true,
                        '[' => is_in_character_class = true,
                        ']' => is_in_character_class = false,
                        _ if is_in_character_class && i < regex_literal.len() => {
                            if !has_u_flag && has_surrogate_pair(&regex_literal[i..]) {
                                return Some(RuleState {
                                    range: expr.syntax().text_range(),
                                    message: Message::SurrogatePairWithoutUFlag,
                                });
                            }

                            if has_combining_class_or_vs16(&regex_literal[i..]) {
                                return Some(RuleState {
                                    range: expr.syntax().text_range(),
                                    message: Message::CombiningClassOrVs16,
                                });
                            }

                            if has_regional_indicator_symbol(&regex_literal[i..]) {
                                return Some(RuleState {
                                    range: expr.syntax().text_range(),
                                    message: Message::RegionalIndicatorSymbol,
                                });
                            }

                            if has_emoji_modifier(&regex_literal[i..]) {
                                return Some(RuleState {
                                    range: expr.syntax().text_range(),
                                    message: Message::EmojiModifier,
                                });
                            }

                            if zwj(&regex_literal[i..]) {
                                return Some(RuleState {
                                    range: expr.syntax().text_range(),
                                    message: Message::JoinedCharSequence,
                                });
                            }
                        }
                        _ => {}
                    }
                }
                return None;
            }

            AnyRegexExpression::JsNewExpression(x) => {
                todo!();
            }

            AnyRegexExpression::JsCallExpression(x) => {
                todo!();
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range,
            state.message.as_str(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        match node {
            AnyRegexExpression::JsRegexLiteralExpression(expr) => {
                if matches!(state.message, Message::SurrogatePairWithoutUFlag) {
                    let prev_token = expr.value_token().ok()?;
                    let text = prev_token.text();
                    let next_token = JsSyntaxToken::new_detached(
                        JsSyntaxKind::JS_REGEX_LITERAL,
                        &format!("{}u", text),
                        [],
                        [],
                    );

                    let mut mutation = ctx.root().begin();
                    mutation.replace_token(prev_token, next_token);

                    Some(JsRuleAction {
                        category: ActionCategory::QuickFix,
                        applicability: Applicability::MaybeIncorrect,
                        message: markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }
                            .to_owned(),
                        mutation,
                    })
                } else {
                    None
                }
            }

            AnyRegexExpression::JsNewExpression(_) => todo!(),

            AnyRegexExpression::JsCallExpression(_) => todo!(),

            _ => None,
        }
    }
}

fn is_emoji_modifier(code: u32) -> bool {
    (0x1F3FB..=0x1F3FF).contains(&code)
}

fn has_emoji_modifier(chars: &str) -> bool {
    let char_vec: Vec<char> = chars.chars().collect();

    char_vec.iter().enumerate().any(|(i, &c)| {
        i != 0 && is_emoji_modifier(c as u32) && !is_emoji_modifier(char_vec[i - 1] as u32)
    })
}

fn is_regional_indicator_symbol(code: u32) -> bool {
    (0x1F1E6..=0x1F1FF).contains(&code)
}

fn has_regional_indicator_symbol(chars: &str) -> bool {
    let char_vec: Vec<char> = chars.chars().collect();

    char_vec.iter().enumerate().any(|(i, &c)| {
        i != 0
            && is_regional_indicator_symbol(c as u32)
            && is_regional_indicator_symbol(char_vec[i - 1] as u32)
    })
}

fn is_combining_character(ch: char) -> bool {
    match ch {
        '\u{0300}'..='\u{036F}' | // Combining Diacritical Marks
        '\u{1AB0}'..='\u{1AFF}' | // Combining Diacritical Marks Extended
        '\u{1DC0}'..='\u{1DFF}' | // Combining Diacritical Marks Supplement
        '\u{20D0}'..='\u{20FF}' | // Combining Diacritical Marks for Symbols
        '\u{FE20}'..='\u{FE2F}' // Combining Half Marks
        => true,
        _ => false
  }
}

fn is_variation_selector_16(ch: char) -> bool {
    ('\u{FE00}'..='\u{FE0F}').contains(&ch)
}

fn has_combining_class_or_vs16(chars: &str) -> bool {
    chars.chars().enumerate().any(|(i, c)| {
        i != 0
            && (is_combining_character(c) || is_variation_selector_16(c))
            && !(is_combining_character(chars.chars().nth(i - 1).unwrap())
                || is_variation_selector_16(chars.chars().nth(i - 1).unwrap()))
    })
}

fn zwj(chars: &str) -> bool {
    let char_vec: Vec<char> = chars.chars().collect();
    let last_index = char_vec.len() - 1;
    char_vec.iter().enumerate().any(|(i, &c)| {
        i != 0
            && i != last_index
            && c as u32 == 0x200D
            && char_vec[i - 1] as u32 != 0x200D
            && char_vec[i + 1] as u32 != 0x200D
    })
}

fn has_surrogate_pair(s: &str) -> bool {
    s.chars().any(|c| c as u32 > 0xFFFF)
}

fn is_unicode_char(ch: char) -> bool {
    ch.is_ascii_digit() || ('a'..='f').contains(&ch) || ('A'..='F').contains(&ch)
}

/// Convert unicode escape sequence string to unicode character
/// - unicode escape sequences: \u{XXXX}
/// - unicode escape sequences without parenthesis: \uXXXX
/// - surrogate pair: \uXXXX\uXXXX
/// If the unicode escape sequence is not valid, it will be treated as a simple string.
///
/// ```example
/// \uD83D\uDC4D -> 👍
/// \u0041\u0301 -> Á
/// \uD83D\uDC68\u200D\uD83D\uDC69\u200D\uD83D\uDC66 -> 👨‍👩‍👦
/// \u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F466} -> 👨‍👩‍👦
/// \u899\uD83D\uDC4D -> \u899👍
/// ````
fn replace_escaped_unicode(input: &str) -> String {
    let mut result = String::new();
    let mut chars_iter = input.chars().peekable();

    while let Some(ch) = chars_iter.next() {
        if ch == '\\' {
            match handle_escape_sequence(&mut chars_iter) {
                Some(unicode_char) => result.push_str(&unicode_char),
                None => result.push(ch),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn handle_escape_sequence(chars_iter: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
    if chars_iter.peek() != Some(&'u') {
        return None;
    }
    chars_iter.next();

    if chars_iter.peek() == Some(&'{') {
        handle_braced_escape_sequence(chars_iter)
    } else {
        handle_simple_or_surrogate_escape_sequence(chars_iter)
    }
}

fn handle_braced_escape_sequence(
    chars_iter: &mut std::iter::Peekable<std::str::Chars>,
) -> Option<String> {
    chars_iter.next();
    let mut codepoint_str = String::new();
    while let Some(&next_char) = chars_iter.peek() {
        if next_char == '}' {
            chars_iter.next();
            break;
        } else {
            codepoint_str.push(next_char);
            chars_iter.next();
        }
    }
    u32::from_str_radix(&codepoint_str, 16)
        .ok()
        .and_then(char::from_u32)
        .map(|c| c.to_string())
}

fn handle_simple_or_surrogate_escape_sequence(
    chars_iter: &mut std::iter::Peekable<std::str::Chars>,
) -> Option<String> {
    let mut invalid_pair = String::new();
    let mut high_surrogate_str = String::new();

    for _ in 0..4 {
        if let Some(next_char) = chars_iter.peek() {
            if !is_unicode_char(*next_char) {
                // Return as simple string. This is not unicode
                return Some(format!("\\u{}", high_surrogate_str));
            }
            high_surrogate_str.push(*next_char);
            chars_iter.next();
        } else {
            // Return as simple string. This is not unicode
            return Some(format!("\\u{}", high_surrogate_str));
        }
    }

    if let Ok(high_surrogate) = u32::from_str_radix(&high_surrogate_str, 16) {
        // check if it is in the high surrogate range(0xD800-0xDBFF) in UTF-16.
        if (0xD800..=0xDBFF).contains(&high_surrogate) {
            // Handle surrogate pair
            if chars_iter.next() == Some('\\') && chars_iter.next() == Some('u') {
                let mut low_surrogate_str = String::new();
                for _ in 0..4 {
                    if let Some(next_char) = chars_iter.peek() {
                        if !is_unicode_char(*next_char) {
                            // high_surrogate is the correct codepoint but treating low_surrogate_str a string because it is not a valid unicode codepoint
                            invalid_pair.push_str(&format!("{high_surrogate}"));
                            invalid_pair.push_str(&low_surrogate_str);
                            return Some(invalid_pair);
                        }
                        low_surrogate_str.push(*next_char);
                        chars_iter.next();
                    }
                }
                if let Ok(low_surrogate) = u32::from_str_radix(&low_surrogate_str, 16) {
                    // check if it is in the low surrogate range(0xDC00-0xDFFF) in UTF-16.
                    if (0xDC00..=0xDFFF).contains(&low_surrogate) {
                        // calculate the codepoint from the surrogate pair
                        let codepoint =
                            ((high_surrogate - 0xD800) << 10) + (low_surrogate - 0xDC00) + 0x10000;
                        return char::from_u32(codepoint).map(|c| c.to_string());
                    };
                } else {
                    // high_surrogate is the correct codepoint but treating low_surrogate_str a string because it is not a valid unicode codepoint
                    invalid_pair.push_str(&format!("{}", high_surrogate));
                    invalid_pair.push_str(&format!("\\u{}", low_surrogate_str));
                }
            }
        } else {
            // Handle standard escape sequence
            return char::from_u32(high_surrogate).map(|c| c.to_string());
        }
    } else {
        invalid_pair.push_str(&format!("\\u{}", high_surrogate_str));
    }
    // return as simple string, not unicode
    Some(invalid_pair)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_replace_escaped_unicode() {
        assert_eq!(replace_escaped_unicode(r#"/[\uD83D\uDC4D]/"#), "/[👍]/");
        assert_eq!(replace_escaped_unicode(r#"/[\u0041\u0301]/"#), "/[Á]/");
        assert_eq!(
            replace_escaped_unicode("/[\u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F466}]/u"),
            "/[👨‍👩‍👦]/u"
        );
        assert_eq!(
            replace_escaped_unicode(r#"/[\uD83D\uDC68\u200D\uD83D\uDC69\u200D\uD83D\uDC66]/u"#),
            "/[👨‍👩‍👦]/u"
        );
        assert_eq!(
            replace_escaped_unicode(r#"/[\u899\uD83D\uDC4D]/"#),
            r#"/[\u899👍]/"#
        );
    }
}
