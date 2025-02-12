use crate::react::{is_react_call_api, ReactLibrary};
use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsMemberExpression, AnyJsTemplateElement,
    JsBinaryExpression, JsCallArgumentList, JsCallArguments, JsCallExpression, JsFormalParameter,
    JsObjectExpression, JsObjectMemberList, JsParameterList, JsParameters, JsPropertyObjectMember,
    JsReferenceIdentifier, JsxAttribute,
};
use biome_rowan::{declare_node_union, AstNode, TextRange};

declare_lint_rule! {
    /// Discourage the usage of Array index in keys.
    ///
    /// > We don’t recommend using indexes for keys if the order of items may change.
    /// This can negatively impact performance and may cause issues with component state.
    /// Check out Robin Pokorny’s article for an
    /// [in-depth explanation on the negative impacts of using an index as a key](https://robinpokorny.com/blog/index-as-a-key-is-an-anti-pattern/).
    /// If you choose not to assign an explicit key to list items then React will default to using indexes as keys.
    ///
    /// Source [React documentation](https://reactjs.org/docs/lists-and-keys.html#keys)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// something.forEach((Element, index) => {
    ///     <Component key={index} >foo</Component>
    /// });
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// React.Children.map(this.props.children, (child, index) => (
    ///     React.cloneElement(child, { key: index })
    /// ))
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// something.forEach((Element, index) => {
    ///     <Component key={`test-key-${index}`} >foo</Component>
    /// });
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// something.forEach((Element, index) => {
    ///     <Component key={"test" + index} >foo</Component>
    /// });
    /// ```
    ///
    /// ### Valid
    /// ```jsx
    /// something.forEach((item) => {
    ///     <Component key={item.id} >foo</Component>
    /// });
    /// ```
    ///
    /// ```jsx
    /// something.forEach((item) => {
    ///     <Component key={item.baz.foo} >foo</Component>
    /// });
    /// ```
    ///
    pub NoArrayIndexKey {
        version: "1.0.0",
        name: "noArrayIndexKey",
        language: "jsx",
        sources: &[RuleSource::EslintReact("no-array-index-key")],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub NoArrayIndexKeyQuery = JsxAttribute | JsPropertyObjectMember
}

impl NoArrayIndexKeyQuery {
    const fn is_property_object_member(&self) -> bool {
        matches!(self, NoArrayIndexKeyQuery::JsPropertyObjectMember(_))
    }

    fn is_key_property(&self) -> Option<bool> {
        Some(match self {
            NoArrayIndexKeyQuery::JsxAttribute(attribute) => {
                let attribute_name = attribute.name().ok()?;
                let name = attribute_name.as_jsx_name()?;
                let name_token = name.value_token().ok()?;
                name_token.text_trimmed() == "key"
            }
            NoArrayIndexKeyQuery::JsPropertyObjectMember(object_member) => {
                let object_member_name = object_member.name().ok()?;
                let name = object_member_name.as_js_literal_member_name()?;
                let name = name.value().ok()?;
                name.text_trimmed() == "key"
            }
        })
    }

    /// Extracts the reference from the possible invalid prop
    fn as_js_expression(&self) -> Option<AnyJsExpression> {
        match self {
            NoArrayIndexKeyQuery::JsxAttribute(attribute) => attribute
                .initializer()?
                .value()
                .ok()?
                .as_jsx_expression_attribute_value()?
                .expression()
                .ok(),
            NoArrayIndexKeyQuery::JsPropertyObjectMember(object_member) => {
                object_member.value().ok()
            }
        }
    }
}

pub struct NoArrayIndexKeyState {
    /// The incorrect prop
    incorrect_prop: TextRange,
    /// Where the incorrect prop was defined
    binding_origin: TextRange,
}

impl Rule for NoArrayIndexKey {
    type Query = Semantic<NoArrayIndexKeyQuery>;
    type State = NoArrayIndexKeyState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if !node.is_key_property()? {
            return None;
        }

        let model = ctx.model();
        let reference = node.as_js_expression()?;

        let mut capture_array_index = None;

        match reference {
            AnyJsExpression::JsIdentifierExpression(identifier_expression) => {
                capture_array_index = Some(identifier_expression.name().ok()?);
            }
            AnyJsExpression::JsTemplateExpression(template_expression) => {
                let template_elements = template_expression.elements();
                for element in template_elements {
                    if let AnyJsTemplateElement::JsTemplateElement(template_element) = element {
                        let cap_index_value = template_element
                            .expression()
                            .ok()?
                            .as_js_identifier_expression()?
                            .name()
                            .ok();
                        capture_array_index = cap_index_value;
                    }
                }
            }
            AnyJsExpression::JsBinaryExpression(binary_expression) => {
                let _ = cap_array_index_value(&binary_expression, &mut capture_array_index);
            }
            _ => {}
        };

        let reference = capture_array_index?;

        // Given the reference identifier retrieved from the key property,
        // find the declaration and ensure it resolves to the parameter of a function,
        // and navigate up to the closest call expression
        let parameter = model
            .binding(&reference)
            .and_then(|declaration| declaration.syntax().parent())
            .and_then(JsFormalParameter::cast)?;
        let function = parameter
            .parent::<JsParameterList>()
            .and_then(|list| list.parent::<JsParameters>())
            .and_then(|parameters| parameters.parent::<AnyJsFunction>())?;
        let call_expression = function
            .parent::<JsCallArgumentList>()
            .and_then(|arguments| arguments.parent::<JsCallArguments>())
            .and_then(|arguments| arguments.parent::<JsCallExpression>())?;

        // Check if the caller is an array method and the parameter is the array index of that method
        let is_array_method_index = is_array_method_index(&parameter, &call_expression)?;

        if !is_array_method_index {
            return None;
        }

        if node.is_property_object_member() {
            let object_expression = node
                .parent::<JsObjectMemberList>()
                .and_then(|list| list.parent::<JsObjectExpression>())?;

            // Check if the object expression is passed to a `React.cloneElement` call
            let call_expression = object_expression
                .parent::<JsCallArgumentList>()
                .and_then(|list| list.parent::<JsCallArguments>())
                .and_then(|arguments| arguments.parent::<JsCallExpression>())?;
            let callee = call_expression.callee().ok()?.omit_parentheses();

            if is_react_call_api(&callee, model, ReactLibrary::React, "cloneElement") {
                let binding = parameter.binding().ok()?;
                let binding_origin = binding.as_any_js_binding()?.as_js_identifier_binding()?;
                Some(NoArrayIndexKeyState {
                    binding_origin: binding_origin.range(),
                    incorrect_prop: reference.range(),
                })
            } else {
                None
            }
        } else {
            let binding = parameter.binding().ok()?;
            let binding_origin = binding.as_any_js_binding()?.as_js_identifier_binding()?;
            Some(NoArrayIndexKeyState {
                binding_origin: binding_origin.range(),
                incorrect_prop: reference.range(),
            })
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let NoArrayIndexKeyState {
            binding_origin: incorrect_key,
            incorrect_prop,
        } = state;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            incorrect_prop,
            markup! {"Avoid using the index of an array as key property in an element."},
        )
        .detail(
            incorrect_key,
            markup! {"This is the source of the key value."},
        ).note(
            markup! {"The order of the items may change, and this also affects performances and component state."}
        ).note(
            markup! {
                "Check the "<Hyperlink href="https://reactjs.org/docs/lists-and-keys.html#keys">"React documentation"</Hyperlink>". "
            }
        );

        Some(diagnostic)
    }
}

/// Given a parameter and a call expression, it navigates the `callee` of the call
/// and check if the method called by this function belongs to an array method
/// and if the parameter is an array index
///
/// ```jsx
/// Array.map((_, index) => {
///     return <Component key={index} />
/// })
/// ```
///
/// Given this example, the input node is the `index` and `Array.map(...)` call and we navigate to
/// retrieve the name `map` and we check if it belongs to an `Array.prototype` method.
fn is_array_method_index(
    parameter: &JsFormalParameter,
    call_expression: &JsCallExpression,
) -> Option<bool> {
    let member_expression =
        AnyJsMemberExpression::cast(call_expression.callee().ok()?.into_syntax())?;
    let name = member_expression.member_name()?;
    let name = name.text();
    if matches!(
        name,
        "map" | "flatMap" | "from" | "forEach" | "filter" | "some" | "every" | "find" | "findIndex"
    ) {
        Some(parameter.syntax().index() == 2)
    } else if matches!(name, "reduce" | "reduceRight") {
        Some(parameter.syntax().index() == 4)
    } else {
        None
    }
}

fn cap_array_index_value(
    binary_expression: &JsBinaryExpression,
    capture_array_index: &mut Option<JsReferenceIdentifier>,
) -> Option<()> {
    let left = binary_expression.left().ok()?;
    let right = binary_expression.right().ok()?;

    // recursive call if left or right again are binary_expressions
    if let Some(left_binary) = left.as_js_binary_expression() {
        cap_array_index_value(left_binary, capture_array_index);
    };

    if let Some(right_binary) = right.as_js_binary_expression() {
        cap_array_index_value(right_binary, capture_array_index);
    };

    if let Some(left_expression) = left.as_js_identifier_expression() {
        *capture_array_index = left_expression.name().ok();
    };

    if let Some(right_expression) = right.as_js_identifier_expression() {
        *capture_array_index = right_expression.name().ok();
    };

    Some(())
}
