use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnyHtmlElement, AnyHtmlTagName, HtmlAttribute,
    HtmlFileSource, HtmlSyntaxKind,
};
use biome_rowan::{AstNode, WalkEvent};
use biome_rule_options::no_label_without_control::NoLabelWithoutControlOptions;

declare_lint_rule! {
    /// Enforce that a label element or component has a text label and an associated input.
    ///
    /// An "input" is considered one of the following elements: `input`, `meter`, `output`, `progress`, `select`, `textarea` or `button`.
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
    /// ```html,expect_diagnostic
    /// <label for="html_id"></label>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <label for="html_id"><input /></label>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <label>A label</label>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div><label></label><input /></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <label for="html_id" aria-label="A label"></label>
    /// <label for="html_id" aria-labelledby="A label"></label>
    /// <label>A label<input /></label>
    /// <label>A label<textarea></textarea></label>
    /// <label><img alt="A label" /><input /></label>
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
    /// ```json,options
    /// {
    ///     "options": {
    ///         "inputComponents": ["CustomInput"],
    ///         "labelAttributes": ["label"],
    ///         "labelComponents": ["CustomLabel"]
    ///     }
    /// }
    /// ```
    ///
    pub NoLabelWithoutControl {
        version: "next",
        name: "noLabelWithoutControl",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("label-has-associated-control").inspired()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoLabelWithoutControl {
    type Query = Ast<AnyHtmlElement>;
    type State = NoLabelWithoutControlState;
    type Signals = Option<Self::State>;
    type Options = NoLabelWithoutControlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();
        let source_type = ctx.source_type::<HtmlFileSource>();

        let element_name = node.name()?;
        let element_name = element_name.trim();
        let is_allowed_element = has_element_name(options, element_name, source_type);

        if !is_allowed_element {
            return None;
        }

        let has_text_content = has_accessible_label(options, node);
        let has_control_association =
            has_for_attribute(node) || has_nested_control(options, node, source_type);

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
                markup! { "Consider adding a \""<Emphasis>"for"</Emphasis>"\" attribute to the label element or moving the input element to inside the label element." },
            );
        }

        Some(diagnostic)
    }
}

/// Returns `true` whether the passed `attribute` meets one of the following conditions:
/// - Has a label attribute that corresponds to the `label_attributes` parameter
/// - Has a label among `DEFAULT_LABEL_ATTRIBUTES`
fn has_label_attribute(options: &NoLabelWithoutControlOptions, attribute: &HtmlAttribute) -> bool {
    let Ok(attribute_name) = attribute.name() else {
        return false;
    };
    let Some(attribute_name) = attribute_name.token_text_trimmed() else {
        return false;
    };
    if !options
        .label_attributes()
        .iter()
        .any(|name| *name == attribute_name)
    {
        return false;
    }
    attribute
        .initializer()
        .and_then(|init| init.value().ok())
        .is_some_and(|v| has_label_attribute_value(&v))
}

/// Returns `true` whether the passed `html_element` meets one of the following conditions:
/// - Has a label attribute that corresponds to the `label_attributes` parameter
/// - Has a label among `DEFAULT_LABEL_ATTRIBUTES`
/// - Has a child that acts as a label
fn has_accessible_label(
    options: &NoLabelWithoutControlOptions,
    html_element: &AnyHtmlElement,
) -> bool {
    let mut child_iter = html_element.syntax().preorder();
    while let Some(event) = child_iter.next() {
        match event {
            WalkEvent::Enter(child) => match child.kind() {
                HtmlSyntaxKind::HTML_TEXT_EXPRESSION | HtmlSyntaxKind::HTML_CONTENT => {
                    return true;
                }
                HtmlSyntaxKind::HTML_ELEMENT
                | HtmlSyntaxKind::HTML_OPENING_ELEMENT
                | HtmlSyntaxKind::HTML_ELEMENT_LIST
                | HtmlSyntaxKind::HTML_SELF_CLOSING_ELEMENT
                | HtmlSyntaxKind::HTML_ATTRIBUTE_LIST => {}
                HtmlSyntaxKind::HTML_ATTRIBUTE => {
                    let attribute = HtmlAttribute::unwrap_cast(child);
                    if has_label_attribute(options, &attribute) {
                        return true;
                    }
                    child_iter.skip_subtree();
                }
                _ => {
                    child_iter.skip_subtree();
                }
            },
            WalkEvent::Leave(_) => {}
        }
    }
    false
}

/// Returns whether the passed `AnyHtmlElement` have a child that is considered an input component
/// according to the passed `input_components` parameter
fn has_nested_control(
    options: &NoLabelWithoutControlOptions,
    html_element: &AnyHtmlElement,
    source_type: &HtmlFileSource,
) -> bool {
    let mut child_iter = html_element.syntax().preorder();
    while let Some(event) = child_iter.next() {
        match event {
            WalkEvent::Enter(child) => match child.kind() {
                HtmlSyntaxKind::HTML_ELEMENT
                | HtmlSyntaxKind::HTML_OPENING_ELEMENT
                | HtmlSyntaxKind::HTML_ELEMENT_LIST
                | HtmlSyntaxKind::HTML_SELF_CLOSING_ELEMENT => {}
                _ => {
                    let Some(element_name) = AnyHtmlTagName::cast(child) else {
                        child_iter.skip_subtree();
                        continue;
                    };
                    let Some(element_name) = element_name.token_text_trimmed() else {
                        continue;
                    };
                    let element_name = element_name.text();
                    if options.input_components().iter().any(|name| {
                        if source_type.is_html() {
                            element_name.eq_ignore_ascii_case(name)
                        } else {
                            &element_name == name
                        }
                    }) {
                        return true;
                    }
                }
            },
            WalkEvent::Leave(_) => {}
        }
    }
    false
}

fn has_element_name(
    options: &NoLabelWithoutControlOptions,
    element_name: &str,
    source_type: &HtmlFileSource,
) -> bool {
    options
        .label_components()
        .iter()
        .any(|label_component_name| {
            if source_type.is_html() {
                element_name.eq_ignore_ascii_case(label_component_name)
            } else {
                &element_name == label_component_name
            }
        })
}

pub struct NoLabelWithoutControlState {
    pub has_text_content: bool,
    pub has_control_association: bool,
}

/// Returns true whether the passed `AnyHtmlElement` has a `for` attribute
fn has_for_attribute(html_element: &AnyHtmlElement) -> bool {
    let Some(attributes) = html_element.attributes() else {
        return false;
    };

    attributes.into_iter().any(|attribute| match attribute {
        AnyHtmlAttribute::HtmlAttribute(html_attribute) => html_attribute
            .name()
            .ok()
            .and_then(|attribute_name| attribute_name.token_text_trimmed())
            .is_some_and(|text| text.eq_ignore_ascii_case("for")),
        _ => false,
    })
}

/// Returns whether the passed `html_attribute_value` has a valid value inside it
fn has_label_attribute_value(html_attribute_value: &AnyHtmlAttributeInitializer) -> bool {
    match html_attribute_value {
        AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(_) => true,
        AnyHtmlAttributeInitializer::HtmlString(html_string) => !html_string
            .inner_string_text()
            .is_ok_and(|text| text.text().trim().is_empty()),
        _ => false,
    }
}
