use std::{borrow::Cow, cmp::Ordering};

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
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, SyntaxElement};
use biome_rule_options::use_sorted_attributes::{SortOrder, UseSortedAttributesOptions};
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
    /// ```jsx,use_options,expect_diagnostic
    /// <Hello tel={5555555} {...this.props} opt1="John" opt2="" opt12="" opt11="" />;
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
    /// ```jsx,use_options,expect_diagnostic
    /// <Hello tel={5555555} {...this.props} opt1="John" opt2="" opt12="" opt11="" />;
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
    type State = SortedMapping;
    type Signals = Option<Self::State>;
    type Options = UseSortedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let list = ctx.query();
        let options = ctx.options();
        let sort_by = options.sort_order.unwrap_or_default();

        let comparator = match sort_by {
            SortOrder::Natural => PropElement::ascii_nat_cmp,
            SortOrder::Lexicographic => PropElement::lexicographic_cmp,
        };

        // Build the sorted slot mapping for the entire list.
        // `sorted_indices[i]` is the index of the attribute that should end up
        // at slot `i` in the sorted list.
        let len = list.len();
        let mut sorted_indices: Vec<usize> = (0..len).collect();
        let mut has_unsorted = false;

        // Identify groups of consecutive JsxAttribute nodes (delimited by
        // spread/shorthand attributes) and sort each group independently.
        let mut group_start: Option<usize> = None;
        for (i, attr) in list.iter().enumerate() {
            match attr {
                AnyJsxAttribute::JsxAttribute(_) => {
                    if group_start.is_none() {
                        group_start = Some(i);
                    }
                }
                AnyJsxAttribute::JsxSpreadAttribute(_)
                | AnyJsxAttribute::JsxShorthandAttribute(_) => {
                    if let Some(start) = group_start.take() {
                        if sort_group(list, start, i, &comparator, &mut sorted_indices) {
                            has_unsorted = true;
                        }
                    }
                }
                AnyJsxAttribute::JsMetavariable(_) => {}
            }
        }
        // Handle trailing group
        if let Some(start) = group_start {
            if sort_group(list, start, len, &comparator, &mut sorted_indices) {
                has_unsorted = true;
            }
        }

        if has_unsorted {
            Some(SortedMapping { sorted_indices })
        } else {
            None
        }
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
        ctx.query().syntax().ancestors().skip(1).find_map(|node| {
            JsxOpeningElement::cast_ref(&node)
                .map(|element| element.range())
                .or_else(|| JsxSelfClosingElement::cast_ref(&node).map(|element| element.range()))
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let list = ctx.query();
        let mut mutation = ctx.root().begin();

        // Build a new attribute list with children in sorted order.
        // Replacing the entire list as a single node avoids slot-level
        // conflicts with mutations propagated from nested JSX elements
        // whose own attributes are also being sorted in the same fix pass.
        let slots: Vec<_> = list.syntax().slots().collect();
        let new_list = list.syntax().clone().splice_slots(
            0..slots.len(),
            state
                .sorted_indices
                .iter()
                .map(|&i| slots[i].clone().into_node().map(SyntaxElement::Node)),
        );

        mutation.replace_element_discard_trivia(list.syntax().clone().into(), new_list.into());

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the JSX props." },
            mutation,
        ))
    }
}

/// Sort a group of consecutive `JsxAttribute` nodes from `start..end` within
/// the attribute list. Returns `true` if the group was unsorted.
fn sort_group(
    list: &JsxAttributeList,
    start: usize,
    end: usize,
    comparator: &impl Fn(&PropElement, &PropElement) -> Ordering,
    sorted_indices: &mut [usize],
) -> bool {
    // Collect JsxAttribute nodes together with their slot indices so we can
    // map sorted positions back to the correct slots even when non-attribute
    // nodes (e.g. JsMetavariable) are interspersed.
    let group: Vec<(usize, PropElement)> = (start..end)
        .filter_map(|i| match list.iter().nth(i) {
            Some(AnyJsxAttribute::JsxAttribute(attr)) => Some((i, PropElement { prop: attr })),
            _ => None,
        })
        .collect();

    if group.is_empty()
        || group
            .windows(2)
            .all(|w| comparator(&w[0].1, &w[1].1) != Ordering::Greater)
    {
        return false;
    }

    // Build a mapping from sorted position to original slot index
    let mut order: Vec<usize> = (0..group.len()).collect();
    order.sort_by(|&a, &b| comparator(&group[a].1, &group[b].1));

    // The slot indices occupied by the group's attributes (in original order)
    let slot_positions: Vec<usize> = group.iter().map(|(slot, _)| *slot).collect();

    // Apply the mapping: sorted position `i` gets the slot of the attribute
    // that should appear at position `i`.
    for (sorted_pos, &orig_group_idx) in order.iter().enumerate() {
        sorted_indices[slot_positions[sorted_pos]] = slot_positions[orig_group_idx];
    }

    true
}

#[derive(Clone)]
pub struct SortedMapping {
    /// `sorted_indices[i]` is the original slot index of the attribute that
    /// should appear at position `i` in the sorted list.
    sorted_indices: Vec<usize>,
}

#[derive(PartialEq, Eq, Clone)]
struct PropElement {
    prop: JsxAttribute,
}

impl PropElement {
    fn ascii_nat_cmp(&self, other: &Self) -> Ordering {
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

    fn lexicographic_cmp(&self, other: &Self) -> Ordering {
        let (Ok(self_name), Ok(other_name)) = (self.prop.name(), other.prop.name()) else {
            return Ordering::Equal;
        };
        let (Ok(self_name), Ok(other_name)) = (self_name.name(), other_name.name()) else {
            return Ordering::Equal;
        };

        self_name
            .text_trimmed()
            .lexicographic_cmp(other_name.text_trimmed())
    }
}
