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
use biome_js_syntax::{JsObjectExpression, JsObjectMemberList, T};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::use_sorted_keys::UseSortedKeysOptions;
use biome_string_case::comparable_token::ComparableToken;

use crate::JsRuleAction;

declare_source_rule! {
    /// Enforce ordering of a JS object properties.
    ///
    /// This rule checks if keys of the object are sorted in a consistent way.
    /// Keys are sorted in a [natural sort order](https://en.wikipedia.org/wiki/Natural_sort_order).
    /// This rule will consider spread/calculated keys e.g [k]: 1 as non-sortable.
    /// Instead, whenever it encounters a non-sortable key, it will sort all the
    /// previous sortable keys up until the nearest non-sortable key, if one
    /// exist.
    /// This prevents breaking the override of certain keys using spread
    /// keys.
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
    pub UseSortedKeys {
        version: "2.0.0",
        name: "useSortedKeys",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintPerfectionist("sort-objects").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsObjectMemberList>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        is_separated_list_sorted_by(ctx.query(), |node| node.name().map(ComparableToken::new))
            .ok()?
            .not()
            .then_some(())
    }

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

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let list = ctx.query();

        let new_list = sorted_separated_list_by(
            list,
            |node| node.name().map(ComparableToken::new),
            || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
        )
        .ok()?;

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
