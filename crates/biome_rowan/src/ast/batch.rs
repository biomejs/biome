use crate::syntax::SyntaxKind;
use crate::{
    chain_trivia_pieces, AstNode, Language, SyntaxElement, SyntaxNode, SyntaxSlot, SyntaxToken,
};
use biome_text_edit::{TextEdit, TextEditBuilder};
use biome_text_size::TextRange;
use std::{
    cmp,
    collections::BinaryHeap,
    iter::{empty, once},
};

pub trait BatchMutationExt<L>: AstNode<Language = L>
where
    L: Language,
{
    /// It starts a [BatchMutation]
    #[must_use = "This method consumes the node and return the BatchMutation api that returns the new SyntaxNode on commit"]
    fn begin(self) -> BatchMutation<L>;
}

impl<L, T> BatchMutationExt<L> for T
where
    L: Language,
    T: AstNode<Language = L>,
{
    #[must_use = "This method consumes the node and return the BatchMutation api that returns the new SyntaxNode on commit"]
    fn begin(self) -> BatchMutation<L> {
        BatchMutation::new(self.into_syntax())
    }
}

/// Stores the changes internally used by the [BatchMutation::commit] algorithm.
/// It needs to be sorted by depth in decreasing order, then by range start and
/// by slot in increasing order.
///
/// This is necesasry so we can aggregate all changes to the same node using "peek".
#[derive(Debug, Clone)]
struct CommitChange<L: Language> {
    parent_depth: usize,
    parent: Option<SyntaxNode<L>>,
    parent_range: Option<(u32, u32)>,
    new_node_slot: usize,
    new_node: Option<SyntaxElement<L>>,
    is_from_action: bool,
}

impl<L: Language> CommitChange<L> {
    /// Returns the "ordering key" for a change, controlling in what order this
    /// change will be applied relatively to other changes. The key consists of
    /// a tuple of numeric values representing the depth, parent start and slot
    /// index of the corresponding change.
    ///
    /// So, we order first by depth. Then by the range of the node. Then by the
    /// slot index of the node.
    ///
    /// The first is important to guarantee that all nodes that will be changed
    /// in the future are still valid with using SyntaxNode that we have.
    ///
    /// The second and third are essential to guarantee that the ".peek()" we do
    /// below is sufficient to see the same node in case of two or more nodes
    /// having the same parent.
    ///
    /// All of them will be prioritized in the descending order in a binary heap
    /// to ensure one change won't invalidate its following changes.
    fn key(&self) -> (usize, u32, usize) {
        (
            self.parent_depth,
            self.parent_range.map(|(start, _)| start).unwrap_or(0),
            self.new_node_slot,
        )
    }
}

impl<L: Language> PartialEq for CommitChange<L> {
    fn eq(&self, other: &Self) -> bool {
        self.key() == other.key()
    }
}
impl<L: Language> Eq for CommitChange<L> {}

impl<L: Language> PartialOrd for CommitChange<L> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<L: Language> Ord for CommitChange<L> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.key().cmp(&other.key())
    }
}

#[derive(Debug, Clone)]
pub struct BatchMutation<L>
where
    L: Language,
{
    root: SyntaxNode<L>,
    changes: BinaryHeap<CommitChange<L>>,
}

impl<L> BatchMutation<L>
where
    L: Language,
{
    pub fn new(root: SyntaxNode<L>) -> Self {
        Self {
            root,
            changes: BinaryHeap::new(),
        }
    }

    /// Push a change to replace the "prev_node" with "next_node".
    /// Trivia from "prev_node" is automatically copied to "next_node".
    ///
    /// Changes to take effect must be committed.
    pub fn replace_node<T>(&mut self, prev_node: T, next_node: T)
    where
        T: AstNode<Language = L>,
    {
        self.replace_element(
            prev_node.into_syntax().into(),
            next_node.into_syntax().into(),
        )
    }

    /// Push a change to replace the "prev_token" with "next_token".
    /// Trivia from "prev_token" is automatically copied to "next_token".
    ///
    /// Changes to take effect must be committed.
    pub fn replace_token(&mut self, prev_token: SyntaxToken<L>, next_token: SyntaxToken<L>) {
        self.replace_element(prev_token.into(), next_token.into())
    }

    /// Push a change to replace the "prev_element" with "next_element".
    /// Trivia from "prev_element" is automatically copied to "next_element".
    ///
    /// Changes to take effect must be committed.
    pub fn replace_element(
        &mut self,
        prev_element: SyntaxElement<L>,
        next_element: SyntaxElement<L>,
    ) {
        let (prev_leading_trivia, prev_trailing_trivia) = match &prev_element {
            SyntaxElement::Node(node) => (
                node.first_token().map(|token| token.leading_trivia()),
                node.last_token().map(|token| token.trailing_trivia()),
            ),
            SyntaxElement::Token(token) => {
                (Some(token.leading_trivia()), Some(token.trailing_trivia()))
            }
        };

        let next_element = match next_element {
            SyntaxElement::Node(mut node) => {
                if let Some(token) = node.first_token() {
                    let new_token = match prev_leading_trivia {
                        Some(prev_leading_trivia) => {
                            token.with_leading_trivia_pieces(prev_leading_trivia.pieces())
                        }
                        None => token.with_leading_trivia_pieces(empty()),
                    };

                    node = node.replace_child(token.into(), new_token.into()).unwrap();
                }

                if let Some(token) = node.last_token() {
                    let new_token = match prev_trailing_trivia {
                        Some(prev_trailing_trivia) => {
                            token.with_trailing_trivia_pieces(prev_trailing_trivia.pieces())
                        }
                        None => token.with_trailing_trivia_pieces([]),
                    };

                    node = node.replace_child(token.into(), new_token.into()).unwrap();
                }

                SyntaxElement::Node(node)
            }
            SyntaxElement::Token(token) => {
                let new_token = match prev_leading_trivia {
                    Some(prev_leading_trivia) => {
                        token.with_leading_trivia_pieces(prev_leading_trivia.pieces())
                    }
                    None => token.with_leading_trivia_pieces([]),
                };

                let new_token = match prev_trailing_trivia {
                    Some(prev_trailing_trivia) => {
                        new_token.with_trailing_trivia_pieces(prev_trailing_trivia.pieces())
                    }
                    None => new_token.with_trailing_trivia_pieces([]),
                };
                SyntaxElement::Token(new_token)
            }
        };

        self.push_change(prev_element, Some(next_element))
    }

    /// Push a change to replace the "prev_node" with "next_node".
    ///
    /// Changes to take effect must be committed.
    pub fn replace_node_discard_trivia<T>(&mut self, prev_node: T, next_node: T)
    where
        T: AstNode<Language = L>,
    {
        self.replace_element_discard_trivia(
            prev_node.into_syntax().into(),
            next_node.into_syntax().into(),
        )
    }

    /// Push a change to replace the "prev_token" with "next_token".
    ///
    /// Changes to take effect must be committed.
    pub fn replace_token_discard_trivia(
        &mut self,
        prev_token: SyntaxToken<L>,
        next_token: SyntaxToken<L>,
    ) {
        self.replace_element_discard_trivia(prev_token.into(), next_token.into())
    }

    /// Push a change to replace the "prev_token" with "next_token".
    ///
    /// - leading trivia of `prev_token`
    /// - leading trivia of `next_token`
    /// - trailing trivia of `prev_token`
    /// - trailing trivia of `next_token`
    pub fn replace_token_transfer_trivia(
        &mut self,
        prev_token: SyntaxToken<L>,
        next_token: SyntaxToken<L>,
    ) {
        let leading_trivia = chain_trivia_pieces(
            prev_token.leading_trivia().pieces(),
            next_token.leading_trivia().pieces(),
        );

        let trailing_trivia = chain_trivia_pieces(
            prev_token.trailing_trivia().pieces(),
            next_token.trailing_trivia().pieces(),
        );
        let new_token = next_token
            .with_leading_trivia_pieces(leading_trivia)
            .with_trailing_trivia_pieces(trailing_trivia);

        self.replace_token_discard_trivia(prev_token, new_token)
    }

    /// Push a change to replace the "prev_element" with "next_element".
    ///
    /// Changes to take effect must be committed.
    pub fn replace_element_discard_trivia(
        &mut self,
        prev_element: SyntaxElement<L>,
        next_element: SyntaxElement<L>,
    ) {
        self.push_change(prev_element, Some(next_element))
    }

    /// Push a change to remove the specified token.
    ///
    /// Changes to take effect must be committed.
    pub fn remove_token(&mut self, prev_token: SyntaxToken<L>) {
        self.remove_element(prev_token.into())
    }

    /// Push a change to remove the specified node.
    ///
    /// Changes to take effect must be committed.
    pub fn remove_node<T>(&mut self, prev_node: T)
    where
        T: AstNode<Language = L>,
    {
        self.remove_element(prev_node.into_syntax().into())
    }

    /// Push a change to remove the specified element.
    ///
    /// Changes to take effect must be committed.
    pub fn remove_element(&mut self, prev_element: SyntaxElement<L>) {
        self.push_change(prev_element, None)
    }

    fn push_change(
        &mut self,
        prev_element: SyntaxElement<L>,
        next_element: Option<SyntaxElement<L>>,
    ) {
        let new_node_slot = prev_element.index();
        let parent = prev_element.parent();
        let parent_range: Option<(u32, u32)> = parent.as_ref().map(|p| {
            let range = p.text_range_with_trivia();
            (range.start().into(), range.end().into())
        });
        let parent_depth = parent.as_ref().map(|p| p.ancestors().count()).unwrap_or(0);

        self.changes.push(CommitChange {
            parent_depth,
            parent,
            parent_range,
            new_node_slot,
            new_node: next_element,
            is_from_action: true,
        });
    }

    /// Returns the range of the document modified by this mutation along with
    /// a list of individual text edits to be performed on the source code, or
    /// [None] if the mutation is empty
    ///
    /// If the new tree is also required,
    /// please use `commit_with_text_range_and_edit`
    pub fn as_text_range_and_edit(self) -> Option<(TextRange, TextEdit)> {
        self.commit_with_text_range_and_edit(true).1
    }

    /// Returns the new tree with all commit changes applied.
    ///
    /// If the text range and text edit are also required,
    /// please use `commit_with_text_range_and_edit`
    pub fn commit(self) -> SyntaxNode<L> {
        self.commit_with_text_range_and_edit(false).0
    }

    /// The core of the batch mutation algorithm can be summarized as:
    ///
    /// 1. Iterate all requested changes;
    /// 2. Insert them into a heap (priority queue) by depth. Deeper changes are done first;
    /// 3. Loop popping requested changes from the heap, taking the deepest change we have for the moment;
    /// 4. Each requested change has a "parent", an "index" and the "new node" (or None);
    /// 5. Clone the current parent's "parent", the "grandparent";
    /// 6. Detach the current "parent" from the tree;
    /// 7. Replace the old node at "index" at the current "parent" with the current "new node";
    /// 8. Insert into the heap the grandparent as the parent and the current "parent" as the "new node";
    ///
    /// This is the simple case. The algorithm also has a more complex case when to changes have a common ancestor,
    /// which can actually be one of the changed nodes.
    ///
    /// To address this case at step 3, when we pop a new change to apply it, we actually aggregate all changes to the current
    /// parent together. This is done by the heap because we also sort by node and it's range.
    ///
    /// Text range and text edit can be collected simultaneously while committing if "with_text_range_and_edit" is true.
    /// They're directly calculated from the commit changes. So you can commit and get text range and text edit in one pass.
    ///
    /// The calculation of text range and text edit can be summarized as:
    ///
    /// While we popping requested changes from the heap, collect the "deleted_text_range" and "optional_inserted_text"
    /// into an ordered vector "text_mutation_list" sorted by the "deleted_text_range". The reason behind it is that
    /// changes on the heap are first ordered by parent depth, but we need to construct the TextEdit from start to end.
    /// So we use binary search and insertion to populate the "text_mutation_list". Reaching the root node means all
    /// changes have been visted. So we start to construct the TextEdit with the help of "text_edit_builder" by iterating
    /// the collected "text_mutation_list".
    pub fn commit_with_text_range_and_edit(
        self,
        with_text_range_and_edit: bool,
    ) -> (SyntaxNode<L>, Option<(TextRange, TextEdit)>) {
        let BatchMutation { root, mut changes } = self;

        // Ordered text mutation list sorted by text range
        let mut text_mutation_list: Vec<(TextRange, Option<String>)> =
            // SAFETY: this is safe bacause changes from actions can only
            // overwrite each other, so the total number of the finalized
            // text mutations will only be less.
            Vec::with_capacity(changes.len());

        // Collect all commit changes
        while let Some(CommitChange {
            new_node: curr_new_node,
            new_node_slot: curr_new_node_slot,
            parent: curr_parent,
            parent_depth: curr_parent_depth,
            is_from_action: curr_is_from_action,
            ..
        }) = changes.pop()
        {
            if let Some(curr_parent) = curr_parent {
                // This must be done before the detachment below
                // because we need nodes that are still valid in the old tree
                let curr_grand_parent = curr_parent.parent();
                let curr_grand_parent_range = curr_grand_parent.as_ref().map(|g| {
                    let range = g.text_range_with_trivia();
                    (range.start().into(), range.end().into())
                });
                let curr_parent_slot = curr_parent.index();

                // Aggregate all modifications to the current parent
                // This works because of the Ord we defined in the [CommitChange] struct
                let mut modifications =
                    vec![(curr_new_node_slot, curr_new_node, curr_is_from_action)];

                while changes
                    .peek()
                    .and_then(|c| c.parent.as_ref())
                    .is_some_and(|p| *p == curr_parent)
                {
                    // SAFETY: We can .pop().unwrap() because we .peek() above
                    let CommitChange {
                        new_node: next_new_node,
                        new_node_slot: next_new_node_slot,
                        is_from_action: next_is_from_action,
                        ..
                    } = changes.pop().expect("changes.pop");

                    // If we have two modifications to the same slot,
                    // last write wins
                    if let Some(&(prev_new_node_slot, ..)) = modifications.last() {
                        if prev_new_node_slot == next_new_node_slot {
                            modifications.pop();
                        }
                    }

                    // Add to the modifications
                    modifications.push((next_new_node_slot, next_new_node, next_is_from_action));
                }

                // Collect text mutations, this has to be done before the detach below,
                // or we'll lose the "deleted_text_range" info
                if with_text_range_and_edit {
                    for (new_node_slot, new_node, is_from_action) in &modifications {
                        if !is_from_action {
                            continue;
                        }
                        let deleted_text_range = match curr_parent.slots().nth(*new_node_slot) {
                            Some(SyntaxSlot::Node(node)) => node.text_range_with_trivia(),
                            Some(SyntaxSlot::Token(token)) => token.text_range(),
                            Some(SyntaxSlot::Empty { index }) => {
                                TextRange::new(index.into(), index.into())
                            }
                            None => continue,
                        };
                        let optional_inserted_text = new_node.as_ref().map(|n| n.to_string());

                        // We use binary search to keep the text mutations in order
                        match text_mutation_list
                            .binary_search_by(|(range, _)| range.ordering(deleted_text_range))
                        {
                            // Overwrite the text mutation with an overlapping text range
                            Ok(pos) => {
                                text_mutation_list[pos] =
                                    (deleted_text_range, optional_inserted_text)
                            }
                            // Insert the text mutation at the correct position
                            Err(pos) => text_mutation_list
                                .insert(pos, (deleted_text_range, optional_inserted_text)),
                        }
                    }
                }

                // Now we detach the current parent, commit all the modifications
                // and push a pending change to its parent
                let mut current_parent = curr_parent.detach();
                let is_list = current_parent.kind().is_list();
                for (new_node_slot, new_node, ..) in modifications {
                    current_parent = if is_list && new_node.is_none() {
                        current_parent.splice_slots(new_node_slot..=new_node_slot, empty())
                    } else {
                        current_parent.splice_slots(new_node_slot..=new_node_slot, once(new_node))
                    }
                }

                changes.push(CommitChange {
                    parent_depth: curr_parent_depth - 1,
                    parent: curr_grand_parent,
                    parent_range: curr_grand_parent_range,
                    new_node_slot: curr_parent_slot,
                    new_node: Some(SyntaxElement::Node(current_parent)),
                    is_from_action: false,
                });
            }
            // If parent is None, we reached the document root
            else {
                let optional_text_range_and_edit = if with_text_range_and_edit {
                    // The root of batch mutation is not necessarily
                    // the document root in some rule actions,
                    // so we need to find the actual document root
                    let mut document_root = root;
                    while let Some(parent) = document_root.parent() {
                        document_root = parent;
                    }

                    if curr_is_from_action {
                        text_mutation_list = vec![(
                            document_root.text_range_with_trivia(),
                            Some(
                                curr_new_node
                                    .as_ref()
                                    .map_or(String::new(), |n| n.to_string()),
                            ),
                        )];
                    }

                    // Build text range and text edit from the text mutation list
                    let root_string = document_root.to_string();
                    let mut text_range = TextRange::default();
                    let mut text_edit_builder = TextEditBuilder::default();

                    let mut pointer: usize = 0;
                    for (deleted_text_range, optional_inserted_text) in text_mutation_list {
                        if let (Ok(range_start), Ok(range_end)) = (
                            usize::try_from(u32::from(deleted_text_range.start())),
                            usize::try_from(u32::from(deleted_text_range.end())),
                        ) {
                            text_range = text_range.cover(deleted_text_range);
                            if range_start > pointer {
                                text_edit_builder.equal(&root_string[pointer..range_start]);
                            }

                            let old = &root_string[range_start..range_end];
                            let new = &optional_inserted_text.map_or(String::new(), |t| t);

                            text_edit_builder.with_unicode_words_diff(old, new);

                            pointer = range_end;
                        }
                    }
                    let end_pos = root_string.len();
                    if end_pos > pointer {
                        text_edit_builder.equal(&root_string[pointer..end_pos]);
                    }

                    let text_edit = text_edit_builder.finish();

                    Some((text_range, text_edit))
                } else {
                    None
                };

                return (
                    // SAFETY: If the change is propagated from the child,
                    // this will allways be a syntax node element because
                    // that's how we construct it above.
                    //
                    // Otherwise root should still exist as a node even if
                    // the code is to be transformed to an empty string.
                    curr_new_node
                        .expect("expected root to exist")
                        .into_node()
                        .expect("expected root to be a node and not a token"),
                    optional_text_range_and_edit,
                );
            }
        }

        (root, None)
    }

    pub fn root(&self) -> &SyntaxNode<L> {
        &self.root
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        raw_language::{LiteralExpression, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
        AstNode, BatchMutationExt, SyntaxNodeCast,
    };

    /// ```
    /// 0: ROOT@0..1
    ///     0: LITERAL_EXPRESSION@0..1
    ///         0: STRING_TOKEN@0..1 "a" [] []
    /// ```
    fn tree_one(a: &str) -> (RawLanguageRoot, String) {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder
            .start_node(RawLanguageKind::ROOT)
            .start_node(RawLanguageKind::LITERAL_EXPRESSION)
            .token(RawLanguageKind::STRING_TOKEN, a)
            .finish_node()
            .finish_node();
        let root = builder.finish().cast::<RawLanguageRoot>().unwrap();
        let s = format!("{:#?}", root.syntax());
        (root, s)
    }

    /// ```
    /// 0: ROOT@0..1
    ///     0: LITERAL_EXPRESSION@0..1
    ///         0: STRING_TOKEN@0..1 "a" [] []
    ///     1: LITERAL_EXPRESSION@0..1
    ///         0: STRING_TOKEN@0..1 "b" [] []
    /// ```
    fn tree_two(a: &str, b: &str) -> (RawLanguageRoot, String) {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder
            .start_node(RawLanguageKind::ROOT)
            .start_node(RawLanguageKind::LITERAL_EXPRESSION)
            .token(RawLanguageKind::STRING_TOKEN, a)
            .finish_node()
            .start_node(RawLanguageKind::LITERAL_EXPRESSION)
            .token(RawLanguageKind::STRING_TOKEN, b)
            .finish_node()
            .finish_node();
        let root = builder.finish().cast::<RawLanguageRoot>().unwrap();
        let s = format!("{:#?}", root.syntax());
        (root, s)
    }

    fn find(root: &RawLanguageRoot, name: &str) -> LiteralExpression {
        root.syntax()
            .descendants()
            .find(|x| x.kind() == RawLanguageKind::LITERAL_EXPRESSION && x.text_trimmed() == name)
            .unwrap()
            .cast::<LiteralExpression>()
            .unwrap()
    }

    fn clone_detach(root: &RawLanguageRoot, name: &str) -> LiteralExpression {
        root.syntax()
            .descendants()
            .find(|x| x.kind() == RawLanguageKind::LITERAL_EXPRESSION && x.text_trimmed() == name)
            .unwrap()
            .detach()
            .cast::<LiteralExpression>()
            .unwrap()
    }

    #[test]
    pub fn ok_batch_mutation_no_changes() {
        let (before, before_debug) = tree_one("a");

        let batch = before.begin();
        let after = batch.commit();

        assert_eq!(before_debug, format!("{after:#?}"));
    }

    #[test]
    pub fn ok_batch_mutation_one_change() {
        let (before, _) = tree_one("a");
        let (expected, expected_debug) = tree_one("b");

        let a = find(&before, "a");
        let b = clone_detach(&expected, "b");

        let mut batch = before.begin();
        batch.replace_node(a, b);
        let root = batch.commit();

        assert_eq!(expected_debug, format!("{root:#?}"));
    }

    #[test]
    pub fn ok_batch_mutation_multiple_changes_different_branches() {
        let (before, _) = tree_two("a", "b");
        let (expected, expected_debug) = tree_two("c", "d");

        let a = find(&before, "a");
        let b = find(&before, "b");
        let c = clone_detach(&expected, "c");
        let d = clone_detach(&expected, "d");

        let mut batch = before.begin();
        batch.replace_node(a, c);
        batch.replace_node(b, d);
        let after = batch.commit();

        assert_eq!(expected_debug, format!("{after:#?}"));
    }
}
