use std::{borrow::Cow, cmp::Ordering, collections::HashSet};

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};

use biome_console::markup;
use biome_js_syntax::{
    AnyJsObjectMemberName, AnyTsTypeMember, JsLanguage, TsInterfaceDeclaration, TsTypeMemberList,
};
use biome_rule_options::use_sorted_interface_members::UseSortedInterfaceMembersOptions;

use crate::JsRuleAction;
use biome_rowan::{AstNode, AstNodeExt, AstNodeList, BatchMutationExt, SyntaxTriviaPiece, TextRange};
use biome_string_case::comparable_token::ComparableToken;
declare_source_rule! {
    /// Sort interface members by key.
    ///
    /// Interface members are sorted according to their names. The rule distinguishes between
    /// two types of members:
    ///
    /// **Sortable members** - Members with explicit, fixed names that can be alphabetically sorted:
    /// - Property signatures: `property: type`
    /// - Method signatures: `method(): type`
    /// - Getter signatures: `get property(): type`
    /// - Setter signatures: `set property(value: type): void`
    ///
    /// **Non-sortable members** - Members without fixed names or with dynamic/computed names:
    /// - Call signatures: `(): type` (represents the interface as a callable function)
    /// - Construct signatures: `new (): type` (represents the interface as a constructor)
    /// - Index signatures: `[key: string]: type` (represents dynamic property access)
    ///
    /// The rule sorts all sortable members alphabetically and places them first,
    /// followed by non-sortable members in their original order. Non-sortable members
    /// cannot be meaningfully sorted by name since they represent different interface
    /// contracts rather than named properties or methods.
    ///
    /// # Examples
    ///
    /// ## Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface MixedMembers {
    ///   z: string;
    ///   a: number;
    ///   (): void;  // Call signature
    ///   y: boolean;
    ///   new (): MixedMembers;  // Construct signature
    ///   b: string;
    ///   [key: string]: any;  // Index signature
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// interface MixedMembers {
    ///   a: number;
    ///   b: string;
    ///   y: boolean;
    ///   z: string;
    ///   (): void;  // Non-sortable members remain in original order
    ///   new (): MixedMembers;
    ///   [key: string]: any;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `partitionByNewLine`
    ///
    /// When enabled, members separated by a blank line are kept in their own
    /// section and sorted only within that section. This preserves logical
    /// groupings that the author intentionally introduced with empty lines.
    ///
    /// > Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "partitionByNewLine": true
    ///     }
    /// }
    /// ```
    ///
    /// With the option enabled, the following interface is considered sorted
    /// because each section is sorted on its own:
    ///
    /// ```ts,use_options
    /// interface User {
    ///   id: string;
    ///
    ///   createdAt: Date;
    ///   updatedAt: Date;
    ///
    ///   email: string;
    ///   name: string;
    ///   passwordHash: string;
    /// }
    /// ```
    ///
    pub UseSortedInterfaceMembers {
        version: "2.4.0",
        name: "useSortedInterfaceMembers",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintPerfectionist("sort-interfaces").inspired(), RuleSource::EslintTypescriptSortKeys("interface").inspired()],
        fix_kind: FixKind::Safe,
    }
}
impl Rule for UseSortedInterfaceMembers {
    type Query = Ast<TsInterfaceDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedInterfaceMembersOptions;
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let interface = ctx.query();
        let body = interface.members();
        let partition_by_new_line = ctx.options().partition_by_new_line.unwrap_or_default();
        if is_interface_members_sorted(&body, comparator, partition_by_new_line) {
            None
        } else {
            Some(())
        }
    }
    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let interface = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            interface.range(),
            markup! {
                "The interface members are not sorted by key."
            },
        ))
    }
    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(ctx.query().range())
    }
    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let interface = ctx.query();
        let list = interface.members();
        let partition_by_new_line = ctx.options().partition_by_new_line.unwrap_or_default();
        let mut mutation = ctx.root().begin();

        // Instead of rebuilding the entire list, replace individual members
        // that are in the wrong position. This preserves comments better.
        // If any token replacements fail, propagate None to skip the fix.
        sort_interface_members_in_place(&list, comparator, partition_by_new_line, &mut mutation)?;

        Some(RuleAction::new(
            rule_action_category!(),
            ctx.metadata().applicability(),
            markup! { "Sort the interface members by key." },
            mutation,
        ))
    }
}
fn comparator(a: &ComparableToken, b: &ComparableToken) -> std::cmp::Ordering {
    ComparableToken::ascii_nat_cmp(a, b)
}
fn get_type_member_name(member: &AnyTsTypeMember) -> Option<AnyJsObjectMemberName> {
    match member {
        // Property signatures have names
        AnyTsTypeMember::TsPropertySignatureTypeMember(prop) => prop.name().ok(),
        AnyTsTypeMember::TsMethodSignatureTypeMember(method) => method.name().ok(),
        AnyTsTypeMember::TsGetterSignatureTypeMember(getter) => getter.name().ok(),
        AnyTsTypeMember::TsSetterSignatureTypeMember(setter) => setter.name().ok(),
        // Call signatures, construct signatures, and index signatures don't have sortable names
        _ => None,
    }
}

/// Returns true if `member`'s leading trivia contains a blank line.
///
/// A blank line is a pair of newlines optionally separated by whitespace
/// pieces. Comments between newlines do not count as a blank line.
fn has_blank_line_before(member: &AnyTsTypeMember) -> bool {
    let Some(first_token) = member.syntax().first_token() else {
        return false;
    };
    let mut seen_newline = false;
    for piece in first_token.leading_trivia().pieces() {
        if piece.is_newline() {
            if seen_newline {
                return true;
            }
            seen_newline = true;
        } else if !piece.is_whitespace() {
            seen_newline = false;
        }
    }
    false
}

/// Returns the index of the second newline of the last blank line in `pieces`.
/// This is the boundary between "detached" (section-break) trivia and
/// "attached" (member-attached) trivia.
///
/// Whitespace pieces between two newlines are ignored.
fn find_last_blank_line_idx(pieces: &[SyntaxTriviaPiece<JsLanguage>]) -> Option<usize> {
    let mut last = None;
    let mut seen_newline = false;
    for (i, piece) in pieces.iter().enumerate() {
        if piece.is_newline() {
            if seen_newline {
                last = Some(i);
            }
            seen_newline = true;
        } else if !piece.is_whitespace() {
            seen_newline = false;
        }
    }
    last
}

/// Returns the boundaries (start indices and trailing length sentinel) of
/// sections in the given member list.
///
/// When `partition_by_new_line` is false, the returned vector contains just
/// `[0, members.len()]` (a single section).
fn compute_section_boundaries(members: &[AnyTsTypeMember], partition_by_new_line: bool) -> Vec<usize> {
    let mut boundaries = vec![0_usize];
    if partition_by_new_line {
        // `skip(1)` as the first boundary is always 0 (the start of the list)
        for (i, member) in members.iter().enumerate().skip(1) {
            if has_blank_line_before(member) {
                boundaries.push(i);
            }
        }
    }
    boundaries.push(members.len());
    boundaries
}

fn is_interface_members_sorted(
    list: &TsTypeMemberList,
    comparator: impl Fn(&ComparableToken, &ComparableToken) -> std::cmp::Ordering,
    partition_by_new_line: bool,
) -> bool {
    let mut prev_key: Option<ComparableToken> = None;
    let mut saw_non_sortable = false;

    for (i, member) in list.iter().enumerate() {
        if partition_by_new_line && i > 0 && has_blank_line_before(&member) {
            // New section: reset state.
            prev_key = None;
            saw_non_sortable = false;
        }

        if let Some(name) = get_type_member_name(&member)
            && let Some(token_text) = name.name()
        {
            if saw_non_sortable {
                // sortable member found after a non-sortable
                return false;
            }

            let current = ComparableToken::new(token_text);

            if let Some(prev) = &prev_key
                && comparator(prev, &current) == Ordering::Greater
            {
                return false;
            }

            prev_key = Some(current);

            continue;
        }

        // Non-sortable member
        saw_non_sortable = true;
    }
    true
}

fn sort_interface_members_in_place(
    list: &TsTypeMemberList,
    comparator: impl Fn(&ComparableToken, &ComparableToken) -> std::cmp::Ordering,
    partition_by_new_line: bool,
    mutation: &mut biome_rowan::BatchMutation<biome_js_syntax::JsLanguage>,
) -> Option<()> {
    let members: Vec<_> = list.iter().collect();
    let section_boundaries = compute_section_boundaries(&members, partition_by_new_line);

    // For each section, compute the expected order:
    // sortable members sorted alphabetically, followed by non-sortable members
    // in their original order.
    let mut expected_indices = Vec::with_capacity(members.len());
    for &[start, end] in section_boundaries.array_windows::<2>() {
        let mut sortable = Vec::new();
        let mut non_sortable = Vec::new();

        for (idx, member) in members.iter().enumerate().take(end).skip(start) {
            if let Some(name) = get_type_member_name(member)
                && name.name().is_some()
            {
                sortable.push(idx);
            } else {
                non_sortable.push(idx);
            }
        }

        sortable.sort_by(|&a, &b| {
            let key_a = get_type_member_name(&members[a])
                .and_then(|name| name.name())
                .map(ComparableToken::new);
            let key_b = get_type_member_name(&members[b])
                .and_then(|name| name.name())
                .map(ComparableToken::new);

            match (key_a, key_b) {
                (Some(a), Some(b)) => comparator(&a, &b),
                _ => std::cmp::Ordering::Equal,
            }
        });

        expected_indices.extend(sortable);
        expected_indices.extend(non_sortable);
    }

    // Positions (other than 0) that begin a new section. At these positions
    // the blank line of the original section start must be preserved, while
    // the moving member should contribute only its attached trivia.
    let section_start_positions: HashSet<_> = section_boundaries
        .iter()
        .copied()
        .filter(|&i| i > 0 && i < members.len())
        .collect();

    for (current_index, current_member) in members.iter().enumerate() {
        let expected_index = expected_indices[current_index];

        if current_index == expected_index {
            continue;
        }

        let expected_member = &members[expected_index];

        let expected_leading: Vec<_> = expected_member
            .syntax()
            .first_token()
            .map(|token| token.leading_trivia().pieces().collect())
            .unwrap_or_default();
        let expected_trailing: Vec<_> = expected_member
            .syntax()
            .last_token()
            .map(|token| token.trailing_trivia().pieces().collect())
            .unwrap_or_default();

        // Build the new leading trivia for the expected member at this position.
        let new_leading = if partition_by_new_line {
            // The expected member's attached trivia (everything after its
            // own last blank line, if any).
            let attached_start = find_last_blank_line_idx(&expected_leading).unwrap_or(0);
            let attached_iter = expected_leading.iter().skip(attached_start).cloned();

            if section_start_positions.contains(&current_index) {
                // Preserve the section break (blank line + any preceding
                // detached trivia) from the original member at this position.
                let current_leading: Vec<_> = current_member
                    .syntax()
                    .first_token()
                    .map(|token| token.leading_trivia().pieces().collect())
                    .unwrap_or_default();
                let detached_end = find_last_blank_line_idx(&current_leading).unwrap_or(0);
                current_leading
                    .iter()
                    .take(detached_end)
                    .cloned()
                    .chain(attached_iter)
                    .collect()
            } else {
                attached_iter.collect()
            }
        } else {
            expected_leading.clone()
        };

        let mut new_member = expected_member.clone();

        if let Some(first_token) = new_member.syntax().first_token() {
            let new_first = first_token.with_leading_trivia_pieces(new_leading.iter().cloned());
            new_member = new_member.replace_token_discard_trivia(first_token.clone(), new_first)?;
        }
        if let Some(last_token) = new_member.syntax().last_token() {
            let new_last = last_token.with_trailing_trivia_pieces(expected_trailing.iter().cloned());
            new_member = new_member.replace_token_discard_trivia(last_token.clone(), new_last)?;
        }

        // Use replace_node_discard_trivia to avoid transferring trivia from current_member
        mutation.replace_node_discard_trivia(current_member.clone(), new_member);
    }

    Some(())
}
