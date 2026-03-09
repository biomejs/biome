use crate::HtmlRuleAction;
use biome_analyze::shared::sort_attributes::{AttributeGroup, SortableAttribute};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_source_rule,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::{Applicability, category};
use biome_html_syntax::{
    AnyHtmlAttribute, HtmlAttribute, HtmlAttributeList, HtmlLanguage, HtmlOpeningElement,
    HtmlSelfClosingElement,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxResult, SyntaxToken};
use biome_rule_options::use_sorted_attributes::{SortOrder, UseSortedAttributesOptions};
use std::{borrow::Cow, cmp::Ordering, iter::zip};

declare_source_rule! {
    /// Enforce attribute sorting in HTML elements.
    ///
    /// This rule checks if the HTML attributes are sorted in a consistent way.
    /// Attributes are sorted alphabetically using a [natural sort order](https://en.wikipedia.org/wiki/Natural_sort_order).
    ///
    /// This rule will not consider spread props, Vue, Svelte, or Astro directives as sortable.
    /// Instead, it will sort each group of consecutive HTML attributes within the element,
    /// leaving any spread props, Vue, Svelte, or Astro directives in place.
    /// This prevents breaking the override of certain props using spread
    /// props and avoids changing the behavior of Vue or Svelte code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <input type="text" id="name" name="name" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <textarea id="mytextarea" name="textarea" rows="5" cols="20" data-1="" data-11="" data-12="" data-2="">Hello, world!</textarea>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <input id="name" name="name" type="text" />
    /// ```
    ///
    /// ```html
    /// <textarea cols="20" data-1="" data-2="" data-11="" data-12="" id="mytextarea" name="textarea" rows="5">Hello, world!</textarea>
    /// ```
    ///
    /// ## Options
    ///
    /// The following options are available
    ///
    /// ### `sortOrder`
    /// The sort ordering to enforce.
    /// Values:
    ///
    /// - `"natural"`
    /// - `"lexicographic"`
    ///
    /// Default: `"natural"`
    ///
    /// #### Examples for `"sortOrder": "lexicographic"`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "lexicographic"
    ///     }
    /// }
    /// ```
    /// ```html,use_options,expect_diagnostic
    /// <textarea id="mytextarea" name="textarea" rows="5" cols="20" data-1="" data-2="" data-11="" data-12="">Hello, world!</textarea>
    /// ```
    ///
    pub UseSortedAttributes {
        version: "next",
        name: "useSortedAttributes",
        language: "html",
        recommended: false,
        sources: &[RuleSource::HtmlEslint("sort-attrs").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedAttributes {
    type Query = Ast<HtmlAttributeList>;
    type State = AttributeGroup<SortableHtmlAttribute>;
    type Signals = Box<[Self::State]>;
    type Options = UseSortedAttributesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attrs = ctx.query();
        let options = ctx.options();

        let mut current_attr_group = AttributeGroup::default();
        let mut attr_groups = Vec::new();
        let sort_by = options.sort_order.unwrap_or_default();

        let comparator = match sort_by {
            SortOrder::Natural => SortableHtmlAttribute::ascii_nat_cmp,
            SortOrder::Lexicographic => SortableHtmlAttribute::lexicographic_cmp,
        };

        // Convert to boolean-based comparator for is_sorted_by
        let boolean_comparator = |a: &SortableHtmlAttribute, b: &SortableHtmlAttribute| {
            comparator(a, b) != Ordering::Greater
        };

        for attr in attrs {
            match attr {
                AnyHtmlAttribute::HtmlAttribute(attr) => {
                    current_attr_group
                        .attrs
                        .push(SortableHtmlAttribute { attr });
                }
                _ => {
                    if !current_attr_group.is_empty()
                        && !current_attr_group.is_sorted(boolean_comparator)
                    {
                        attr_groups.push(current_attr_group);
                        current_attr_group = AttributeGroup::default();
                    } else {
                        // Reuse the same buffer
                        current_attr_group.clear();
                    }
                }
            }
        }
        if !current_attr_group.is_empty() && !current_attr_group.is_sorted(boolean_comparator) {
            attr_groups.push(current_attr_group);
        }
        attr_groups.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            category!("assist/source/useSortedAttributes"),
            Self::text_range(ctx, state)?,
            markup! {
                "The attributes are not sorted."
            },
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query().syntax().ancestors().skip(1).find_map(|node| {
            HtmlOpeningElement::cast_ref(&node)
                .map(|element| element.range())
                .or_else(|| HtmlSelfClosingElement::cast_ref(&node).map(|element| element.range()))
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        let options = ctx.options();
        let sort_by = options.sort_order.unwrap_or_default();

        let comparator = match sort_by {
            SortOrder::Natural => SortableHtmlAttribute::ascii_nat_cmp,
            SortOrder::Lexicographic => SortableHtmlAttribute::lexicographic_cmp,
        };

        for (SortableHtmlAttribute { attr }, SortableHtmlAttribute { attr: sorted_attr }) in
            zip(state.attrs.iter(), state.get_sorted_attributes(comparator))
        {
            mutation.replace_node_discard_trivia(attr.clone(), sorted_attr);
        }

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the HTML attributes." },
            mutation,
        ))
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct SortableHtmlAttribute {
    attr: HtmlAttribute,
}

impl SortableAttribute for SortableHtmlAttribute {
    type Language = HtmlLanguage;

    fn name(&self) -> SyntaxResult<SyntaxToken<Self::Language>> {
        self.attr.name()?.value_token()
    }
}
