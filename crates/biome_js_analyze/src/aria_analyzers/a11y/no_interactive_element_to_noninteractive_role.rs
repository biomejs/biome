use crate::{aria_services::Aria, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
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
    /// ```jsx,expect_diagnostic
    /// <input role="img" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <input role="button" />;
    /// ```
    ///
    pub NoInteractiveElementToNoninteractiveRole {
        version: "1.3.0",
        name: "noInteractiveElementToNoninteractiveRole",
        source: RuleSource::EslintJsxA11y("no-interactive-element-to-noninteractive-role"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoInteractiveElementToNoninteractiveRole {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();
        if node.is_element() {
            let role_attribute = node.find_attribute_by_name("role")?;
            let role_attribute_static_value = role_attribute.as_static_value()?;
            let role_attribute_value = role_attribute_static_value.text();
            let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;

            let attributes = ctx.extract_attributes(&node.attributes());
            if !aria_roles.is_not_interactive_element(element_name.text_trimmed(), attributes)
                && !aria_roles.is_role_interactive(role_attribute_value)
            {
                // <div> and <span> are considered neither interactive nor non-interactive, depending on the presence or absence of the role attribute.
                // We don't report <div> and <span> here, because we cannot determine whether they are interactive or non-interactive.
                let role_sensitive_elements = ["div", "span"];
                if role_sensitive_elements.contains(&element_name.text_trimmed()) {
                    return None;
                }

                // a tag without href is considered non-interactive
                if element_name.text_trimmed() == "a"
                    && node.find_attribute_by_name("href").is_none()
                {
                    return None;
                }

                return Some(());
            }
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

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let role_attribute = node.find_attribute_by_name("role")?;

        let mut mutation = ctx.root().begin();
        mutation.remove_node(role_attribute);
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"role"</Emphasis>" attribute." }.to_owned(),
            mutation,
        })
    }
}
