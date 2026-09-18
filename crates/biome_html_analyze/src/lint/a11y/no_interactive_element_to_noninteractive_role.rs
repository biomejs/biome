use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_interactive_element_to_noninteractive_role::NoInteractiveElementToNoninteractiveRoleOptions;

use crate::{Aria, HtmlRuleAction, utils::is_html_tag};

declare_lint_rule! {
    /// Enforce that non-interactive ARIA roles are not assigned to interactive HTML elements.
    ///
    /// Interactive HTML elements indicate controls in the user interface.
    /// Interactive elements include `<a href>`, `<button>`, `<input>`, `<select>`, `<textarea>`.
    /// Non-interactive HTML elements and non-interactive ARIA roles indicate content and containers in the user interface.
    /// Non-interactive elements include `<main>`, `<area>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.
    ///
    /// [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) should not be used to convert an interactive element to a non-interactive element.
    /// Non-interactive ARIA roles include `article`, `banner`, `complementary`, `img`, `listitem`, `main`, `region` and `tooltip`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <input role="img" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <input role="button" />
    /// ```
    ///
    /// ```html
    /// <canvas role="img"></canvas>
    /// ```
    ///
    pub NoInteractiveElementToNoninteractiveRole {
        version: "2.5.0",
        name: "noInteractiveElementToNoninteractiveRole",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-interactive-element-to-noninteractive-role").inspired()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoInteractiveElementToNoninteractiveRole {
    type Query = Aria<AnyHtmlTagElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoInteractiveElementToNoninteractiveRoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let source_type = ctx.source_type::<HtmlFileSource>();

        if node.is_custom_component() {
            return None;
        }

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_static_value = role_attribute
            .initializer()?
            .value()
            .ok()?
            .as_static_value()?;
        let role_attribute_value = role_attribute_static_value.text();

        // `hr` implicitly maps to `separator`, and `presentation`/`none` is explicitly
        // allowed on separators.
        if is_html_tag(node, source_type, "hr")
            && matches!(role_attribute_value, "presentation" | "none")
        {
            return None;
        }

        if !ctx.aria_roles().is_not_interactive_element(node)
            && AriaRole::from_roles(role_attribute_value)
                .is_some_and(|role| role.is_non_interactive())
        {
            // <div> and <span> are considered neither interactive nor non-interactive, depending on the presence or absence of the role attribute.
            // We don't report <div> and <span> here, because we cannot determine whether they are interactive or non-interactive.

            if ROLE_SENSITIVE_ELEMENTS
                .iter()
                .any(|el| is_html_tag(node, source_type, el))
            {
                return None;
            }

            // A <svg> element can be given an "img" to make it non-interactive for a11y reasons.
            if is_html_tag(node, source_type, "svg") && role_attribute_value == "img" {
                return None;
            }

            // A <canvas> element can be given an "img" to make it non-interactive for a11y reasons.
            if is_html_tag(node, source_type, "canvas") && role_attribute_value == "img" {
                return None;
            }

            // a tag without href is considered non-interactive
            if is_html_tag(node, source_type, "a") && node.find_attribute_by_name("href").is_none()
            {
                return None;
            }

            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),

                "Interactive elements should not be assigned non-interactive roles."

            )
            .note(
                "WAI-ARIA roles should not be used to convert an interactive element to a non-interactive element."
            )
            .note(
                "Wrap your interactive element in a <div> with the desired role or put the content inside your interactive element."
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let role_attribute = node.find_attribute_by_name("role")?;

        let mut mutation = ctx.root().begin();
        mutation.remove_node(role_attribute);
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"role"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

static ROLE_SENSITIVE_ELEMENTS: [&str; 3] = ["div", "span", "source"];
