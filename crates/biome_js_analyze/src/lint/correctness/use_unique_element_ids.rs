use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsxAttributeValue, JsCallExpression, JsPropertyObjectMember, JsxAttribute,
    JsxAttributeList, JsxElement, JsxExpressionChild, jsx_ext::AnyJsxElement,
};
use biome_rowan::{AstNode, TokenText, declare_node_union};
use biome_rule_options::use_unique_element_ids::UseUniqueElementIdsOptions;

use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Prevent the usage of static string literal `id` attribute on elements outside SVG contexts.
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
    /// Static IDs are allowed in SVG contexts:
    ///
    /// ```jsx
    /// <svg>
    ///     <defs>
    ///         <pattern id="dots" width="10" height="10" />
    ///     </defs>
    ///     <rect fill="url(#dots)" width="100%" height="100%" />
    /// </svg>
    /// ```
    ///
    /// ## Options
    ///
    /// The following option is available
    ///
    /// ### `excludedComponents`
    ///
    /// List of unqualified component names to ignore.
    /// Use it to list components expecting an `id` attribute that does not represent
    /// a DOM element ID.
    ///
    /// **Default**: empty list.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "excludedComponents": [
    ///             "FormattedMessage"
    ///         ]
    ///     }
    /// }
    /// ```
    ///
    /// ```jsx,use_options
    /// <FormattedMessage id="static" />
    /// ```
    ///
    /// ```jsx,use_options
    /// <Library.FormattedMessage id="static" />
    /// ```
    ///
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
    fn create_element_call(&self, model: &SemanticModel) -> Option<ReactCreateElementCall> {
        match self {
            Self::JsCallExpression(expression) => {
                ReactCreateElementCall::from_call_expression(expression, model)
            }
            &Self::AnyJsxElement(_) => None,
        }
    }

    fn element_name(&self, model: &SemanticModel) -> Option<TokenText> {
        match self {
            Self::AnyJsxElement(jsx) => jsx
                .name_value_token()
                .ok()
                .map(|tok| tok.token_text_trimmed()),
            Self::JsCallExpression(_) => self
                .create_element_call(model)?
                .element_type
                .as_any_js_expression()?
                .get_callee_member_name()
                .map(|tok| tok.token_text_trimmed()),
        }
    }

    fn is_in_svg_context(&self) -> bool {
        if let Self::AnyJsxElement(element) = self
            && element.matches_name("svg").is_ok_and(|matches| matches)
        {
            return true;
        }

        let Some(parent) = (match self {
            Self::AnyJsxElement(AnyJsxElement::JsxOpeningElement(element)) => {
                element.syntax().grand_parent()
            }
            Self::AnyJsxElement(AnyJsxElement::JsxSelfClosingElement(element)) => {
                element.syntax().parent()
            }
            Self::JsCallExpression(expression) => expression
                .syntax()
                .parent()
                .and_then(JsxExpressionChild::cast)
                .and_then(|child| child.syntax().parent()),
        }) else {
            return false;
        };

        for ancestor in parent.ancestors() {
            if JsxAttributeList::can_cast(ancestor.kind()) {
                return false;
            }
            let Some(name) = JsxElement::cast(ancestor)
                .and_then(|element| element.opening_element().ok())
                .and_then(|element| element.name().ok())
            else {
                continue;
            };
            if name
                .matches_name("foreignObject")
                .is_ok_and(|matches| matches)
            {
                return false;
            }
            if name.matches_name("svg").is_ok_and(|matches| matches) {
                return true;
            }
        }
        false
    }

    fn find_id_attribute(&self, model: &SemanticModel) -> Option<IdProp> {
        match self {
            Self::AnyJsxElement(jsx) => jsx.find_attribute_by_name("id").map(IdProp::from),
            Self::JsCallExpression(_) => self
                .create_element_call(model)?
                .find_prop_by_name("id")
                .map(IdProp::from),
        }
    }
}

impl Rule for UseUniqueElementIds {
    type Query = Semantic<UseUniqueElementIdsQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseUniqueElementIdsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let options = ctx.options();
        if let Some(name) = node.element_name(model)
            && let Some(excluded_components) = &options.excluded_components
            && excluded_components.contains(name.text())
        {
            return None;
        }
        let id_attribute = node.find_id_attribute(model)?;

        let has_static_id = match id_attribute {
            IdProp::JsxAttribute(jsx_attribute) => {
                let jsx_any_attribute_value = jsx_attribute.initializer()?.value().ok()?;
                matches!(jsx_any_attribute_value, AnyJsxAttributeValue::JsxString(_))
            }
            IdProp::JsPropertyObjectMember(js_object_member) => {
                let expression = js_object_member.value().ok()?;
                matches!(expression, AnyJsExpression::AnyJsLiteralExpression(_))
            }
        };

        (has_static_id && !node.is_in_svg_context()).then_some(())
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
