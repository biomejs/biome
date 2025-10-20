use std::{borrow::Cow, ops::Not};

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource,
    context::RuleContext,
    declare_source_rule,
    utils::{is_separated_list_sorted_by, sorted_separated_list_by},
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::{Applicability, category};
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, JsObjectExpression, JsObjectMemberList, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::use_sorted_keys::{SortOrder, UseSortedKeysOptions};
use biome_string_case::comparable_token::ComparableToken;

use crate::JsRuleAction;

declare_source_rule! {
    /// Sort properties of a JS object in natural order.
    ///
    /// [Natural order](https://en.wikipedia.org/wiki/Natural_sort_order) means
    /// that uppercase letters come before lowercase letters (e.g. `A` < `a` <
    /// `B` < `b`) and numbers are compared in a human way (e.g. `9` < `10`).
    ///
    /// This rule will consider spread/calculated keys e.g [k]: 1 as
    /// non-sortable. Instead, whenever it encounters a non-sortable key, it
    /// will sort all the previous sortable keys up until the nearest
    /// non-sortable key, if one exist. This prevents breaking the override of
    /// certain keys using spread keys.
    ///
    /// Sorting the keys of an object technically changes the semantics of the
    /// program. It affects the result of operations like
    /// `Object.getOwnPropertyNames`. Since ES2020, operations like `for-in`
    /// loops, `Object.keys`, and `JSON.stringify` are guaranteed to process
    /// string keys in insertion order.
    ///
    /// In cases where the order of such operations is important, you can
    /// disable the assist action using a suppression comment:
    ///
    /// `// biome-ignore assist/source/useSortedKeys`
    ///
    /// ## Examples
    ///
    /// ```js,expect_diff
    /// {
    ///   x: 1,
    ///   a: 2,
    /// };
    /// ```
    ///
    /// ```js,expect_diff
    /// {
    ///   x: 1,
    ///   ...f,
    ///   y: 4,
    ///   a: 2,
    ///   [calculated()]: true,
    ///   b: 3,
    ///   a: 1,
    /// };
    /// ```
    ///
    /// ```js,expect_diff
    /// {
    ///   get aab() {
    ///     return this._aab;
    ///   },
    ///   set aac(v) {
    ///     this._aac = v;
    ///   },
    ///   w: 1,
    ///   x: 1,
    ///   ...g,
    ///   get aaa() {
    ///     return "";
    ///   },
    ///   u: 1,
    ///   v: 1,
    ///   [getProp()]: 2,
    ///   o: 1,
    ///   p: 1,
    ///   q: 1,
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
    /// ```js,use_options,expect_diagnostic
    /// const obj = {
    ///     val13: 1,
    ///     val1: 1,
    ///     val2: 1,
    ///     val21: 1,
    ///     val11: 1,
    /// };
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
    /// ```js,use_options,expect_diagnostic
    /// const obj = {
    ///     val13: 1,
    ///     val1: 1,
    ///     val2: 1,
    ///     val21: 1,
    ///     val11: 1,
    /// };
    /// ```
    ///
    /// ### `groupByNesting`
    /// When enabled, groups object keys by their value's nesting depth before sorting alphabetically.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groupByNesting": true
    ///     }
    /// }
    /// ```
    /// ```js,use_options,expect_diagnostic
    /// const obj = {
    ///     name: "Sample",
    ///     details: {
    ///         description: "nested"
    ///     },
    ///     id: "123"
    /// };
    /// ```
    ///
    pub UseSortedKeys {
        version: "2.0.0",
        name: "useSortedKeys",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintPerfectionist("sort-objects").inspired()],
        fix_kind: FixKind::Safe,
    }
}

/// Compute a simple nesting depth indicator for a JavaScript expression used when grouping object members.
///
/// The function returns `1` for object expressions and for array expressions that span multiple lines;
/// it returns `0` for single-line arrays and all other expression kinds.
///
/// # Examples
///
/// ```rust,ignore
/// // object literal -> depth 1
/// let obj_expr: AnyJsExpression = parse_expression("({ a: 1 })");
/// assert_eq!(get_nesting_depth_js(&obj_expr), 1);
///
/// // single-line array -> depth 0
/// let arr_expr: AnyJsExpression = parse_expression("[1, 2, 3]");
/// assert_eq!(get_nesting_depth_js(&arr_expr), 0);
///
/// // multi-line array -> depth 1
/// let multi_arr_expr: AnyJsExpression = parse_expression("[\n  1,\n  2\n]");
/// assert_eq!(get_nesting_depth_js(&multi_arr_expr), 1);
/// ```
///
/// # Returns
///
/// `1` for object expressions and for arrays that contain a newline (span multiple lines), `0` otherwise.
fn get_nesting_depth_js(value: &AnyJsExpression) -> u8 {
    match value {
        AnyJsExpression::JsObjectExpression(_) => 1,
        AnyJsExpression::JsArrayExpression(array) => {
            // Check if array spans multiple lines by looking for newlines
            if array.syntax().text_trimmed().contains_char('\n') {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}

/// Extracts the value expression from an object member
fn get_member_value(node: &AnyJsObjectMember) -> Option<AnyJsExpression> {
    match node {
        AnyJsObjectMember::JsPropertyObjectMember(prop) => prop.value().ok(),
        _ => None, // Getters, setters, methods, etc. treated as non-nested
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsObjectMemberList>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedKeysOptions;

    /// Determines whether the object member list in the query is sorted according to the rule options.
    ///
    /// When `group_by_nesting` is enabled, members are compared by a pair (nesting depth, property name),
    /// where nesting depth is computed from the member's value and property name is the member key token;
    /// otherwise members are compared by property name alone. The comparator is chosen from `sort_order`
    /// (natural or lexicographic).
    ///
    /// # Returns
    ///
    /// `Some(())` if the members are not sorted according to the configured options, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use crate::{RuleContext, UseSortedKeys};
    /// # fn example(ctx: &RuleContext<UseSortedKeys>) {
    /// let signal = UseSortedKeys::run(ctx);
    /// # }
    /// ```
    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
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
                    let value = get_member_value(node)?;
                    let depth = get_nesting_depth_js(&value);
                    let name = node.name().map(ComparableToken::new)?;
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
                |node| node.name().map(ComparableToken::new),
                comparator,
            )
            .ok()?
            .not()
            .then_some(())
        }
    }

    /// Produces a diagnostic that marks an object whose properties are not sorted by key.
    ///
    /// The diagnostic uses the category `assist/source/useSortedKeys` and targets the query range
    /// with the message "The object properties are not sorted by key."
    ///
    /// # Examples
    ///
    /// ```
    /// // In a rule implementation, `diagnostic(ctx, &state)` is used to create the diagnostic
    /// // shown below. `ctx` is provided by the rule runner.
    /// use biome_diagnostics::RuleDiagnostic;
    ///
    /// let expected = RuleDiagnostic::new(
    ///     category!("assist/source/useSortedKeys"),
    ///     /* range */ Default::default(),
    ///     "The object properties are not sorted by key.",
    /// );
    /// ```
    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedKeys"),
            ctx.query().range(),
            markup! {
                "The object properties are not sorted by key."
            },
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query()
            .syntax()
            .ancestors()
            .find_map(JsObjectExpression::cast)
            .map(|object| object.range())
    }

    /// Builds a rule action that replaces the object property list with a sorted version according to the rule options.
    ///
    /// When `group_by_nesting` is enabled the list is sorted by a tuple of (nesting depth of the member's value, property key);
    /// otherwise the list is sorted by property key only. The sorting uses either natural or lexicographic ordering depending
    /// on the configured `sort_order`. If sorting or list construction fails, the function returns `None`.
    ///
    /// # Returns
    ///
    /// `Some(JsRuleAction)` containing a mutation that replaces the original object member list with the sorted list if sorting
    /// succeeded, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a rule context `ctx` and state `state`, produce the optional fix action:
    /// let action = action(&ctx, &state);
    /// ```
    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
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
                    let value = get_member_value(node)?;
                    let depth = get_nesting_depth_js(&value);
                    let name = node.name().map(ComparableToken::new)?;
                    Some((depth, name))
                },
                || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                |(d1, n1), (d2, n2)| d1.cmp(d2).then_with(|| comparator(n1, n2)),
            )
            .ok()?
        } else {
            sorted_separated_list_by(
                list,
                |node| node.name().map(ComparableToken::new),
                || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                comparator,
            )
            .ok()?
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(list.clone(), new_list);

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the object properties by key." },
            mutation,
        ))
    }
}