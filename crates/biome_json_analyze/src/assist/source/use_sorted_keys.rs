use crate::JsonRuleAction;
use biome_analyze::utils::{is_separated_list_sorted_by, sorted_separated_list_by};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_diagnostics::category;
use biome_json_factory::make;
use biome_json_syntax::{
    AnyJsonValue, JsonLanguage, JsonMemberList, JsonObjectValue, T, TextRange,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxResult, SyntaxToken};
use biome_rule_options::use_sorted_keys::{SortOrder, UseSortedKeysOptions};
use biome_string_case::comparable_token::ComparableToken;
use std::{cmp::Ordering, ops::Not};

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
    /// ```json,use_options,expect_diff
    /// {
    ///     "val13": 1,
    ///     "val1": 1,
    ///     "val2": 1,
    ///     "val21": 1,
    ///     "val11": 1
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
    /// ```json,use_options,expect_diff
    /// {
    ///     "val13": 1,
    ///     "val1": 1,
    ///     "val2": 1,
    ///     "val21": 1,
    ///     "val11": 1
    /// }
    /// ```
    ///
    /// ### `groupByNesting`
    /// When enabled, groups object keys by their value's nesting depth before sorting alphabetically.
    /// Simple values (primitives, single-line arrays, and single-line objects) are sorted first,
    /// followed by nested values (multi-line arrays and multi-line objects).
    ///
    /// > Default: `false`
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
    ///     "id": 123
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

/// Checks if an object/array spans multiple lines by examining CST trivia.
/// For non-empty containers, checks the first token of the members/elements.
/// For empty containers, checks the closing brace/bracket token.
fn has_multiline_content(
    members_first_token: Option<SyntaxToken<JsonLanguage>>,
    closing_token: SyntaxResult<SyntaxToken<JsonLanguage>>,
) -> bool {
    members_first_token.map_or_else(
        || {
            closing_token
                .map(|token| token.has_leading_newline())
                .unwrap_or(false)
        },
        |token| token.has_leading_newline(),
    )
}

/// Determines the nesting depth of a JSON value for grouping purposes.
/// Multi-line objects and multi-line arrays are considered nested (depth 1).
/// Primitives, single-line arrays, and single-line objects are considered simple (depth 0).
fn get_nesting_depth(value: &AnyJsonValue) -> Ordering {
    match value {
        AnyJsonValue::JsonObjectValue(obj) => {
            let members = obj.json_member_list();
            if has_multiline_content(members.syntax().first_token(), obj.r_curly_token()) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        AnyJsonValue::JsonArrayValue(array) => {
            let elements = array.elements();
            if has_multiline_content(elements.syntax().first_token(), array.r_brack_token()) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        _ => Ordering::Equal, // primitives: string, number, boolean, null
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsonMemberList>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let sort_order = options.sort_order.unwrap_or_default();
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        if options.group_by_nesting.unwrap_or(false) {
            is_separated_list_sorted_by(
                ctx.query(),
                |node| {
                    let value = node.value().ok()?;
                    let depth = get_nesting_depth(&value);
                    let name = node
                        .name()
                        .ok()?
                        .as_json_member_name()?
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
                        .as_json_member_name()?
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

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let options = ctx.options();
        let message = if options.group_by_nesting.unwrap_or(false) {
            markup! {
                "The members are not sorted by nesting level and key."
            }
        } else {
            markup! {
                "The members are not sorted by key."
            }
        };
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedKeys"),
            Self::text_range(ctx, state),
            message,
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
        let sort_order = options.sort_order.unwrap_or_default();
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        let new_list = if options.group_by_nesting.unwrap_or(false) {
            sorted_separated_list_by(
                list,
                |node| {
                    let value = node.value().ok()?;
                    let depth = get_nesting_depth(&value);
                    let name = node
                        .name()
                        .ok()?
                        .as_json_member_name()?
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
                        .as_json_member_name()?
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
