use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};

use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::{Applicability, category};
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsObjectMemberName, AnyTsTypeMember, TsInterfaceDeclaration, TsTypeMemberList,
};

use crate::JsRuleAction;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, SyntaxResult};
use biome_rule_options::use_sorted_keys::{SortOrder, UseSortedKeysOptions};
use biome_string_case::comparable_token::ComparableToken;
use std::borrow::Cow;

trait TsTypeMemberName {
    fn name(&self) -> SyntaxResult<Option<AnyJsObjectMemberName>>;
}
impl TsTypeMemberName for AnyTsTypeMember {
    fn name(&self) -> SyntaxResult<Option<AnyJsObjectMemberName>> {
        match self {
            // Property signatures have names
            Self::TsPropertySignatureTypeMember(prop) => prop.name().map(Some),
            // Method signatures have names
            Self::TsMethodSignatureTypeMember(method) => method.name().map(Some),
            // Getter signatures have names
            Self::TsGetterSignatureTypeMember(getter) => getter.name().map(Some),
            // Setter signatures have names
            Self::TsSetterSignatureTypeMember(setter) => setter.name().map(Some),
            // Call signatures, construct signatures, and index signatures don't have sortable names
            _ => Ok(None),
        }
    }
}
fn is_interface_members_sorted(
    list: &TsTypeMemberList,
    comparator: impl Fn(&ComparableToken, &ComparableToken) -> std::cmp::Ordering,
) -> bool {
    // Collect all sortable members with their keys
    let mut sortable_keys: Vec<ComparableToken> = Vec::new();

    for member in list.iter() {
        if let Ok(Some(name)) = member.name()
            && let Some(token_text) = name.name()
        {
            sortable_keys.push(ComparableToken::new(token_text));
        }
    }

    // Check if the sortable keys are already sorted
    for i in 1..sortable_keys.len() {
        if comparator(&sortable_keys[i - 1], &sortable_keys[i]) == std::cmp::Ordering::Greater {
            return false;
        }
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
        if let Ok(Some(name)) = member.name() {
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
declare_source_rule! {
    /// Sort interface members by key.
    ///
    /// Interface members are sorted according to their names.
    ///
    /// This rule sorts all sortable members (properties, methods, getters, setters)
    /// and groups them at the beginning of the interface. Non-sortable members
    /// (call signatures, construct signatures, and index signatures) are grouped
    /// at the end of the interface.
    pub UseSortedInterfaceMembers {
        version: "2.0.0",
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
    type Options = UseSortedKeysOptions;
    fn action(ctx: &RuleContext<Self>, (): &Self::State) -> Option<JsRuleAction> {
        let interface = ctx.query();
        let list = interface.members();

        let options = ctx.options();
        let sort_order = options.sort_order;
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        let new_list = sort_interface_members(&list, comparator);

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(list, new_list);

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the interface members by key." },
            mutation,
        ))
    }
    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let interface = ctx.query();
        let body = interface.members();

        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedInterfaceMembers"),
            body.range(),
            markup! {
                "The interface members are not sorted by key."
            },
        ))
    }
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let interface = ctx.query();

        // Get the interface body (type member list)
        let body = interface.members();

        let options = ctx.options();
        let sort_order = options.sort_order;
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        if is_interface_members_sorted(&body, comparator) {
            None
        } else {
            Some(())
        }
    }
    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(ctx.query().range())
    }
}
