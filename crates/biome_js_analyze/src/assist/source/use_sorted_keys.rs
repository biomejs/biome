use std::borrow::Cow;
use std::cmp::Ordering;

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, context::RuleContext, declare_source_rule,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::{Applicability, category};
use biome_js_syntax::{
    AnyJsObjectMember, AnyJsObjectMemberName, JsObjectExpression, JsObjectMemberList,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxResult, TokenText};
use biome_string_case::StrLikeExtension;

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
    /// Source: https://perfectionist.dev/rules/sort-objects
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
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsObjectMemberList>;
    type State = Vec<ObjectMember>;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member_list = ctx.query();
        let mut chunks = Vec::new();
        let mut current_chunk_members = Vec::with_capacity(member_list.len());

        let get_name = |name: SyntaxResult<AnyJsObjectMemberName>| name.ok()?.name();

        for (index, element) in member_list.elements().enumerate() {
            if let Ok(element) = element.into_node() {
                let name = match &element {
                    AnyJsObjectMember::JsSpread(_) | AnyJsObjectMember::JsBogusMember(_) => None,
                    AnyJsObjectMember::JsPropertyObjectMember(member) => get_name(member.name()),
                    AnyJsObjectMember::JsGetterObjectMember(member) => get_name(member.name()),
                    AnyJsObjectMember::JsSetterObjectMember(member) => get_name(member.name()),
                    AnyJsObjectMember::JsMethodObjectMember(member) => get_name(member.name()),
                    AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
                        member.name().and_then(|name| name.name()).ok()
                    }
                };
                if let Some(name) = name {
                    current_chunk_members.push(ObjectMember::new(element, name));
                } else {
                    // If a name cannot be extracted, then the current chunk of named properties stops here.
                    if !current_chunk_members.is_empty() && !current_chunk_members.is_sorted() {
                        chunks.push(current_chunk_members);
                        // Create a new buffer with the number of remaining members to test
                        current_chunk_members = Vec::with_capacity(member_list.len() - index - 1);
                    } else {
                        // Reuse the buffer
                        current_chunk_members.clear();
                    }
                }
            }
        }

        if !current_chunk_members.is_empty() && !current_chunk_members.is_sorted() {
            chunks.push(current_chunk_members);
        }

        chunks.into_boxed_slice()
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

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut sorted_state = state.clone();

        // We use a stable sort to ensure that properties with an identical name (a getter and a setter for a property)
        // keep their initial relative order.
        sorted_state.sort();

        let mut mutation = ctx.root().begin();

        for (unsorted, sorted) in state.iter().zip(sorted_state.iter()) {
            mutation.replace_node_discard_trivia(unsorted.member.clone(), sorted.member.clone());
        }

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the object properties." },
            mutation,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct ObjectMember {
    member: AnyJsObjectMember,
    name: TokenText,
}
impl ObjectMember {
    fn new(member: AnyJsObjectMember, name: TokenText) -> Self {
        Self { member, name }
    }
}
impl Eq for ObjectMember {}
impl PartialEq for ObjectMember {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Ord for ObjectMember {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.text().ascii_nat_cmp(other.name.text())
    }
}
impl PartialOrd for ObjectMember {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test() {}
