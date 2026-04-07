use std::{borrow::Cow, cmp::Ordering, iter::zip};

use biome_analyze::shared::sort_attributes::{AttributeGroup, SortableAttribute};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::Applicability;
use biome_js_syntax::{
    AnyJsxAttribute, JsLanguage, JsxAttribute, JsxAttributeList, JsxOpeningElement,
    JsxSelfClosingElement,
};
use biome_rowan::{AstNode, AstNodeExt, BatchMutationExt, SyntaxToken};
use biome_rule_options::use_sorted_attributes::{SortOrder, UseSortedAttributesOptions};

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
    type State = AttributeGroup<SortableJsxAttribute>;
    type Signals = Box<[Self::State]>;
    type Options = UseSortedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let props = ctx.query();
        let mut current_prop_group = AttributeGroup::default();
        let mut prop_groups = Vec::new();
        let options = ctx.options();
        let sort_by = options.sort_order.unwrap_or_default();

        let comparator = match sort_by {
            SortOrder::Natural => SortableJsxAttribute::ascii_nat_cmp,
            SortOrder::Lexicographic => SortableJsxAttribute::lexicographic_cmp,
        };

        // Convert to boolean-based comparator for is_sorted_by
        let boolean_comparator = |a: &SortableJsxAttribute, b: &SortableJsxAttribute| {
            comparator(a, b) != Ordering::Greater
        };

        for prop in props {
            match prop {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    current_prop_group.attrs.push(SortableJsxAttribute(attr));
                }
                // spread prop reset sort order
                AnyJsxAttribute::JsxSpreadAttribute(_) => {
                    if !current_prop_group.is_empty()
                        && !current_prop_group.is_sorted(boolean_comparator)
                    {
                        prop_groups.push(current_prop_group);
                        current_prop_group = AttributeGroup::default();
                    } else {
                        // Reuse the same buffer
                        current_prop_group.clear();
                    }
                }
                AnyJsxAttribute::JsMetavariable(_) => {}
            }
        }
        if !current_prop_group.is_empty() && !current_prop_group.is_sorted(boolean_comparator) {
            prop_groups.push(current_prop_group);
        }
        prop_groups.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
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
        let mut mutation = ctx.root().begin();
        let options = ctx.options();
        let sort_by = options.sort_order.unwrap_or_default();

        let comparator = match sort_by {
            SortOrder::Natural => SortableJsxAttribute::ascii_nat_cmp,
            SortOrder::Lexicographic => SortableJsxAttribute::lexicographic_cmp,
        };

        for (SortableJsxAttribute(attr), SortableJsxAttribute(sorted_attr)) in
            zip(state.attrs.iter(), state.get_sorted_attributes(comparator)?)
        {
            mutation.replace_node_discard_trivia(attr.clone(), sorted_attr);
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
pub struct SortableJsxAttribute(JsxAttribute);

impl SortableAttribute for SortableJsxAttribute {
    type Language = JsLanguage;

    fn name(&self) -> Option<SyntaxToken<Self::Language>> {
        self.0.name().ok()?.name_token().ok()
    }

    fn node(&self) -> &impl AstNode<Language = Self::Language> {
        &self.0
    }

    fn replace_token(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self(
            self.0
                .replace_token_discard_trivia(prev_token, next_token)?,
        ))
    }
}
