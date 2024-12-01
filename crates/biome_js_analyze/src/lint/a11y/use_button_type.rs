use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::services::semantic::Semantic;
use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsxElementName, JsCallExpression, JsxAttribute, JsxOpeningElement, JsxSelfClosingElement,
    TextRange,
};
use biome_rowan::{declare_node_union, AstNode};

declare_lint_rule! {
    /// Enforces the usage of the attribute `type` for the element `button`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <button>Do something</button>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button type="incorrectType">Do something</button>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('button');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <button type="button">Do something</button>
    ///     <button type={buttonType}>Do something</button>
    /// </>
    /// ```
    pub UseButtonType {
        version: "1.0.0",
        name: "useButtonType",
        language: "jsx",
        sources: &[RuleSource::EslintReact("button-has-type")],
        recommended: true,
        severity: Severity::Error,
    }
}

const ALLOWED_BUTTON_TYPES: [&str; 3] = ["submit", "button", "reset"];

declare_node_union! {
    pub UseButtonTypeQuery = JsxSelfClosingElement | JsxOpeningElement | JsCallExpression
}

pub struct UseButtonTypeState {
    range: TextRange,
    missing_prop: bool,
}

impl Rule for UseButtonType {
    type Query = Semantic<UseButtonTypeQuery>;
    type State = UseButtonTypeState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            UseButtonTypeQuery::JsxSelfClosingElement(element) => {
                let name = element.name().ok()?;
                if !is_button(&name)? {
                    return None;
                }
                let type_attribute = element.find_attribute_by_name("type");
                let Some(attribute) = type_attribute else {
                    let has_spread_prop = element
                        .attributes()
                        .into_iter()
                        .any(|attr| attr.as_jsx_spread_attribute().is_some());
                    if has_spread_prop {
                        return None;
                    } else {
                        return Some(UseButtonTypeState {
                            range: element.range(),
                            missing_prop: true,
                        });
                    }
                };
                inspect_jsx_type_attribute(&attribute)
            }
            UseButtonTypeQuery::JsxOpeningElement(element) => {
                let name = element.name().ok()?;
                if !is_button(&name)? {
                    return None;
                }
                let type_attribute = element.find_attribute_by_name("type");
                let Some(attribute) = type_attribute else {
                    let has_spread_prop = element
                        .attributes()
                        .into_iter()
                        .any(|attr| attr.as_jsx_spread_attribute().is_some());
                    if has_spread_prop {
                        return None;
                    } else {
                        return Some(UseButtonTypeState {
                            range: element.range(),
                            missing_prop: true,
                        });
                    }
                };
                inspect_jsx_type_attribute(&attribute)
            }
            UseButtonTypeQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(call_expression, model)?;
                // first argument needs to be a string
                let first_argument = react_create_element
                    .element_type
                    .as_any_js_expression()?
                    .as_static_value()?;
                if first_argument.text() != "button" {
                    return None;
                }
                let Some(props) = react_create_element.props.as_ref() else {
                    return Some(UseButtonTypeState {
                        range: first_argument.range(),
                        missing_prop: true,
                    });
                };
                let Some(member) = react_create_element.find_prop_by_name("type") else {
                    // We haven't found the property `type`
                    return Some(UseButtonTypeState {
                        range: props.range(),
                        missing_prop: false,
                    });
                };
                let property_value = member.value().ok()?.as_static_value()?;
                if ALLOWED_BUTTON_TYPES.contains(&property_value.text()) {
                    None
                } else {
                    Some(UseButtonTypeState {
                        range: property_value.range(),
                        missing_prop: false,
                    })
                }
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = if state.missing_prop {
            (markup! {
                "Provide an explicit "<Emphasis>"type"</Emphasis>" prop for the "<Emphasis>"button"</Emphasis>" element."
            }).to_owned()
        } else {
            (markup!{
                "Provide a valid "<Emphasis>"type"</Emphasis>" prop for the "<Emphasis>"button"</Emphasis>" element."
            }).to_owned()
        };
        Some(RuleDiagnostic::new(rule_category!(),
            state.range,
            message
        )
            .note(markup! {
                "The default "<Emphasis>"type"</Emphasis>" of a button is "<Emphasis>"submit"</Emphasis>", which causes the submission of a form when placed inside a `form` element. "
                "This is likely not the behaviour that you want inside a React application."
            })
            .note(
            markup! {

                "Allowed button types are: "<Emphasis>"submit"</Emphasis>", "<Emphasis>"button"</Emphasis>" or "<Emphasis>"reset"</Emphasis>""
            }
        ))
    }
}

fn inspect_jsx_type_attribute(attribute: &JsxAttribute) -> Option<UseButtonTypeState> {
    let Some(initializer) = attribute.initializer() else {
        return Some(UseButtonTypeState {
            range: attribute.range(),
            missing_prop: false,
        });
    };
    let value = initializer.value().ok()?;
    let Some(value) = value.as_jsx_string() else {
        // computed value
        return None;
    };
    if ALLOWED_BUTTON_TYPES.contains(&&*value.inner_string_text().ok()?) {
        return None;
    }
    Some(UseButtonTypeState {
        range: value.range(),
        missing_prop: false,
    })
}

/// Checks whether the current element is a button
///
/// Case sensitive is important, `<button>` is different from `<Button>`
fn is_button(name: &AnyJsxElementName) -> Option<bool> {
    // case sensitive is important, <button> is different from <Button>
    Some(match name {
        AnyJsxElementName::JsxName(name) => {
            let name = name.value_token().ok()?;
            name.text_trimmed() == "button"
        }
        _ => false,
    })
}
