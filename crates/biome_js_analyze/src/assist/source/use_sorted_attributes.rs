use std::{borrow::Cow, cmp::Ordering, iter::zip};

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::{Applicability, category};
use biome_js_syntax::{
    AnyJsxAttribute, JsxAttribute, JsxAttributeList, JsxOpeningElement, JsxSelfClosingElement,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_sorted_attributes::UseSortedAttributesOptions;
use biome_string_case::StrLikeExtension;

use crate::JsRuleAction;

declare_source_rule! {
    /// Enforce attribute sorting in JSX elements.
    ///
    /// This rule checks if the JSX props are sorted in a consistent way.
    /// Props are sorted alphabetically using a [natural sort order](https://en.wikipedia.org/wiki/Natural_sort_order).
    /// This rule will not consider spread props as sortable.
    /// Instead, whenever it encounters a spread prop, it will sort all the
    /// previous non spread props up until the nearest spread prop, if one
    /// exist.
    /// This prevents breaking the override of certain props using spread
    /// props.
    ///
    /// ## Examples
    ///
    /// ```jsx,expect_diff
    /// <Hello lastName="Smith" firstName="John" />;
    /// ```
    ///
    /// ```jsx,expect_diff
    /// <Hello lastName="Smith" firstName="John" {...this.props} tel="0000" address="111 Main Street"  {...another.props} lastName="Smith" />;
    /// ```
    ///
    pub UseSortedAttributes {
        version: "2.0.0",
        name: "useSortedAttributes",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-sort-props").same()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedAttributes {
    type Query = Ast<JsxAttributeList>;
    type State = PropGroup;
    type Signals = Box<[Self::State]>;
    type Options = UseSortedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let props = ctx.query();
        let mut current_prop_group = PropGroup::default();
        let mut prop_groups = Vec::new();
        for prop in props {
            match prop {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    current_prop_group.props.push(PropElement { prop: attr });
                }
                // spread prop reset sort order
                AnyJsxAttribute::JsxSpreadAttribute(_) => {
                    if !current_prop_group.is_empty() && !current_prop_group.is_sorted() {
                        prop_groups.push(current_prop_group);
                        current_prop_group = PropGroup::default();
                    } else {
                        // Reuse the same buffer
                        current_prop_group.clear();
                    }
                }
                AnyJsxAttribute::JsMetavariable(_) => {}
            }
        }
        if !current_prop_group.is_empty() && !current_prop_group.is_sorted() {
            prop_groups.push(current_prop_group);
        }
        prop_groups.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedAttributes"),
            Self::text_range(ctx, state)?,
            markup! {
                "The attributes are not sorted. "
            },
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query().syntax().ancestors().find_map(|node| {
            JsxOpeningElement::cast_ref(&node)
                .map(|element| element.range())
                .or_else(|| JsxSelfClosingElement::cast_ref(&node).map(|element| element.range()))
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        for (PropElement { prop }, PropElement { prop: sorted_prop }) in
            zip(state.props.iter(), state.get_sorted_props())
        {
            mutation.replace_node(prop.clone(), sorted_prop);
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
        let (Ok(self_name), Ok(other_name)) = (self_name.name(), other_name.name()) else {
            return Ordering::Equal;
        };

        self_name
            .text_trimmed()
            .ascii_nat_cmp(other_name.text_trimmed())
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
    fn is_empty(&self) -> bool {
        self.props.is_empty()
    }

    fn is_sorted(&self) -> bool {
        self.props.is_sorted()
    }

    fn get_sorted_props(&self) -> Vec<PropElement> {
        let mut new_props = self.props.clone();
        new_props.sort_unstable();
        new_props
    }

    fn clear(&mut self) {
        self.props.clear();
    }
}
