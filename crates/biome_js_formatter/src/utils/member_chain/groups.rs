use crate::prelude::*;
use crate::utils::member_chain::chain_member::ChainMember;
use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use std::cell::RefCell;

#[derive(Default)]
pub(super) struct MemberChainGroupsBuilder {
    /// keeps track of the groups created
    groups: Vec<MemberChainGroup>,
    /// keeps track of the current group that is being created/updated
    current_group: Option<MemberChainGroup>,
}

impl MemberChainGroupsBuilder {
    /// starts a new group
    pub fn start_group(&mut self, member: ChainMember) {
        debug_assert!(self.current_group.is_none());
        let mut group = MemberChainGroup::default();
        group.members.push(member);
        self.current_group = Some(group);
    }

    /// continues of starts a new group
    pub fn start_or_continue_group(&mut self, member: ChainMember) {
        match &mut self.current_group {
            None => self.start_group(member),
            Some(group) => group.members.push(member),
        }
    }

    /// adds the passed element to the current group.
    ///
    /// # Panics
    ///
    /// If there's no started group.
    pub fn continue_group(&mut self, member: ChainMember) {
        match &mut self.current_group {
            None => {
                panic!("It is necessary to start a group first using `start_group`.");
            }
            Some(group) => {
                group.members.push(member);
            }
        }
    }

    /// clears the current group, and adds it to the groups collection
    pub fn close_group(&mut self) {
        if let Some(group) = self.current_group.take() {
            self.groups.push(group);
        }
    }

    pub(super) fn finish(self) -> TailChainGroups {
        let mut groups = self.groups;

        if let Some(group) = self.current_group {
            groups.push(group);
        }

        TailChainGroups { groups }
    }
}

/// Groups following on the head group.
///
/// May be empty if all members are part of the head group
#[derive(Clone, Debug)]
pub(super) struct TailChainGroups {
    groups: Vec<MemberChainGroup>,
}

impl TailChainGroups {
    /// Returns `true` if there are no tail groups.
    pub(crate) fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns the number of tail groups.
    pub(crate) fn len(&self) -> usize {
        self.groups.len()
    }

    /// Returns the first group
    pub(crate) fn first(&self) -> Option<&MemberChainGroup> {
        self.groups.first()
    }

    /// Returns the last group
    pub(crate) fn last(&self) -> Option<&MemberChainGroup> {
        self.groups.last()
    }

    /// Removes the first group and returns it
    pub(super) fn pop_first(&mut self) -> Option<MemberChainGroup> {
        match self.groups.len() {
            0 => None,
            _ => Some(self.groups.remove(0)),
        }
    }

    /// Checks if the groups contain comments.
    pub fn has_comments(&self, comments: &JsComments) -> bool {
        let mut members = self.groups.iter().flat_map(|item| item.members.iter());

        let has_comments = members.any(|item| {
            comments.has_trailing_comments(item.syntax())
                || comments.has_leading_comments(item.syntax())
        });

        let cutoff_has_leading_comments = if !self.groups.is_empty() {
            let group = self.groups.get(1);
            if let Some(group) = group {
                let first_item = group.members.first();
                first_item
                    .is_some_and(|first_item| comments.has_leading_comments(first_item.syntax()))
            } else {
                false
            }
        } else {
            false
        };

        has_comments || cutoff_has_leading_comments
    }

    /// Here we check if the length of the groups exceeds the cutoff or there are comments
    /// This function is the inverse of the prettier function
    /// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/member-chain.js#L342
    pub(crate) fn is_member_call_chain(&self, comments: &JsComments) -> bool {
        self.groups.len() > 1 || self.has_comments(comments)
    }

    /// Returns an iterator over the groups.
    pub(super) fn iter(&self) -> impl DoubleEndedIterator<Item = &MemberChainGroup> {
        self.groups.iter()
    }

    /// Test if any group except the last group [break](FormatElements::will_break).
    pub(super) fn any_except_last_will_break(&self, f: &mut JsFormatter) -> FormatResult<bool> {
        for group in &self.groups[..self.groups.len().saturating_sub(1)] {
            if group.will_break(f)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Returns an iterator over all members
    pub(super) fn members(&self) -> impl DoubleEndedIterator<Item = &ChainMember> {
        self.groups.iter().flat_map(|group| group.members().iter())
    }
}

impl Format<JsFormatContext> for TailChainGroups {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        f.join().entries(self.groups.iter()).finish()
    }
}

#[derive(Clone, Default)]
pub(super) struct MemberChainGroup {
    members: Vec<ChainMember>,

    /// Stores the formatted result of this group.
    ///
    /// Manual implementation of `Memoized` to only memorizing the formatted result
    /// if [MemberChainGroup::will_break] is called but not otherwise.
    formatted: RefCell<Option<FormatElement>>,
}

impl MemberChainGroup {
    pub(super) fn into_members(self) -> Vec<ChainMember> {
        self.members
    }

    /// Returns the chain members of the group.
    pub(super) fn members(&self) -> &[ChainMember] {
        &self.members
    }

    /// Extends the members of this group with the passed in members
    pub(super) fn extend_members(&mut self, members: impl IntoIterator<Item = ChainMember>) {
        self.members.extend(members)
    }

    /// Tests if the formatted result of this group results in a [break](FormatElements::will_break).
    pub(super) fn will_break(&self, f: &mut JsFormatter) -> FormatResult<bool> {
        let mut cell = self.formatted.borrow_mut();
        let result = match cell.as_ref() {
            Some(formatted) => formatted.will_break(),
            None => {
                let interned = f.intern(&FormatMemberChainGroup { group: self })?;

                if let Some(interned) = interned {
                    let breaks = interned.will_break();
                    *cell = Some(interned);
                    breaks
                } else {
                    false
                }
            }
        };

        Ok(result)
    }

    pub(super) fn has_comments(&self, comments: &JsComments) -> bool {
        self.members.iter().enumerate().any(|(index, member)| {
            if index == 0 {
                comments.has_trailing_comments(member.syntax())
            } else if index < self.members.len() {
                comments.has_leading_comments(member.syntax())
                    || comments.has_trailing_comments(member.syntax())
            } else {
                false
            }
        })
    }

    pub(super) fn needs_empty_line_before(&self) -> bool {
        let first = self.members.first();
        first.is_some_and(|first| match first {
            ChainMember::StaticMember { expression } => {
                let operator = expression.operator_token();

                match operator {
                    Ok(operator) => get_lines_before_token(&operator) > 1,
                    _ => false,
                }
            }
            ChainMember::ComputedMember { expression } => {
                let l_brack_token = expression.l_brack_token();

                match l_brack_token {
                    Ok(l_brack_token) => {
                        get_lines_before_token(
                            &expression.optional_chain_token().unwrap_or(l_brack_token),
                        ) > 1
                    }
                    _ => false,
                }
            }
            _ => false,
        })
    }
}

impl From<Vec<ChainMember>> for MemberChainGroup {
    fn from(entries: Vec<ChainMember>) -> Self {
        Self {
            members: entries,
            formatted: RefCell::new(None),
        }
    }
}

impl std::fmt::Debug for MemberChainGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MemberChainGroup")
            .field(&self.members)
            .finish()
    }
}

impl Format<JsFormatContext> for MemberChainGroup {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        if let Some(formatted) = self.formatted.borrow().as_ref() {
            return f.write_element(formatted.clone());
        }

        FormatMemberChainGroup { group: self }.fmt(f)
    }
}

pub struct FormatMemberChainGroup<'a> {
    group: &'a MemberChainGroup,
}

impl Format<JsFormatContext> for FormatMemberChainGroup<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let group = self.group;

        let last = group.members.last();

        let needs_parens = last.is_some_and(|last| match last {
            ChainMember::StaticMember { expression, .. } => expression.needs_parentheses(),
            ChainMember::ComputedMember { expression, .. } => expression.needs_parentheses(),
            _ => false,
        });

        let format_entries = format_with(|f| f.join().entries(group.members.iter()).finish());

        if needs_parens {
            write!(f, [text("("), format_entries, text(")")])
        } else {
            write!(f, [format_entries])
        }
    }
}
