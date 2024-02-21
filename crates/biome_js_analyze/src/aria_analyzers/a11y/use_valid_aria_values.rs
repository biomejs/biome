use crate::aria_services::Aria;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_aria::AriaPropertyTypeEnum;
use biome_console::markup;
use biome_js_syntax::{JsSyntaxToken, JsxAttribute, TextRange};
use biome_rowan::AstNode;
use std::slice::Iter;

declare_rule! {
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
        source: RuleSource::EslintJsxA11y("aria-proptypes"),
        recommended: true,
    }
}

pub struct UseValidAriaValuesState {
    attribute_value_range: TextRange,
    allowed_values: Iter<'static, &'static str>,
    attribute_name: JsSyntaxToken,
    property_type: AriaPropertyTypeEnum,
}

impl Rule for UseValidAriaValues {
    type Query = Aria<JsxAttribute>;
    type State = UseValidAriaValuesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_properties = ctx.aria_properties();

        let attribute_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;

        if let Some(aria_property) = aria_properties.get_property(attribute_name.text_trimmed()) {
            let attribute_value_range = node.range();
            let attribute_static_value = node.as_static_value()?;
            let attribute_text = attribute_static_value.text();
            if !aria_property.contains_correct_value(attribute_text) {
                return Some(UseValidAriaValuesState {
                    attribute_value_range,
                    allowed_values: aria_property.values(),
                    attribute_name,
                    property_type: aria_property.property_type(),
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
            AriaPropertyTypeEnum::Boolean => {
                diagnostic.footer_list(
                    markup!{
                        "The only supported values for the "<Emphasis>{attribute_name}</Emphasis>" property is one of the following:"
                    },
                    &["true", "false"]
                )
            }
            AriaPropertyTypeEnum::Integer => {
                diagnostic.note(
                    markup!{
                        "The only value supported is a number without fractional components."
                    }
                )
            }
            AriaPropertyTypeEnum::Id |
            AriaPropertyTypeEnum::Idlist |
            AriaPropertyTypeEnum::String => {
                diagnostic.note(
                    markup!{
                        "The only supported value is text."
                    }
                )
            }

            AriaPropertyTypeEnum::Number => {
                diagnostic.note(
                    markup!{
                        "The only supported value is number."
                    }
                )
            }
            AriaPropertyTypeEnum::Token => {
                diagnostic.footer_list(
                    markup!{
                    "The only supported value for the "<Emphasis>{attribute_name}</Emphasis>" property is one of the following:"
                },
                    state.allowed_values.as_slice()
                )
            }
            AriaPropertyTypeEnum::Tokenlist => {
                diagnostic.footer_list(
                    markup!{
                    "The values supported for "<Emphasis>{attribute_name}</Emphasis>" property are one or more of the following:"
                },
                    state.allowed_values.as_slice()
                )
            }
            AriaPropertyTypeEnum::Tristate => {
                diagnostic.footer_list(
                    markup!{
                        "The only supported value for the "<Emphasis>{attribute_name}</Emphasis>" property one of the following:"
                    },
                    &["true", "false", "mixed"]
                )
            }
        };

        Some(diagnostic)
    }
}
