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
use biome_rowan::{AstNode, AstNodeList, AstNodeListExt, BatchMutationExt};
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
    type State = SortedAttrList;
    type Signals = Option<Self::State>;
    type Options = UseSortedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let list = ctx.query();
        let options = ctx.options();
        let comparator = match options.sort_order.unwrap_or_default() {
            SortOrder::Natural => PropElement::ascii_nat_cmp,
            SortOrder::Lexicographic => PropElement::lexicographic_cmp,
        };

        // Snapshot all attributes once. `sorted_slots[i]` is the original
        // slot index of the attribute that should appear at position `i`.
        // Non-attribute slots (spread, shorthand, metavariable) keep their
        // identity mapping; only JsxAttribute groups get reordered.
        let attrs: Vec<AnyJsxAttribute> = list.iter().collect();
        let mut sorted_slots: Vec<usize> = (0..attrs.len()).collect();
        let mut any_unsorted = false;

        // Identify groups of consecutive JsxAttributes split by spread/shorthand:
        // a spread carries an opaque expression that may have side effects on
        // the resulting prop set, so attributes on either side cannot be
        // freely reordered across it.
        let mut group_start: Option<usize> = None;
        for (i, attr) in attrs.iter().enumerate() {
            match attr {
                AnyJsxAttribute::JsxAttribute(_) => {
                    if group_start.is_none() {
                        group_start = Some(i);
                    }
                }
                AnyJsxAttribute::JsxSpreadAttribute(_)
                | AnyJsxAttribute::JsxShorthandAttribute(_) => {
                    if let Some(start) = group_start.take()
                        && sort_group(&attrs, start, i, &comparator, &mut sorted_slots)
                    {
                        any_unsorted = true;
                    }
                }
                AnyJsxAttribute::JsMetavariable(_) => {}
            }
        }
        if let Some(start) = group_start
            && sort_group(&attrs, start, attrs.len(), &comparator, &mut sorted_slots)
        {
            any_unsorted = true;
        }

        any_unsorted.then_some(SortedAttrList { sorted_slots })
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
        let attrs: Vec<AnyJsxAttribute> = list.iter().collect();

        // Replace the entire list as a single mutation. This avoids slot-level
        // conflicts with mutations propagated up from nested JSX elements
        // whose own attributes are also being sorted in the same fix pass.
        let new_list = list.clone().splice(
            0..attrs.len(),
            state.sorted_slots.iter().map(|&i| attrs[i].clone()),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(list.clone(), new_list);

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the JSX props." },
            mutation,
        ))
    }
}

/// Sort the JsxAttribute slots in `attrs[start..end]` in place by writing the
/// resulting slot order into `sorted_slots`. Returns `true` if the group was
/// not already sorted (i.e. the rule should report a diagnostic).
fn sort_group(
    attrs: &[AnyJsxAttribute],
    start: usize,
    end: usize,
    comparator: &impl Fn(&PropElement, &PropElement) -> Ordering,
    sorted_slots: &mut [usize],
) -> bool {
    // Collect (slot_index, PropElement) only for JsxAttribute slots; non-attribute
    // slots within the range (e.g. JsMetavariable) keep their identity mapping.
    let group: Vec<(usize, PropElement)> = (start..end)
        .filter_map(|i| match &attrs[i] {
            AnyJsxAttribute::JsxAttribute(attr) => Some((
                i,
                PropElement {
                    prop: attr.clone(),
                },
            )),
            _ => None,
        })
        .collect();

    if group.is_empty() {
        return false;
    }
    if group
        .windows(2)
        .all(|w| comparator(&w[0].1, &w[1].1) != Ordering::Greater)
    {
        return false;
    }

    let mut sorted = group.clone();
    sorted.sort_by(|a, b| comparator(&a.1, &b.1));
    for (i, (orig_slot, _)) in group.iter().enumerate() {
        sorted_slots[*orig_slot] = sorted[i].0;
    }
    true
}

#[derive(Clone)]
pub struct SortedAttrList {
    /// `sorted_slots[i]` is the original slot index of the attribute that
    /// should appear at position `i` in the sorted list.
    sorted_slots: Vec<usize>,
}

#[derive(PartialEq, Eq, Clone)]
struct PropElement {
    prop: JsxAttribute,
}

impl PropElement {
    pub fn ascii_nat_cmp(&self, other: &Self) -> Ordering {
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

    pub fn lexicographic_cmp(&self, other: &Self) -> Ordering {
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
