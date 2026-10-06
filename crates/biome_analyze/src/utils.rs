use biome_rowan::{
    AstNode, AstSeparatedElement, AstSeparatedList, Language, SyntaxError, SyntaxNode, SyntaxToken,
    SyntaxTriviaPiece, TriviaPieceKind, chain_trivia_pieces, trim_leading_trivia_pieces,
    trim_trailing_trivia_pieces,
};
use std::cmp::Ordering;

/// Returns `true` if `list` is sorted by `get_key`.
/// The function returns an error if we encounter a buggy node or separator.
///
/// The list is divided into chunks of nodes with keys.
/// Thus, a node without key acts as a chuck delimiter.
/// Chunks are sorted separately.
pub fn is_separated_list_sorted_by<'a, L: Language + 'a, N: AstNode<Language = L> + 'a, Key>(
    list: &impl AstSeparatedList<Language = L, Node = N>,
    get_key: impl Fn(&N) -> Option<Key>,
    comparator: impl Fn(&Key, &Key) -> Ordering,
) -> Result<bool, SyntaxError> {
    let mut is_sorted = true;

    if list.len() > 1 {
        let mut previous_key: Option<Key> = None;
        for AstSeparatedElement {
            node,
            trailing_separator,
        } in list.elements()
        {
            // We have to check if the separator is not buggy.
            let _separator = trailing_separator?;
            previous_key = if let Some(key) = get_key(&node?) {
                if previous_key.is_some_and(|previous_key| comparator(&previous_key, &key).is_gt())
                {
                    // We don't return early because we want to return the error if we met one.
                    is_sorted = false;
                }
                Some(key)
            } else {
                // If a name cannot be extracted, then the current chunk stops here.
                None
            };
        }
    }
    Ok(is_sorted)
}

/// Returns the items and their separators resulting from sorting `list` by `get_key`.
/// When elements are reordered, `make_separator` is called to add missing separators in the middle of the list.
///
/// The list is divided into chunks of nodes with keys.
/// Thus, a node without key acts as a chuck delimiter.
/// Chunks are sorted separately.
///
/// This sort is stable (i.e., does not reorder equal elements).
///
/// When sorting moves a trailing single-line comment so that it would swallow
/// the token following it on the same line, a line break is inserted after the
/// comment. A single-line comment runs to the end of its line, so without the
/// break the following token would become part of the comment and the
/// reordered list would be invalid.
pub fn sorted_separated_list_by<'a, L, List, Node, Key>(
    list: &List,
    get_key: impl Fn(&Node) -> Option<Key>,
    make_separator: fn() -> SyntaxToken<L>,
    comparator: impl Fn(&Key, &Key) -> Ordering,
) -> Result<List, SyntaxError>
where
    L: Language + 'a,
    List: AstSeparatedList<Language = L, Node = Node> + AstNode<Language = L> + 'a,
    Node: AstNode<Language = L> + 'a,
{
    let mut elements = Vec::with_capacity(list.len());
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in list.elements()
    {
        let node = node?;
        let trailing_separator = trailing_separator?;
        elements.push((get_key(&node), node, trailing_separator));
    }

    // Iterate over chunks of node with a key
    for slice in elements.split_mut(|(key, _, _)| key.is_none()) {
        let last_has_separator = slice.last().is_some_and(|(_, _, sep)| sep.is_some());
        slice.sort_by(|(key1, _, _), (key2, _, _)| match (key1, key2) {
            (Some(k1), Some(k2)) => comparator(k1, k2),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => Ordering::Equal,
        });
        fix_separators(
            slice.iter_mut().map(|(_, node, sep)| (node, sep)),
            last_has_separator,
            make_separator,
        );
    }

    let separators: Vec<_> = elements
        .iter_mut()
        .filter_map(|(_, _, sep)| sep.take())
        .collect();
    let mut separators = separators.into_iter();
    let mut items = elements.into_iter().map(|(_, node, _)| node);

    let sorted = List::unwrap_cast(SyntaxNode::new_detached(
        list.syntax().kind(),
        (0..list.len() + separators.len()).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ));

    Ok(break_line_after_trailing_line_comments(list, &sorted).unwrap_or(sorted))
}

/// Breaks the line after a trailing single-line comment when sorting would
/// otherwise place the following token on the same line, where the comment
/// would swallow it.
///
/// Sorting carries each member together with its trailing separator (and the
/// separator's trivia). When that trivia ends with a single-line comment, the
/// token that ends up right after it on the same line would become part of the
/// comment. Breaking the line after the comment keeps the comment attached to
/// its member and the reordered list valid.
fn break_line_after_trailing_line_comments<'a, L, List, Node>(
    original_list: &List,
    sorted_list: &List,
) -> Option<List>
where
    L: Language + 'a,
    List: AstSeparatedList<Language = L, Node = Node> + AstNode<Language = L> + 'a,
    Node: AstNode<Language = L> + 'a,
{
    let indent = broken_line_indent(original_list);
    // The token after the list keeps its own leading trivia, so a comment
    // trailing the last token is only dangerous when that token doesn't start
    // on a new line.
    let following_starts_on_new_line = original_list
        .syntax()
        .next_sibling_or_token()
        .and_then(|element| element.into_token())
        .is_some_and(|token| {
            token
                .leading_trivia()
                .pieces()
                .any(|piece| piece.is_newline())
        });

    // Rebuilding the list allocates, so first check (without allocating)
    // whether any line break is needed at all.
    if !needs_any_break(sorted_list, following_starts_on_new_line)? {
        return Some(sorted_list.clone());
    }

    // Rebuild the list, breaking the line after single-line comments that
    // would otherwise swallow the following token. Members keep traveling with
    // their own trivia; only separators that need a break, and members that
    // then start on a new line, are touched.
    let mut break_before_next = false;
    let mut new_children: Vec<_> = Vec::new();
    let mut elements = sorted_list.elements().peekable();

    while let Some(element) = elements.next() {
        let node_syntax = element.node.ok()?.into_syntax();

        // The previous separator ended with a single-line comment: the member
        // now starts on a new line, so drop the whitespace that used to
        // separate it from the comment. Leading comments are kept.
        if break_before_next {
            break_before_next = false;
            if let Some(stripped) = without_leading_whitespace(&node_syntax) {
                new_children.push(Some(stripped.into()));
            } else {
                new_children.push(Some(node_syntax.into()));
            }
        } else {
            new_children.push(Some(node_syntax.into()));
        }

        if let Some(separator) = element.trailing_separator.ok()? {
            if separator_needs_break(&separator, elements.peek(), following_starts_on_new_line)? {
                new_children.push(Some(append_line_break(&separator, &indent).into()));
                break_before_next = true;
            } else {
                new_children.push(Some(separator.into()));
            }
        }
    }

    let syntax = List::unwrap_cast(SyntaxNode::new_detached(
        sorted_list.syntax().kind(),
        new_children,
    ))
    .into_syntax();

    let syntax = restore_boundary_layout(original_list, &syntax)?;

    // A single-line comment trailing the last member itself (no separator to
    // break after) would swallow the token after the list when that token
    // stays on the same line.
    if following_starts_on_new_line {
        return Some(List::unwrap_cast(syntax));
    }
    let last_token = syntax.last_token()?;
    let mut syntax = syntax;
    if let Some(new_token) = terminate_line_comment(&last_token, &indent) {
        syntax = syntax.replace_child(last_token.into(), new_token.into())?;
    }

    Some(List::unwrap_cast(syntax))
}

/// Whether any line break is needed: a separator ends with a single-line
/// comment that would swallow the following token, or the last token ends with
/// such a comment that would swallow the token after the list.
fn needs_any_break<'a, L, List, Node>(
    sorted_list: &List,
    following_starts_on_new_line: bool,
) -> Option<bool>
where
    L: Language + 'a,
    List: AstSeparatedList<Language = L, Node = Node> + AstNode<Language = L> + 'a,
    Node: AstNode<Language = L> + 'a,
{
    let mut elements = sorted_list.elements().peekable();
    while let Some(element) = elements.next() {
        if let Some(separator) = element.trailing_separator.ok()?
            && separator_needs_break(&separator, elements.peek(), following_starts_on_new_line)?
        {
            return Some(true);
        }
    }
    Some(
        !following_starts_on_new_line
            && sorted_list
                .syntax()
                .last_token()
                .is_some_and(|token| has_unterminated_line_comment(&token)),
    )
}

/// Whether a line break must be inserted after `separator`: it ends with a
/// single-line comment and the following token doesn't start on a new line.
fn separator_needs_break<'a, L, Node>(
    separator: &SyntaxToken<L>,
    next: Option<&AstSeparatedElement<L, Node>>,
    following_starts_on_new_line: bool,
) -> Option<bool>
where
    L: Language + 'a,
    Node: AstNode<Language = L> + 'a,
{
    if !has_line_comment(separator.trailing_trivia().pieces()) {
        return Some(false);
    }
    Some(!next_starts_on_new_line(
        next,
        following_starts_on_new_line,
    )?)
}

/// Whether the token following a separator starts on a new line. The last
/// separator is followed by the token after the list, which keeps its own
/// leading trivia.
fn next_starts_on_new_line<'a, L, Node>(
    next: Option<&AstSeparatedElement<L, Node>>,
    following_starts_on_new_line: bool,
) -> Option<bool>
where
    L: Language + 'a,
    Node: AstNode<Language = L> + 'a,
{
    match next {
        Some(next) => Some(
            next.node
                .as_ref()
                .ok()?
                .syntax()
                .first_token()
                .is_some_and(|token| {
                    token
                        .leading_trivia()
                        .pieces()
                        .any(|piece| piece.is_newline())
                }),
        ),
        None => Some(following_starts_on_new_line),
    }
}

/// Whether the pieces contain a single-line comment.
fn has_line_comment<L: Language>(mut pieces: impl Iterator<Item = SyntaxTriviaPiece<L>>) -> bool {
    pieces.any(|piece| piece.kind().is_single_line_comment())
}

/// Whether the token's trailing trivia ends with a single-line comment that
/// isn't terminated by a newline.
fn has_unterminated_line_comment<L: Language>(token: &SyntaxToken<L>) -> bool {
    let pieces = token.trailing_trivia().pieces();
    pieces
        .clone()
        .rposition(|piece| piece.kind().is_single_line_comment())
        .is_some_and(|index| !pieces.skip(index + 1).any(|piece| piece.is_newline()))
}

/// Appends a line break (plus `indent`) to the token's trailing trivia when it
/// ends with a single-line comment that isn't terminated by a newline.
/// Returns `None` when there's nothing to terminate.
fn terminate_line_comment<L: Language>(
    token: &SyntaxToken<L>,
    indent: &str,
) -> Option<SyntaxToken<L>> {
    has_unterminated_line_comment(token).then(|| append_line_break(token, indent))
}

/// Returns the node with leading whitespace and newlines dropped from its
/// first token, keeping any comments. Returns `None` when there's nothing to
/// drop.
fn without_leading_whitespace<L: Language>(node: &SyntaxNode<L>) -> Option<SyntaxNode<L>> {
    let first_token = node.first_token()?;
    let pieces = first_token.leading_trivia().pieces();
    let trimmed = trim_leading_trivia_pieces(pieces.clone());
    if trimmed.len() == pieces.len() {
        return None;
    }
    let new_token = first_token.with_leading_trivia_pieces(trimmed);
    node.clone()
        .replace_child(first_token.into(), new_token.into())
}

/// Appends a line break (plus `indent`) to the trailing trivia of `token`,
/// terminating any single-line comment it may end with.
fn append_line_break<L: Language>(token: &SyntaxToken<L>, indent: &str) -> SyntaxToken<L> {
    let old_pieces: Vec<SyntaxTriviaPiece<L>> = token.trailing_trivia().pieces().collect();
    let mut new_trailing: Vec<_> = Vec::with_capacity(old_pieces.len() + 2);
    new_trailing.extend(old_pieces.iter().map(|piece| (piece.kind(), piece.text())));
    new_trailing.push((TriviaPieceKind::Newline, "\n"));
    if !indent.is_empty() {
        new_trailing.push((TriviaPieceKind::Whitespace, indent));
    }
    token.with_trailing_trivia(new_trailing)
}

/// Indentation for lines broken after a single-line comment: the indentation
/// of the original members, falling back to the indentation of the line where
/// the token that opens the list starts.
fn broken_line_indent<'a, L, List, Node>(list: &List) -> String
where
    L: Language + 'a,
    List: AstSeparatedList<Language = L, Node = Node> + AstNode<Language = L> + 'a,
    Node: AstNode<Language = L> + 'a,
{
    let mut indent = String::new();
    if let Some(first_token) = list.syntax().first_token() {
        for piece in first_token.leading_trivia().pieces() {
            if piece.is_newline() {
                indent.clear();
            } else if piece.kind() == TriviaPieceKind::Whitespace {
                indent.push_str(piece.text());
            } else {
                // A comment or anything else: don't guess the indent.
                indent.clear();
                break;
            }
        }
    }
    if indent.is_empty() {
        // The whitespace after the opening token belongs to that token, so
        // align the broken lines with the line where the list starts instead.
        indent = enclosing_line_indent(list);
    }
    indent
}

/// Whitespace after the last newline preceding the token that opens `list`,
/// or empty when that token isn't preceded by a newline on its line.
fn enclosing_line_indent<'a, L, List, Node>(list: &List) -> String
where
    L: Language + 'a,
    List: AstSeparatedList<Language = L, Node = Node> + AstNode<Language = L> + 'a,
    Node: AstNode<Language = L> + 'a,
{
    let mut indent = String::new();
    let open_token = list
        .syntax()
        .prev_sibling_or_token()
        .and_then(|element| element.into_token());
    if let Some(open_token) = open_token {
        let mut seen_newline = false;
        for piece in open_token.leading_trivia().pieces() {
            if piece.is_newline() {
                seen_newline = true;
                indent.clear();
            } else if seen_newline && piece.kind() == TriviaPieceKind::Whitespace {
                indent.push_str(piece.text());
            }
        }
    }
    indent
}

/// Restores the boundary layout of the original list onto the sorted list.
///
/// Sorting carries each member (with its trivia) to a new position, so the
/// first and last tokens of the sorted list don't start and end the way the
/// original list did. Only the layout — whitespace and newlines preceding any
/// comment — is restored: comments stay attached to the member they were
/// written for and travel with it.
fn restore_boundary_layout<'a, L, List, Node>(
    original_list: &List,
    new_syntax: &SyntaxNode<L>,
) -> Option<SyntaxNode<L>>
where
    L: Language + 'a,
    List: AstSeparatedList<Language = L, Node = Node> + AstNode<Language = L> + 'a,
    Node: AstNode<Language = L> + 'a,
{
    let mut new_syntax = new_syntax.clone();

    if let (Some(original_first), Some(new_first)) = (
        original_list.syntax().first_token(),
        new_syntax.first_token(),
    ) {
        // Keep the member's own leading comments; only the indentation comes
        // from the original boundary.
        let merged = chain_trivia_pieces(
            LayoutTrivia::new(original_first.leading_trivia().pieces()),
            trim_leading_trivia_pieces(new_first.leading_trivia().pieces()),
        );
        let new_token = new_first.with_leading_trivia_pieces(merged);
        new_syntax = new_syntax.replace_child(new_first.into(), new_token.into())?;
    }

    if let (Some(original_last), Some(new_last)) =
        (original_list.syntax().last_token(), new_syntax.last_token())
    {
        let merged = chain_trivia_pieces(
            LayoutTrivia::new(original_last.trailing_trivia().pieces()),
            trim_leading_trivia_pieces(new_last.trailing_trivia().pieces()),
        );
        let new_token = new_last.with_trailing_trivia_pieces(merged);
        new_syntax = new_syntax.replace_child(new_last.into(), new_token.into())?;
    }

    Some(new_syntax)
}

/// Iterator over the whitespace and newline pieces that precede the first
/// comment of a trivia.
///
/// Unlike [`Iterator::filter`], this implements [`ExactSizeIterator`], so the
/// pieces can be attached to a token with the `*_pieces` APIs.
struct LayoutTrivia<I> {
    inner: I,
    len: usize,
}

impl<I, L> LayoutTrivia<I>
where
    L: Language,
    I: Iterator<Item = SyntaxTriviaPiece<L>> + Clone,
{
    fn new(pieces: I) -> Self {
        let len = pieces
            .clone()
            .take_while(|piece| piece.is_whitespace() || piece.is_newline())
            .count();
        Self { inner: pieces, len }
    }
}

impl<I, L> Iterator for LayoutTrivia<I>
where
    L: Language,
    I: Iterator<Item = SyntaxTriviaPiece<L>>,
{
    type Item = SyntaxTriviaPiece<L>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            // Fused: once the layout ends, stay ended even if the caller
            // keeps polling.
            return None;
        }
        let piece = self.inner.next()?;
        if piece.is_whitespace() || piece.is_newline() {
            self.len -= 1;
            Some(piece)
        } else {
            // The layout ends where the first comment starts.
            self.len = 0;
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<I, L> ExactSizeIterator for LayoutTrivia<I>
where
    L: Language,
    I: Iterator<Item = SyntaxTriviaPiece<L>>,
{
}

/// Fix the ordered sequence of nodes and separators adding missing separators and removing an extra separator.
///
/// If a separator is missing in the middle of the sequence, then a new one is created using `make_separator`.
/// If the last node has no separator, then a new one is created only if `needs_last_separator` is set to `true`.
/// If the last node has a separator and `needs_last_separator` is set to false, then the separator is removed.
/// The separator is always kept if some comments are attached.
///
/// This utility is notably useful when a delimited list with an optional last separator is reordered.
/// It allows you to add missing separators and remove an extra separator.
/// Usually, you collect every pair of nodes and separators in a vector and then pass a mutable iterator to `fix_separators`.
///
/// See [`sorted_separated_list_by`] as a usage example.
pub fn fix_separators<'a, L: Language + 'a, N: AstNode<Language = L> + 'a>(
    // Mutable iterator of a list of nodes and their optional separators
    iter: impl std::iter::ExactSizeIterator<Item = (&'a mut N, &'a mut Option<SyntaxToken<L>>)>,
    needs_last_separator: bool,
    make_separator: fn() -> SyntaxToken<L>,
) {
    let last_index = iter.len().saturating_sub(1);
    for (i, (node, optional_separator)) in iter.enumerate() {
        if let Some(separator) = optional_separator {
            // Remove the last separator at the separator has no attached comments
            if i == last_index
                && !(needs_last_separator
                    || separator.has_leading_comments()
                    || separator.has_trailing_comments())
            {
                // Transfer the separator trivia
                if let Some(new_node) = node.clone().append_trivia_pieces(chain_trivia_pieces(
                    separator.leading_trivia().pieces(),
                    separator.trailing_trivia().pieces(),
                )) {
                    *node = new_node;
                }
                *optional_separator = None;
            }
        } else if i != last_index || needs_last_separator {
            // The last node is moved and has no trailing separator.
            // Thus we build a new separator and remove its trailing trivia.
            *optional_separator = Some(match node.syntax().last_trailing_trivia() {
                // Transfer the trailing trivia to the separator
                Some(trivia) => make_separator()
                    .append_trivia_pieces(trim_trailing_trivia_pieces(trivia.pieces())),
                _ => make_separator(),
            });

            if let Some(new_node) = node.clone().with_trailing_trivia_pieces([]) {
                *node = new_node;
            }
        }
    }
}

/// Splits the list into two new lists according to a partitioning function.
///
/// Every item of `list` is passed to `partition` that decides if the item is
/// part of the left or the right returned list.
/// The `partition` function can change the passed item before returning it.
/// This allows supporting cases where the passed AST node must be modified.
///
/// This function returns `None` if it encounters a buggy item or
/// if `partition` returns `None` for at least one item.
///
/// The trailing separators are moved with their node.
pub fn split_separated_list<'a, L, List, Node>(
    list: &List,
    partition: impl Fn(Node) -> Option<either::Either<Node, Node>>,
) -> Option<(List, List)>
where
    L: Language + 'a,
    List: AstSeparatedList<Language = L, Node = Node> + AstNode<Language = L> + 'a,
    Node: AstNode<Language = L> + 'a,
{
    let mut left_items = Vec::with_capacity(list.len());
    let mut left_separators = Vec::with_capacity(list.len());
    let mut right_items = Vec::with_capacity(list.len());
    let mut right_separators = Vec::with_capacity(list.len());

    for AstSeparatedElement {
        node,
        trailing_separator,
    } in list.elements()
    {
        // Abort the split if a node is buggy or if `partition` returns `None`.
        let node = node.ok()?;
        let trailing_separator = trailing_separator.ok()?;
        let (items, separators, node) = match partition(node)? {
            either::Either::Left(node) => (&mut left_items, &mut left_separators, node),
            either::Either::Right(node) => (&mut right_items, &mut right_separators, node),
        };
        items.push(node);
        if let Some(trailing_separator) = trailing_separator {
            separators.push(trailing_separator);
        }
    }

    let mut left_items = left_items.into_iter();
    let mut left_separators = left_separators.into_iter();
    let left_list = List::unwrap_cast(SyntaxNode::new_detached(
        list.syntax().kind(),
        (0..left_items.len() + left_separators.len()).map(|index| {
            if index % 2 == 0 {
                Some(left_items.next()?.into_syntax().into())
            } else {
                Some(left_separators.next()?.into())
            }
        }),
    ));

    let mut right_items = right_items.into_iter();
    let mut right_separators = right_separators.into_iter();
    let right_list = List::unwrap_cast(SyntaxNode::new_detached(
        list.syntax().kind(),
        (0..right_items.len() + right_separators.len()).map(|index| {
            if index % 2 == 0 {
                Some(right_items.next()?.into_syntax().into())
            } else {
                Some(right_separators.next()?.into())
            }
        }),
    ));

    Some((left_list, right_list))
}

/// Counts lines in a syntax tree, used by `noExcessiveLinesPerFile`.
///
/// When `skip_blank_lines` is true, counts tokens with leading newlines (excluding blank lines).
/// When false, counts all newline trivia pieces in leading trivia.
/// EOF tokens and newlines inside comments or token text are excluded.
/// Returns total + 1 to account for the first line.
pub fn count_lines_in_file<L: Language>(
    node: &SyntaxNode<L>,
    is_eof_token: impl Fn(&SyntaxToken<L>) -> bool,
    skip_blank_lines: bool,
) -> usize {
    let mut count = 0;
    for descendant in node.descendants() {
        for token in descendant.tokens() {
            if is_eof_token(&token) {
                continue;
            }
            if skip_blank_lines {
                count += token.has_leading_newline() as usize;
            } else {
                count += token
                    .leading_trivia()
                    .pieces()
                    .filter(|piece| piece.is_newline())
                    .count();
            }
        }
    }
    count + 1
}

#[cfg(test)]
mod tests {
    use super::count_lines_in_file;
    use biome_rowan::{
        SyntaxNode, TriviaPiece,
        raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder},
    };

    fn assert_line_counts(node: &SyntaxNode<RawLanguage>, all: usize, non_blank: usize) {
        for (skip_blank_lines, expected) in [(false, all), (true, non_blank)] {
            assert_eq!(
                count_lines_in_file(
                    node,
                    |token| token.kind() == RawLanguageKind::EOF,
                    skip_blank_lines,
                ),
                expected,
                "skip_blank_lines={skip_blank_lines}, tree={node:#?}",
            );
        }
    }

    #[test]
    fn count_lines_in_file_empty_tree() {
        let root = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |_| {});
        assert_line_counts(&root, 1, 1);
    }

    #[test]
    fn count_lines_in_file_uses_only_non_eof_leading_newline_pieces() {
        for (newline, len) in [("\n", 1), ("\r\n", 2)] {
            let root = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
                builder.token(RawLanguageKind::NUMBER_TOKEN, "0");
                for _ in 0..2 {
                    builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
                    builder.token_with_trivia(
                        RawLanguageKind::STRING_TOKEN,
                        &format!("/*{newline}*/{newline}{newline} \"a{newline}b\" /*c*/{newline} "),
                        &[
                            TriviaPiece::multi_line_comment(4 + len),
                            TriviaPiece::newline(len),
                            TriviaPiece::newline(len),
                            TriviaPiece::whitespace(1),
                        ],
                        &[
                            TriviaPiece::whitespace(1),
                            TriviaPiece::multi_line_comment(5),
                            TriviaPiece::newline(len),
                            TriviaPiece::whitespace(1),
                        ],
                    );
                    builder.finish_node();
                }
                builder.token_with_trivia(
                    RawLanguageKind::EOF,
                    newline,
                    &[TriviaPiece::newline(len)],
                    &[],
                );
            });
            assert_line_counts(&root, 5, 3);
        }
    }
}
