use biome_graphql_syntax::{GraphqlLanguage, GraphqlSyntaxToken};
use biome_rowan::{AstNode, AstNodeList, SyntaxTriviaPiece, TokenText};
use biome_string_case::comparable_token::ComparableToken;
use std::cmp::Ordering;

/// Compares two names in natural order. A missing name sorts after every present name.
fn compare_names(a: Option<TokenText>, b: Option<TokenText>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => ComparableToken::new(a).ascii_nat_cmp(&ComparableToken::new(b)),
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

/// Returns whether `list` is already in the order that [`sort_list`] produces.
pub(crate) fn is_list_sorted<L>(list: &L, name: impl Fn(&L::Node) -> Option<TokenText>) -> bool
where
    L: AstNodeList<Language = GraphqlLanguage>,
{
    let mut previous = None;
    for item in list.iter() {
        if previous
            .as_ref()
            .is_some_and(|previous| compare_names(name(previous), name(&item)) == Ordering::Greater)
        {
            return false;
        }
        previous = Some(item);
    }
    true
}

type TriviaPieces = Vec<SyntaxTriviaPiece<GraphqlLanguage>>;

/// The trivia around an item of a list, split between the trivia that moves with the item
/// and the trivia that stays at its position in the list.
struct ItemTrivia<N> {
    node: N,
    /// The leading trivia before its first comment. It stays at the position of the item.
    position_leading: TriviaPieces,
    /// The leading trivia from its first comment onward. It moves with the item.
    attached_leading: TriviaPieces,
    trailing: TriviaPieces,
    /// Whether `trailing` contains an end-of-line comment, in which case the whole trailing
    /// trivia moves with the item instead of staying in place.
    trailing_is_attached: bool,
}

impl<N: AstNode<Language = GraphqlLanguage>> ItemTrivia<N> {
    fn new(node: N) -> Self {
        let syntax = node.syntax();
        let leading: TriviaPieces = syntax
            .first_token()
            .map(|token| token.leading_trivia().pieces().collect())
            .unwrap_or_default();
        let trailing: TriviaPieces = syntax
            .last_token()
            .map(|token| token.trailing_trivia().pieces().collect())
            .unwrap_or_default();

        let first_comment = leading
            .iter()
            .position(|piece| piece.is_comments())
            .unwrap_or(leading.len());
        let mut position_leading = leading;
        let attached_leading = position_leading.split_off(first_comment);
        let trailing_is_attached = trailing.iter().any(|piece| piece.is_comments());

        Self {
            node,
            position_leading,
            attached_leading,
            trailing,
            trailing_is_attached,
        }
    }

    /// Whether this position in the list starts on a new line.
    ///
    /// Leading trivia is either empty or starts with a newline, because the trivia up to the
    /// end of a line belongs to the trailing trivia of the previous token.
    fn position_on_new_line(&self) -> bool {
        self.position_leading
            .first()
            .is_some_and(|piece| piece.is_newline())
    }
}

/// Returns the items of `list` sorted by the name returned by `name`, or `None` when they can't
/// be sorted without placing a moved comment on the same line as other code.
///
/// `r_paren` is the token that closes the list. The end-of-line comment of the item that ends up
/// last must not be followed by `r_paren` on the same line.
///
/// Comments above an item and comments at the end of its line move with the item.
/// The other trivia, such as commas and the whitespace between items, stays in place.
pub(crate) fn sort_list<L, N>(
    list: &L,
    r_paren: &GraphqlSyntaxToken,
    name: impl Fn(&N) -> Option<TokenText>,
) -> Option<Vec<N>>
where
    L: AstNodeList<Language = GraphqlLanguage, Node = N>,
    N: AstNode<Language = GraphqlLanguage>,
{
    let r_paren_on_new_line = r_paren
        .leading_trivia()
        .pieces()
        .next()
        .is_some_and(|piece| piece.is_newline());
    let items: Vec<ItemTrivia<N>> = list.iter().map(ItemTrivia::new).collect();

    let mut order: Vec<usize> = (0..items.len()).collect();
    order.sort_by(|&a, &b| compare_names(name(&items[a].node), name(&items[b].node)));

    let mut nodes = Vec::with_capacity(items.len());
    for (position, &index) in order.iter().enumerate() {
        let slot = &items[position];
        let item = &items[index];

        // A comment above the item must stay on its own line. Otherwise it would follow
        // the previous token and describe a different item.
        if !item.attached_leading.is_empty() && !slot.position_on_new_line() {
            return None;
        }

        let trailing = if item.trailing_is_attached {
            // A line comment runs until the end of the line, so the code after it must start
            // on a new line. Otherwise the comment would swallow that code.
            let next_on_new_line = items
                .get(position + 1)
                .map_or(r_paren_on_new_line, ItemTrivia::position_on_new_line);
            if !next_on_new_line {
                return None;
            }
            item.trailing.clone()
        } else if slot.trailing_is_attached {
            Vec::new()
        } else {
            slot.trailing.clone()
        };

        let leading: TriviaPieces = slot
            .position_leading
            .iter()
            .chain(&item.attached_leading)
            .cloned()
            .collect();
        let node = item
            .node
            .clone()
            .with_leading_trivia_pieces(leading)?
            .with_trailing_trivia_pieces(trailing)?;
        nodes.push(node);
    }

    Some(nodes)
}
