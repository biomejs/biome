use crate::JsonRuleAction;
use biome_analyze::utils::{is_separated_list_sorted_by, sorted_separated_list_by};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMemberList, JsonObjectValue, T, TextRange};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_sorted_keys::{SortOrder, UseSortedKeysOptions};
use biome_string_case::comparable_token::ComparableToken;
use std::ops::Not;

declare_source_rule! {
    /// Sort the keys of a JSON object in natural order.
    ///
    /// [Natural order](https://en.wikipedia.org/wiki/Natural_sort_order) means
    /// that uppercase letters come before lowercase letters (e.g. `A` < `a` <
    /// `B` < `b`) and numbers are compared in a human way (e.g. `9` < `10`).
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
    /// Following will apply the natural sort order.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "natural"
    ///     }
    /// }
    /// ```
    /// ```json,use_options,expect_diagnostic
    /// {
    ///     "val13": 1,
    ///     "val1": 1,
    ///     "val2": 1,
    ///     "val21": 1,
    ///     "val11": 1,
    /// }
    /// ```
    ///
    /// Following will apply the lexicographic sort order.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "lexicographic"
    ///     }
    /// }
    /// ```
    /// ```json,use_options,expect_diagnostic
    /// {
    ///     "val13": 1,
    ///     "val1": 1,
    ///     "val2": 1,
    ///     "val21": 1,
    ///     "val11": 1,
    /// }
    /// ```
    ///
    /// ### `groupByNesting`
    /// When enabled, groups object keys by their value's nesting depth before sorting alphabetically.
    /// Simple values (primitives and single-line arrays) are sorted first, followed by nested values
    /// (objects and multi-line arrays).
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groupByNesting": true
    ///     }
    /// }
    /// ```
    /// ```json,use_options,expect_diagnostic
    /// {
    ///     "name": "Sample",
    ///     "details": {
    ///         "description": "nested"
    ///     },
    ///     "id": "123"
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

/// Determines the nesting depth of a JSON value for grouping purposes.
/// Objects and multi-line arrays are considered nested (depth 1).
/// Primitives and single-line arrays are considered simple (depth 0).
fn get_nesting_depth(value: &AnyJsonValue) -> u8 {
    match value {
        AnyJsonValue::JsonObjectValue(_) => 1,
        AnyJsonValue::JsonArrayValue(array) => {
            // Check if array spans multiple lines by looking for newlines
            if array.to_string().contains('\n') {
                1
            } else {
                0
            }
        }
        _ => 0, // primitives: string, number, boolean, null
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsonMemberList>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedKeysOptions;

    /// Determines whether the queried JSON object members are sorted according to the rule options and signals when they are not.
    ///
    /// When `group_by_nesting` is enabled in the rule options, members are ordered first by their nesting depth (objects and multi-line arrays count as deeper) and then by key using the configured sort order (natural or lexicographic). When `group_by_nesting` is disabled, members are ordered by key only using the configured sort order.
    ///
    /// # Returns
    ///
    /// `Some(())` if the member list in the query is not sorted according to the configured options, `None` if it is sorted.
    ///
    /// # Examples
    ///
    /// ```
    /// // The function returns `Some(())` to indicate unsorted lists and `None` for sorted lists.
    /// let unsorted: Option<()> = Some(());
    /// let sorted: Option<()> = None;
    /// assert_eq!(unsorted, Some(()));
    /// assert_eq!(sorted, None);
    /// ```
    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let sort_order = options.sort_order;
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        if options.group_by_nesting {
            is_separated_list_sorted_by(
                ctx.query(),
                |node| {
                    let value = node.value().ok()?;
                    let depth = get_nesting_depth(&value);
                    let name = node
                        .name()
                        .ok()?
                        .inner_string_text()
                        .ok()
                        .map(ComparableToken::new)?;
                    Some((depth, name))
                },
                |(d1, n1), (d2, n2)| d1.cmp(d2).then_with(|| comparator(n1, n2)),
            )
            .ok()?
            .not()
            .then_some(())
        } else {
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
    }

    /// Create a diagnostic indicating that the members of the containing JSON object are not sorted by key.
    ///
    /// The diagnostic uses category "assist/source/useSortedKeys", the text range of the enclosing object, and the message "The members are not sorted by key."
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a rule context `ctx` and state `state` produced by `run`, this returns a diagnostic
    /// // that can be reported to the user.
    /// let diag = UseSortedKeys::diagnostic(&ctx, &state);
    /// assert!(diag.is_some());
    /// ```
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

    /// Produces an automatic fix that replaces the JSON object member list with a version sorted by key.
    ///
    /// When applied, the returned action mutates the source to sort members according to the rule's
    /// options: keys are ordered either by natural or lexicographic comparison, and when
    /// `group_by_nesting` is enabled members are grouped by their nesting depth before sorting by name.
    ///
    /// # Returns
    ///
    /// `Some(JsonRuleAction)` containing a mutation that replaces the original member list with the
    /// sorted list; `None` if no action can be constructed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// // Given a rule context `ctx` for a JsonMemberList, produce the fix action:
    /// let action = action(&ctx, &());
    /// if let Some(rule_action) = action {
    ///     // apply or inspect `rule_action`
    /// }
    /// ```
    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsonRuleAction> {
        let list = ctx.query();
        let options = ctx.options();
        let sort_order = options.sort_order;
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        let new_list = if options.group_by_nesting {
            sorted_separated_list_by(
                list,
                |node| {
                    let value = node.value().ok()?;
                    let depth = get_nesting_depth(&value);
                    let name = node
                        .name()
                        .ok()?
                        .inner_string_text()
                        .ok()
                        .map(ComparableToken::new)?;
                    Some((depth, name))
                },
                || make::token(T![,]),
                |(d1, n1), (d2, n2)| d1.cmp(d2).then_with(|| comparator(n1, n2)),
            )
            .ok()?
        } else {
            sorted_separated_list_by(
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
            .ok()?
        };

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