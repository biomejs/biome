use std::{borrow::Cow, cmp::Ordering, iter::zip};

use biome_analyze::shared::sort_attributes::{AttributeGroup, SortableAttribute};
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, RuleSuppressions,
    context::RuleContext, declare_source_rule,
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
    /// Sort JSX attributes by name.
    ///
    /// By default, the action uses [natural order](https://en.wikipedia.org/wiki/Natural_sort_order),
    /// which compares numbers by value so that `prop9` comes before `prop10`.
    ///
    /// A spread attribute such as `{...properties}` can provide or replace any attribute. The
    /// action therefore treats each spread as a boundary and sorts only the named attributes on
    /// each side. It never moves an attribute across a spread.
    ///
    /// ## Examples
    ///
    /// ```jsx,expect_diff
    /// <Hello lastName="Smith" firstName="John" />;
    /// ```
    ///
    /// ```jsx,expect_diff
    /// <Hello lastName="Smith" firstName="John" {...this.props} tel="0000" address="111 Main Street" {...another.props} />;
    /// ```
    ///
    /// ## Options
    ///
    /// ### `sortOrder`
    ///
    /// Selects `natural` or `lexicographic` ordering. Natural ordering compares numbers by value
    /// and is the default. Lexicographic ordering compares names character by character.
    ///
    /// The following configuration uses natural order:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "natural"
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options,expect_diff
    /// <Hello {...this.props} opt1="" opt2="" opt12="" opt11="" />;
    /// ```
    ///
    /// The following configuration uses lexicographic order:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "lexicographic"
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options,expect_diff
    /// <Hello {...this.props} opt1="" opt2="" opt12="" opt11="" />;
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
                // spread or shorthand attribute resets sort order: it carries
                // an opaque expression that may have side effects on the
                // resulting prop set, so attributes on either side cannot be
                // freely reordered across it.
                AnyJsxAttribute::JsxSpreadAttribute(_)
                | AnyJsxAttribute::JsxShorthandAttribute(_) => {
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

    fn suppressed_nodes(
        ctx: &RuleContext<Self>,
        _state: &Self::State,
        suppressions: &mut RuleSuppressions<JsLanguage>,
    ) {
        for list in ctx
            .query()
            .syntax()
            .descendants()
            .skip(1)
            .filter_map(JsxAttributeList::cast)
        {
            suppressions.suppress_node(list.syntax().clone());
        }
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
