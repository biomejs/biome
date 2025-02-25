use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsxAttribute, JsxOpeningElement};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// It detects the use of `role` attributes in JSX elements and suggests using semantic elements instead.
    ///
    /// The `role` attribute is used to define the purpose of an element, but it should be used as a last resort.
    /// Using semantic elements like `<button>`, `<nav>` and others are more accessible and provide better semantics.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="checkbox"></div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="separator"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <input type="checkbox">label</input>
    ///   <hr/>
    /// </>;
    /// ```
    ///
    /// All elements with `role="img"` are ignored:
    ///
    /// ```jsx
    /// <div role="img" aria-label="That cat is so cute">
    ///   <p>&#x1F408; &#x1F602;</p>
    /// </div>
    /// ```
    pub UseSemanticElements {
        version: "1.8.0",
        name: "useSemanticElements",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("prefer-tag-over-role")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseSemanticElements {
    type Query = Ast<JsxOpeningElement>;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let role_attribute = node.find_attribute_by_name("role")?;
        let role_value = role_attribute.as_static_value()?;
        let role_value = role_value.as_string_constant()?;

        // Allow `role="img"` on any element. For more information, see:
        // <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role>
        if role_value == "img" {
            return None;
        }

        let role = AriaRole::from_roles(role_value)?;
        if role.base_html_elements().is_empty() && role.related_html_elements().is_empty() {
            None
        } else {
            Some(role_attribute)
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let role_attribute = state;
        let role_value = role_attribute.as_static_value()?;
        let role_value = role_value.as_string_constant()?;
        let role = AriaRole::from_roles(role_value)?;

        let candidates = role
            .base_html_elements()
            .iter()
            .chain(role.related_html_elements())
            .map(|element| element.to_string())
            .collect::<Vec<_>>();
        let candidate_list = candidates.join("\n");

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                role_attribute.range(),
                markup! {
                    "The elements with this role can be changed to the following elements:\n"
                    {candidate_list}
                }
                .to_owned(),
            )
            .note(markup! {
                "For examples and more information, see " <Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles">"WAI-ARIA Roles"</Hyperlink>
            }),
        )
    }
}
