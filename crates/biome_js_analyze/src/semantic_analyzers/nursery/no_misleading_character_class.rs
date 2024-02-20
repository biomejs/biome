use crate::{semantic_services::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    global_identifier, AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsTemplateElement, JsCallArguments, JsCallExpression, JsNewExpression,
    JsRegexLiteralExpression, JsStringLiteralExpression, JsSyntaxKind, JsSyntaxToken, T,
};
use biome_rowan::{
    declare_node_union, AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, TextRange,
    TriviaPieceKind,
};
declare_rule! {
    /// Disallow characters made with multiple code points in character class syntax.
    ///
    /// Unicode includes the characters which are made with multiple code points. e.g. Á, 🇯🇵, 👨‍👩‍👦.
    /// A RegExp character class `/[abc]/` cannot handle characters with multiple code points.
    /// For example, the character `❇️` consists of two code points: `❇` (U+2747) and `VARIATION SELECTOR-16` (U+FE0F).
    /// If this character is in a RegExp character class, it will match to either `❇` or `VARIATION SELECTOR-16` rather than `❇️`.
    /// This rule reports the regular expressions which include multiple code point characters in character class syntax.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /^[Á]$/u;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /^[❇️]$/u;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /^[👶🏻]$/u;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /^[🇯🇵]$/u;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /^[👨‍👩‍👦]$/u;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /^[👍]$/; // surrogate pair without u flag
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// /^[abc]$/;
    /// /^[👍]$/u;
    /// /^[\q{👶🏻}]$/v;
    /// ```
    pub NoMisleadingCharacterClass {
        version: "1.5.0",
        name: "noMisleadingCharacterClass",
        source: RuleSource::Eslint("no-misleading-character-class"),
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
  pub AnyRegexExpression = JsNewExpression | JsCallExpression | JsRegexLiteralExpression
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
            Self::CombiningClassOrVs16 => "Unexpected combined character in the character class.",
            Self::SurrogatePairWithoutUFlag => {
                "Unexpected surrogate pair in character class. Use the 'u' flag."
            }
            Self::EmojiModifier => "Unexpected modified Emoji in the character class. ",
            Self::RegionalIndicatorSymbol => {
                "Regional indicator symbol characters should not be used in the character class."
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

                if flags.text().contains('v') {
                    return None;
                }
                let regex_pattern = replace_escaped_unicode(pattern.text());
                let has_u_flag = flags.text().contains('u');
                let range = expr.syntax().text_range();
                return diagnostic_regex_pattern(&regex_pattern, has_u_flag, range);
            }

            AnyRegexExpression::JsNewExpression(expr) => {
                if is_regex_expr(expr.callee().ok()?)? {
                    let mut args = expr.arguments()?.args().iter();
                    let raw_regex_pattern = args
                        .next()
                        .and_then(|arg| arg.ok())
                        .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
                        .and_then(|js_string_literal| js_string_literal.inner_string_text().ok())?
                        .to_string();

                    let regex_pattern = replace_escaped_unicode(raw_regex_pattern.as_str());
                    let regexp_flags = args
                        .next()
                        .and_then(|arg| arg.ok())
                        .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
                        .map(|js_string_literal| js_string_literal.text())
                        .unwrap_or_default();

                    if regexp_flags.contains('v') {
                        return None;
                    }
                    let has_u_flag = regexp_flags.contains('u');
                    let range = expr.syntax().text_range();
                    return diagnostic_regex_pattern(&regex_pattern, has_u_flag, range);
                }
            }
            AnyRegexExpression::JsCallExpression(expr) => {
                if is_regex_expr(expr.callee().ok()?)? {
                    let mut args = expr.arguments().ok()?.args().iter();
                    let raw_regex_pattern = args
                        .next()
                        .and_then(|arg| arg.ok())
                        .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
                        .and_then(|js_string_literal| js_string_literal.inner_string_text().ok())?
                        .to_string();

                    let regex_pattern = replace_escaped_unicode(raw_regex_pattern.as_str());

                    let regexp_flags = args
                        .next()
                        .and_then(|arg| arg.ok())
                        .and_then(|arg| JsStringLiteralExpression::cast_ref(arg.syntax()))
                        .map(|js_string_literal| js_string_literal.text())
                        .unwrap_or_default();

                    if regexp_flags.contains('v') {
                        return None;
                    }

                    let has_u_flag = regexp_flags.contains('u');
                    let range = expr.syntax().text_range();
                    return diagnostic_regex_pattern(&regex_pattern, has_u_flag, range);
                }
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
        let is_fixable = matches!(state.message, Message::SurrogatePairWithoutUFlag);
        if is_fixable {
            match node {
                AnyRegexExpression::JsRegexLiteralExpression(expr) => {
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
                        applicability: Applicability::Always,
                        message: markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }
                            .to_owned(),
                        mutation,
                    })
                }

                AnyRegexExpression::JsNewExpression(expr) => {
                    let prev_node = expr.arguments()?;
                    let mut prev_args = prev_node.args().iter();

                    let regex_pattern = prev_args.next().and_then(|a| a.ok())?;
                    let flag = prev_args.next().and_then(|a| a.ok());

                    match make_suggestion(regex_pattern, flag) {
                        Some(suggest) => {
                            let mut mutation = ctx.root().begin();
                            mutation.replace_node(prev_node, suggest);
                            Some(JsRuleAction {
                                category: ActionCategory::QuickFix,
                                applicability: Applicability::Always,
                                message: markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }
                                    .to_owned(),
                                mutation,
                            })
                        }
                        None => None,
                    }
                }

                AnyRegexExpression::JsCallExpression(expr) => {
                    let prev_node = expr.arguments().ok()?;
                    let mut prev_args = expr.arguments().ok()?.args().iter();

                    let regex_pattern = prev_args.next().and_then(|a| a.ok())?;
                    let flag = prev_args.next().and_then(|a| a.ok());

                    match make_suggestion(regex_pattern, flag) {
                        Some(suggest) => {
                            let mut mutation = ctx.root().begin();
                            mutation.replace_node(prev_node, suggest);
                            Some(JsRuleAction {
                                category: ActionCategory::QuickFix,
                                applicability: Applicability::Always,
                                message: markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }
                                    .to_owned(),
                                mutation,
                            })
                        }
                        None => None,
                    }
                }
            }
        } else {
            None
        }
    }
}

fn is_regex_expr(expr: AnyJsExpression) -> Option<bool> {
    match expr {
        AnyJsExpression::JsIdentifierExpression(callee) => {
            Some(callee.name().ok()?.has_name("RegExp"))
        }
        AnyJsExpression::JsStaticMemberExpression(callee) => {
            let is_member_regexp = callee.member().ok()?.value_token().ok()?.text() == "RegExp";
            let callee = callee.object().ok()?;
            let (_, name) = global_identifier(&callee)?;
            let is_global_obj =
                name.text() == "globalThis" || name.text() == "global" || name.text() == "window";
            Some(is_global_obj && is_member_regexp)
        }
        _ => Some(false),
    }
}

fn diagnostic_regex_pattern(
    regex_pattern: &str,
    has_u_flag: bool,
    range: TextRange,
) -> Option<RuleState> {
    let regex_bytes_len = regex_pattern.as_bytes().len();
    let mut is_in_character_class = false;
    let mut escape_next = false;
    // We use `char_indices` to get the byte index of every character
    for (i, ch) in regex_pattern.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }
        match ch {
            '\\' => escape_next = true,
            '[' => is_in_character_class = true,
            ']' => is_in_character_class = false,
            _ if is_in_character_class && i < regex_bytes_len => {
                if !has_u_flag && has_surrogate_pair(&regex_pattern[i..]) {
                    return Some(RuleState {
                        range,
                        message: Message::SurrogatePairWithoutUFlag,
                    });
                }

                if has_combining_class_or_vs16(&regex_pattern[i..]) {
                    return Some(RuleState {
                        range,
                        message: Message::CombiningClassOrVs16,
                    });
                }

                if has_regional_indicator_symbol(&regex_pattern[i..]) {
                    return Some(RuleState {
                        range,
                        message: Message::RegionalIndicatorSymbol,
                    });
                }

                if has_emoji_modifier(&regex_pattern[i..]) {
                    return Some(RuleState {
                        range,
                        message: Message::EmojiModifier,
                    });
                }

                if zwj(&regex_pattern[i..]) {
                    return Some(RuleState {
                        range,
                        message: Message::JoinedCharSequence,
                    });
                }
            }
            _ => {}
        }
    }
    None
}

fn make_suggestion(
    literal: AnyJsCallArgument,
    flag: Option<AnyJsCallArgument>,
) -> Option<JsCallArguments> {
    let suggestion = match flag {
        None => Some(AnyJsCallArgument::AnyJsExpression(
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(make::js_string_literal("u")),
                ),
            ),
        )),
        Some(f) => match f {
            AnyJsCallArgument::AnyJsExpression(expr) => match expr {
                AnyJsExpression::AnyJsLiteralExpression(e) => {
                    let text = e.text();
                    if text.starts_with('\'') {
                        Some(AnyJsCallArgument::AnyJsExpression(
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsStringLiteralExpression(
                                    make::js_string_literal_expression(make::js_string_literal(
                                        &format!("'{}u'", text),
                                    )),
                                ),
                            ),
                        ))
                    } else {
                        Some(AnyJsCallArgument::AnyJsExpression(
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsStringLiteralExpression(
                                    make::js_string_literal_expression(make::js_string_literal(
                                        &format!("{}u", text.replace('"', "")),
                                    )),
                                ),
                            ),
                        ))
                    }
                }
                AnyJsExpression::JsTemplateExpression(expr) => {
                    let mut elements = expr
                        .elements()
                        .iter()
                        .collect::<Vec<AnyJsTemplateElement>>();

                    let uflag = AnyJsTemplateElement::from(make::js_template_chunk_element(
                        make::js_template_chunk("u"),
                    ));
                    elements.push(uflag);
                    Some(AnyJsCallArgument::AnyJsExpression(
                        AnyJsExpression::JsTemplateExpression(
                            make::js_template_expression(
                                make::token(T!['`']),
                                make::js_template_element_list(elements),
                                make::token(T!['`']),
                            )
                            .build(),
                        ),
                    ))
                }
                AnyJsExpression::JsIdentifierExpression(_) => None,
                _ => None,
            },
            AnyJsCallArgument::JsSpread(_) => None,
        },
    };

    suggestion.map(|s| {
        make::js_call_arguments(
            make::token(T!['(']),
            make::js_call_argument_list(
                [literal, s],
                Some(make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])),
            ),
            make::token(T![')']),
        )
    })
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
        if let Some(&next_char) = chars_iter.peek() {
            if next_char.is_ascii_hexdigit() {
                high_surrogate_str.push(next_char);
                chars_iter.next();
            } else {
                // If the character is not a valid Unicode char, return as simple string.
                return Some(format!("\\u{}", high_surrogate_str));
            }
        } else {
            // If not enough characters, return as if it were a simple string.
            return Some(format!("\\u{}", high_surrogate_str));
        }
    }

    if let Ok(high_surrogate) = u32::from_str_radix(&high_surrogate_str, 16) {
        // Check if it is in the high surrogate range(0xD800-0xDBFF) in UTF-16.
        if (0xD800..=0xDBFF).contains(&high_surrogate) {
            // If we have a high surrogate, expect a low surrogate next
            if chars_iter.next() == Some('\\') && chars_iter.next() == Some('u') {
                let mut low_surrogate_str = String::new();
                for _ in 0..4 {
                    if let Some(next_char) = chars_iter.peek() {
                        if !next_char.is_ascii_hexdigit() {
                            // Return as a simple string
                            // - high surrogate on its own doesn't make sense
                            // - low surrogate is not a valid unicode codepoint
                            // e.g \uD83D\u333
                            invalid_pair.push_str(&format!("\\u{}", high_surrogate_str));
                            invalid_pair.push_str(&format!("\\u{}", low_surrogate_str));
                            return Some(invalid_pair);
                        }
                        low_surrogate_str.push(*next_char);
                        chars_iter.next();
                    }
                }
                if let Ok(low_surrogate) = u32::from_str_radix(&low_surrogate_str, 16) {
                    // Check if it is in the low surrogate range(0xDC00-0xDFFF) in UTF-16.
                    if (0xDC00..=0xDFFF).contains(&low_surrogate) {
                        // Calculate the codepoint from the surrogate pair
                        let codepoint =
                            ((high_surrogate - 0xD800) << 10) + (low_surrogate - 0xDC00) + 0x10000;
                        return char::from_u32(codepoint).map(|c| c.to_string());
                    };
                }
            }
        } else {
            match char::from_u32(high_surrogate) {
                Some(c) => return Some(c.to_string()),
                None => invalid_pair.push_str(&format!("\\u{}", high_surrogate_str)),
            }
        }
    }
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
        assert_eq!(
            replace_escaped_unicode(r#"/[\u899\uD83D\u899\uDC4D]/"#),
            r#"/[\u899\uD83D\u899\uDC4D]/"#
        );
    }
}
