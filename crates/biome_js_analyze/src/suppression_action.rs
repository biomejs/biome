use crate::utils::batch::JsBatchMutation;
use biome_analyze::SuppressionCommentEmitterPayload;
use biome_js_factory::make::{jsx_expression_child, jsx_ident, jsx_text, token};
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{
    AnyJsxChild, JsLanguage, JsSyntaxKind, JsSyntaxToken, JsxChildList, JsxElement,
    JsxOpeningElement, JsxSelfClosingElement, JsxText, TextRange, T,
};
use biome_rowan::{AstNode, TokenAtOffset, TriviaPieceKind};

/// Considering that the detection of suppression comments in the linter is "line based", the function starts
/// querying the node covered by the text range of the diagnostic, until it finds the first token that has a newline
/// among its leading trivia.
///
/// There are some edge cases:
/// - JSX elements might have newlines in their content;
/// - JS templates are an exception to the rule. JS templates might contain expressions inside their
/// content, and those expressions can contain diagnostics. The function uses the token `${` as boundary
/// and tries to place the suppression comment after it;
pub(crate) fn apply_suppression_comment(payload: SuppressionCommentEmitterPayload<JsLanguage>) {
    let SuppressionCommentEmitterPayload {
        token_offset,
        mutation,
        suppression_text,
        diagnostic_text_range,
    } = payload;
    // retrieve the most suited, most left token where the diagnostics was emitted
    let original_token = get_token_from_offset(token_offset, diagnostic_text_range);

    // considering that our suppression system works via lines, we need to look for the first newline,
    // so we can place the comment there
    let apply_suppression = original_token
        .as_ref()
        .map(|original_token| find_token_to_apply_suppression(original_token.clone()));

    if let Some(apply_suppression) = apply_suppression {
        let ApplySuppression {
            token_to_apply_suppression,
            token_has_trailing_comments,
            should_insert_leading_newline,
        } = apply_suppression;

        // we check if the token that has the newline is inside a JSX element: JsxOpeningElement or JsxSelfClosingElement
        let current_jsx_element = token_to_apply_suppression.parent().and_then(|parent| {
            if AnyJsxElement::can_cast(parent.kind()) || JsxText::can_cast(parent.kind()) {
                Some(parent)
            } else {
                None
            }
        });

        // When inside a JSX element, we have to apply different logics when applying suppression comments.
        // Newlines are inside JsxText.
        if let Some(current_jsx_element) = current_jsx_element {
            // quick check is the element is inside a list
            if current_jsx_element
                .parent()
                .map(|p| JsxChildList::can_cast(p.kind()))
                .unwrap_or_default()
            {
                let jsx_comment = jsx_expression_child(
                    token(T!['{']).with_trailing_trivia([(
                        TriviaPieceKind::SingleLineComment,
                        format!("/* {}: <explanation> */", suppression_text).as_str(),
                    )]),
                    token(T!['}']),
                )
                .build();
                if let Some(current_element) = JsxOpeningElement::cast_ref(&current_jsx_element) {
                    if let Some(parent) = current_element.parent::<JsxElement>() {
                        mutation.add_jsx_elements_before_element(
                            &parent.into(),
                            [AnyJsxChild::JsxExpressionChild(jsx_comment)],
                        );
                    }
                } else if let Some(current_element) =
                    JsxSelfClosingElement::cast_ref(&current_jsx_element)
                {
                    mutation.add_jsx_elements_before_element(
                        &AnyJsxChild::JsxSelfClosingElement(current_element),
                        [AnyJsxChild::JsxExpressionChild(jsx_comment)],
                    );
                } else if let Some(current_element) = JsxText::cast_ref(&current_jsx_element) {
                    // We want to add an additional JsxText to keep the indentation
                    let indentation_text = make_indentation_from_jsx_element(&current_element);
                    mutation.add_jsx_elements_after_element(
                        &AnyJsxChild::JsxText(current_element),
                        [
                            AnyJsxChild::JsxExpressionChild(jsx_comment),
                            AnyJsxChild::JsxText(indentation_text),
                        ],
                    );
                }
            } else {
                let mut new_token = token_to_apply_suppression.clone();
                if !should_insert_leading_newline {
                    new_token = new_token.with_leading_trivia([
                        (TriviaPieceKind::Newline, "\n"),
                        (
                            TriviaPieceKind::SingleLineComment,
                            format!("// {}: <explanation>", suppression_text).as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                } else {
                    new_token = new_token.with_leading_trivia([
                        (
                            TriviaPieceKind::SingleLineComment,
                            format!("// {}: <explanation>", suppression_text).as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                };
                mutation.replace_token_transfer_trivia(token_to_apply_suppression, new_token);
            }
        } else {
            let mut new_token = token_to_apply_suppression.clone();
            if !should_insert_leading_newline {
                if token_has_trailing_comments {
                    new_token = new_token.with_trailing_trivia([
                        (TriviaPieceKind::Newline, "\n"),
                        (
                            TriviaPieceKind::SingleLineComment,
                            format!("// {}: <explanation>", suppression_text).as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                } else {
                    new_token = new_token.with_leading_trivia([
                        (TriviaPieceKind::Newline, "\n"),
                        (
                            TriviaPieceKind::SingleLineComment,
                            format!("// {}: <explanation>", suppression_text).as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                }
            } else if token_has_trailing_comments {
                new_token = new_token.with_trailing_trivia([
                    (
                        TriviaPieceKind::SingleLineComment,
                        format!("// {}: <explanation>", suppression_text).as_str(),
                    ),
                    (TriviaPieceKind::Newline, "\n"),
                ])
            } else {
                let comment = format!("// {}: <explanation>", suppression_text);
                let mut trivia = vec![
                    (TriviaPieceKind::SingleLineComment, comment.as_str()),
                    (TriviaPieceKind::Newline, "\n"),
                ];
                let leading_whitespace: Vec<_> = new_token
                    .leading_trivia()
                    .pieces()
                    .filter(|p| p.is_whitespace())
                    .collect();

                for w in leading_whitespace.iter() {
                    trivia.push((TriviaPieceKind::Whitespace, w.text()));
                }
                new_token = new_token.with_leading_trivia(trivia);
            };
            mutation.replace_token_transfer_trivia(token_to_apply_suppression, new_token);
        }
    }
}

/// Convenient type to store useful information
struct ApplySuppression {
    /// If the token is following by trailing comments
    token_has_trailing_comments: bool,
    /// The token to apply attach the suppression
    token_to_apply_suppression: JsSyntaxToken,
    /// If the suppression should have a leading newline
    should_insert_leading_newline: bool,
}

/// It checks if the current token has leading trivia newline. If not, it
/// it peeks the previous token and recursively call itself.
///
/// Due to the nature of JSX, sometimes the current token might contain text that contains
/// some newline. In case that happens, we choose that token.
///
/// Due to the nature of JavaScript templates, we also check if the tokens we browse are
/// `${` and if so, we stop there.
fn find_token_to_apply_suppression(token: JsSyntaxToken) -> ApplySuppression {
    let mut apply_suppression = ApplySuppression {
        token_has_trailing_comments: false,
        token_to_apply_suppression: token.clone(),
        should_insert_leading_newline: false,
    };
    let mut current_token = token;
    let mut should_insert_leading_newline = loop {
        let trivia = current_token.leading_trivia();
        // There are some tokens that might contains newlines in their tokens, only
        // few nodes matches this criteria. If the token is inside one of those nodes,
        // then we check its content.
        let nodes_that_might_contain_newlines = current_token
            .parent()
            .map(|node| {
                matches!(
                    node.kind(),
                    JsSyntaxKind::JSX_TEXT
                        | JsSyntaxKind::JS_STRING_LITERAL
                        | JsSyntaxKind::TEMPLATE_CHUNK
                )
            })
            .unwrap_or_default();
        if current_token
            .trailing_trivia()
            .pieces()
            .any(|trivia| trivia.kind().is_multiline_comment())
        {
            break true;
        } else if trivia.pieces().any(|trivia| trivia.is_newline())
            || (nodes_that_might_contain_newlines
                && current_token.text_trimmed().contains(['\n', '\r']))
        {
            break false;
        } else if matches!(current_token.kind(), JsSyntaxKind::DOLLAR_CURLY) {
            if let Some(next_token) = current_token.next_token() {
                current_token = next_token;
                break false;
            }
        } else if let Some(token) = current_token.prev_token() {
            current_token = token;
        } else {
            break true;
        }
    };
    // If the flag has been set to `true`, it means we are at the beginning of the file.
    if !should_insert_leading_newline {
        // Still, if there's a a multiline comment, we want to try to attach the suppression comment
        // to the existing multiline comment without newlines.
        should_insert_leading_newline = current_token
            .leading_trivia()
            .pieces()
            .all(|piece| !piece.kind().is_multiline_comment());
    }

    apply_suppression.should_insert_leading_newline = should_insert_leading_newline;
    apply_suppression.token_has_trailing_comments = current_token
        .trailing_trivia()
        .pieces()
        .any(|trivia| trivia.kind().is_multiline_comment());
    apply_suppression.token_to_apply_suppression = current_token;

    apply_suppression
}

/// Finds the first token, starting with the current token and traversing backwards,
/// until if find one that has has a leading newline trivia.
///
/// Sometimes, the offset is between tokens, we need to decide which one to take.
///
/// For example:
/// ```jsx
/// function f() {
///     return <div
///     ><img /> {/* <--- diagnostic emitted in this line */}
///     </div>
/// }
/// ```
///
/// In these case it's best to peek the right token, because it belongs to the node where error actually occurred,
/// and becomes easier to add the suppression comment.
fn get_token_from_offset(
    token_offset: TokenAtOffset<JsSyntaxToken>,
    diagnostic_text_range: &TextRange,
) -> Option<JsSyntaxToken> {
    match token_offset {
        TokenAtOffset::None => None,
        TokenAtOffset::Single(token) => Some(token),
        TokenAtOffset::Between(left_token, right_token) => {
            let chosen_token = if right_token.text_range().start() == diagnostic_text_range.start()
            {
                right_token
            } else {
                left_token
            };
            Some(chosen_token)
        }
    }
}

/// Creates a new [JsxText], where its content are the computed spaces from `current_element`.
///
/// This new element will serve as trailing "newline" for the suppression comment.
fn make_indentation_from_jsx_element(current_element: &JsxText) -> JsxText {
    if let Ok(text) = current_element.value_token() {
        let chars = text.text().chars();
        let mut newlines = 0;
        let mut spaces = 0;
        let mut string_found = false;
        for char in chars {
            if char == '\"' {
                if string_found {
                    string_found = false;
                } else {
                    string_found = true;
                    continue;
                }
            }
            if string_found {
                continue;
            }

            if matches!(char, '\r' | '\n') {
                newlines += 1;
            }
            if matches!(char, ' ') && newlines == 1 && !string_found {
                spaces += 1;
            }
        }

        let content = format!("\n{}", " ".repeat(spaces));
        jsx_text(jsx_ident(content.as_str()))
    } else {
        jsx_text(jsx_ident("\n"))
    }
}
