use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue, AnyJsxElementName, AnyJsxTag,
    JsxAttribute, JsxName, JsxOpeningElement, JsxReferenceIdentifier, JsxTagExpression, JsxText,
};
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub NoLabelWithoutControl {
        version: "next",
        name: "noLabelWithoutControl",
        language: "js",
        recommended: false,
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoLabelWithoutControlOptions {
    pub label_components: Vec<String>,
    pub label_attributes: Vec<String>,
    pub control_components: Vec<String>,
}

impl Rule for NoLabelWithoutControl {
    type Query = Ast<JsxTagExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoLabelWithoutControlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let label_attributes = get_option(&options.label_attributes, &["aria-label"]);
        let label_components = get_option(&options.label_components, &["label"]);
        let control_components = get_option(
            &options.control_components,
            &["input", "meter", "output", "progress", "select", "textarea"],
        );
        let element_name = get_element_name(node)?;
        let is_allowed_element = label_components.contains(&element_name);

        if !is_allowed_element {
            return None;
        }

        if has_accessible_label(node, &label_attributes)
            && (has_for_attribute(node)? || has_nested_control(node, &control_components))
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

/// Returns an array option merged with the default value
fn get_option(value: &[String], default_value: &[&str]) -> Vec<String> {
    value
        .iter()
        .cloned()
        .chain(default_value.iter().map(|value| (*value).to_string()))
        .collect()
}

/// Returns the `JsxOpeningElement` inside of the passed `JsxTagExpression`
fn get_opening_element(jsx_tag_expression: &JsxTagExpression) -> Option<JsxOpeningElement> {
    match jsx_tag_expression.tag().ok()? {
        AnyJsxTag::JsxElement(element) => element.opening_element().ok(),
        _ => None,
    }
}

/// Returns the element name of a `JsxTagExpression`
fn get_element_name(jsx_tag_expression: &JsxTagExpression) -> Option<String> {
    let opening_element = get_opening_element(jsx_tag_expression)?;

    match opening_element.name().ok()? {
        AnyJsxElementName::JsxName(jsx_name) => Some(jsx_name.text()),
        AnyJsxElementName::JsxReferenceIdentifier(jsx_reference_identifier) => {
            Some(jsx_reference_identifier.text())
        }
        _ => None,
    }
}

/// Returns whether the passed `JsxTagExpression` have a `for` or `htmlFor` attribute
fn has_for_attribute(jsx_tag_expression: &JsxTagExpression) -> Option<bool> {
    let for_attributes = &["for", "htmlFor"];
    let opening_element = get_opening_element(jsx_tag_expression)?;

    Some(opening_element.attributes().into_iter().any(|attribute| {
        match attribute {
            AnyJsxAttribute::JsxAttribute(jsx_attribute) => jsx_attribute
                .name()
                .is_ok_and(|jsx_name| for_attributes.contains(&jsx_name.text().as_str())),
            _ => false,
        }
    }))
}

/// Returns whether the passed `JsxTagExpression` have a child that is considered a control component
/// according to the passed `control_components` parameter
fn has_nested_control(
    jsx_tag_expression: &JsxTagExpression,
    control_components: &[String],
) -> bool {
    jsx_tag_expression.syntax().descendants().any(|descendant| {
        match (
            JsxName::cast(descendant.clone()),
            JsxReferenceIdentifier::cast(descendant.clone()),
        ) {
            (Some(jsx_name), _) => control_components.contains(&jsx_name.text()),
            (_, Some(jsx_reference_identifier)) => {
                control_components.contains(&jsx_reference_identifier.text())
            }
            _ => false,
        }
    })
}

/// Returns whether the passed `JsxTagExpression` meets one of the following conditions:
/// - Has a label attribute that corresponds to the `label_attributes` parameter
/// - Has an `aria-labelledby` attribute
/// - Has a child `JsxText` node
fn has_accessible_label(
    jsx_tag_expression: &JsxTagExpression,
    label_attributes: &[String],
) -> bool {
    let jsx_attributes = jsx_tag_expression
        .syntax()
        .descendants()
        .filter_map(JsxAttribute::cast)
        .collect::<Vec<JsxAttribute>>();

    let has_accessible_attribute = jsx_attributes.iter().any(|jsx_attribute| {
        match (
            jsx_attribute.name().ok(),
            jsx_attribute
                .initializer()
                .and_then(|initializer| initializer.value().ok()),
        ) {
            (Some(AnyJsxAttributeName::JsxName(jsx_name)), Some(jsx_attribute_value)) => {
                let has_label_attribute = label_attributes.contains(&jsx_name.text());
                let is_aria_labelledby_attribute = jsx_name.text() == "aria-labelledby";
                let has_value = has_jsx_attribute_value(jsx_attribute_value);

                has_value && (is_aria_labelledby_attribute || has_label_attribute)
            }
            _ => false,
        }
    });

    let has_text = jsx_tag_expression
        .syntax()
        .descendants()
        .any(|descendant| JsxText::cast(descendant).is_some());

    has_accessible_attribute || has_text
}

/// Returns whether the passed `jsx_attribute_value` has a valid value inside it
fn has_jsx_attribute_value(jsx_attribute_value: AnyJsxAttributeValue) -> bool {
    match jsx_attribute_value {
        AnyJsxAttributeValue::JsxString(jsx_string) => jsx_string
            .inner_string_text()
            .ok()
            .map(|inner| inner.text().to_string())
            .is_some_and(|escaped_text| !escaped_text.trim().is_empty()),
        AnyJsxAttributeValue::JsxExpressionAttributeValue(_) => true,
        _ => false,
    }
}
