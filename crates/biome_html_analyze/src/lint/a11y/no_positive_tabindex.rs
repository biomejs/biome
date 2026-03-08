use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::no_positive_tabindex::NoPositiveTabindexOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Prevent the usage of positive integers on `tabindex` attribute.
    ///
    /// Avoid positive `tabindex` attribute values to synchronize the flow of the page with keyboard tab order.
    ///
    /// ## Accessibility guidelines
    ///
    /// [WCAG 2.4.3](https://www.w3.org/WAI/WCAG21/Understanding/focus-order)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div tabindex="1"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div tabindex="5"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div tabindex="0"></div>
    /// ```
    ///
    /// ```html
    /// <div tabindex="-1"></div>
    /// ```
    ///
    pub NoPositiveTabindex {
        version: "2.4.0",
        name: "noPositiveTabindex",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("tabindex-no-positive").same(), RuleSource::HtmlEslint("no-positive-tabindex").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct NoPositiveTabindexState {
    attribute: HtmlAttribute,
    value_range: TextRange,
}

impl Rule for NoPositiveTabindex {
    type Query = Ast<AnyHtmlElement>;
    type State = NoPositiveTabindexState;
    type Signals = Option<Self::State>;
    type Options = NoPositiveTabindexOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let tabindex_attribute = element.find_attribute_by_name("tabindex")?;
        let initializer = tabindex_attribute.initializer()?;
        let value = initializer.value().ok()?;
        let string_value = value.string_value()?;
        let value_range = value.range();

        if !is_tabindex_valid(&string_value) {
            return Some(NoPositiveTabindexState {
                attribute: tabindex_attribute,
                value_range,
            });
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.value_range,
            markup! {"Avoid positive values for the "<Emphasis>"tabindex"</Emphasis>" attribute."}.to_owned(),
        )
        .note(
            markup! {
                "Elements with a positive "<Emphasis>"tabindex"</Emphasis>" override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard."
            }.to_owned(),
        )
        .note(
            markup! {
                "Use only 0 and -1 as "<Emphasis>"tabindex"</Emphasis>" values. Avoid using "<Emphasis>"tabindex"</Emphasis>" values greater than 0 and CSS properties that can change the order of focusable HTML elements."
            }
        );

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.attribute.clone());

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"tabindex"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

/// Verify if number string is an integer less than or equal to zero.
/// Non-integer numbers are considered valid (they will be ignored by browsers).
fn is_tabindex_valid(number_like_string: &str) -> bool {
    let number_string_result = number_like_string.trim().parse::<i32>();

    match number_string_result {
        Ok(number) => number <= 0,
        Err(_) => true,
    }
}
