use crate::services::aria::Aria;
use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::JsxAttribute;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

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
    /// <div aria-lorem="foobar" />;
    /// ```
    ///
    /// ## Accessibility guidelines
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    pub UseValidAriaProps {
        version: "1.0.0",
        name: "useValidAriaProps",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("aria-props")],
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseValidAriaProps {
    type Query = Aria<AnyJsxElement>;
    type State = JsxAttribute;
    type Signals = Vec<Self::State>;
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
                        Some(attribute.clone())
                    } else {
                        None
                    }
                })
                .collect();

            attributes
        } else {
            vec![]
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, attribute: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let attribute_name = attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "The element contains invalid ARIA attribute(s)"
            },
        ).detail(
            attribute.range(),
            markup! {
                    <Emphasis>{attribute_name.text_trimmed()}</Emphasis>" is not a valid ARIA attribute."
                },
        ))
    }

    fn action(ctx: &RuleContext<Self>, attribute: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        mutation.remove_node(attribute.clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message:
                markup! { "Remove the invalid "<Emphasis>"aria-*"</Emphasis>" attribute.
                Check the list of all "<Hyperlink href="https://developer.mozilla.org/en-US/docs/web/Accessibility/ARIA/Attributes#aria_attribute_types">"valid"</Hyperlink>" aria-* attributes." }
                    .to_owned(),
            mutation,
        })
    }
}
