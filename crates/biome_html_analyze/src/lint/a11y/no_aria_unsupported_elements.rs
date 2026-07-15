use std::str::FromStr;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaAttribute;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyVueDirective, HtmlSyntaxKind, T, element_ext::AnyHtmlTagElement,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::no_aria_unsupported_elements::NoAriaUnsupportedElementsOptions;
use biome_string_case::StrLikeExtension;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <meta charset="UTF-8" role="meta" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <html aria-required="true"></html>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <meta charset="UTF-8" />
    /// ```
    ///
    /// ```html
    /// <html></html>
    /// ```
    ///
    pub NoAriaUnsupportedElements {
        version: "2.5.0",
        name: "noAriaUnsupportedElements",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("aria-unsupported-elements").inspired()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

const ARIA_UNSUPPORTED_ELEMENTS: TokenSet<HtmlSyntaxKind> =
    token_set!(T![meta], T![html], T![script], T![style]);

#[derive(Debug)]
enum AttributeKind {
    Role,
    Aria,
}

impl AttributeKind {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Role => "role",
            Self::Aria => "aria-*",
        }
    }
}

#[derive(Debug)]
pub struct RuleState {
    attribute_kind: AttributeKind,
}

impl Rule for NoAriaUnsupportedElements {
    type Query = Ast<AnyHtmlTagElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoAriaUnsupportedElementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let is_unsupported = node
            .tag_name_kind()
            .is_some_and(|kind| ARIA_UNSUPPORTED_ELEMENTS.contains(kind));

        if is_unsupported {
            let report = node.attributes().iter().find_map(|attribute| {
                let text = match attribute {
                    AnyHtmlAttribute::HtmlAttribute(a) => {
                        Some(a.name().ok()?.value_token().ok()?.token_text_trimmed())
                    }
                    AnyHtmlAttribute::AnyVueDirective(vue) => match vue {
                        AnyVueDirective::VueVBindShorthandDirective(d) => Some(
                            d.arg()
                                .ok()?
                                .arg()?
                                .as_vue_static_argument()?
                                .name_token()
                                .ok()?
                                .token_text_trimmed(),
                        ),
                        AnyVueDirective::VueDirective(d) => {
                            if !d.is_binding() {
                                return None;
                            }
                            Some(
                                d.arg()?
                                    .arg()?
                                    .as_vue_static_argument()?
                                    .name_token()
                                    .ok()?
                                    .token_text_trimmed(),
                            )
                        }
                        _ => None,
                    },
                    _ => None,
                }?;

                let text_lower = text.to_ascii_lowercase_cow();
                if text_lower.starts_with("aria-") && AriaAttribute::from_str(&text_lower).is_ok() {
                    return Some(RuleState {
                        attribute_kind: AttributeKind::Aria,
                    });
                }

                if text.eq_ignore_ascii_case("role") {
                    return Some(RuleState {
                        attribute_kind: AttributeKind::Role,
                    });
                }

                None
            });
            return report;
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let attribute_kind = state.attribute_kind.as_str();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Avoid the "<Emphasis>"role"</Emphasis>" attribute and "<Emphasis>"aria-*"</Emphasis>" attributes when using "<Emphasis>"meta"</Emphasis>", "<Emphasis>"html"</Emphasis>", "<Emphasis>"script"</Emphasis>", and "<Emphasis>"style"</Emphasis>" elements."
                },
            )
            .note(markup! {
                "Using "{attribute_kind}" on elements that do not support them can cause issues with screen readers."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let element = ctx.query();
        let mut mutation = ctx.root().begin();

        let attribute = element.attributes().iter().find_map(|attribute| {
            let html_attribute = attribute.as_html_attribute()?;
            let attribute_name = html_attribute.name().ok()?.value_token().ok()?;
            let attribute_name_text = attribute_name.token_text_trimmed();
            let attribute_name_lower = attribute_name_text.to_ascii_lowercase_cow();
            (attribute_name_text.eq_ignore_ascii_case("role")
                || (attribute_name_lower.starts_with("aria-")
                    && AriaAttribute::from_str(&attribute_name_lower).is_ok()))
            .then_some(attribute)
        })?;

        let removed_attribute = attribute.to_string();
        mutation.remove_node(attribute);

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>""{removed_attribute}""</Emphasis>" attribute." }
                .to_owned(),
            mutation,
        ))
    }
}
