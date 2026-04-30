use crate::GraphqlRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_graphql_syntax::{
    AnyGraphqlSelection, GraphqlField, GraphqlFragmentSpread, GraphqlInlineFragment,
    GraphqlLanguage, GraphqlSelectionList, GraphqlSelectionSet,
};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutation, BatchMutationExt, NodeOrToken, SyntaxNode, TokenText,
};
use biome_rule_options::use_sorted_selection_set::UseSortedSelectionSetOptions;
use biome_string_case::comparable_token::ComparableToken;
use std::cmp::Ordering;

declare_source_rule! {
    /// Sort GraphQL selection sets.
    ///
    /// This rule orders fields first, fragment spreads next, and inline fragments last.
    /// Within each category, identifiers are sorted alphabetically.
    ///
    /// For fields, aliases are used as the sort key when present.
    /// This keeps selections deterministic and easier to scan in reviews.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```graphql,expect_diff
    /// query {
    ///   id
    ///   ...bFragment
    ///   firstName: name
    ///   ... on Contact {
    ///     phoneNumber
    ///     email
    ///   }
    ///   age
    ///   ... on Address {
    ///     street
    ///     city
    ///   }
    ///   ...aFragment
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```graphql
    /// query {
    ///   age
    ///   firstName: name
    ///   id
    ///   ...aFragment
    ///   ...bFragment
    ///   ... on Address {
    ///     city
    ///     street
    ///   }
    ///   ... on Contact {
    ///     email
    ///     phoneNumber
    ///   }
    /// }
    /// ```
    ///
    pub UseSortedSelectionSet {
        version: "next",
        name: "useSortedSelectionSet",
        language: "graphql",
        recommended: false,
        fix_kind: FixKind::Unsafe,
        sources: &[RuleSource::EslintGraphql("alphabetize").inspired()],
    }
}

impl Rule for UseSortedSelectionSet {
    type Query = Ast<GraphqlSelectionSet>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedSelectionSetOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let selections = node.selections();
        if is_selection_list_sorted(&selections) {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(RuleDiagnostic::new(
            rule_category!(),
            span,
            markup! {
                "Selection set is not sorted."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<GraphqlRuleAction> {
        let node = ctx.query();
        let fields = node.selections();
        let mut mutation = ctx.root().begin();

        sort_selection_list(&fields, &mut mutation)?;

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Sort selection set." },
            mutation,
        ))
    }
}

fn get_field_key(node: &GraphqlField) -> Option<TokenText> {
    if let Some(alias) = node.alias()
        && let Some(name) = alias.value().ok()
    {
        return name.value_token().ok().map(|t| t.token_text_trimmed());
    }

    node.name()
        .ok()?
        .value_token()
        .ok()
        .map(|t| t.token_text_trimmed())
}

fn get_fragment_spread_key(node: &GraphqlFragmentSpread) -> Option<TokenText> {
    node.name()
        .ok()?
        .value_token()
        .ok()
        .map(|t| t.token_text_trimmed())
}

fn get_inline_fragment_key(node: &GraphqlInlineFragment) -> Option<TokenText> {
    node.type_condition()?
        .ty()
        .ok()?
        .value_token()
        .ok()
        .map(|t| t.token_text_trimmed())
}

fn selection_group(selection: &AnyGraphqlSelection) -> u8 {
    match selection {
        AnyGraphqlSelection::GraphqlField(_) => 0,
        AnyGraphqlSelection::GraphqlFragmentSpread(_) => 1,
        AnyGraphqlSelection::GraphqlInlineFragment(_) => 2,
        _ => 3,
    }
}

fn selection_key(selection: &AnyGraphqlSelection) -> Option<TokenText> {
    match selection {
        AnyGraphqlSelection::GraphqlField(field) => get_field_key(field),
        AnyGraphqlSelection::GraphqlFragmentSpread(spread) => get_fragment_spread_key(spread),
        AnyGraphqlSelection::GraphqlInlineFragment(fragment) => get_inline_fragment_key(fragment),
        _ => None,
    }
}

fn compare_selection_keys(k1: &Option<TokenText>, k2: &Option<TokenText>) -> Ordering {
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

fn compare_selections(a: &AnyGraphqlSelection, b: &AnyGraphqlSelection) -> Ordering {
    let group_cmp = selection_group(a).cmp(&selection_group(b));
    if group_cmp != Ordering::Equal {
        return group_cmp;
    }

    let key_a = selection_key(a);
    let key_b = selection_key(b);
    compare_selection_keys(&key_a, &key_b)
}

fn is_selection_list_sorted(list: &GraphqlSelectionList) -> bool {
    let mut prev: Option<AnyGraphqlSelection> = None;
    for item in list {
        if prev
            .as_ref()
            .is_some_and(|previous| compare_selections(previous, &item) == Ordering::Greater)
        {
            return false;
        }

        prev = Some(item);
    }

    true
}

fn sort_selection_list(
    list: &GraphqlSelectionList,
    mutation: &mut BatchMutation<GraphqlLanguage>,
) -> Option<()> {
    let mut items: Vec<AnyGraphqlSelection> = list.iter().collect();
    items.sort_by(compare_selections);

    let sorted = GraphqlSelectionList::cast(SyntaxNode::new_detached(
        list.syntax().kind(),
        items
            .into_iter()
            .map(|selection| Some(NodeOrToken::Node(selection.into_syntax()))),
    ))?;

    mutation.replace_node(list.clone(), sorted);

    Some(())
}
