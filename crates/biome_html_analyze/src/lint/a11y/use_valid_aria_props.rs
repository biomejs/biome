use std::str::FromStr;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaAttribute;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlAttribute, AnyHtmlElement};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::use_valid_aria_props::UseValidAriaPropsOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Ensures that ARIA properties `aria-*` are all valid.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <input type="text" aria-labell="" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div aria-lorem="foobar"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <input type="text" aria-label="Enter your name" />
    /// <div aria-hidden="true"></div>
    /// ```
    ///
    /// ## Accessibility guidelines
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    pub UseValidAriaProps {
        version: "next",
        name: "useValidAriaProps",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("aria-props").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseValidAriaProps {
    type Query = Ast<AnyHtmlElement>;
    type State = String;
    type Signals = Box<[Self::State]>;
    type Options = UseValidAriaPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let Some(attributes) = node.attributes() else {
            return Vec::new().into_boxed_slice();
        };

        let invalid_attrs: Vec<_> = attributes
            .iter()
            .filter_map(|attr: AnyHtmlAttribute| {
                let attribute = attr.as_html_attribute()?;
                let name_token = attribute.name().ok()?.value_token().ok()?;
                let name = name_token.text_trimmed();
                if name.starts_with("aria-") && AriaAttribute::from_str(name).is_err() {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect();

        invalid_attrs.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, attr_name: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The element contains invalid ARIA attribute(s)"
                },
            )
            .detail(
                node.find_attribute_by_name(attr_name)?.range(),
                markup! {
                    <Emphasis>{attr_name}</Emphasis>" is not a valid ARIA attribute."
                },
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, attr_name: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let attribute = node.find_attribute_by_name(attr_name)?;
        let mut mutation = ctx.root().begin();
        mutation.remove_node(attribute);
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the invalid "<Emphasis>"aria-*"</Emphasis>" attribute.
                Check the list of all "<Hyperlink href="https://developer.mozilla.org/en-US/docs/web/Accessibility/ARIA/Attributes#aria_attribute_types">"valid"</Hyperlink>" aria-* attributes." }
                .to_owned(),
            mutation,
        ))
    }
}
