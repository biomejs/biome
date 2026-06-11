use biome_analyze::context::RuleContext;
use biome_analyze::{FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{HtmlAttribute, element_ext::AnyHtmlTagElement};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TokenText};
use biome_rule_options::no_noninteractive_tabindex::NoNoninteractiveTabindexOptions;

use crate::{Aria, HtmlRuleAction};

declare_lint_rule! {
    /// Enforce that `tabindex` is not assigned to non-interactive HTML elements.
    ///
    /// When using the tab key to navigate a webpage, limit it to interactive elements.
    /// You don't need to add tabindex to items in an unordered list as assistive technology can navigate through the HTML.
    /// Keep the tab ring small, which is the order of elements when tabbing, for a more efficient and accessible browsing experience.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div tabindex="0"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div role="article" tabindex="0"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <article tabindex="0"></article>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div></div>
    /// ```
    ///
    /// ```html
    /// <button tabindex="0"></button>
    /// ```
    ///
    /// ```html
    /// <article tabindex="-1"></article>
    /// ```
    ///
    pub NoNoninteractiveTabindex {
        version: "next",
        name: "noNoninteractiveTabindex",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-noninteractive-tabindex").inspired()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

pub struct RuleState {
    attribute_range: TextRange,
    attribute: HtmlAttribute,
    element_name: TokenText,
}

impl Rule for NoNoninteractiveTabindex {
    type Query = Aria<AnyHtmlTagElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoNoninteractiveTabindexOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !ctx.aria_roles().is_not_interactive_element(node) {
            return None;
        }

        let tabindex_attribute = node.find_attribute_by_name("tabindex")?;

        let is_negative = tabindex_attribute
            .initializer()
            .and_then(|init| init.value().ok())
            .and_then(|value| value.string_value())
            .is_some_and(|value| is_negative_tabindex(&value));

        if is_negative {
            return None;
        }

        let role_attribute = node.find_attribute_by_name("role");
        if let Some(role_attr) = role_attribute {
            let role_value = role_attr.initializer()?.value().ok()?.string_value()?;
            let role = AriaRole::from_roles(role_value.trim());

            if let Some(aria_role) = role
                && aria_role.is_interactive()
            {
                return None;
            }
        }

        let element_name = node.tag_name()?;
        let attribute_range = tabindex_attribute.range();

        Some(RuleState {
            attribute_range,
            attribute: tabindex_attribute,
            element_name,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let element_name = state.element_name.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.attribute_range,
                markup! {
                    "The HTML element "<Emphasis>{element_name}</Emphasis>" is non-interactive. Do not use "<Emphasis>"tabindex"</Emphasis>"."
                },
            )
            .note(markup! {
                "Adding non-interactive elements to the keyboard navigation flow can confuse users."
            }),
        )
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

/// Returns `true` only for valid integers strictly less than 0.
fn is_negative_tabindex(number_like_string: &str) -> bool {
    matches!(number_like_string.trim().parse::<i64>(), Ok(n) if n < 0)
}
