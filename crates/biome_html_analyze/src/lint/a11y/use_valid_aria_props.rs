use std::str::FromStr;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaAttribute;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{AnyHtmlAttribute, AnyVueDirective};
use biome_rowan::{AstNode, BatchMutationExt, TokenText};
use biome_rule_options::use_valid_aria_props::UseValidAriaPropsOptions;
use biome_string_case::StrLikeExtension;

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
    type Query = Ast<AnyHtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseValidAriaPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let attr = ctx.query();
        let name = extract_attribute_name(attr)?;
        let name_lower = name.to_ascii_lowercase_cow();
        if name_lower.starts_with("aria-") && AriaAttribute::from_str(&name_lower).is_err() {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let attr = ctx.query();
        let name = extract_attribute_name(attr)?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                attr.range(),
                markup! {
                    "The element contains an invalid ARIA attribute."
                },
            )
            .detail(
                attr.range(),
                markup! {
                    <Emphasis>{name.text()}</Emphasis>" is not a valid ARIA attribute."
                },
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let attr = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(attr.clone());
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

fn extract_attribute_name(attr: &AnyHtmlAttribute) -> Option<TokenText> {
    match attr {
        AnyHtmlAttribute::HtmlAttribute(a) => {
            Some(a.name().ok()?.value_token().ok()?.token_text_trimmed())
        }
        AnyHtmlAttribute::AnyVueDirective(vue) => match vue {
            AnyVueDirective::VueVBindShorthandDirective(d) => Some(
                d.arg()
                    .ok()?
                    .arg()
                    .ok()?
                    .as_vue_static_argument()?
                    .name_token()
                    .ok()?
                    .token_text_trimmed(),
            ),
            AnyVueDirective::VueDirective(d) => {
                if d.name_token().ok()?.text_trimmed() != "v-bind" {
                    return None;
                }
                Some(
                    d.arg()?
                        .arg()
                        .ok()?
                        .as_vue_static_argument()?
                        .name_token()
                        .ok()?
                        .token_text_trimmed(),
                )
            }
            _ => None,
        },
        _ => None,
    }
}
