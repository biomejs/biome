use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsRegexLiteralExpression, JsSyntaxKind, JsSyntaxToken};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TextSize};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unnecessary escape sequence in regular expression literals.
    ///
    /// Escaping non-special characters in regular expression literals doesn't have any effect.
    /// Hence, they may confuse a reader.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /\a/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /[\-]/;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /[\&]/v;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// /\^\d\b/
    /// ```
    ///
    /// ```js
    /// /[\b]/
    /// ```
    pub NoUselessEscapeInRegex {
        version: "1.9.0",
        name: "noUselessEscapeInRegex",
        language: "js",
        sources: &[RuleSource::Eslint("no-useless-escape")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessEscapeInRegex {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let (pattern, flags) = node.decompose().ok()?;
        let bytes = pattern.as_bytes();
        let mut byte_it = bytes.iter().enumerate();
        let has_v_flag = flags.text().as_bytes().contains(&b'v');
        let has_u_flag = flags.text().as_bytes().contains(&b'u');
        let is_unicode_aware = has_v_flag || has_u_flag;
        while let Some((index, byte)) = byte_it.next() {
            match byte {
                b'\\' => {
                    let Some((_, escaped)) = byte_it.next() else {
                        break;
                    };
                    match escaped {
                        b'\\'
                        | b'/'
                        // Anchrors
                        | b'^' | b'$'
                        // chartacaters sets
                        | b'.' | b'd' | b'D' | b'w' | b'W' | b's' | b'S' |
                        b't' | b'r' | b'n' | b'v' | b'f' | b'0' | b'c' | b'x' | b'u'
                        // char claass
                        | b'[' | b']'
                        // Word boundary
                        | b'b' | b'B'
                        // quantrifiers
                        | b'*' | b'+' | b'?' | b'{' | b'}'
                        // Backreferences
                        | b'1'..=b'9'
                        // Groups
                        | b'(' | b')'
                        // Alternation
                        | b'|' => {}
                        b'p' | b'P' | b'k' | b'q' if is_unicode_aware => {}
                        _ => {
                            return Some(State {
                                backslash_index: index as u16,
                                escaped: *escaped,
                                in_char_class: false,
                            });
                        }
                    }
                }
                b'[' => {
                    let char_class_start_index = index;
                    let mut inner_class_count = 0;
                    while let Some((index, byte)) = byte_it.next() {
                        match byte {
                            b'\\' => {
                                let Some((escaped_index, escaped)) = byte_it.next() else {
                                    break;
                                };
                                match escaped {
                                    // `^` can be escaped to avoid the negation of the char class.
                                    b'^' if escaped_index == (char_class_start_index + 2) => {}
                                    // No need to escape `-` at the start
                                    b'-' if has_v_flag || escaped_index != (char_class_start_index + 2) => {}
                                    b'\\'
                                    | b']'
                                    // chartacaters sets
                                    | b'd' | b'D' | b'w' | b'W' | b's' | b'S' |
                                    b't' | b'r' | b'n' | b'v' | b'f' | b'b' | b'0' |
                                    b'c' | b'x' | b'u' => {}
                                    b'p' | b'P' | b'k' | b'q' if is_unicode_aware => {}
                                    // Invalid speccial characters in char class under the `v` flag.
                                    b'(' | b')' | b'[' | b'{' | b'}' | b'/' | b'|' if has_v_flag => {}
                                    // Perhaps a doubled punctuator
                                    b'&' | b'!' | b'#' | b'$' | b'%' | b'*' | b'+' | b','
                                    | b'.' | b':' | b';' | b'<' | b'=' | b'>' | b'?'
                                    | b'@' | b'`' | b'~' if has_v_flag => {
                                        // SAFETY: there is at least one preceding character (`[`)
                                        if bytes[index-1] != *escaped && byte_it.next().is_none_or(|(_, byte)| byte != escaped) {
                                            return Some(State {
                                                backslash_index: index as u16,
                                                escaped: *escaped,
                                                in_char_class: true,
                                            });
                                        }
                                    }
                                    b'_' if has_v_flag => {
                                        // `[\_^^]`
                                        if byte_it.next().is_none_or(|(_, byte)| *byte != b'^') &&
                                            byte_it.next().is_none_or(|(_, byte)| *byte != b'^') {
                                            return Some(State {
                                                backslash_index: index as u16,
                                                escaped: *escaped,
                                                in_char_class: true,
                                            });
                                        }
                                    }
                                    b'^' if has_v_flag  => {
                                        let must_be_escaped =
                                            // `[_^\^]`
                                            // `[^^\^]`
                                            (
                                                matches!(bytes[index-2], b'_' | b'^')
                                                && bytes[index-1] == b'^'
                                            ) || (
                                                byte_it.next().is_some_and(|(_, byte)| *byte == b'^'
                                            ) && (
                                                // `[_\^^]`
                                                // `[^\^^]`
                                                matches!(bytes[index-1], b'_' | b'^') ||
                                                // `[\^^^]`
                                                byte_it.next().is_some_and(|(_, byte)| *byte == b'^')
                                            ));
                                        if !must_be_escaped {
                                            return Some(State {
                                                backslash_index: index as u16,
                                                escaped: *escaped,
                                                in_char_class: true,
                                            });
                                        }
                                    }
                                    _ => {
                                        return Some(State {
                                            backslash_index: index as u16,
                                            escaped: *escaped,
                                            in_char_class: true,
                                        });
                                    }
                                }
                            }
                            b'[' => {
                                if has_v_flag {
                                    inner_class_count += 1;
                                }
                            }
                            b']' => {
                                if has_v_flag && inner_class_count != 0 {
                                    inner_class_count -= 1;
                                } else if !has_v_flag
                                    && index >= 2 // handle edge case `[]`
                                    && bytes[index - 2] == b'\\'
                                    && bytes[index - 1] == b'-'
                                {
                                    return Some(State {
                                        backslash_index: (index - 2) as u16,
                                        escaped: b'-',
                                        in_char_class: false,
                                    });
                                } else {
                                    break;
                                }
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

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let State {
            backslash_index,
            escaped,
            in_char_class,
        } = state;
        // Add 1 because the index was computed in the pattern (it doesn't take `/` into account).
        let adjusted_backslash_index = (*backslash_index as u32) + 1;
        let node = ctx.query();
        let backslash_position = node.range().start() + TextSize::from(adjusted_backslash_index);
        // To compute the correct text range, we need the byte length of the escaped character.
        // To get that, we take a string slice from the escaped character and iterate until thenext character.
        // The index of the next character corresponds to the byte length of the escaped character.
        let (escaped_byte_len, _) = &node.value_token().ok()?.text_trimmed()
            [(adjusted_backslash_index as usize + 1)..]
            .char_indices()
            .nth(1)?;
        let diag = RuleDiagnostic::new(
            rule_category!(),
            TextRange::at(backslash_position, (1 + *escaped_byte_len as u32).into()),
            markup! {
                "The character doesn't need to be escaped."
            },
        );
        Some(if matches!(escaped, b'p' | b'P' | b'k') {
            diag.note("The escape sequence is only useful if the regular expression is unicode-aware. To be unicode-aware, the `u` or `v` flag should be used.")
        } else if *in_char_class {
            match escaped {
                b'^' => {
                    diag.note("The character should only be escaped if it is the first character of the class.")
                }
                b'B' => {
                    diag.note("The escape sequence only has meaning outside a character class.")
                }
                b'(' | b')' | b'[' | b'{' | b'}' | b'/' | b'|' => {
                    diag.note("The character should only be escaped if it is outside a character class or under the `v` flag.")
                }
                b'.' | b'$' | b'*' | b'+' | b'?' => {
                    diag.note("The character should only be escaped if it is outside a character class.")
                }
                b'-' => {
                    diag.note("The character should only be escaped if it appears in the middle of the character class or under the `v` flag.")
                }
                _ => diag,
            }
        } else {
            diag
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let State {
            backslash_index, ..
        } = state;
        // Add 1 because the index was computed in the pattern (it doesn't take `/` into account).
        let adjusted_backslash_index = (*backslash_index as usize) + 1;
        let node = ctx.query();
        let value_token = node.value_token().ok()?;
        let regex_text = value_token.text_trimmed();
        debug_assert!(
            regex_text.as_bytes().get(adjusted_backslash_index) == Some(&b'\\'),
            "backslash_index should points to a backslash."
        );
        let new_regex = JsSyntaxToken::new_detached(
            JsSyntaxKind::JS_REGEX_LITERAL,
            &format!(
                "{}{}",
                &regex_text[..adjusted_backslash_index],
                &regex_text[(adjusted_backslash_index + 1)..]
            ),
            [],
            [],
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_token(value_token, new_regex);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Unescape the character." }.to_owned(),
            mutation,
        ))
    }
}

pub struct State {
    backslash_index: u16,
    escaped: u8,
    in_char_class: bool,
}
