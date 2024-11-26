use std::str::FromStr;

use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_aria_metadata::{AriaAttribute, AriaValueType};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsSyntaxToken, JsxAttribute, TextRange};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Enforce that ARIA state and property values are valid.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx, expect_diagnostic
    /// <span role="checkbox" aria-checked="test">some text</span>
    /// ```
    ///
    /// ```jsx, expect_diagnostic
    /// <span aria-labelledby="">some text</span>
    /// ```
    ///
    /// ```jsx, expect_diagnostic
    /// <span aria-valuemax="hey">some text</span>
    /// ```
    ///
    /// ```jsx, expect_diagnostic
    /// <span aria-orientation="hey">some text</span>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <span role="checkbox" aria-checked={checked} >some text</span>
    ///     <span aria-labelledby="fooId barId" >some text</span>
    /// </>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    /// ### Resources
    ///
    /// - [ARIA Spec, States and Properties](https://www.w3.org/TR/wai-aria/#states_and_properties)
    /// - [Chrome Audit Rules, AX_ARIA_04](https://github.com/GoogleChrome/accessibility-developer-tools/wiki/Audit-Rules#ax_aria_04)
    pub UseValidAriaValues {
        version: "1.0.0",
        name: "useValidAriaValues",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("aria-proptypes")],
        recommended: true,
        severity: Severity::Error,
    }
}

pub struct UseValidAriaValuesState {
    attribute_name: JsSyntaxToken,
    attribute_value_range: TextRange,
    property_type: AriaValueType,
}

impl Rule for UseValidAriaValues {
    type Query = Ast<JsxAttribute>;
    type State = UseValidAriaValuesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let attribute_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;

        if let Ok(aria_property) = AriaAttribute::from_str(attribute_name.text_trimmed()) {
            let attribute_static_value = node.as_static_value()?;
            let attribute_text = attribute_static_value.text();
            if !aria_property.value_type().contains(attribute_text) {
                return Some(UseValidAriaValuesState {
                    attribute_name,
                    attribute_value_range: node.range(),
                    property_type: aria_property.value_type(),
                });
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let attribute_name = state.attribute_name.text_trimmed();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.attribute_value_range,
            markup! {
                "The value of the ARIA attribute "<Emphasis>{attribute_name}</Emphasis>" is not correct."
            },
        );
        let diagnostic = match state.property_type {
            AriaValueType::Boolean => {
                diagnostic.footer_list(
                    markup!{
                        "The only supported values for the "<Emphasis>{attribute_name}</Emphasis>" property is one of the following:"
                    },
                    ["false", "true"]
                )
            }
            AriaValueType::OptionalBoolean => {
                diagnostic.footer_list(
                    markup!{
                        "The only supported values for the "<Emphasis>{attribute_name}</Emphasis>" property is one of the following:"
                    },
                    ["undefined", "false", "true"]
                )
            }
            AriaValueType::Integer => {
                diagnostic.note(
                    markup!{
                        "The only value supported is a number without fractional components."
                    }
                )
            }
            AriaValueType::IdReference => {
                diagnostic.note(
                    markup!{
                        "The only supported value is an HTML identifier."
                    }
                )
            }
            AriaValueType::IdReferenceList => {
                diagnostic.note(
                    markup!{
                        "The only supported value is a space-separated list of HTML identifiers."
                    }
                )
            }
            AriaValueType::String => {
                diagnostic.note(
                    markup!{
                        "The only supported value is text."
                    }
                )
            }
            AriaValueType::Number => {
                diagnostic.note(
                    markup!{
                        "The only supported value is number."
                    }
                )
            }
            AriaValueType::Token(tokens) => {
                diagnostic.footer_list(
                    markup!{
                    "The only supported value for the "<Emphasis>{attribute_name}</Emphasis>" property is one of the following:"
                },
                    tokens
                )
            }
            AriaValueType::TokenList(tokens) => {
                diagnostic.footer_list(
                    markup!{
                    "The values supported for "<Emphasis>{attribute_name}</Emphasis>" property are one or more of the following:"
                },
                    tokens
                )
            }
            AriaValueType::Tristate => {
                diagnostic.footer_list(
                    markup!{
                        "The only supported value for the "<Emphasis>{attribute_name}</Emphasis>" property one of the following:"
                    },
                    ["false", "true", "mixed"]
                )
            }
        };
        Some(diagnostic)
    }
}
