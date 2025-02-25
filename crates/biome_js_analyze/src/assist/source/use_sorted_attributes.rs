use std::{borrow::Cow, cmp::Ordering, iter::zip};

use biome_analyze::{
    context::RuleContext, declare_source_rule, Ast, Rule, RuleAction, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{AnyJsxAttribute, JsxAttribute, JsxAttributeList};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_source_rule! {
    /// Enforce attribute sorting in JSX elements.
    ///
    /// This rule checks if the JSX props are sorted in a consistent way.
    /// Props are sorted alphabetically.
    /// This rule will not consider spread props as sortable.
    /// Instead, whenever it encounters a spread prop, it will sort all the
    /// previous non spread props up until the nearest spread prop, if one
    /// exist.
    /// This prevents breaking the override of certain props using spread
    /// props.
    ///
    /// ## Examples
    ///
    /// ```js,expect_diff
    /// <Hello lastName="Smith" firstName="John" />;
    /// ```
    ///
    /// ```js,expect_diff
    /// <Hello lastName="Smith" firstName="John" {...this.props} tel="0000" address="111 Main Street"  {...another.props} lastName="Smith" />;
    /// ```
    ///
    pub UseSortedAttributes {
        version: "2.0.0",
        name: "useSortedAttributes",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-sort-props")],
        source_kind: RuleSourceKind::SameLogic,
    }
}

impl Rule for UseSortedAttributes {
    type Query = Ast<JsxAttributeList>;
    type State = PropGroup;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let props = ctx.query().clone();
        let mut current_prop_group = PropGroup::default();
        let mut prop_groups = Vec::new();
        for prop in props.clone() {
            match prop {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    current_prop_group.props.push(PropElement { prop: attr });
                }
                // spread prop reset sort order
                AnyJsxAttribute::JsxSpreadAttribute(_) => {
                    prop_groups.push(current_prop_group);
                    current_prop_group = PropGroup::default();
                }
            }
        }
        prop_groups.push(current_prop_group);
        prop_groups.into_boxed_slice()
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if state.is_sorted() {
            return None;
        }
        let mut mutation = ctx.root().begin();

        for (PropElement { prop }, PropElement { prop: sorted_prop }) in
            zip(state.props.clone(), state.get_sorted_props())
        {
            mutation.replace_node(prop, sorted_prop);
        }

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the JSX props." },
            mutation,
        ))
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct PropElement {
    prop: JsxAttribute,
}

impl Ord for PropElement {
    fn cmp(&self, other: &Self) -> Ordering {
        let (Ok(self_name), Ok(other_name)) = (self.prop.name(), other.prop.name()) else {
            return Ordering::Equal;
        };
        let (a_name, b_name) = (
            self_name.to_trimmed_string(),
            other_name.to_trimmed_string(),
        );

        a_name.cmp(&b_name)
    }
}

impl PartialOrd for PropElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Default)]
pub struct PropGroup {
    props: Vec<PropElement>,
}

impl PropGroup {
    fn is_sorted(&self) -> bool {
        let mut new_props = self.props.clone();
        new_props.sort();
        new_props == self.props
    }

    fn get_sorted_props(&self) -> Vec<PropElement> {
        let mut new_props = self.props.clone();
        new_props.sort();
        new_props
    }
}
