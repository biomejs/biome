use crate::{semantic_services::Semantic, JsRuleAction};
use biome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{
    JsCallExpression, JsNewExpression, JsRegexLiteralExpression, JsSyntaxKind, JsSyntaxToken,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, TextRange};
use regex::Regex;
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
    node: AnyRegexExpression,
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
                let range = expr.syntax().text_range();

                let Ok((pattern, flags)) = expr.decompose() else {
                    return None;
                };

                let has_v_flag = flags.text().contains('v');

                if has_v_flag {
                    return None;
                }

                let l = replace_escaped_unicode(pattern.text());

                let has_u_flag = flags.text().contains('u');

                if !has_u_flag && has_surrogate_pair(&l) {
                    return Some(RuleState {
                        range,

                        message: Message::SurrogatePairWithoutUFlag,

                        node: expr.clone().into(),
                    });
                }

                if has_combining_class_or_vs16(&l) {
                    return Some(RuleState {
                        range,

                        message: Message::CombiningClassOrVs16,

                        node: expr.clone().into(),
                    });
                }

                if has_regional_indicator_symbol(&l) {
                    return Some(RuleState {
                        range,

                        message: Message::RegionalIndicatorSymbol,

                        node: expr.clone().into(),
                    });
                }

                if has_emoji_modifier(&l) {
                    return Some(RuleState {
                        range,

                        message: Message::EmojiModifier,

                        node: expr.clone().into(),
                    });
                }

                if zwj(&l) {
                    return Some(RuleState {
                        range,

                        message: Message::JoinedCharSequence,

                        node: expr.clone().into(),
                    });
                }
            }

            AnyRegexExpression::JsNewExpression(x) => {
                // let regex = x.expression().ok()?.as_js_regex_literal_expression()?;

                // Self::check_regex(ctx, regex).map(|state| RuleState {

                // range: state.range,

                // message: state.message,

                // node: AnyRegexExpression::JsNewExpression(x.clone()),

                // })

                todo!();
            }

            AnyRegexExpression::JsCallExpression(x) => {
                // let regex = x.expression().ok()?.as_js_regex_literal_expression()?;

                // Self::check_regex(ctx, regex).map(|state| RuleState {

                // range: state.range,

                // message: state.message,

                // node: AnyRegexExpression::JsCallExpression(x.clone()),

                // })

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
        match &state.node {
            AnyRegexExpression::JsRegexLiteralExpression(expr) => {
                let mut mutation = ctx.root().begin();

                let prev_token = expr.value_token().ok()?;

                let text = prev_token.text();

                let next_token = JsSyntaxToken::new_detached(
                    JsSyntaxKind::JS_REGEX_LITERAL,
                    &format!("{}u", text),
                    [],
                    [],
                );

                mutation.replace_token(prev_token, next_token);

                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,

                    applicability: Applicability::MaybeIncorrect,

                    message: markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }
                        .to_owned(),

                    mutation,
                })
            }

            AnyRegexExpression::JsNewExpression(_) => todo!(),

            AnyRegexExpression::JsCallExpression(_) => todo!(),

            _ => None,
        }

        // if matches!(state.message, Message::SurrogatePairWithoutUFlag) {

        // let n = ctx.query();

        // let mut mutation = ctx.root().begin();

        // let prev_token = n.value_token().ok()?;

        // let text = n.text();

        // let next_token = JsSyntaxToken::new_detached(

        // JsSyntaxKind::JS_REGEX_LITERAL,

        // &format!("{}u", text),

        // [],

        // [],

        // );

        // mutation.replace_token(prev_token, next_token);

        // Some(JsRuleAction {

        // category: ActionCategory::QuickFix,

        // applicability: Applicability::MaybeIncorrect,

        // message: markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }

        // .to_owned(),

        // mutation,

        // })

        // } else {

        // None

        // }
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

fn replace_escaped_unicode(input: &str) -> String {
    let re = Regex::new(r"\\u\{([0-9a-fA-F]+)\}").unwrap();

    re.replace_all(input, |caps: &regex::Captures| {
        u32::from_str_radix(&caps[1], 16)
            .ok()
            .and_then(char::from_u32)
            .map_or_else(String::new, |c| c.to_string())
    })
    .into_owned()
}
