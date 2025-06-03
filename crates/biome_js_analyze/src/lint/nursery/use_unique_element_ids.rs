use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsxAttributeValue, AnyJsxElementName, JsCallExpression, JsPropertyObjectMember, JsxAttribute,
    jsx_ext::AnyJsxElement,
};
use biome_rowan::{AstNode, declare_node_union};

use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::services::semantic::Semantic;

/// SVG element tags where `id` attributes have local scope within the SVG
const SVG_TAGS: [&str; 82] = [
    "a",
    "altGlyph",
    "altGlyphDef",
    "altGlyphItem",
    "animate",
    "animateColor",
    "animateMotion",
    "animateTransform",
    "circle",
    "clipPath",
    "color-profile",
    "cursor",
    "defs",
    "desc",
    "ellipse",
    "feBlend",
    "feColorMatrix",
    "feComponentTransfer",
    "feComposite",
    "feConvolveMatrix",
    "feDiffuseLighting",
    "feDisplacementMap",
    "feDistantLight",
    "feDropShadow",
    "feFlood",
    "feFuncA",
    "feFuncB",
    "feFuncG",
    "feFuncR",
    "feGaussianBlur",
    "feImage",
    "feMerge",
    "feMergeNode",
    "feMorphology",
    "feOffset",
    "fePointLight",
    "feSpecularLighting",
    "feSpotLight",
    "feTile",
    "feTurbulence",
    "filter",
    "font",
    "font-face",
    "font-face-format",
    "font-face-name",
    "font-face-src",
    "font-face-uri",
    "foreignObject",
    "g",
    "glyph",
    "glyphRef",
    "hatch",
    "hatchpath",
    "hkern",
    "image",
    "line",
    "linearGradient",
    "marker",
    "mask",
    "metadata",
    "missing-glyph",
    "mpath",
    "path",
    "pattern",
    "polygon",
    "polyline",
    "radialGradient",
    "rect",
    "script",
    "set",
    "stop",
    "style",
    "svg",
    "switch",
    "symbol",
    "text",
    "textPath",
    "title",
    "tspan",
    "use",
    "view",
    "vkern",
];

/// Checks if the given element name is an SVG tag
fn is_svg_element(element_name: &str) -> bool {
    SVG_TAGS.binary_search(&element_name).is_ok()
}

/// Checks if an element is an SVG element or is nested within an SVG context
fn is_in_svg_context(element: &AnyJsxElement) -> bool {
    // Check if current element is SVG
    if let Ok(name) = element.name() {
        if let Some(element_name) = get_element_name(&name) {
            if is_svg_element(&element_name) {
                return true;
            }
        }
    }

    // Walk up the tree to find if we're inside an SVG element
    let mut current = element.syntax().parent();
    while let Some(parent) = current {
        // Check if parent is a JSX element
        if let Some(jsx_element) = AnyJsxElement::cast(parent.clone()) {
            if let Ok(name) = jsx_element.name() {
                if let Some(element_name) = get_element_name(&name) {
                    if is_svg_element(&element_name) {
                        return true;
                    }
                }
            }
        }
        current = parent.parent();
    }

    false
}

/// Extracts the element name from AnyJsxElementName
fn get_element_name(name: &AnyJsxElementName) -> Option<String> {
    match name {
        AnyJsxElementName::JsxName(jsx_name) => {
            jsx_name.value_token().ok().map(|token| token.text_trimmed().to_string())
        }
        AnyJsxElementName::JsxMemberName(member_name) => {
            // For member names like React.Fragment, we only care about the first part
            member_name.object().ok().and_then(|obj| {
                if let biome_js_syntax::AnyJsxObjectName::JsxReferenceIdentifier(ref_id) = obj {
                    ref_id.value_token().ok().map(|token| token.text_trimmed().to_string())
                } else {
                    None
                }
            })
        }
        AnyJsxElementName::JsxNamespaceName(namespace_name) => {
            // For namespace names like svg:circle, we care about the namespace part
            namespace_name.namespace().ok().and_then(|ns| {
                ns.value_token().ok().map(|token| token.text_trimmed().to_string())
            })
        }
        AnyJsxElementName::JsxReferenceIdentifier(ref_id) => {
            ref_id.value_token().ok().map(|token| token.text_trimmed().to_string())
        }
        AnyJsxElementName::JsMetavariable(_) => None,
    }
}

declare_lint_rule! {
    /// Prevent the usage of static string literal `id` attribute on elements.
    ///
    /// In React, hardcoding IDs is discouraged because IDs have to be unique in the DOM.
    /// You should use [`useId`](https://react.dev/reference/react/useId) to generate unique IDs for accessibility purposes.
    ///
    /// Please keep in mind this rule doesn't check whether ids are actually unique or not, and does check whether static literal id isn't passed to the elements or not. So you're encouraged to check by yourself if the ids are actually unique.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div id="foo">bar</div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// React.createElement("div", { id: "foo" });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// const id = useId();
    /// <div id={id}>bar</div>;
    /// ```
    ///
    /// ```jsx
    /// const id = useId();
    /// React.createElement("div", { id });
    /// ```
    ///
    pub UseUniqueElementIds {
        version: "2.0.0",
        name: "useUniqueElementIds",
        language: "jsx",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::React],
    }
}

declare_node_union! {
    pub IdProp = JsxAttribute | JsPropertyObjectMember
}

declare_node_union! {
    pub UseUniqueElementIdsQuery = AnyJsxElement | JsCallExpression
}

impl UseUniqueElementIdsQuery {
    fn find_id_attribute(&self, model: &SemanticModel) -> Option<IdProp> {
        match self {
            Self::AnyJsxElement(jsx) => jsx.find_attribute_by_name("id").map(IdProp::from),
            Self::JsCallExpression(expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(expression, model)?;
                react_create_element
                    .find_prop_by_name("id")
                    .map(IdProp::from)
            }
        }
    }

    fn is_in_svg_context(&self, model: &SemanticModel) -> bool {
        match self {
            Self::AnyJsxElement(jsx) => is_in_svg_context(jsx),
            Self::JsCallExpression(expression) => {
                // For React.createElement calls, check if the element type is an SVG element
                if let Some(react_create_element) = ReactCreateElementCall::from_call_expression(expression, model) {
                    // Extract element name from the element_type argument
                    if let Some(element_expr) = react_create_element.element_type.as_any_js_expression() {
                        if let Some(static_value) = element_expr.as_static_value() {
                            if let Some(element_name) = static_value.as_string_constant() {
                                return is_svg_element(element_name);
                            }
                        }
                    }
                }
                false
            }
        }
    }
}

impl Rule for UseUniqueElementIds {
    type Query = Semantic<UseUniqueElementIdsQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        // Skip validation for SVG elements and elements within SVG context
        if node.is_in_svg_context(model) {
            return None;
        }

        let id_attribute = node.find_id_attribute(model)?;

        match id_attribute {
            IdProp::JsxAttribute(jsx_attribute) => {
                let jsx_any_attribute_value = jsx_attribute.initializer()?.value().ok()?;
                if matches!(jsx_any_attribute_value, AnyJsxAttributeValue::JsxString(_)) {
                    return Some(());
                }
                None
            }
            IdProp::JsPropertyObjectMember(js_object_member) => {
                let expression = js_object_member.value().ok()?;
                if matches!(expression, AnyJsExpression::AnyJsLiteralExpression(_)) {
                    return Some(());
                }
                None
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    <Emphasis>"id"</Emphasis>" attribute should not be a static string literal. Generate unique IDs using "<Emphasis>"useId()"</Emphasis>"."
                },
            )
            .note(markup! {
                "In React, if you hardcode IDs and use the component multiple times, it can lead to duplicate IDs in the DOM. Instead, generate unique IDs using "<Emphasis>"useId()"</Emphasis>"."
            }),
        )
    }
}
