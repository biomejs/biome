use std::iter::empty;

use crate::syntax::SyntaxTrivia;
use crate::{Language, SyntaxElement, SyntaxToken, SyntaxTriviaPiece, chain_trivia_pieces};

#[derive(Clone, Copy)]
pub(super) enum TokenlessSource {
    PreserveDestination,
    ClearDestination,
}

pub(super) fn copy<L: Language>(
    source: &SyntaxElement<L>,
    destination: SyntaxElement<L>,
    tokenless: TokenlessSource,
) -> SyntaxElement<L> {
    match outer_trivia(source) {
        Some(source) => rewrite(
            destination,
            source.leading.pieces(),
            source.trailing.pieces(),
        ),
        None => match tokenless {
            TokenlessSource::PreserveDestination => destination,
            TokenlessSource::ClearDestination => rewrite(
                destination,
                empty::<SyntaxTriviaPiece<L>>(),
                empty::<SyntaxTriviaPiece<L>>(),
            ),
        },
    }
}

pub(super) fn transfer<L: Language>(
    source: &SyntaxElement<L>,
    destination: SyntaxElement<L>,
) -> Option<SyntaxElement<L>> {
    let source = outer_trivia(source)?;
    let destination_trivia = outer_trivia(&destination)?;

    Some(rewrite(
        destination,
        chain_trivia_pieces(source.leading.pieces(), destination_trivia.leading.pieces()),
        chain_trivia_pieces(
            source.trailing.pieces(),
            destination_trivia.trailing.pieces(),
        ),
    ))
}

pub(super) fn strip<L: Language>(element: SyntaxElement<L>) -> SyntaxElement<L> {
    rewrite(
        element,
        empty::<SyntaxTriviaPiece<L>>(),
        empty::<SyntaxTriviaPiece<L>>(),
    )
}

pub(super) fn surround<L: Language>(
    source: &SyntaxElement<L>,
    destination: SyntaxElement<L>,
) -> SyntaxElement<L> {
    if let Some(source) = outer_trivia(source)
        && let Some(destination_trivia) = outer_trivia(&destination)
    {
        rewrite(
            destination,
            chain_trivia_pieces(source.leading.pieces(), destination_trivia.leading.pieces()),
            chain_trivia_pieces(
                destination_trivia.trailing.pieces(),
                source.trailing.pieces(),
            ),
        )
    } else {
        destination
    }
}

pub(super) fn redirect_to_leading<L: Language>(
    source: &SyntaxElement<L>,
    destination: SyntaxToken<L>,
) -> SyntaxToken<L> {
    let destination_leading = destination.leading_trivia();
    match outer_trivia(source) {
        Some(source) => destination.with_leading_trivia_pieces(chain_trivia_pieces(
            chain_trivia_pieces(source.leading.pieces(), source.trailing.pieces()),
            destination_leading.pieces(),
        )),
        None => destination.with_leading_trivia_pieces(destination_leading.pieces()),
    }
}

struct OuterTrivia<L: Language> {
    leading: SyntaxTrivia<L>,
    trailing: SyntaxTrivia<L>,
}

fn outer_trivia<L: Language>(element: &SyntaxElement<L>) -> Option<OuterTrivia<L>> {
    Some(OuterTrivia {
        leading: element.leading_trivia()?,
        trailing: element.trailing_trivia()?,
    })
}

fn rewrite<L, Leading, Trailing>(
    element: SyntaxElement<L>,
    leading: Leading,
    trailing: Trailing,
) -> SyntaxElement<L>
where
    L: Language,
    Leading: IntoIterator<Item = SyntaxTriviaPiece<L>>,
    Leading::IntoIter: ExactSizeIterator,
    Trailing: IntoIterator<Item = SyntaxTriviaPiece<L>>,
    Trailing::IntoIter: ExactSizeIterator,
{
    match element {
        SyntaxElement::Node(node) => {
            let Some(first_token) = node.first_token() else {
                return SyntaxElement::Node(node);
            };
            let last_token = node
                .last_token()
                .expect("a node with a first token must have a last token");

            if first_token == last_token {
                let new_token = first_token.with_trivia_pieces(leading, trailing);
                let node = node
                    .replace_child(first_token.into(), new_token.into())
                    .expect("an edge token must belong to its node");
                SyntaxElement::Node(node)
            } else {
                let new_first_token = first_token.with_leading_trivia_pieces(leading);
                let node = node
                    .replace_child(first_token.into(), new_first_token.into())
                    .expect("an edge token must belong to its node");
                let last_token = node
                    .last_token()
                    .expect("a node with a first token must have a last token");
                let new_last_token = last_token.with_trailing_trivia_pieces(trailing);
                let node = node
                    .replace_child(last_token.into(), new_last_token.into())
                    .expect("an edge token must belong to its node");
                SyntaxElement::Node(node)
            }
        }
        SyntaxElement::Token(token) => {
            SyntaxElement::Token(token.with_trivia_pieces(leading, trailing))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenlessSource, copy, redirect_to_leading, strip, surround, transfer};
    use crate::raw_language::{LiteralExpression, RawLanguage, RawLanguageKind, RawLanguageRoot};
    use crate::{
        AstNode, AstNodeExt, BatchMutationExt, SyntaxElement, SyntaxNode, SyntaxNodeCast,
        SyntaxToken, TriviaPiece,
    };

    const SOURCE_LEADING: &str = "/*sl*/";
    const SOURCE_TRAILING: &str = "/*st*/";
    const DESTINATION_LEADING: &str = "/*dl*/";
    const DESTINATION_TRAILING: &str = "/*dt*/";

    fn token(trimmed: &str, leading: &str, trailing: &str) -> SyntaxToken<RawLanguage> {
        let text = format!("{leading}{trimmed}{trailing}");
        let leading = (!leading.is_empty())
            .then(|| TriviaPiece::multi_line_comment(leading.len() as u32))
            .into_iter();
        let trailing = (!trailing.is_empty())
            .then(|| TriviaPiece::multi_line_comment(trailing.len() as u32))
            .into_iter();
        SyntaxToken::new_detached(RawLanguageKind::STRING_TOKEN, &text, leading, trailing)
    }

    fn tokenless_node() -> SyntaxNode<RawLanguage> {
        SyntaxNode::new_detached(
            RawLanguageKind::LITERAL_EXPRESSION,
            [None::<SyntaxElement<RawLanguage>>],
        )
    }

    fn node(
        first: SyntaxToken<RawLanguage>,
        last: SyntaxToken<RawLanguage>,
    ) -> SyntaxNode<RawLanguage> {
        SyntaxNode::new_detached(
            RawLanguageKind::BOGUS,
            [Some(first.into()), Some(last.into())],
        )
    }

    fn text(element: &SyntaxElement<RawLanguage>) -> String {
        match element {
            SyntaxElement::Node(node) => node.to_string(),
            SyntaxElement::Token(token) => token.text().to_string(),
        }
    }

    fn root_with_literal(token: Option<SyntaxToken<RawLanguage>>) -> RawLanguageRoot {
        let literal = SyntaxNode::new_detached(
            RawLanguageKind::LITERAL_EXPRESSION,
            [token.map(SyntaxElement::Token)],
        );
        RawLanguageRoot::cast(SyntaxNode::new_detached(
            RawLanguageKind::ROOT,
            [Some(SyntaxElement::Node(literal))],
        ))
        .unwrap()
    }

    fn literal(root: &RawLanguageRoot) -> LiteralExpression {
        root.syntax().first_child().unwrap().cast().unwrap()
    }

    #[test]
    fn copy_distinguishes_tokenless_sources_from_empty_trivia() {
        let source = SyntaxElement::Node(tokenless_node());
        let destination =
            SyntaxElement::Token(token("d", DESTINATION_LEADING, DESTINATION_TRAILING));

        let preserved = copy(
            &source,
            destination.clone(),
            TokenlessSource::PreserveDestination,
        );
        assert_eq!(
            text(&preserved),
            format!("{DESTINATION_LEADING}d{DESTINATION_TRAILING}")
        );

        let cleared = copy(&source, destination, TokenlessSource::ClearDestination);
        assert_eq!(text(&cleared), "d");

        let empty_source = SyntaxElement::Token(token("s", "", ""));
        let copied = copy(
            &empty_source,
            SyntaxElement::Token(token("d", DESTINATION_LEADING, DESTINATION_TRAILING)),
            TokenlessSource::PreserveDestination,
        );
        assert_eq!(text(&copied), "d");
    }

    #[test]
    fn policies_preserve_their_trivia_order() {
        let source = SyntaxElement::Token(token("s", SOURCE_LEADING, SOURCE_TRAILING));
        let destination =
            || SyntaxElement::Token(token("d", DESTINATION_LEADING, DESTINATION_TRAILING));

        let copied = copy(&source, destination(), TokenlessSource::PreserveDestination);
        assert_eq!(text(&copied), format!("{SOURCE_LEADING}d{SOURCE_TRAILING}"));
        assert_eq!(text(&strip(source.clone())), "s");

        let transferred = transfer(&source, destination()).unwrap();
        assert_eq!(
            text(&transferred),
            format!(
                "{SOURCE_LEADING}{DESTINATION_LEADING}d{SOURCE_TRAILING}{DESTINATION_TRAILING}"
            )
        );

        let surrounded = surround(&source, destination());
        assert_eq!(
            text(&surrounded),
            format!(
                "{SOURCE_LEADING}{DESTINATION_LEADING}d{DESTINATION_TRAILING}{SOURCE_TRAILING}"
            )
        );

        let redirected = redirect_to_leading(
            &source,
            token("d", DESTINATION_LEADING, DESTINATION_TRAILING),
        );
        assert_eq!(
            redirected.text(),
            format!(
                "{SOURCE_LEADING}{SOURCE_TRAILING}{DESTINATION_LEADING}d{DESTINATION_TRAILING}"
            )
        );
    }

    #[test]
    fn rewriting_multi_token_nodes_preserves_inner_trivia() {
        let source = SyntaxElement::Node(node(
            token("s1", SOURCE_LEADING, "/*source-inner-left*/"),
            token("s2", "/*source-inner-right*/", SOURCE_TRAILING),
        ));
        let destination = SyntaxElement::Node(node(
            token("d1", DESTINATION_LEADING, "/*destination-inner-left*/"),
            token("d2", "/*destination-inner-right*/", DESTINATION_TRAILING),
        ));

        let result = copy(&source, destination, TokenlessSource::PreserveDestination);
        assert_eq!(
            text(&result),
            format!(
                "{SOURCE_LEADING}d1/*destination-inner-left*//*destination-inner-right*/d2{SOURCE_TRAILING}"
            )
        );
    }

    #[test]
    fn tokenless_elements_follow_each_operation_policy() {
        let tokenless = SyntaxElement::Node(tokenless_node());
        let destination =
            SyntaxElement::Token(token("d", DESTINATION_LEADING, DESTINATION_TRAILING));

        assert!(transfer(&tokenless, destination.clone()).is_none());
        assert!(transfer(&destination, SyntaxElement::Node(tokenless_node())).is_none());
        assert_eq!(text(&strip(tokenless.clone())), "");
        assert_eq!(
            text(&surround(&tokenless, destination.clone())),
            text(&destination)
        );
        assert_eq!(
            text(&surround(
                &destination,
                SyntaxElement::Node(tokenless_node())
            )),
            ""
        );
        assert_eq!(
            redirect_to_leading(
                &tokenless,
                token("d", DESTINATION_LEADING, DESTINATION_TRAILING),
            )
            .text(),
            format!("{DESTINATION_LEADING}d{DESTINATION_TRAILING}")
        );
    }

    #[test]
    fn immediate_and_batch_replacement_preserve_their_tokenless_source_behavior() {
        let immediate_source = root_with_literal(None);
        let replacement =
            root_with_literal(Some(token("d", DESTINATION_LEADING, DESTINATION_TRAILING)));
        let immediate = immediate_source
            .clone()
            .replace_node(literal(&immediate_source), literal(&replacement).detach())
            .unwrap();
        assert_eq!(
            immediate.syntax().to_string(),
            format!("{DESTINATION_LEADING}d{DESTINATION_TRAILING}")
        );

        let batch_source = root_with_literal(None);
        let mut batch = batch_source.clone().begin();
        batch.replace_node(literal(&batch_source), literal(&replacement).detach());
        assert_eq!(batch.commit().to_string(), "d");
    }

    #[test]
    fn batch_transfer_uses_the_transfer_policy() {
        let source = root_with_literal(Some(token("s", SOURCE_LEADING, SOURCE_TRAILING)));
        let replacement =
            root_with_literal(Some(token("d", DESTINATION_LEADING, DESTINATION_TRAILING)));

        let mut batch = source.clone().begin();
        assert_eq!(
            batch.replace_node_transfer_trivia(literal(&source), literal(&replacement).detach(),),
            Some(())
        );
        assert_eq!(
            batch.commit().to_string(),
            format!(
                "{SOURCE_LEADING}{DESTINATION_LEADING}d{SOURCE_TRAILING}{DESTINATION_TRAILING}"
            )
        );
    }
}
