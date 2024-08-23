use std::{borrow::Cow, cmp::Ordering};

use biome_analyze::{
    context::RuleContext, declare_source_rule, ActionCategory, Ast, Rule, RuleAction, RuleSource,
    RuleSourceKind, SourceActionKind,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::jsx_attribute_list;
use biome_js_syntax::{AnyJsxAttribute, JsxAttribute, JsxAttributeList};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_source_rule! {
    /// Enforce props sorting in JSX elements.
    ///
    /// This rule checks if the JSX props are sorted in a consistent way.
    /// Props are sorted alphabetically.
    /// A spread prop resets the sorting order.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <Hello lastName="Smith" firstName="John" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <Hello firstName="John" lastName="Smith" />;
    /// <Hello tel={5555555} {...this.props} firstName="John" lastName="Smith" />;
    /// ```
    ///
    pub SortJsxProps {
        version: "next",
        name: "sortJsxProps",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-sort-props")],
        source_kind: RuleSourceKind::SameLogic,
    }
}

impl Rule for SortJsxProps {
    type Query = Ast<JsxAttributeList>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_ctx: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let props = ctx.query().clone();
        let mut non_spread_props = Vec::new();
        let mut new_props = Vec::new();
        for prop in props.clone() {
            match prop {
                AnyJsxAttribute::JsxAttribute(attr) => {
                    non_spread_props.push(attr);
                }
                AnyJsxAttribute::JsxSpreadAttribute(_) => {
                    if !non_spread_props.is_empty() {
                        non_spread_props.sort_by(compare_props());
                        new_props.extend(
                            non_spread_props
                                .clone()
                                .into_iter()
                                .map(AnyJsxAttribute::JsxAttribute),
                        );
                    }
                    non_spread_props.clear();
                    new_props.push(prop);
                }
            }
        }
        if !non_spread_props.is_empty() {
            non_spread_props.sort_by(compare_props());
            new_props.extend(
                non_spread_props
                    .into_iter()
                    .map(AnyJsxAttribute::JsxAttribute),
            );
        }
        if new_props == props.clone().into_iter().collect::<Vec<_>>() {
            return None;
        }
        let mut mutation = ctx.root().begin();
        mutation.replace_node(props, jsx_attribute_list(new_props));

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the JSX props." },
            mutation,
        ))
    }
}

fn compare_props() -> impl FnMut(&JsxAttribute, &JsxAttribute) -> Ordering {
    |a: &JsxAttribute, b: &JsxAttribute| -> Ordering {
        let (Ok(a_name), Ok(b_name)) = (a.name(), b.name()) else {
            return Ordering::Equal;
        };
        let (a_name, b_name) = (a_name.text(), b_name.text());

        a_name.cmp(&b_name)
    }
}
