use crate::utils::batch::JsBatchMutation;
use biome_analyze::{ApplySuppression, SuppressionAction};
use biome_js_factory::make::{jsx_expression_child, jsx_ident, jsx_text, token};
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{
    AnyJsxChild, JsLanguage, JsSyntaxKind, JsSyntaxToken, JsxChildList, JsxElement,
    JsxOpeningElement, JsxSelfClosingElement, JsxText, T,
};
use biome_rowan::{AstNode, BatchMutation, TriviaPieceKind};

/// Creates a new [JsxText], where its content are the computed spaces from `current_element`.
///
/// This new element will serve as trailing "newline" for the suppression comment.
fn make_indentation_from_jsx_element(current_element: &JsxText) -> JsxText {
    if let Ok(text) = current_element.value_token() {
        let bytes = text.text().bytes();
        let mut newlines = 0;
        let mut spaces = 0;
        let mut string_found = false;
        for byte in bytes {
            if byte == b'\"' {
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

            if matches!(byte, b'\r' | b'\n') {
                newlines += 1;
            }
            if matches!(byte, b' ') && newlines == 1 && !string_found {
                spaces += 1;
            }
        }

        let content = format!("\n{}", " ".repeat(spaces));
        jsx_text(jsx_ident(content.as_str()))
    } else {
        jsx_text(jsx_ident("\n"))
    }
}

pub struct JsSuppressionAction;

impl SuppressionAction for JsSuppressionAction {
    type Language = JsLanguage;

    fn apply_top_level_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        token: JsSyntaxToken,
        suppression_text: &str,
    ) {
        let new_token = token.with_leading_trivia([
            (
                TriviaPieceKind::SingleLineComment,
                format!("/** {suppression_text}: <explanation> */").as_str(),
            ),
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Newline, "\n"),
        ]);

        mutation.replace_token_discard_trivia(token, new_token);
    }

    fn find_token_for_inline_suppression(
        &self,
        token: JsSyntaxToken,
    ) -> Option<ApplySuppression<Self::Language>> {
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
            let nodes_that_might_contain_newlines = current_token.parent().is_some_and(|node| {
                matches!(
                    node.kind(),
                    JsSyntaxKind::JSX_TEXT
                        | JsSyntaxKind::JS_STRING_LITERAL
                        | JsSyntaxKind::TEMPLATE_CHUNK
                )
            });
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

        Some(apply_suppression)
    }

    /// Considering that the detection of suppression comments in the linter is "line based", the function starts
    /// querying the node covered by the text range of the diagnostic, until it finds the first token that has a newline
    /// among its leading trivia.
    ///
    /// There are some edge cases:
    /// - JSX elements might have newlines in their content;
    /// - JS templates are an exception to the rule. JS templates might contain expressions inside their
    ///     content, and those expressions can contain diagnostics. The function uses the token `${` as boundary
    ///     and tries to place the suppression comment after it;
    fn apply_inline_suppression(
        &self,
        mutation: &mut BatchMutation<Self::Language>,
        apply_suppression: ApplySuppression<Self::Language>,
        suppression_text: &str,
        suppression_reason: &str,
    ) {
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
                .is_some_and(|p| JsxChildList::can_cast(p.kind()))
            {
                let jsx_comment = jsx_expression_child(
                    token(T!['{']).with_trailing_trivia([(
                        TriviaPieceKind::SingleLineComment,
                        format!("/** {suppression_text}: {suppression_reason} */").as_str(),
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
                            format!("// {suppression_text}: {suppression_reason}").as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                } else {
                    new_token = new_token.with_leading_trivia([
                        (
                            TriviaPieceKind::SingleLineComment,
                            format!("// {suppression_text}: {suppression_reason}").as_str(),
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
                            format!("// {suppression_text}: {suppression_reason}").as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                } else {
                    new_token = new_token.with_leading_trivia([
                        (TriviaPieceKind::Newline, "\n"),
                        (
                            TriviaPieceKind::SingleLineComment,
                            format!("// {suppression_text}: {suppression_reason}").as_str(),
                        ),
                        (TriviaPieceKind::Newline, "\n"),
                    ])
                }
            } else if token_has_trailing_comments {
                new_token = new_token.with_trailing_trivia([
                    (
                        TriviaPieceKind::SingleLineComment,
                        format!("// {suppression_text}: {suppression_reason}").as_str(),
                    ),
                    (TriviaPieceKind::Newline, "\n"),
                ])
            } else {
                let comment = format!("// {suppression_text}: {suppression_reason}");
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
                // Trim trailing trivia to prevent double insertion of trailing whitespaces in `replace_token_transfer_trivia`.
                new_token = new_token.with_leading_trivia(trivia).trim_trailing_trivia();
            };
            mutation.replace_token_transfer_trivia(token_to_apply_suppression, new_token);
        }
    }
}
