use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeValue, AnyJsxTag, JsxAttribute, JsxAttributeList, JsxName,
    JsxReferenceIdentifier, JsxText,
};
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Enforce that a label element or component has a text label and an associated input.
    ///
    /// An "input" is considered one of the following elements: `input`, `meter`, `output`, `progress`, `select` or `textarea`.
    ///
    /// There are two supported ways to associate a label with an input:
    /// - Wrapping an input in a label element.
    /// - Adding a `for` attribute (or `htmlFor` in React) to a label and assigning it a DOM ID string associated with an input on the page.
    ///
    ///
    /// This rule checks that any `label` element (or an indicated custom component that will output a `label` element) meets one of these conditions:
    /// - Wraps an `input` element (or an indicated custom component that will output an `input` element)
    /// - Has a `for` or `htmlFor` attribute and that the `label` element/component has accessible text content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <label for="js_id" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label for="js_id"><input /></label>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label htmlFor="js_id" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label htmlFor="js_id"><input /></label>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label>A label</label>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div><label /><input /></div>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <label for="js_id" aria-label="A label" />;
    /// <label for="js_id" aria-labelledby="A label" />;
    /// <label htmlFor="js_id" aria-label="A label" />;
    /// <label htmlFor="js_id" aria-labelledby="A label" />;
    /// <label>A label<input /></label>;
    /// <label>A label<textarea /></label>;
    /// <label><img alt="A label" /><input /></label>;
    /// ```
    ///
    /// ## Options
    ///
    /// The rule supports the following options:
    /// - `inputComponents` - An array of component names that should be considered the same as an `input` element.
    /// - `labelAttributes` - An array of attributes that should be treated as the `label` accessible text content.
    /// - `labelComponents` - An array of component names that should be considered the same as a `label` element.
    ///
    /// Both options `inputComponents` and `labelComponents` don't have support for namespace components (e.g. `<Control.Input>`).
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "inputComponents": ["CustomInput"],
    ///         "labelAttributes": ["label"],
    ///         "labelComponents": ["CustomLabel"]
    ///     }
    /// }
    /// ```
    ///
    pub NoLabelWithoutControl {
        version: "1.8.0",
        name: "noLabelWithoutControl",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("label-has-associated-control")],
        recommended: true,
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoLabelWithoutControlOptions {
    /// Array of component names that should be considered the same as an `input` element.
    pub input_components: Vec<String>,
    /// Array of attributes that should be treated as the `label` accessible text content.
    pub label_attributes: Vec<String>,
    /// Array of component names that should be considered the same as a `label` element.
    pub label_components: Vec<String>,
}

pub struct NoLabelWithoutControlState {
    pub has_text_content: bool,
    pub has_control_association: bool,
}

const DEFAULT_LABEL_ATTRIBUTES: &[&str; 2] = &["aria-label", "alt"];
const DEFAULT_LABEL_COMPONENTS: &[&str; 1] = &["label"];
const DEFAULT_INPUT_COMPONENTS: &[&str; 6] =
    &["input", "meter", "output", "progress", "select", "textarea"];

impl Rule for NoLabelWithoutControl {
    type Query = Ast<AnyJsxTag>;
    type State = NoLabelWithoutControlState;
    type Signals = Option<Self::State>;
    type Options = NoLabelWithoutControlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let label_attributes = &options.label_attributes;
        let label_components = &options.label_components;
        let input_components = &options.input_components;

        let element_name = get_element_name(node)?;
        let is_allowed_element = label_components.contains(&element_name)
            || DEFAULT_LABEL_COMPONENTS.contains(&element_name.as_str());

        if !is_allowed_element {
            return None;
        }

        let has_text_content = has_accessible_label(node, label_attributes);
        let has_control_association =
            has_for_attribute(node)? || has_nested_control(node, input_components);

        if has_text_content && has_control_association {
            return None;
        }

        Some(NoLabelWithoutControlState {
            has_text_content,
            has_control_association,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "A form label must be associated with an input."
            },
        );

        if !state.has_text_content {
            diagnostic = diagnostic.note(
                markup! { "Consider adding an accessible text content to the label element." },
            );
        }

        if !state.has_control_association {
            diagnostic = diagnostic.note(
                markup! { "Consider adding a `for` or `htmlFor` attribute to the label element or moving the input element to inside the label element." },
            );
        }

        Some(diagnostic)
    }
}

/// Returns the `JsxAttributeList` of the passed `AnyJsxTag`
fn get_element_attributes(jsx_tag: &AnyJsxTag) -> Option<JsxAttributeList> {
    match jsx_tag {
        AnyJsxTag::JsxElement(element) => Some(element.opening_element().ok()?.attributes()),
        AnyJsxTag::JsxSelfClosingElement(element) => Some(element.attributes()),
        _ => None,
    }
}

/// Returns the element name of a `AnyJsxTag`
fn get_element_name(jsx_tag: &AnyJsxTag) -> Option<String> {
    match jsx_tag {
        AnyJsxTag::JsxElement(element) => Some(element.opening_element().ok()?.name().ok()?.text()),
        AnyJsxTag::JsxSelfClosingElement(element) => Some(element.name().ok()?.text()),
        _ => None,
    }
}

/// Returns whether the passed `AnyJsxTag` have a `for` or `htmlFor` attribute
fn has_for_attribute(jsx_tag: &AnyJsxTag) -> Option<bool> {
    let for_attributes = &["for", "htmlFor"];
    let attributes = get_element_attributes(jsx_tag)?;

    Some(attributes.into_iter().any(|attribute| {
        match attribute {
            AnyJsxAttribute::JsxAttribute(jsx_attribute) => jsx_attribute
                .name()
                .is_ok_and(|jsx_name| for_attributes.contains(&jsx_name.text().as_str())),
            _ => false,
        }
    }))
}

/// Returns whether the passed `AnyJsxTag` have a child that is considered an input component
/// according to the passed `input_components` parameter
fn has_nested_control(jsx_tag: &AnyJsxTag, input_components: &[String]) -> bool {
    jsx_tag.syntax().descendants().any(|descendant| {
        match (
            JsxName::cast(descendant.clone()),
            JsxReferenceIdentifier::cast(descendant.clone()),
        ) {
            (Some(jsx_name), _) => {
                let attribute_name = jsx_name.text();
                input_components.contains(&attribute_name)
                    || DEFAULT_INPUT_COMPONENTS.contains(&attribute_name.as_str())
            }
            (_, Some(jsx_reference_identifier)) => {
                let attribute_name = jsx_reference_identifier.text();
                input_components.contains(&attribute_name)
                    || DEFAULT_INPUT_COMPONENTS.contains(&attribute_name.as_str())
            }
            _ => false,
        }
    })
}

/// Returns whether the passed `AnyJsxTag` meets one of the following conditions:
/// - Has a label attribute that corresponds to the `label_attributes` parameter
/// - Has an `aria-labelledby` attribute
/// - Has a child `JsxText` node
fn has_accessible_label(jsx_tag: &AnyJsxTag, label_attributes: &[String]) -> bool {
    let mut has_text = false;
    let mut has_accessible_attribute = false;

    for descendant in jsx_tag.syntax().descendants() {
        if let Some(jsx_attribute) = JsxAttribute::cast(descendant.clone()) {
            if let (Some(jsx_name), Some(jsx_attribute_value)) = (
                jsx_attribute.name().ok(),
                jsx_attribute
                    .initializer()
                    .and_then(|initializer| initializer.value().ok()),
            ) {
                let attribute_name = jsx_name.text();
                let has_label_attribute = label_attributes.contains(&attribute_name)
                    || DEFAULT_LABEL_ATTRIBUTES.contains(&attribute_name.as_str());
                let is_aria_labelledby_attribute = jsx_name.text() == "aria-labelledby";
                let has_value = has_jsx_attribute_value(&jsx_attribute_value);

                if has_value && (is_aria_labelledby_attribute || has_label_attribute) {
                    has_accessible_attribute = true
                }
            }
        }

        if JsxText::cast(descendant.clone()).is_some() {
            has_text = true;
        }
    }

    has_accessible_attribute || has_text
}

/// Returns whether the passed `jsx_attribute_value` has a valid value inside it
fn has_jsx_attribute_value(jsx_attribute_value: &AnyJsxAttributeValue) -> bool {
    jsx_attribute_value
        .as_static_value()
        .is_some_and(|static_value| !static_value.text().trim().is_empty())
}
