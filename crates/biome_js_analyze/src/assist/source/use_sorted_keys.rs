use std::borrow::Cow;
use std::cmp::Ordering;

use biome_analyze::{Ast, Rule, RuleAction, context::RuleContext, declare_source_rule};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{AnyJsObjectMember, AnyJsObjectMemberName, JsObjectMemberList};
use biome_rowan::{AstSeparatedList, BatchMutationExt, SyntaxResult};

use crate::JsRuleAction;

declare_source_rule! {
    /// Enforce ordering of a JS object properties.
    ///
    /// This rule checks if keys of the object are sorted in a consistent way.
    /// Keys are sorted in a natural order.
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
        version: "next",
        name: "useSortedKeys",
        language: "js",
        recommended: false,
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsObjectMemberList>;
    type State = Vec<Vec<ObjectMember>>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut members = vec![];
        let mut groups = vec![];

        let get_name =
            |name: SyntaxResult<AnyJsObjectMemberName>| Some(name.ok()?.name()?.text().into());

        for element in ctx.query().elements() {
            if let Ok(element) = element.node() {
                match element {
                    AnyJsObjectMember::JsSpread(_) | AnyJsObjectMember::JsBogusMember(_) => {
                        // Keep the spread order because it's not safe to change order of it.
                        // Logic here is similar to /crates/biome_js_analyze/src/assists/source/use_sorted_attributes.rs
                        groups.push(members.clone());
                        members.clear();
                        members.push(ObjectMember::new(element.clone(), None));
                    }
                    AnyJsObjectMember::JsPropertyObjectMember(member) => {
                        members.push(ObjectMember::new(element.clone(), get_name(member.name())));
                    }
                    AnyJsObjectMember::JsGetterObjectMember(member) => {
                        members.push(ObjectMember::new(element.clone(), get_name(member.name())));
                    }
                    AnyJsObjectMember::JsSetterObjectMember(member) => {
                        members.push(ObjectMember::new(element.clone(), get_name(member.name())));
                    }
                    AnyJsObjectMember::JsMethodObjectMember(member) => {
                        members.push(ObjectMember::new(element.clone(), get_name(member.name())));
                    }
                    AnyJsObjectMember::JsShorthandPropertyObjectMember(member) => {
                        let name = member
                            .name()
                            .ok()
                            .map(|name| Some(name.name().ok()?.text().into()))
                            .unwrap_or_default();

                        members.push(ObjectMember::new(element.clone(), name));
                    }
                }
            }
        }

        if !members.is_empty() {
            groups.push(members);
        }

        Some(groups)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut sorted_state = state.clone();

        for group in sorted_state.iter_mut() {
            group.sort();
        }

        if sorted_state == *state {
            return None;
        }

        let mut mutation = ctx.root().begin();

        for (unsorted, sorted) in state.iter().flatten().zip(sorted_state.iter().flatten()) {
            mutation.replace_node(unsorted.member.clone(), sorted.member.clone());
        }

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the object properties." },
            mutation,
        ))
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct ObjectMember {
    member: AnyJsObjectMember,
    name: Option<Box<str>>,
}

impl ObjectMember {
    fn new(member: AnyJsObjectMember, name: Option<Box<str>>) -> Self {
        ObjectMember { member, name }
    }
}

impl Ord for ObjectMember {
    fn cmp(&self, other: &Self) -> Ordering {
        // If some doesn't have a name (e.g spread/calculated property) - keep the order.
        let (Some(self_name), Some(other_name)) = (&self.name, &other.name) else {
            return Ordering::Equal;
        };

        natord::compare(self_name, other_name)
    }
}

impl PartialOrd for ObjectMember {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
