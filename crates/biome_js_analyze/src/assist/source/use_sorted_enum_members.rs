use std::cmp::Ordering;

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule, utils::sorted_separated_list_by,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyTsEnumMemberName, JsLanguage, T, TsEnumDeclaration, TsEnumMember, TsEnumMemberList,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, TokenText, TriviaPieceKind,
};
use biome_rule_options::use_sorted_enum_members::UseSortedEnumMembersOptions;
use biome_string_case::comparable_token::ComparableToken;

use crate::JsRuleAction;

declare_source_rule! {
    /// Sort the members of an enum in natural order.
    ///
    /// Enforce a consistent natural sort order for TypeScript enum members with string initializers.
    ///
    /// This rule sorts members in string enums so declarations stay predictable and easier to scan.
    /// Members that cannot be compared, such as computed names, are left in place and split the enum into sortable groups.
    ///
    /// Members are sorted in a [Natural order](https://en.wikipedia.org/wiki/Natural_sort_order),
    /// meaning that uppercase letters come before lowercase letters (e.g. `A` < `a` < `B` < `b`)
    /// and numbers are compared to their numerical value (e.g. `9` < `10`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diff
    /// enum Status {
    /// 	InProgress = 'In Progress',
    /// 	Completed = 'Completed',
    /// 	OnHold = 'On Hold',
    /// 	Cancelled = 'Cancelled',
    /// 	NotStarted = 'Not Started',
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// enum Status {
    /// 	Cancelled = 'Cancelled',
    /// 	Completed = 'Completed',
    /// 	InProgress = 'In Progress',
    /// 	NotStarted = 'Not Started',
    /// 	OnHold = 'On Hold',
    /// }
    /// ```
    ///
    pub UseSortedEnumMembers {
        version: "next",
        name: "useSortedEnumMembers",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintPerfectionist("sort-enums").inspired(), RuleSource::EslintTypescriptSortKeys("string-enum").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedEnumMembers {
    type Query = Ast<TsEnumDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedEnumMembersOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let members = node.members();

        if members.iter().any(|member| {
            if let Ok(member) = member
                && let Some(initializer) = member.initializer()
                && let Some(expression) = initializer.expression().ok()
                && let Some(static_value) = expression.as_static_value()
            {
                return static_value.as_string_constant().is_none();
            }

            true
        }) {
            return None;
        }

        if is_enum_member_list_sorted(&members) {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let ident = node.id().ok()?;
        let binding = ident.as_js_identifier_binding()?;
        let name = binding.name_token().ok()?;
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The members of the enum "<Emphasis>{name.text_trimmed()}</Emphasis>" are not sorted."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        sort_enum_member_list(&node.members(), &mut mutation)?;

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort the enum members." },
            mutation,
        ))
    }
}

fn get_value_definition_key(node: &TsEnumMember) -> Option<TokenText> {
    let name = node.name().ok()?;

    match name {
        AnyTsEnumMemberName::JsComputedMemberName(_) => None,
        AnyTsEnumMemberName::TsLiteralEnumMemberName(enum_member_name) => {
            let value = enum_member_name.value().ok()?;
            Some(value.token_text_trimmed())
        }
    }
}

fn is_enum_member_list_sorted(list: &TsEnumMemberList) -> bool {
    let mut prev = None;
    for item in list.into_iter().flatten() {
        match get_value_definition_key(&item) {
            Some(key) => {
                if prev
                    .as_ref()
                    .is_some_and(|p| locale_compare(p, &key) == Ordering::Greater)
                {
                    return false;
                }
                prev = Some(key);
            }
            None => {
                // Keep detection in sync with `sorted_separated_list_by` chunk boundaries.
                prev = None;
            }
        }
    }
    true
}

fn locale_compare(k1: &TokenText, k2: &TokenText) -> Ordering {
    let a = ComparableToken::new(k1.clone());
    let b = ComparableToken::new(k2.clone());
    ComparableToken::ascii_nat_cmp(&a, &b)
}

fn sort_enum_member_list(
    list: &TsEnumMemberList,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    let new_list = sorted_separated_list_by(
        list,
        get_value_definition_key,
        || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        locale_compare,
    )
    .ok()?;

    mutation.replace_node_discard_trivia(list.clone(), new_list);

    Some(())
}
