use crate::JsonRuleAction;
use biome_analyze::utils::{is_separated_list_sorted_by, sorted_separated_list_by};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{JsonMemberList, JsonObjectValue, T, TextRange};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_sorted_keys::{SortOrder, UseSortedKeysOptions};
use biome_string_case::comparable_token::ComparableToken;
use std::ops::Not;

declare_source_rule! {
    /// Sorts the keys of a JSON object in natural order
    ///
    /// ## Examples
    ///
    /// ```json,expect_diff
    /// {
    ///     "vase": "fancy",
    ///     "nested": {
    ///         "omega": "bar",
    ///         "alpha": "foo"
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    /// This actions accepts following options
    ///
    /// ### `sortOrder`
    /// This options supports `natural` and `lexicographic` values. Where as `natural` is the default.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "natural"
    ///     }
    /// }
    /// ```
    ///
    pub UseSortedKeys {
        version: "1.9.0",
        name: "useSortedKeys",
        language: "json",
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsonMemberList>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let sort_order = options.sort_order;
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        is_separated_list_sorted_by(
            ctx.query(),
            |node| {
                node.name()
                    .ok()?
                    .inner_string_text()
                    .ok()
                    .map(ComparableToken::new)
            },
            comparator,
        )
        .ok()?
        .not()
        .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedKeys"),
            Self::text_range(ctx, state),
            markup! {
                "The members are not sorted by key."
            },
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query()
            .syntax()
            .ancestors()
            .find_map(JsonObjectValue::cast)
            .map(|node| node.range())
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsonRuleAction> {
        let list = ctx.query();
        let options = ctx.options();
        let sort_order = options.sort_order;
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        let new_list = sorted_separated_list_by(
            list,
            |node| {
                node.name()
                    .ok()?
                    .inner_string_text()
                    .ok()
                    .map(ComparableToken::new)
            },
            || make::token(T![,]),
            comparator,
        )
        .ok()?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(list.clone(), new_list);

        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Sort the members by key."
            },
            mutation,
        ))
    }
}
