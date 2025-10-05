use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlAttribute, AnyHtmlElement, HtmlAttribute, HtmlAttributeList};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// The scope prop should be used only on `<th>` elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div scope="col"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div scope></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <th scope="col"></th>
    /// ```
    ///
    /// ```html
    /// <th scope></th>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.3.1](https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships)
    /// - [WCAG 4.1.1](https://www.w3.org/WAI/WCAG21/Understanding/parsing)
    ///
    pub NoHeaderScope {
        version: "next",
        name: "noHeaderScope",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("scope").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoHeaderScope {
    type Query = Ast<AnyHtmlElement>;
    type State = HtmlAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        // Check if element is NOT a th element and has a scope attribute
        if is_th_element(element) {
            return None;
        }

        // Check if element has a scope attribute
        let attributes = get_element_attributes(element)?;
        let scope_attribute = find_attribute_by_name(&attributes, "scope")?;

        Some(scope_attribute)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            markup! {"Avoid using the "<Emphasis>"scope"</Emphasis>" attribute on elements other than "<Emphasis>"th"</Emphasis>" elements."}
                .to_owned(),
        ).note(markup!{
            "The "<Emphasis>"scope"</Emphasis>" attribute is used to associate a data cell with its corresponding header cell in a data table,
            so it should be placed on "<Emphasis>"th"</Emphasis>" elements to provide accessibility to screen readers."
        }).note(markup!{
            "Follow the links for more information,
            "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships">"WCAG 1.3.1"</Hyperlink>"
            "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/parsing">"WCAG 4.1.1"</Hyperlink>""
        });

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.clone());

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"scope"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

// Helper function to check if element is a th element
fn is_th_element(element: &AnyHtmlElement) -> bool {
    match element {
        AnyHtmlElement::HtmlElement(el) => {
            if let Ok(opening_element) = el.opening_element() {
                if let Ok(name) = opening_element.name() {
                    if let Ok(name_token) = name.value_token() {
                        return name_token.text_trimmed() == "th";
                    }
                }
            }
            false
        }
        AnyHtmlElement::HtmlSelfClosingElement(el) => {
            if let Ok(name) = el.name() {
                if let Ok(name_token) = name.value_token() {
                    return name_token.text_trimmed() == "th";
                }
            }
            false
        }
        _ => false,
    }
}

// Helper function to get element attributes
fn get_element_attributes(element: &AnyHtmlElement) -> Option<HtmlAttributeList> {
    match element {
        AnyHtmlElement::HtmlElement(el) => {
            let opening_element = el.opening_element().ok()?;
            Some(opening_element.attributes())
        }
        AnyHtmlElement::HtmlSelfClosingElement(el) => Some(el.attributes()),
        _ => None,
    }
}

// Helper function to find attribute by name
fn find_attribute_by_name(
    attributes: &HtmlAttributeList,
    name_to_lookup: &str,
) -> Option<HtmlAttribute> {
    attributes.iter().find_map(|attribute| {
        if let AnyHtmlAttribute::HtmlAttribute(attribute) = attribute {
            let name = attribute.name().ok()?;
            let name_token = name.value_token().ok()?;
            if name_token.text_trimmed() == name_to_lookup {
                return Some(attribute);
            }
        }
        None
    })
}
