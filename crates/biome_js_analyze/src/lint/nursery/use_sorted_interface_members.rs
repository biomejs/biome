use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_lint_rule,
};

use biome_console::markup;
use biome_deserialize::TextRange;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsObjectMemberName, AnyTsTypeMember, TsInterfaceDeclaration, TsTypeMemberList,
};

use crate::JsRuleAction;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_string_case::comparable_token::ComparableToken;
declare_lint_rule! {
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
        version: "next",
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
    type Options = ();
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let interface = ctx.query();
        let body = interface.members();
        let comparator = ComparableToken::ascii_nat_cmp;

        if is_interface_members_sorted(&body, comparator) {
            None
        } else {
            Some(())
        }
    }
    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let interface = ctx.query();
        let body = interface.members();

        Some(RuleDiagnostic::new(
            rule_category!(),
            body.range(),
            markup! {
                "The interface members are not sorted by key."
            },
        ))
    }
    fn action(ctx: &RuleContext<Self>, (): &Self::State) -> Option<JsRuleAction> {
        let interface = ctx.query();
        let list = interface.members();
        let mut mutation = ctx.root().begin();
        let comparator = ComparableToken::ascii_nat_cmp;
        let new_list = sort_interface_members(&list, comparator);
        mutation.replace_node(list, new_list);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort the interface members by key." },
            mutation,
        ))
    }
    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(ctx.query().range())
    }
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
fn sort_interface_members(
    list: &TsTypeMemberList,
    comparator: impl Fn(&ComparableToken, &ComparableToken) -> std::cmp::Ordering,
) -> TsTypeMemberList {
    let mut sortable_members = Vec::new();
    let mut non_sortable_members = Vec::new();

    // Separate sortable from non-sortable members
    for member in list.iter() {
        if let Some(name) = get_type_member_name(&member) {
            if let Some(token_text) = name.name() {
                sortable_members.push((member, ComparableToken::new(token_text)));
            } else {
                // Name exists but is not sortable (computed/dynamic)
                non_sortable_members.push(member);
            }
        } else {
            // No name (call signatures, construct signatures, index signatures)
            non_sortable_members.push(member);
        }
    }

    // Sort the sortable members
    sortable_members.sort_by(|(_, a), (_, b)| comparator(a, b));

    // Combine: all sortable members first, then all non-sortable members
    let mut new_members: Vec<AnyTsTypeMember> = sortable_members
        .into_iter()
        .map(|(member, _)| member)
        .collect();

    new_members.extend(non_sortable_members);

    make::ts_type_member_list(new_members)
}
