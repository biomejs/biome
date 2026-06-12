use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_rowan::AstNode;
use biome_rule_options::use_focusable_interactive::UseFocusableInteractiveOptions;

use crate::Aria;

declare_lint_rule! {
    /// Elements with an interactive role and interaction handlers must be focusable.
    ///
    /// HTML elements with interactive roles must have `tabindex` defined to ensure they are
    /// focusable. Without `tabindex`, assistive technologies may not recognize these elements as
    /// interactive.
    /// You could also consider switching from an interactive role to its semantic HTML element
    /// instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div role="button"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div role="tab"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div role="button" tabindex="0"></div>
    /// ```
    ///
    /// ```html
    /// <div></div>
    /// ```
    ///
    pub UseFocusableInteractive {
        version: "2.5.0",
        name: "useFocusableInteractive",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("interactive-supports-focus").inspired()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseFocusableInteractive {
    type Query = Aria<AnyHtmlTagElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseFocusableInteractiveOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_custom_component() {
            return None;
        }

        if ctx.aria_roles().is_not_interactive_element(node) {
            let role_attribute = node.find_attribute_by_name("role");
            if let Some(role_attribute) = role_attribute {
                let tabindex_attribute = node.find_attribute_by_name("tabindex");
                let role_attribute_value = role_attribute.initializer()?.value().ok()?;
                let role = AriaRole::from_roles(role_attribute_value.as_static_value()?.text())?;
                if role.is_interactive() && !role.is_composite() && tabindex_attribute.is_none() {
                    return Some(());
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_value = role_attribute.initializer()?.value().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The HTML element with the interactive role "<Emphasis>{role_attribute_value.to_trimmed_string()}</Emphasis>" is not focusable."
                },
            ).note(markup! {
                "A non-interactive HTML element that is not focusable may not be reachable for users that rely on keyboard navigation, even with an added role like "<Emphasis>{role_attribute_value.to_trimmed_string()}</Emphasis>"."
            })
            .note(markup! {
                "Add a "<Emphasis>"tabindex"</Emphasis>" attribute to make this element focusable."
            }),
        )
    }
}
