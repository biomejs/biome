use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    global_identifier, static_value::StaticValue, AnyJsCallArgument, AnyJsExpression,
    AnyJsLiteralExpression, AnyJsTemplateElement, JsCallArguments, JsSyntaxKind, JsSyntaxToken, T,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, TextRange, TriviaPieceKind,
};

use super::no_control_characters_in_regex::AnyRegexExpression;

declare_lint_rule! {
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
        language: "js",
        sources: &[RuleSource::Eslint("no-misleading-character-class")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

pub enum Message {
    SurrogatePairWithoutUFlag,
    EmojiModifier,
    RegionalIndicatorSymbol,
    CombiningClassOrVs16,
    JoinedCharSequence,
}

impl Message {
    fn diagnostic(&self) -> &str {
        match self {
            Self::CombiningClassOrVs16 => "A character class cannot match a character and a combining character.",
            Self::SurrogatePairWithoutUFlag => {
                "A character class cannot match a surrogate pair. Add the 'u' unicode flag to match against them."
            }
            Self::EmojiModifier => "A character class cannot match an emoji with a skin tone modifier.",
            Self::RegionalIndicatorSymbol => {
                "A character class cannot match a pair of regional indicator symbols."
            }
            Self::JoinedCharSequence => "A character class cannot match a joined character sequence.",
        }
    }

    fn note(&self) -> &str {
        match self {
            Self::CombiningClassOrVs16 => "A character and a combining character forms a new character. Replace the character class with an alternation.",
            Self::SurrogatePairWithoutUFlag => {
                "A surrogate pair forms a single codepoint, but is encoded as a pair of two characters. Without the unicode flag, the regex matches a single surrogate character."
            }
            Self::EmojiModifier => "Replace the character class with an alternation.",
            Self::RegionalIndicatorSymbol => {
                "A pair of regional indicator symbols encodes a country code. Replace the character class with an alternation."
            }
            Self::JoinedCharSequence => "A zero width joiner composes several emojis into a new one. Replace the character class with an alternation.",
        }
    }
}

pub struct RuleState {
    range: TextRange,
    message: Message,
}

impl Rule for NoMisleadingCharacterClass {
    type Query = Ast<AnyRegexExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex = ctx.query();
        let (callee, arguments) = match regex {
            AnyRegexExpression::JsRegexLiteralExpression(expr) => {
                let (pattern, flags) = expr.decompose().ok()?;
                let RuleState { range, message } =
                    diagnostic_regex_pattern(pattern.text(), flags.text(), false)?;
                return Some(RuleState {
                    range: range.checked_add(expr.range().start().checked_add(1.into())?)?,
                    message,
                });
            }
            AnyRegexExpression::JsNewExpression(expr) => (expr.callee().ok()?, expr.arguments()?),
            AnyRegexExpression::JsCallExpression(expr) => {
                (expr.callee().ok()?, expr.arguments().ok()?)
            }
        };
        if is_regex_expr(callee)? {
            let mut args = arguments.args().iter();
            let pattern = args
                .next()?
                .ok()?
                .as_any_js_expression()?
                .as_static_value()?;
            let pattern_range = pattern.range();
            let pattern = pattern.as_string_constant()?;
            let flags = args
                .next()
                .and_then(|arg| arg.ok()?.as_any_js_expression()?.as_static_value());
            let flags = if let Some(StaticValue::String(flags)) = &flags {
                flags.text()
            } else {
                ""
            };
            let RuleState { range, message } = diagnostic_regex_pattern(pattern, flags, true)?;
            return Some(RuleState {
                range: range.checked_add(pattern_range.start().checked_add(1.into())?)?,
                message,
            });
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(rule_category!(), state.range, state.message.diagnostic())
                .note(state.message.note()),
        )
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
                        &format!("{text}u"),
                        [],
                        [],
                    );
                    let mut mutation = ctx.root().begin();
                    mutation.replace_token(prev_token, next_token);
                    Some(JsRuleAction::new(
                        ctx.metadata().action_category(ctx.category(), ctx.group()),
                        ctx.metadata().applicability(),
                        markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }
                            .to_owned(),
                        mutation,
                    ))
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
                            Some(JsRuleAction::new(
                                ctx.metadata().action_category(ctx.category(), ctx.group()),
                                ctx.metadata().applicability(),
                                markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }
                                    .to_owned(),
                                mutation,
                            ))
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
                            Some(JsRuleAction::new(
                                ctx.metadata().action_category(ctx.category(), ctx.group()),
                                ctx.metadata().applicability(),
                                markup! { "Add unicode "<Emphasis>"u"</Emphasis>" flag to regex" }
                                    .to_owned(),
                                mutation,
                            ))
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
    flags: &str,
    is_in_string: bool,
) -> Option<RuleState> {
    if flags.contains('v') {
        return None;
    }
    let has_u_flag = flags.contains('u');
    let mut bytes_iter = regex_pattern.bytes().enumerate();
    while let Some((i, byte)) = bytes_iter.next() {
        match byte {
            b'\\' => {
                bytes_iter.next();
            }
            b'[' => {
                while let Some((j, byte)) = bytes_iter.next() {
                    match byte {
                        b'\\' => {
                            bytes_iter.next();
                        }
                        b']' => {
                            let char_class = &regex_pattern[i + 1..j];
                            if let Some(RuleState { range, message }) =
                                diagnostic_regex_class(char_class, has_u_flag, is_in_string)
                            {
                                return Some(RuleState {
                                    range: range.checked_add(((i + 1) as u32).into())?,
                                    message,
                                });
                            }
                            break;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    None
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum CharType {
    CombiningOrVariationSelectorS16,
    EmojiModifier,
    None,
    RegionalIndicator,
    Regular,
    ZeroWidthJoiner,
}

fn diagnostic_regex_class(
    char_class: &str,
    has_u_flag: bool,
    is_in_string: bool,
) -> Option<RuleState> {
    let mut prev_char_index = 0;
    let mut prev_char_type = CharType::None;
    let mut iter = char_class.char_indices();
    while let Some((i, c)) = iter.next() {
        let (codepoint, end) = if c == '\\' {
            // Maybe  unicode esccapes \u{XXX} \uXXXX
            let Some((codepoint, len)) = decode_next_codepoint(&char_class[i..], is_in_string)
            else {
                prev_char_index = i;
                continue;
            };
            for _ in c.len_utf8()..len {
                iter.next();
            }
            (codepoint, i + len)
        } else {
            (c as u32, i + c.len_utf8())
        };
        match codepoint {
            // Non-BMP characters are encoded as surrogate pairs in UTF-16 / UCS-2
            0x10000.. if !has_u_flag => {
                return Some(RuleState {
                    range: TextRange::new((i as u32).into(), (end as u32).into()),
                    message: Message::SurrogatePairWithoutUFlag,
                });
            }
            // Combining Diacritical Marks
            0x0300..=0x036F
            // Mongolian Free Variation Selector (FVS1 to FVS4)
            | 0x180B..=0x180D | 0x180F
            // Combining Diacritical Marks Extended
            | 0x1AB0..=0x1AFF
            // Combining Diacritical Marks Supplement
            | 0x1DC0..=0x1DFF
            // Combining Diacritical Marks for Symbols
            | 0x20D0..=0x20FF
            // Variation Selector (VS1 to VS16)
            | 0xFE00..=0xFE0F
            // Combining Half Marks
            | 0xFE20..=0xFE2F
            // Variation Selectors Supplement (VS17 to VS256)
            | 0xE0100..=0xE01EF => {
                if prev_char_type == CharType::Regular {
                    return Some(RuleState {
                        range: TextRange::new((prev_char_index as u32).into(), (end as u32).into()),
                        message: Message::CombiningClassOrVs16,
                    });
                }
                prev_char_type = CharType::CombiningOrVariationSelectorS16;
            }
            // Regional indicator
            0x1F1E6..=0x1F1FF => {
                if matches!(prev_char_type, CharType::RegionalIndicator) {
                    return Some(RuleState {
                        range: TextRange::new((prev_char_index as u32).into(), (end as u32).into()),
                        message: Message::RegionalIndicatorSymbol,
                    });
                }
                prev_char_type = CharType::RegionalIndicator;
            }
            // Emoji skin modifier
            0x1F3FB..=0x1F3FF => {
                if prev_char_type == CharType::Regular {
                    return Some(RuleState {
                        range: TextRange::new((prev_char_index as u32).into(), (end as u32).into()),
                        message: Message::EmojiModifier,
                    });
                }
                prev_char_type = CharType::EmojiModifier;
            }
            // Zero Width Joiner (used to combine emoji)
            0x200D => {
                if
                    !matches!(prev_char_type, CharType::None | CharType::ZeroWidthJoiner)
                    && end < char_class.len()
                {
                    if let Some((c, len)) = decode_next_codepoint(&char_class[end..], is_in_string) {
                        if c != 0x200D {
                            return Some(RuleState {
                                range: TextRange::new((prev_char_index as u32).into(), ((end + len) as u32).into()),
                                message: Message::JoinedCharSequence,
                            });
                        }
                    }
                }
                prev_char_type = CharType::ZeroWidthJoiner;
            }
            _ => {
                prev_char_type = CharType::Regular;
            }
        }
        prev_char_index = i;
    }
    None
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UnicodeEscapeKind {
    // "\u{XXX}"
    // "\uXXXX"
    String,
    // /\u{XXX}/ and "\\u{XXX}"
    RegexBraced,
    // /\uXXX/ and "\\uXXX"
    RegexPlain,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct UnicodeEscape {
    codepoint: u32,
    kind: UnicodeEscapeKind,
    len: usize,
}

/// Convert unicode escape sequence string to unicode character
/// - unicode escape sequences: \u{XXXX}
/// - unicode escape sequences without parenthesis: \uXXXX
/// - surrogate pair: \uXXXX\uXXXX
///     If the unicode escape sequence is not valid, it will be treated as a simple string.
///
/// ```example
/// \uD83D\uDC4D -> 👍
/// \u0041\u0301 -> Á
/// \uD83D\uDC68\u200D\uD83D\uDC69\u200D\uD83D\uDC66 -> 👨‍👩‍👦
/// \u{1F468}\u{200D}\u{1F469}\u{200D}\u{1F466} -> 👨‍👩‍👦
/// \u899\uD83D\uDC4D -> \u899👍
/// ````
fn decode_next_codepoint(char_class: &str, is_in_string: bool) -> Option<(u32, usize)> {
    let c = char_class.chars().next()?;
    // `\u{XXX}`
    // `\uXXXX`
    let Some(UnicodeEscape {
        kind,
        codepoint,
        len,
    }) = decode_unicode_escape_sequence(char_class, is_in_string)
    else {
        // Ignore the escape sequence
        return Some((c as u32, c.len_utf8()));
    };
    if kind != UnicodeEscapeKind::RegexBraced
        && matches!(codepoint, 0xD800..=0xDBFF)
        && len <= char_class.len()
    {
        if let Some(UnicodeEscape {
            kind: low_kind,
            codepoint: low_codepoint,
            len: low_len,
        }) = decode_unicode_escape_sequence(&char_class[len..], is_in_string)
        {
            let (final_codepoint, final_len) = if kind == low_kind
                && matches!(low_codepoint, 0xDC00..=0xDFFF)
            {
                let surrogate = ((codepoint - 0xD800) << 10) + (low_codepoint - 0xDC00) + 0x10000;
                (surrogate, len + low_len)
            } else {
                (codepoint, len)
            };
            Some((final_codepoint, final_len))
        } else {
            Some((codepoint, len))
        }
    } else {
        Some((codepoint, len))
    }
}

fn decode_unicode_escape_sequence(s: &str, is_in_string: bool) -> Option<UnicodeEscape> {
    let bytes = s.as_bytes();
    if bytes.len() < 5 || bytes[0] != b'\\' {
        return None;
    }
    let (offset, is_regex_escape) = if is_in_string && bytes[1] == b'\\' {
        (1, true)
    } else {
        (0, !is_in_string)
    };
    if bytes[offset + 1] != b'u' {
        return None;
    }
    if bytes[offset + 2] == b'{' {
        let (end, _) = bytes
            .iter()
            .enumerate()
            .skip(offset + 3)
            .find(|(_, &c)| c == b'}')?;
        Some(UnicodeEscape {
            // SAFETY: slicing is safe because `{` is at `offset + 2` and `}` is at `end`.
            codepoint: u32::from_str_radix(&s[offset + 3..end], 16).ok()?,
            kind: if is_regex_escape {
                UnicodeEscapeKind::RegexBraced
            } else {
                UnicodeEscapeKind::String
            },
            len: end + 1,
        })
    } else {
        Some(UnicodeEscape {
            // We use `get` for slicing to handle malformed escape sequence that end with a multi-byte char.
            codepoint: u32::from_str_radix(s.get(offset + 2..offset + 6)?, 16).ok()?,
            kind: if is_regex_escape {
                UnicodeEscapeKind::RegexPlain
            } else {
                UnicodeEscapeKind::String
            },
            len: offset + 6,
        })
    }
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
                    let text = e.to_trimmed_string();
                    if text.starts_with('\'') {
                        Some(AnyJsCallArgument::AnyJsExpression(
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsStringLiteralExpression(
                                    make::js_string_literal_expression(make::js_string_literal(
                                        &format!("'{text}u'"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_unicode_escape_sequence() {
        assert_eq!(decode_unicode_escape_sequence(r"", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\\", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\\", true), None);
        assert_eq!(decode_unicode_escape_sequence(r"\n", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\u", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\uZ", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\u{", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\u{}", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\u{Z}", false), None);
        assert_eq!(decode_unicode_escape_sequence(r"\\u{31}", false), None);

        assert_eq!(
            decode_unicode_escape_sequence(r"\u0031 test", false),
            Some(UnicodeEscape {
                codepoint: 0x31,
                kind: UnicodeEscapeKind::RegexPlain,
                len: 6
            })
        );
        assert_eq!(
            decode_unicode_escape_sequence(r"\u0031 test", true),
            Some(UnicodeEscape {
                codepoint: 0x31,
                kind: UnicodeEscapeKind::String,
                len: 6
            })
        );
        assert_eq!(
            decode_unicode_escape_sequence(r"\\u0031 test", true),
            Some(UnicodeEscape {
                codepoint: 0x31,
                kind: UnicodeEscapeKind::RegexPlain,
                len: 7
            })
        );

        assert_eq!(
            decode_unicode_escape_sequence(r"\u{31} test", false),
            Some(UnicodeEscape {
                codepoint: 0x31,
                kind: UnicodeEscapeKind::RegexBraced,
                len: 6
            })
        );
        assert_eq!(
            decode_unicode_escape_sequence(r"\u{31} test", true),
            Some(UnicodeEscape {
                codepoint: 0x31,
                kind: UnicodeEscapeKind::String,
                len: 6
            })
        );
        assert_eq!(
            decode_unicode_escape_sequence(r"\\u{31} test", true),
            Some(UnicodeEscape {
                codepoint: 0x31,
                kind: UnicodeEscapeKind::RegexBraced,
                len: 7
            })
        );

        assert_eq!(
            decode_unicode_escape_sequence(r"\u{1} test", false),
            Some(UnicodeEscape {
                codepoint: 1,
                kind: UnicodeEscapeKind::RegexBraced,
                len: 5
            })
        );
    }

    #[test]
    fn test_decode_next_codepoint() {
        assert_eq!(decode_next_codepoint(r"", false), None);
        assert_eq!(decode_next_codepoint(r"1 test", false), Some((0x31, 1)));

        assert_eq!(
            decode_next_codepoint(r"\u0031\u0031", false),
            Some((0x31, 6))
        );
        assert_eq!(decode_next_codepoint(r"\u0031 test", true), Some((0x31, 6)));
        assert_eq!(
            decode_next_codepoint(r"\\u0031 test", true),
            Some((0x31, 7))
        );

        assert_eq!(
            decode_next_codepoint(r"\u{31}\u{31}", false),
            Some((0x31, 6))
        );
        assert_eq!(decode_next_codepoint(r"\u{31} test", true), Some((0x31, 6)));
        assert_eq!(
            decode_next_codepoint(r"\\u{31} test", true),
            Some((0x31, 7))
        );

        // Surrogate pairs
        assert_eq!(
            decode_next_codepoint(r"\uD83D\uDC4D", false),
            Some(('👍' as u32, 12))
        );
        assert_eq!(
            decode_next_codepoint(r"\uD83D\uDC4D", true),
            Some(('👍' as u32, 12))
        );
        assert_eq!(
            decode_next_codepoint(r"\\uD83D\\uDC4D", true),
            Some(('👍' as u32, 14))
        );
        assert_eq!(
            decode_next_codepoint(r"\u{D83D}\u{DC4D}", true),
            Some(('👍' as u32, 16))
        );
        assert_eq!(
            decode_next_codepoint(r"\uD83D\u{DC4D}", true),
            Some(('👍' as u32, 14))
        );
        assert_eq!(
            decode_next_codepoint(r"\u{D83D}\uDC4D", true),
            Some(('👍' as u32, 14))
        );

        // Lone high surrogate
        assert_eq!(
            decode_next_codepoint(r"\u{D83D}\u{DC4D}", false),
            Some((0xD83D, 8))
        );
        assert_eq!(
            decode_next_codepoint(r"\\u{D83D}\\u{DC4D}", true),
            Some((0xD83D, 9))
        );
        assert_eq!(
            decode_next_codepoint(r"\\uD83D\\u{DC4D}", true),
            Some((0xD83D, 7))
        );
        assert_eq!(
            decode_next_codepoint(r"\\u{D83D}\\uDC4D", true),
            Some((0xD83D, 9))
        );
        assert_eq!(
            decode_next_codepoint(r"\u{D83D}\\uDC4D", true),
            Some((0xD83D, 8))
        );
        assert_eq!(
            decode_next_codepoint(r"\uD83D\\uDC4D", true),
            Some((0xD83D, 6))
        );
    }
}
