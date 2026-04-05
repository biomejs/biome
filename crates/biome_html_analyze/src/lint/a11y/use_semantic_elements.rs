use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlElement, HtmlAttribute, HtmlFileSource};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// It detects the use of `role` attributes in HTML elements and suggests using semantic elements instead.
    ///
    /// The `role` attribute is used to define the purpose of an element, but it should be used as a last resort.
    /// Using semantic elements like `<button>`, `<nav>` and others are more accessible and provide better semantics.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div role="checkbox"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div role="separator"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <input type="checkbox" />
    /// <hr />
    /// <div role="status"></div>
    /// ```
    ///
    /// All elements with `role="img"` are ignored:
    ///
    /// ```html
    /// <div role="img" aria-label="That cat is so cute">
    ///   <p>&#x1F408; &#x1F602;</p>
    /// </div>
    /// ```
    ///
    /// Semantic elements with a matching role are not flagged (see [noRedundantRoles](https://biomejs.dev/linter/rules/no-redundant-roles/)):
    ///
    /// ```html
    /// <nav role="navigation"></nav>
    /// <footer role="contentinfo"></footer>
    /// ```
    ///
    pub UseSemanticElements {
        version: "next",
        name: "useSemanticElements",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("prefer-tag-over-role").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for UseSemanticElements {
    type Query = Ast<AnyHtmlElement>;
    type State = HtmlAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_value = role_attribute.initializer()?.value().ok()?.string_value()?;
        let role_value = role_value.trim();

        let role = AriaRole::from_roles(role_value)?;

        if matches!(
            role,
            AriaRole::Img
                | AriaRole::Combobox
                | AriaRole::Listbox
                | AriaRole::Option
                | AriaRole::Status
                | AriaRole::Alert
        ) {
            return None;
        }
        let semantic_elements = role.base_html_elements();
        let related_elements = role.related_html_elements();
        if semantic_elements.is_empty() && related_elements.is_empty() {
            return None;
        }

        let element_name = node.name()?;
        let is_html = ctx.source_type::<HtmlFileSource>().is_html();
        let is_already_semantic =
            semantic_elements
                .iter()
                .chain(related_elements.iter())
                .any(|instance| {
                    let name_matches = if is_html {
                        instance
                            .element
                            .as_str()
                            .eq_ignore_ascii_case(element_name.text())
                    } else {
                        instance.element.as_str() == element_name.text()
                    };
                    name_matches
                        && instance.attributes.iter().all(|required_attr| {
                            node.find_attribute_by_name(required_attr.attribute.as_str())
                                .and_then(|attr| {
                                    attr.initializer()?.value().ok()?.string_value()
                                })
                                .is_some_and(|value| {
                                    if is_html {
                                        value
                                            .text()
                                            .eq_ignore_ascii_case(required_attr.value)
                                    } else {
                                        value.text() == required_attr.value
                                    }
                                })
                        })
                });
        if is_already_semantic {
            return None;
        }

        Some(role_attribute)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let role_attribute = state;
        let role_value = role_attribute.initializer()?.value().ok()?.string_value()?;
        let role_value = role_value.trim();
        let role = AriaRole::from_roles(role_value)?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                role_attribute.range(),
                markup! {
                    "The elements with this role can be changed to semantic elements."
                },
            )
            .footer_list(
                markup! { "Replace with one of these elements:" },
                role.base_html_elements()
                    .iter()
                    .chain(role.related_html_elements())
                    .map(|element| element.to_string()),
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
