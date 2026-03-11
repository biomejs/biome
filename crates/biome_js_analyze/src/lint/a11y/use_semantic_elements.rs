use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::Severity;
use biome_js_syntax::JsxAttribute;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::use_semantic_elements::UseSemanticElementsOptions;

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
    /// ```jsx,expect_diagnostic
    /// <div role="checkbox" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="separator" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <input type="checkbox">label</input>
    ///   <hr/>
    ///   <div role="status"></div>
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
    ///
    /// Semantic elements with a matching role are not flagged (see [noRedundantRoles](https://biomejs.dev/linter/rules/no-redundant-roles/)):
    ///
    /// ```jsx
    /// <>
    ///   <nav role="navigation"></nav>
    ///   <footer role="contentinfo"></footer>
    /// </>;
    /// ```
    pub UseSemanticElements {
        version: "1.8.0",
        name: "useSemanticElements",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("prefer-tag-over-role").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseSemanticElements {
    type Query = Ast<AnyJsxElement>;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;
    type Options = UseSemanticElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_custom_component() || node.is_custom_element() {
            return None;
        }

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_value = role_attribute.as_static_value()?;
        let role_value = role_value.as_string_constant()?;

        // Allow `role="img"` on any element. For more information, see:
        // <https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Roles/img_role>
        if role_value == "img" {
            return None;
        }

        // For the following roles, the associated elements are impractical:
        // - combobox: <select> is not possible to implement many valid comboboxes (see https://www.w3.org/WAI/ARIA/apg/patterns/combobox/)
        // - option: <option> in browsers have divergent/unexpected behavior, with Safari hiding elements by default.
        // - listbox: <datalist> isn't always correct for all listbox uses
        // See https://www.w3.org/WAI/ARIA/apg/patterns/combobox/. In most examples, roles are explicit
        // - status: <output> is only a relatedConcept, not a baseConcept of the status role.
        //   Using <output> for status is misleading (see #9245, eslint-plugin-jsx-a11y#920)
        // - alert: <output> is only a relatedConcept, same issue as status
        if role_value == "combobox"
            || role_value == "listbox"
            || role_value == "option"
            || role_value == "status"
            || role_value == "alert"
        {
            return None;
        }

        let role = AriaRole::from_roles(role_value)?;
        let semantic_elements = role.base_html_elements();
        let related_elements = role.related_html_elements();
        if semantic_elements.is_empty() && related_elements.is_empty() {
            return None;
        }

        // If the current element is already a semantic element for this role
        // (matching both the tag name and any required attributes),
        // don't flag it. That case is handled by `noRedundantRoles`.
        let element_name = node.name_value_token().ok()?;
        let element_name = element_name.text_trimmed();
        let is_already_semantic =
            semantic_elements
                .iter()
                .chain(related_elements.iter())
                .any(|instance| {
                    instance.element.as_str() == element_name
                        && instance.attributes.iter().all(|required_attr| {
                            node.find_attribute_by_name(required_attr.attribute.as_str())
                                .and_then(|attr| attr.as_static_value())
                                .is_some_and(|value| value.text() == required_attr.value)
                        })
                });
        if is_already_semantic {
            return None;
        }

        Some(role_attribute)
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

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        Some(ctx.query().range())
    }
}
