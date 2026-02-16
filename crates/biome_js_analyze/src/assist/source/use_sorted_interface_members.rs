use std::borrow::Cow;

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};

use biome_console::markup;
use biome_diagnostics::category;
use biome_js_syntax::{
    AnyJsObjectMemberName, AnyTsTypeMember, TsInterfaceDeclaration, TsTypeMemberList,
};
use biome_rule_options::use_sorted_interface_members::UseSortedInterfaceMembersOptions;

use crate::JsRuleAction;
use biome_rowan::{AstNode, AstNodeExt, AstNodeList, BatchMutationExt, TextRange};
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
    pub UseSortedInterfaceMembers {
        version: "2.4.0",
        name: "useSortedInterfaceMembers",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintPerfectionist("sort-interfaces").inspired()],
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
        if is_interface_members_sorted(&body, comparator) {
            None
        } else {
            Some(())
        }
    }
    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let interface = ctx.query();

        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedInterfaceMembers"),
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
        let mut mutation = ctx.root().begin();

        // Instead of rebuilding the entire list, replace individual members
        // that are in the wrong position. This preserves comments better.
        // If any token replacements fail, propagate None to skip the fix.
        sort_interface_members_in_place(&list, comparator, &mut mutation)?;

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
fn is_interface_members_sorted(
    list: &TsTypeMemberList,
    comparator: impl Fn(&ComparableToken, &ComparableToken) -> std::cmp::Ordering,
) -> bool {
    use std::cmp::Ordering;
    let mut prev_key: Option<ComparableToken> = None;
    let mut saw_non_sortable = false;

    for member in list.iter() {
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
    mutation: &mut biome_rowan::BatchMutation<biome_js_syntax::JsLanguage>,
) -> Option<()> {
    // Collect current members with their trivia
    let members_with_trivia: Vec<_> = list
        .iter()
        .map(|member| {
            let syntax = member.syntax();
            let leading_trivia: Vec<_> = syntax
                .first_token()
                .map(|token| token.leading_trivia().pieces().collect())
                .unwrap_or_default();
            let trailing_trivia: Vec<_> = syntax
                .last_token()
                .map(|token| token.trailing_trivia().pieces().collect())
                .unwrap_or_default();

            (member, leading_trivia, trailing_trivia)
        })
        .collect();

    // Separate sortable from non-sortable members
    let mut sortable_indices = Vec::new();
    let mut non_sortable_indices = Vec::new();

    for (index, (member, _, _)) in members_with_trivia.iter().enumerate() {
        if let Some(name) = get_type_member_name(member)
            && name.name().is_some()
        {
            sortable_indices.push(index);
        } else {
            non_sortable_indices.push(index);
        }
    }

    // Sort the sortable members by their keys
    sortable_indices.sort_by(|&a, &b| {
        let (member_a, _, _) = &members_with_trivia[a];
        let (member_b, _, _) = &members_with_trivia[b];

        let key_a = get_type_member_name(member_a)
            .and_then(|name| name.name())
            .map(ComparableToken::new);
        let key_b = get_type_member_name(member_b)
            .and_then(|name| name.name())
            .map(ComparableToken::new);

        match (key_a, key_b) {
            (Some(a), Some(b)) => comparator(&a, &b),
            _ => std::cmp::Ordering::Equal,
        }
    });

    // Collect current members in order
    // Build the expected order: sortable first, then everything else
    let current_members: Vec<_> = list.iter().collect();
    let expected_indices: Vec<_> = sortable_indices
        .into_iter()
        .chain(non_sortable_indices)
        .collect();

    // Replace each member that's in the wrong position
    for (current_index, current_member) in current_members.iter().enumerate() {
        let expected_index = expected_indices[current_index];

        if current_index != expected_index {
            let (expected_member, leading, trailing) = &members_with_trivia[expected_index];
            let mut new_member = expected_member.clone();

            if let Some(first_token) = new_member.syntax().first_token() {
                let new_first = first_token
                    .with_leading_trivia(leading.iter().map(|piece| (piece.kind(), piece.text())));
                new_member =
                    new_member.replace_token_discard_trivia(first_token.clone(), new_first)?;
            }
            if let Some(last_token) = new_member.syntax().last_token() {
                let new_last = last_token.with_trailing_trivia_pieces(trailing.iter().cloned());
                new_member =
                    new_member.replace_token_discard_trivia(last_token.clone(), new_last)?;
            }

            // Use replace_node_discard_trivia to avoid transferring trivia from current_member
            mutation.replace_node_discard_trivia(current_member.clone(), new_member);
        }
    }

    Some(())
}
