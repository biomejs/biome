use std::borrow::Cow;

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::{Applicability, category};
use biome_js_factory::make;
use biome_js_syntax::{JsObjectExpression, JsObjectMemberList, T};
use biome_rowan::{
    AstNode, AstSeparatedElement, AstSeparatedList, BatchMutationExt, TriviaPieceKind,
};

use crate::{
    JsRuleAction,
    assist::source::organize_imports::{
        comparable_token::ComparableToken, specifiers_attributes::handle_trvia,
    },
};

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
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let list = ctx.query();
        let mut previous_name: Option<ComparableToken> = None;
        for element in list.iter() {
            if let Some(name) = element.ok()?.name() {
                let name = ComparableToken(name);
                if let Some(previous_name) = previous_name {
                    if previous_name > name {
                        return Some(());
                    }
                }
                previous_name = Some(name);
            } else {
                // If a name cannot be extracted, then the current chunk of named properties stops here.
                previous_name = None;
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedKeys"),
            ctx.query().range(),
            markup! {
                "The keys are not sorted."
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

        // Collect all members
        let mut elements = Vec::with_capacity(list.len());
        for AstSeparatedElement {
            node,
            trailing_separator,
        } in list.elements()
        {
            let node = node.ok()?;
            let name = node.name().map(ComparableToken);
            let trailing_separator = trailing_separator.ok()?;
            elements.push((name, node, trailing_separator));
        }

        // Iterate over chunks of named properties
        for slice in elements.split_mut(|(name, _, _)| name.is_none()) {
            let last_has_separator = slice.last().is_some_and(|(_, _, sep)| sep.is_some());
            // Sort named properties
            slice.sort_by(|(name1, _, _), (name2, _, _)| name1.cmp(name2));
            handle_trvia(
                slice.iter_mut().map(|(_, node, sep)| (node, sep)),
                last_has_separator,
                || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            );
        }

        let separators: Vec<_> = elements
            .iter_mut()
            .filter_map(|(_, _, sep)| sep.take())
            .collect();
        let items: Vec<_> = elements.into_iter().map(|(_, node, _)| node).collect();
        let new_list = make::js_object_member_list(items, separators);

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(list.clone(), new_list);

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the object properties." },
            mutation,
        ))
    }
}
