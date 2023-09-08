use crate::aria_services::Aria;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_rowan::{AstNode, AstNodeList};

declare_rule! {
    /// Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
    ///
    /// Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-unsupported-elements.md
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
    /// ## Valid
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
    pub(crate) NoAriaUnsupportedElements {
        version: "1.0.0",
        name: "noAriaUnsupportedElements",
        recommended: true,
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
    type Query = Aria<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_properties = ctx.aria_properties();

        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let element_name = element_name.text_trimmed();

        if ARIA_UNSUPPORTED_ELEMENTS.contains(&element_name) {
            // Check if the unsupported element has `role` or `aria-*` attribute
            let report = node.attributes().iter().find_map(|attribute| {
                let attribute = attribute.as_jsx_attribute()?;
                let attribute_name = attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;

                if attribute_name.text_trimmed().starts_with("aria-")
                    && aria_properties
                        .get_property(attribute_name.text_trimmed())
                        .is_some()
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
}
