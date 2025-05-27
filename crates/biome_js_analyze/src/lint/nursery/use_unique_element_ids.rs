use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsxAttributeValue, JsCallExpression, JsPropertyObjectMember, JsxAttribute,
    jsx_ext::AnyJsxElement,
};
use biome_rowan::{AstNode, declare_node_union};

use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::services::semantic::Semantic;

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
}

impl Rule for UseUniqueElementIds {
    type Query = Semantic<UseUniqueElementIdsQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
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
