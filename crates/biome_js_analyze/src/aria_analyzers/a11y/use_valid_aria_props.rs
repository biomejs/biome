use crate::aria_services::Aria;
use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, ActionCategory, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TextRange};

declare_rule! {
    /// Ensures that ARIA properties `aria-*` are all valid.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx, expect_diagnostic
    /// <input className="" aria-labell="" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div aria-lorem="foobar"  aria-ipsum="foobar" />;
    /// ```
    ///
    /// ## Accessibility guidelines
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    pub(crate) UseValidAriaProps {
        version: "1.0.0",
        name: "useValidAriaProps",
        recommended: true,
    }
}

impl Rule for UseValidAriaProps {
    type Query = Aria<AnyJsxElement>;
    type State = Vec<(TextRange, String)>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_properties = ctx.aria_properties();

        // check attributes that belong only to HTML elements
        if node.is_element() {
            let attributes: Vec<_> = node
                .attributes()
                .iter()
                .filter_map(|attribute| {
                    let attribute = attribute.as_jsx_attribute()?;
                    let attribute_name =
                        attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;
                    if attribute_name.text_trimmed().starts_with("aria-")
                        && aria_properties
                            .get_property(attribute_name.text_trimmed())
                            .is_none()
                    {
                        Some((attribute.range(), attribute_name.to_string()))
                    } else {
                        None
                    }
                })
                .collect();

            if attributes.is_empty() {
                None
            } else {
                Some(attributes)
            }
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, attributes: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The element contains invalid ARIA attribute(s)"
            },
        );

        for (range, attribute_name) in attributes {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    <Emphasis>{attribute_name}</Emphasis>" is not a valid ARIA attribute."
                },
            );
        }

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let element = ctx.query();
        let mut mutation = ctx.root().begin();
        let aria_properties = ctx.aria_properties();

        for attribute in element.attributes() {
            let attribute = attribute.as_jsx_attribute()?;
            let attribute_name = attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;

            if attribute_name.text_trimmed().starts_with("aria-")
                && aria_properties
                    .get_property(attribute_name.text_trimmed())
                    .is_none()
            {
                mutation.remove_node(attribute.clone());
            }
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message:
                markup! { "Remove all the invalid "<Emphasis>"aria-*"</Emphasis>" attribute. 
                Check the list of all "<Hyperlink href="https://developer.mozilla.org/en-US/docs/web/Accessibility/ARIA/Attributes#aria_attribute_types">"valid"</Hyperlink>" aria-* attributes." }
                    .to_owned(),
            mutation,
        })
    }
}
