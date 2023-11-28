use crate::aria_services::Aria;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;

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
    /// Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-interactive-element-to-noninteractive-role.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input role="img" />;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <input role="button" />;
    /// ```
    ///
    pub(crate) NoInteractiveElementToNoninteractiveRole {
        version: "1.3.0",
        name: "noInteractiveElementToNoninteractiveRole",
        recommended: true,
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
}
