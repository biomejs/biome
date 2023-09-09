use crate::react::{is_react_call_api, ReactLibrary};
use crate::semantic_services::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use rome_js_syntax::{
    AnyJsFunction, AnyJsMemberExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsFormalParameter, JsObjectExpression, JsObjectMemberList, JsParameterList, JsParameters,
    JsPropertyObjectMember, JsReferenceIdentifier, JsxAttribute,
};
use rome_rowan::{declare_node_union, AstNode, TextRange};

declare_rule! {
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
    pub(crate) NoArrayIndexKey {
        version: "1.0.0",
        name: "noArrayIndexKey",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) NoArrayIndexKeyQuery = JsxAttribute | JsPropertyObjectMember
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
    fn as_js_reference_identifier(&self) -> Option<JsReferenceIdentifier> {
        match self {
            NoArrayIndexKeyQuery::JsxAttribute(attribute) => attribute
                .initializer()?
                .value()
                .ok()?
                .as_jsx_expression_attribute_value()?
                .expression()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok(),
            NoArrayIndexKeyQuery::JsPropertyObjectMember(object_member) => object_member
                .value()
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok(),
        }
    }
}

pub(crate) struct NoArrayIndexKeyState {
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
        let reference = node.as_js_reference_identifier()?;

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
            let callee = call_expression.callee().ok()?;

            if is_react_call_api(callee, model, ReactLibrary::React, "cloneElement") {
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
/// ```js
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
        AnyJsMemberExpression::cast_ref(call_expression.callee().ok()?.syntax())?;
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
