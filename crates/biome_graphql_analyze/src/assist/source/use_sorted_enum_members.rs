use std::cmp::Ordering;

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    GraphqlEnumTypeDefinition, GraphqlEnumTypeExtension, GraphqlEnumValueDefinition,
    GraphqlEnumValueList, GraphqlLanguage,
};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutation, BatchMutationExt, NodeOrToken, SyntaxNode, TokenText,
    declare_node_union,
};
use biome_rule_options::use_sorted_enum_members::UseSortedEnumMembersOptions;
use biome_string_case::comparable_token::ComparableToken;

use crate::GraphqlRuleAction;

declare_source_rule! {
    /// Sort the members of an enum in natural order.
    ///
    /// Enforce a consistent natural sort order for GraphQL enum values.
    ///
    /// Keeping enum values sorted makes schema definitions easier to review and maintain,
    /// especially as enums grow over time.
    ///
    /// Members are sorted in a [Natural order](https://en.wikipedia.org/wiki/Natural_sort_order),
    /// meaning that uppercase letters come before lowercase letters (e.g. `A` < `a` <`B` < `b`)
    /// and numbers are compared in a human way (e.g. `9` < `10`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diagnostic
    /// enum Role {
    ///   SUPER_ADMIN
    ///   ADMIN
    ///   USER
    ///   GOD
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// enum Role {
    ///   ADMIN
    ///   GOD
    ///   SUPER_ADMIN
    ///   USER
    /// }
    /// ```
    ///
    pub UseSortedEnumMembers {
        version: "next",
        name: "useSortedEnumMembers",
        language: "graphql",
        recommended: false,
        sources: &[RuleSource::EslintGraphql("alphabetize").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedEnumMembers {
    type Query = Ast<AnyUseSortedEnumMembersQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedEnumMembersOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let enum_values = match node {
            AnyUseSortedEnumMembersQuery::GraphqlEnumTypeDefinition(type_def) => {
                type_def.enum_values()
            }
            AnyUseSortedEnumMembersQuery::GraphqlEnumTypeExtension(type_ext) => {
                type_ext.enum_values()
            }
        }?;

        if is_enum_value_list_sorted(&enum_values.values()) {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Enum is not sorted."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<GraphqlRuleAction> {
        let node = ctx.query();
        let enum_values = match node {
            AnyUseSortedEnumMembersQuery::GraphqlEnumTypeDefinition(type_def) => {
                type_def.enum_values()
            }
            AnyUseSortedEnumMembersQuery::GraphqlEnumTypeExtension(type_ext) => {
                type_ext.enum_values()
            }
        }?;
        let mut mutation = ctx.root().begin();

        sort_enum_value_list(&enum_values.values(), &mut mutation)?;

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort enum." },
            mutation,
        ))
    }
}

declare_node_union! {
    pub AnyUseSortedEnumMembersQuery =
        GraphqlEnumTypeDefinition
        | GraphqlEnumTypeExtension
}

fn get_value_definition_key(node: &GraphqlEnumValueDefinition) -> Option<TokenText> {
    node.value()
        .ok()?
        .value_token()
        .ok()
        .map(|t| t.token_text_trimmed())
}

fn is_enum_value_list_sorted(list: &GraphqlEnumValueList) -> bool {
    let mut prev = None;
    for item in list {
        if prev
            .as_ref()
            .is_some_and(|previous| compare_enum_value_list(previous, &item) == Ordering::Greater)
        {
            return false;
        }
        prev = Some(item);
    }
    true
}

fn locale_compare(k1: &Option<TokenText>, k2: &Option<TokenText>) -> Ordering {
    match (k1, k2) {
        (Some(a), Some(b)) => {
            let a = ComparableToken::new(a.clone());
            let b = ComparableToken::new(b.clone());
            ComparableToken::ascii_nat_cmp(&a, &b)
        }
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

fn compare_enum_value_list(
    a: &GraphqlEnumValueDefinition,
    b: &GraphqlEnumValueDefinition,
) -> Ordering {
    let key_a = get_value_definition_key(a);
    let key_b = get_value_definition_key(b);
    locale_compare(&key_a, &key_b)
}

fn sort_enum_value_list(
    list: &GraphqlEnumValueList,
    mutation: &mut BatchMutation<GraphqlLanguage>,
) -> Option<()> {
    let mut items: Vec<GraphqlEnumValueDefinition> = list.iter().collect();
    items.sort_by(compare_enum_value_list);

    let sorted = GraphqlEnumValueList::cast(SyntaxNode::new_detached(
        list.syntax().kind(),
        items
            .into_iter()
            .map(|def| Some(NodeOrToken::Node(def.into_syntax()))),
    ))?;

    mutation.replace_node(list.clone(), sorted);

    Some(())
}
