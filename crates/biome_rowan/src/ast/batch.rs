use crate::syntax::SyntaxKind;
use crate::{
    AstNode, Language, SyntaxElement, SyntaxNode, SyntaxSlot, SyntaxToken, chain_trivia_pieces,
};
use biome_text_edit::{TextEdit, TextEditBuilder};
use biome_text_size::{TextRange, TextSize};
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
    fn begin(self) -> BatchMutation<L> {
        BatchMutation::new(self.into_syntax())
    }
}

/// Stores the changes internally used by the [BatchMutation::commit] algorithm.
/// It needs to be sorted by depth in decreasing order, then by range start and
/// by slot in increasing order. The mutation order breaks ties between
/// independent insertions at the same list slot.
///
/// This is necessary so we can aggregate all changes to the same node using "peek".
#[derive(Debug, Clone)]
struct CommitChange<L: Language> {
    parent_depth: usize,
    parent: Option<SyntaxNode<L>>,
    parent_range: Option<(u32, u32)>,
    new_node_slot: usize,
    new_node: Option<SyntaxElement<L>>,
    is_from_action: bool,
    is_insertion: bool,
    is_header_transfer: bool,
    is_header_cleanup: bool,
    previous_range: Option<TextRange>,
    order: usize,
}

impl<L: Language> CommitChange<L> {
    /// Returns the "ordering key" for a change, controlling in what order this
    /// change will be applied relatively to other changes. The key consists of
    /// a tuple of numeric values representing the depth, parent start and slot
    /// index of the corresponding change.
    ///
    /// So, we order first by depth. Then by the range of the node. Then by the
    /// slot index of the node and the mutation order.
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
    fn key(&self) -> (usize, u32, usize, u8, usize) {
        (
            self.parent_depth,
            self.parent_range.map(|(start, _)| start).unwrap_or(0),
            self.new_node_slot,
            u8::from(!self.is_insertion),
            self.order,
        )
    }

    /// Reports whether both changes insert identical elements into the same parent slot.
    fn is_duplicate_insertion(&self, other: &Self) -> bool {
        self.is_insertion
            && other.is_insertion
            && self.parent == other.parent
            && self.new_node_slot == other.new_node_slot
            && match (&self.new_node, &other.new_node) {
                (Some(left), Some(right)) => {
                    left.kind() == right.kind() && left.to_string() == right.to_string()
                }
                (None, None) => true,
                _ => false,
            }
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
    next_order: usize,
}

impl<L> BatchMutation<L>
where
    L: Language,
{
    #[cfg(debug_assertions)]
    fn debug_assert_slot_compatibility(
        parent: Option<&SyntaxNode<L>>,
        slot_index: usize,
        prev_element: &SyntaxElement<L>,
        next_element: Option<&SyntaxElement<L>>,
    ) {
        let Some(parent) = parent else {
            return;
        };

        let Some(slot) = parent.slots().nth(slot_index) else {
            debug_assert!(
                false,
                "batch mutation targeted missing slot {slot_index} in parent kind {:?}: {parent:?}",
                parent.kind()
            );
            return;
        };

        let prev_is_node = matches!(prev_element, SyntaxElement::Node(_));
        let prev_kind = match prev_element {
            SyntaxElement::Node(node) => format!("{:?}", node.kind()),
            SyntaxElement::Token(token) => format!("{:?}", token.kind()),
        };
        let slot_is_node = matches!(slot, SyntaxSlot::Node(_));
        let slot_is_token = matches!(slot, SyntaxSlot::Token(_));
        let slot_kind = match &slot {
            SyntaxSlot::Node(node) => Some(format!("{:?}", node.kind())),
            SyntaxSlot::Token(token) => Some(format!("{:?}", token.kind())),
            SyntaxSlot::Empty { .. } => None,
        };

        debug_assert!(
            (prev_is_node && slot_is_node) || (!prev_is_node && slot_is_token),
            "batch mutation targeted a {} {:?} in slot {slot_index} of parent kind {:?}, but the current slot contains a {} {:?}: {parent:?}",
            if prev_is_node { "node" } else { "token" },
            prev_kind,
            parent.kind(),
            match &slot {
                SyntaxSlot::Node(_) => "node",
                SyntaxSlot::Token(_) => "token",
                SyntaxSlot::Empty { .. } => "missing element",
            },
            slot_kind
        );

        let Some(next_element) = next_element else {
            return;
        };

        let next_kind = match next_element {
            SyntaxElement::Node(node) => format!("{:?}", node.kind()),
            SyntaxElement::Token(token) => format!("{:?}", token.kind()),
        };

        debug_assert!(
            matches!(
                (prev_element, next_element),
                (SyntaxElement::Node(_), SyntaxElement::Node(_))
                    | (SyntaxElement::Token(_), SyntaxElement::Token(_))
            ),
            "batch mutation attempted to replace a {} {:?} with a {} {:?} in parent kind {:?}; use the matching token/node replacement API",
            if prev_is_node { "node" } else { "token" },
            prev_kind,
            if matches!(next_element, SyntaxElement::Node(_)) {
                "node"
            } else {
                "token"
            },
            next_kind,
            parent.kind()
        );
    }

    pub fn new(root: SyntaxNode<L>) -> Self {
        Self {
            root,
            changes: BinaryHeap::new(),
            next_order: 0,
        }
    }

    /// Merge all changes from `other` into this mutation.
    ///
    /// Both mutations must reference nodes from the same tree. The merged
    /// changes are applied together in a single [`Self::commit`] call.
    /// Non-overlapping changes combine naturally. If two changes target the
    /// same slot of the same parent, the existing last-write-wins semantics
    /// apply.
    pub fn merge(&mut self, other: Self) {
        debug_assert!(
            self.root == other.root,
            "Cannot merge mutations from different trees"
        );
        self.changes.extend(other.changes);
    }

    /// Merge mutations produced by separate analyzer actions.
    ///
    /// Action mutations need additional ordering and trivia handling because
    /// they are collected independently and applied together during fix-all.
    /// This keeps identical list insertions from being repeated, coalesces
    /// header cleanup with replacements, and preserves the order in which
    /// actions were collected.
    pub fn merge_actions(&mut self, other: Self) {
        debug_assert!(
            self.root == other.root,
            "Cannot merge mutations from different trees"
        );
        let order_offset = self.next_order;
        self.next_order += other.next_order;
        let mut existing_changes = std::mem::take(&mut self.changes).into_vec();
        let incoming_changes = other.changes.into_vec();
        let incoming_replacement_ranges = incoming_changes
            .iter()
            .filter(|change| {
                !change.is_insertion && !change.is_header_cleanup && change.new_node.is_some()
            })
            .filter_map(|change| change.previous_range)
            .collect::<Vec<_>>();
        let existing_replacement_ranges = existing_changes
            .iter()
            .filter(|change| {
                !change.is_insertion && !change.is_header_cleanup && change.new_node.is_some()
            })
            .filter_map(|change| change.previous_range)
            .collect::<Vec<_>>();
        existing_changes.retain(|change| {
            !change.is_header_cleanup
                || !change
                    .previous_range
                    .is_some_and(|range| incoming_replacement_ranges.contains(&range))
        });
        let existing_insertions = existing_changes
            .iter()
            .filter(|change| change.is_insertion)
            .cloned()
            .collect::<Vec<_>>();
        let existing_header_transfers = existing_changes
            .iter()
            .filter(|change| change.is_header_transfer)
            .filter_map(|change| {
                change
                    .parent
                    .clone()
                    .map(|parent| (parent, change.new_node_slot))
            })
            .collect::<Vec<_>>();
        self.changes.extend(existing_changes);
        self.changes
            .extend(incoming_changes.into_iter().filter_map(|mut change| {
                if existing_insertions
                    .iter()
                    .any(|existing| existing.is_duplicate_insertion(&change))
                {
                    return None;
                }
                change.order += order_offset;
                if change.is_header_cleanup
                    && change
                        .previous_range
                        .is_some_and(|range| existing_replacement_ranges.contains(&range))
                {
                    return None;
                }
                let has_matching_header_transfer = change.is_header_transfer
                    && existing_header_transfers.iter().any(|(parent, slot)| {
                        *slot == change.new_node_slot
                            && change
                                .parent
                                .as_ref()
                                .is_some_and(|change_parent| change_parent == parent)
                    });
                if has_matching_header_transfer {
                    change.new_node = change.new_node.map(Self::remove_leading_trivia);
                    change.is_header_transfer = false;
                }
                Some(change)
            }));
    }

    /// Removes leading trivia from the first token of a node or from a token element.
    fn remove_leading_trivia(element: SyntaxElement<L>) -> SyntaxElement<L> {
        match element {
            SyntaxElement::Node(node) => node
                .first_token()
                .and_then(|token| {
                    let replacement = token.with_leading_trivia_pieces([]);
                    node.clone().replace_child(token.into(), replacement.into())
                })
                .map_or_else(|| node.into(), SyntaxElement::Node),
            SyntaxElement::Token(token) => {
                SyntaxElement::Token(token.with_leading_trivia_pieces([]))
            }
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

    /// Push a change that inserts an element into a list at `slot_index`.
    ///
    /// List insertions are kept separate from replacements so independent
    /// mutations can be merged without one list replacement overwriting the other.
    pub fn insert_element(
        &mut self,
        parent: SyntaxNode<L>,
        slot_index: usize,
        new_element: SyntaxElement<L>,
    ) {
        self.insert_element_internal(parent, slot_index, new_element, false);
    }

    /// Push a list insertion whose leading trivia came from the original first item.
    ///
    /// When two such mutations are merged, the first insertion retains the header and later
    /// insertions have only their copied leading trivia removed.
    pub fn insert_element_with_header(
        &mut self,
        parent: SyntaxNode<L>,
        slot_index: usize,
        new_element: SyntaxElement<L>,
    ) {
        self.insert_element_internal(parent, slot_index, new_element, true);
    }

    /// Records a list insertion together with its slot, merge order, and header-transfer state.
    fn insert_element_internal(
        &mut self,
        parent: SyntaxNode<L>,
        slot_index: usize,
        new_element: SyntaxElement<L>,
        is_header_transfer: bool,
    ) {
        debug_assert!(parent.kind().is_list());
        debug_assert!(slot_index <= parent.slots().count());

        let parent_range = parent.text_range_with_trivia();
        self.changes.push(CommitChange {
            parent_depth: parent.ancestors().count(),
            parent: Some(parent),
            parent_range: Some((parent_range.start().into(), parent_range.end().into())),
            new_node_slot: slot_index,
            new_node: Some(new_element),
            is_from_action: true,
            is_insertion: true,
            is_header_transfer,
            is_header_cleanup: false,
            previous_range: None,
            order: self.next_order,
        });
        self.next_order += 1;
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

    /// Pushes a leading-trivia cleanup that can be coalesced with a merged
    /// replacement of the same syntax element.
    pub fn clear_leading_trivia(&mut self, token: SyntaxToken<L>) {
        let next_token = token.with_leading_trivia_pieces([]);
        self.push_header_cleanup_change(token.into(), Some(next_token.into()));
    }

    /// Push a change to replace the "prev_node" with "next_node".
    ///
    /// - leading trivia of `prev_node`
    /// - leading trivia of `next_node`
    /// - trailing trivia of `prev_node`
    /// - trailing trivia of `next_node`
    pub fn replace_node_transfer_trivia<T>(&mut self, prev_node: T, next_node: T) -> Option<()>
    where
        T: AstNode<Language = L>,
    {
        let prev_node = prev_node.into_syntax();
        let next_node = next_node.into_syntax();

        let leading_trivia = chain_trivia_pieces(
            prev_node.first_token()?.leading_trivia().pieces(),
            next_node.first_token()?.leading_trivia().pieces(),
        );

        let trailing_trivia = chain_trivia_pieces(
            prev_node.last_token()?.trailing_trivia().pieces(),
            next_node.last_token()?.trailing_trivia().pieces(),
        );
        let new_node = next_node
            .with_leading_trivia_pieces(leading_trivia)?
            .with_trailing_trivia_pieces(trailing_trivia)?;

        self.replace_element_discard_trivia(prev_node.into(), new_node.into());

        Some(())
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

    /// Push a change to remove the specified node, transferring its trivia
    /// (leading + trailing combined) to the next token's leading trivia.
    /// The next token always exists (EOF token can have leading trivia).
    pub fn remove_node_keep_trivia<T>(&mut self, prev_node: T)
    where
        T: AstNode<Language = L>,
    {
        let node = prev_node.into_syntax();

        let leading_pieces = node
            .first_leading_trivia()
            .map(|t| t.pieces().collect::<Vec<_>>())
            .unwrap_or_default();
        let trailing_pieces = node
            .last_trailing_trivia()
            .map(|t| t.pieces().collect::<Vec<_>>())
            .unwrap_or_default();

        let combined_trivia =
            chain_trivia_pieces(leading_pieces.into_iter(), trailing_pieces.into_iter());

        if let Some(next_token) = node.last_token().and_then(|t| t.next_token()) {
            let new_next_token = next_token.with_leading_trivia_pieces(chain_trivia_pieces(
                combined_trivia,
                next_token.leading_trivia().pieces(),
            ));
            self.replace_token_discard_trivia(next_token, new_next_token);
        }

        self.remove_element(node.into());
    }

    fn push_change(
        &mut self,
        prev_element: SyntaxElement<L>,
        next_element: Option<SyntaxElement<L>>,
    ) {
        self.push_change_internal(prev_element, next_element, false);
    }

    /// Records a leading-trivia removal that can be coalesced with a later replacement.
    fn push_header_cleanup_change(
        &mut self,
        prev_element: SyntaxElement<L>,
        next_element: Option<SyntaxElement<L>>,
    ) {
        self.push_change_internal(prev_element, next_element, true);
    }

    /// Records a replacement or removal and preserves the metadata needed to merge it safely.
    fn push_change_internal(
        &mut self,
        prev_element: SyntaxElement<L>,
        next_element: Option<SyntaxElement<L>>,
        is_header_cleanup: bool,
    ) {
        let new_node_slot = prev_element.index();
        let parent = prev_element.parent();
        #[cfg(debug_assertions)]
        Self::debug_assert_slot_compatibility(
            parent.as_ref(),
            new_node_slot,
            &prev_element,
            next_element.as_ref(),
        );
        let parent_range: Option<(u32, u32)> = parent.as_ref().map(|p| {
            let range = p.text_range_with_trivia();
            (range.start().into(), range.end().into())
        });
        let parent_depth = parent.as_ref().map(|p| p.ancestors().count()).unwrap_or(0);

        let previous_range = match &prev_element {
            SyntaxElement::Node(node) => node.text_range_with_trivia(),
            SyntaxElement::Token(token) => token.text_range(),
        };
        self.changes.push(CommitChange {
            parent_depth,
            parent,
            parent_range,
            new_node_slot,
            new_node: next_element,
            is_from_action: true,
            is_insertion: false,
            is_header_transfer: false,
            is_header_cleanup,
            previous_range: Some(previous_range),
            order: self.next_order,
        });
        self.next_order += 1;
    }

    /// Returns the range of the document modified by this mutation along with
    /// a list of individual text edits to be performed on the source code, or
    /// [None] if the mutation is empty
    ///
    /// If the new tree is also required,
    /// please use `commit_with_text_range_and_edit`
    pub fn to_text_range_and_edit(self) -> Option<(TextRange, TextEdit)> {
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
    /// changes have been visited. So we start to construct the TextEdit with the help of "text_edit_builder" by iterating
    /// the collected "text_mutation_list".
    pub fn commit_with_text_range_and_edit(
        self,
        with_text_range_and_edit: bool,
    ) -> (SyntaxNode<L>, Option<(TextRange, TextEdit)>) {
        let Self {
            root, mut changes, ..
        } = self;

        // Ordered text mutation list sorted by text range
        let mut text_mutation_list: Vec<(TextRange, Option<SyntaxElement<L>>)> =
            // SAFETY: this is safe because changes from actions can only
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
            is_insertion: curr_is_insertion,
            order: curr_order,
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
                let mut modifications = vec![(
                    curr_new_node_slot,
                    curr_new_node,
                    curr_is_from_action,
                    curr_is_insertion,
                    curr_order,
                )];

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
                        is_insertion: next_is_insertion,
                        order: next_order,
                        ..
                    } = changes.pop().expect("changes.pop");

                    // If we have two modifications to the same slot,
                    // last write wins
                    if let Some(&(prev_new_node_slot, _, _, prev_is_insertion, _)) =
                        modifications.last()
                        && prev_new_node_slot == next_new_node_slot
                        && !prev_is_insertion
                        && !next_is_insertion
                    {
                        // Heap priority visits the later replacement first, so keep it and
                        // discard the earlier write to preserve last-write-wins semantics.
                        continue;
                    }

                    // Add to the modifications
                    modifications.push((
                        next_new_node_slot,
                        next_new_node,
                        next_is_from_action,
                        next_is_insertion,
                        next_order,
                    ));
                }

                // Each same-slot splice is placed before the prior insertion. Apply those
                // changes in reverse mutation order so the committed CST follows merge order,
                // while keeping higher slots ahead of lower slots.
                modifications.sort_by(|left, right| {
                    right.0.cmp(&left.0).then_with(|| match (left.3, right.3) {
                        (false, true) => cmp::Ordering::Less,
                        (true, false) => cmp::Ordering::Greater,
                        (true, true) => right.4.cmp(&left.4),
                        (false, false) => right.4.cmp(&left.4),
                    })
                });
                // Collect text mutations, this has to be done before the detach below,
                // or we'll lose the "deleted_text_range" info
                if with_text_range_and_edit {
                    let mut text_modifications = modifications.clone();
                    text_modifications.sort_by(|left, right| {
                        if left.0 == right.0 && left.3 && right.3 {
                            left.4.cmp(&right.4)
                        } else {
                            cmp::Ordering::Equal
                        }
                    });
                    for (new_node_slot, new_node, is_from_action, is_insertion, _) in
                        &text_modifications
                    {
                        if !is_from_action {
                            continue;
                        }
                        let deleted_text_range = if *is_insertion {
                            let position = match curr_parent.slots().nth(*new_node_slot) {
                                Some(SyntaxSlot::Node(node)) => {
                                    node.text_range_with_trivia().start()
                                }
                                Some(SyntaxSlot::Token(token)) => token.text_range().start(),
                                Some(SyntaxSlot::Empty { index }) => index.into(),
                                None => curr_parent.text_range_with_trivia().end(),
                            };
                            TextRange::new(position, position)
                        } else {
                            match curr_parent.slots().nth(*new_node_slot) {
                                Some(SyntaxSlot::Node(node)) => node.text_range_with_trivia(),
                                Some(SyntaxSlot::Token(token)) => token.text_range(),
                                Some(SyntaxSlot::Empty { index }) => {
                                    TextRange::new(index.into(), index.into())
                                }
                                None => continue,
                            }
                        };
                        // Keep the text mutations ordered by source range. Equal insertion
                        // ranges use an explicit upper-bound scan because binary search may
                        // return any one of several equal entries.
                        if deleted_text_range.is_empty() && new_node.is_some() {
                            let mut pos = match text_mutation_list
                                .binary_search_by(|(range, _)| range.ordering(deleted_text_range))
                            {
                                Ok(pos) | Err(pos) => pos,
                            };
                            while text_mutation_list
                                .get(pos)
                                .is_some_and(|(range, _)| *range == deleted_text_range)
                            {
                                pos += 1;
                            }
                            text_mutation_list.insert(pos, (deleted_text_range, new_node.clone()));
                        } else {
                            match text_mutation_list
                                .binary_search_by(|(range, _)| range.ordering(deleted_text_range))
                            {
                                // Overwrite an overlapping non-insertion text mutation.
                                Ok(pos) => {
                                    text_mutation_list[pos] = (deleted_text_range, new_node.clone())
                                }
                                // Insert the text mutation at the correct position.
                                Err(pos) => text_mutation_list
                                    .insert(pos, (deleted_text_range, new_node.clone())),
                            }
                        }
                    }
                }

                // Now we detach the current parent, commit all the modifications
                // and push a pending change to its parent
                let mut current_parent = curr_parent.detach();
                let is_list = current_parent.kind().is_list();
                for (new_node_slot, new_node, _, is_insertion, _) in modifications.clone() {
                    current_parent = if is_insertion {
                        current_parent.splice_slots(new_node_slot..new_node_slot, once(new_node))
                    } else if is_list && new_node.is_none() {
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
                    is_insertion: false,
                    is_header_transfer: false,
                    is_header_cleanup: false,
                    previous_range: None,
                    order: curr_order,
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
                            curr_new_node.clone(),
                        )];
                    }

                    // Build text range and text edit from the text mutation list
                    // Use SyntaxNodeText instead of String to avoid allocating the entire document upfront
                    let root_text = document_root.text_with_trivia();
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
                                // Slice only the needed range instead of using full root_string
                                let slice = root_text.slice(TextRange::new(
                                    TextSize::from(pointer as u32),
                                    TextSize::from(range_start as u32),
                                ));
                                text_edit_builder.equal(&slice.to_string());
                            }

                            // Slice only the deleted range instead of using full root_string
                            let old_slice = root_text.slice(deleted_text_range);
                            let old = old_slice.to_string();

                            match optional_inserted_text {
                                None => {
                                    text_edit_builder.with_unicode_words_diff(&old, "");
                                }
                                Some(element) => match element {
                                    SyntaxElement::Node(node) => {
                                        text_edit_builder
                                            .with_unicode_words_diff(&old, &node.to_string());
                                    }
                                    SyntaxElement::Token(token) => {
                                        text_edit_builder
                                            .with_unicode_words_diff(&old, token.text());
                                    }
                                },
                            }

                            pointer = range_end;
                        }
                    }
                    let end_pos = root_text.len();
                    if end_pos > TextSize::from(pointer as u32) {
                        // Slice the remaining range instead of using full root_string
                        let slice = root_text
                            .slice(TextRange::new(TextSize::from(pointer as u32), end_pos));
                        text_edit_builder.equal(&slice.to_string());
                    }

                    let text_edit = text_edit_builder.finish();

                    Some((text_range, text_edit))
                } else {
                    None
                };

                return (
                    // SAFETY: If the change is propagated from the child,
                    // this will always be a syntax node element because
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

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        AstNode, BatchMutationExt, SyntaxNodeCast, TriviaPiece,
        raw_language::{LiteralExpression, RawLanguageKind, RawLanguageRoot, RawSyntaxTreeBuilder},
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

    fn literal_with_header(value: &str, header: &str) -> LiteralExpression {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            &format!("{header}{value}"),
            &[TriviaPiece::single_line_comment(header.len() as u32)],
            &[],
        );
        builder.finish_node();
        builder
            .finish()
            .cast::<LiteralExpression>()
            .expect("literal expression")
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

    #[test]
    pub fn ok_batch_mutation_merge_same_slot_last_write_wins() {
        let (before, _) = tree_one("a");
        let (expected, expected_debug) = tree_one("c");

        let a = find(&before, "a");
        let b = clone_detach(&tree_one("b").0, "b");
        let c = clone_detach(&expected, "c");

        let mut first = before.clone().begin();
        first.replace_node(a.clone(), b);
        let source = before.syntax().text_with_trivia().to_string();
        let mut second = before.begin();
        second.replace_node(a, c);
        first.merge_actions(second);

        let (after, text_edit) = first.commit_with_text_range_and_edit(true);
        assert_eq!(expected_debug, format!("{after:#?}"));
        let (_, text_edit) = text_edit.expect("replacement should produce a text edit");
        assert_eq!(text_edit.new_string(&source), after.to_string());
    }

    #[test]
    pub fn ok_batch_mutation_merge_header_transfer_is_scoped_to_parent() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        builder
            .start_node(RawLanguageKind::LITERAL_EXPRESSION)
            .token(RawLanguageKind::STRING_TOKEN, "first")
            .finish_node()
            .finish_node();
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        builder
            .start_node(RawLanguageKind::LITERAL_EXPRESSION)
            .token(RawLanguageKind::STRING_TOKEN, "second")
            .finish_node()
            .finish_node();
        builder.finish_node();
        let root = builder.finish().cast::<RawLanguageRoot>().unwrap();
        let lists = root
            .syntax()
            .descendants()
            .filter(|node| node.kind() == RawLanguageKind::EXPRESSION_LIST)
            .collect::<Vec<_>>();
        assert_eq!(lists.len(), 2);

        let mut first = root.clone().begin();
        first.insert_element_with_header(
            lists[0].clone(),
            0,
            literal_with_header("A", "// first\n").into_syntax().into(),
        );
        let mut second = root.begin();
        second.insert_element_with_header(
            lists[1].clone(),
            0,
            literal_with_header("B", "// second\n").into_syntax().into(),
        );
        first.merge_actions(second);

        let output = first.commit().to_string();
        assert_eq!(output.matches("// first\n").count(), 1);
        assert_eq!(output.matches("// second\n").count(), 1);
        assert!(output.contains("// first\nAfirst"));
        assert!(output.contains("// second\nBsecond"));
    }

    /// Builds a tree with two LITERAL_EXPRESSION nodes where the first node's
    /// token has leading and trailing whitespace trivia:
    ///
    /// ```
    /// ROOT
    ///   LITERAL_EXPRESSION  (token: " a " with leading=1ws, trailing=1ws)
    ///   LITERAL_EXPRESSION  (token: "b" with no trivia)
    /// ```
    ///
    /// After `remove_node_keep_trivia` on node "a", the trivia from "a"'s token
    /// (leading ws + trailing ws) should be prepended to "b"'s leading trivia.
    #[test]
    pub fn ok_remove_node_keep_trivia_transfers_trivia_to_next() {
        // Build tree: [" a "] ["b"] where " a " has leading+trailing whitespace
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            " a ",
            &[TriviaPiece::whitespace(1)],
            &[TriviaPiece::whitespace(1)],
        );
        builder.finish_node();
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::STRING_TOKEN, "b");
        builder.finish_node();
        builder.finish_node();
        let root = builder.finish().cast::<RawLanguageRoot>().unwrap();

        let node_a = find(&root, "a");
        let mut batch = root.begin();
        batch.remove_node_keep_trivia(node_a);
        let after = batch.commit();

        // "b"'s token should now have the trivia from "a" prepended:
        // leading = ws(1) from "a"'s leading + ws(1) from "a"'s trailing
        let b_token = after
            .descendants_tokens(crate::Direction::Next)
            .find(|t| t.text_trimmed() == "b")
            .expect("token 'b' should still exist");

        let leading: Vec<_> = b_token.leading_trivia().pieces().collect();
        assert_eq!(leading.len(), 2, "expected 2 leading trivia pieces on 'b'");
        assert!(leading[0].is_whitespace());
        assert!(leading[1].is_whitespace());

        // "a" node should be gone
        let has_a = after
            .descendants_tokens(crate::Direction::Next)
            .any(|t| t.text_trimmed() == "a");
        assert!(!has_a, "node 'a' should have been removed");
    }

    /// Removes the only real node in a tree; its comment trivia must migrate to
    /// the EOF token's leading trivia.
    ///
    /// Tree shape:
    /// ```
    /// ROOT
    ///   LITERAL_EXPRESSION
    ///     STRING_TOKEN "a"  leading=[comment("// hi\n", 7)]  trailing=[]
    ///   EOF               leading=[]  trailing=[]
    /// ```
    ///
    /// After `remove_node_keep_trivia`, the EOF token must carry the comment as
    /// leading trivia and the LITERAL_EXPRESSION must be gone.
    #[test]
    pub fn ok_remove_node_keep_trivia_transfers_to_eof() {
        let comment = "// hi\n";
        let comment_len = comment.len() as u32;

        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token_with_trivia(
            RawLanguageKind::STRING_TOKEN,
            // full token text: leading trivia + trimmed text + trailing trivia
            &format!("{comment}a"),
            &[TriviaPiece::single_line_comment(comment_len)],
            &[],
        );
        builder.finish_node();
        // EOF token — no trivia initially
        builder.token_with_trivia(RawLanguageKind::EOF, "", &[], &[]);
        builder.finish_node();

        let root = builder.finish().cast::<RawLanguageRoot>().unwrap();

        let node_a = find(&root, "a");
        let mut batch = root.begin();
        batch.remove_node_keep_trivia(node_a);
        let after = batch.commit();

        // The EOF token must now carry the comment as its leading trivia.
        let eof = after.last_token().expect("EOF token must exist");
        assert_eq!(eof.kind(), RawLanguageKind::EOF);

        let leading: Vec<_> = eof.leading_trivia().pieces().collect();
        assert_eq!(
            leading.len(),
            1,
            "EOF should have exactly one leading trivia piece"
        );
        assert!(
            leading[0].is_comments(),
            "the trivia piece should be a comment"
        );
        assert_eq!(
            leading[0].text_len(),
            biome_text_size::TextSize::from(comment_len)
        );

        // The LITERAL_EXPRESSION must be gone.
        let has_a = after
            .descendants_tokens(crate::Direction::Next)
            .any(|t: crate::SyntaxToken<_>| t.text_trimmed() == "a");
        assert!(!has_a, "node 'a' should have been removed");
    }
}
