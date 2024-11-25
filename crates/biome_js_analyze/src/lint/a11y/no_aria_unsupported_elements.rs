use std::str::FromStr;

use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_aria_metadata::AriaAttribute;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

declare_lint_rule! {
    /// Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <meta charset="UTF-8" role="meta" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html aria-required="true" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <meta charset="UTF-8" />
    /// ```
    ///
    /// ```jsx
    /// <html></html>
    /// ```
    ///
    ///
    pub NoAriaUnsupportedElements {
        version: "1.0.0",
        name: "noAriaUnsupportedElements",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("aria-unsupported-elements")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

const ARIA_UNSUPPORTED_ELEMENTS: [&str; 4] = ["meta", "html", "script", "style"];

#[derive(Debug)]
enum AttributeKind {
    Role,
    Aria,
}

impl AttributeKind {
    /// Converts an [AttributeKind] to a string.
    fn as_str(&self) -> &'static str {
        match self {
            AttributeKind::Role => "role",
            AttributeKind::Aria => "aria-*",
        }
    }
}

#[derive(Debug)]
pub struct RuleState {
    attribute_kind: AttributeKind,
}

impl Rule for NoAriaUnsupportedElements {
    type Query = Ast<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let element_name = element_name.text_trimmed();

        if ARIA_UNSUPPORTED_ELEMENTS.contains(&element_name) {
            // Check if the unsupported element has `role` or `aria-*` attribute
            let report = node.attributes().iter().find_map(|attribute| {
                let attribute = attribute.as_jsx_attribute()?;
                let attribute_name = attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;

                if attribute_name.text_trimmed().starts_with("aria-")
                    && AriaAttribute::from_str(attribute_name.text_trimmed()).is_ok()
                {
                    return Some(RuleState {
                        attribute_kind: AttributeKind::Aria,
                    });
                }

                if attribute_name.text_trimmed() == "role" {
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

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let element = ctx.query();
        let mut mutation = ctx.root().begin();

        let attribute = element.attributes().into_iter().find_map(|attribute| {
            let jsx_attribute = attribute.as_jsx_attribute()?;
            let attribute_name = jsx_attribute
                .name()
                .ok()?
                .as_jsx_name()?
                .value_token()
                .ok()?;
            let attribute_name = attribute_name.text_trimmed();
            (attribute_name.starts_with("aria-") || attribute_name == "role").then_some(attribute)
        })?;

        let removed_attribute = attribute.to_string();
        mutation.remove_node(attribute);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>""{removed_attribute}""</Emphasis>" attribute." }
                .to_owned(),
            mutation,
        ))
    }
}
