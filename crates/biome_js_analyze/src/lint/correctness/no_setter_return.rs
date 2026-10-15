use crate::services::control_flow::AnyJsControlFlowRoot;
use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsMemberExpression, AnyJsObjectMemberName, JsArrowFunctionExpression,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsFunctionExpression,
    JsMethodObjectMember, JsObjectExpression, JsObjectMemberList, JsParenthesizedExpression,
    JsPropertyObjectMember, JsReturnStatement, JsSetterClassMember, JsSetterObjectMember,
    JsSyntaxNode,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::no_setter_return::NoSetterReturnOptions;

declare_lint_rule! {
    /// Disallow returning a value from a setter
    ///
    /// While returning a value from a setter does not produce an error, the returned value is being ignored. Therefore, returning a value from a setter is either unnecessary or a possible error.
    ///
    /// Only returning without a value is allowed, as it’s a control flow statement.
    ///
    /// The rule also checks setters in property descriptors passed to
    /// `Object.defineProperty`, `Object.defineProperties`, `Object.create`, and
    /// `Reflect.defineProperty`. An arrow function used as that setter reports
    /// its expression body, because the body is an implicit return.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     set foo(x) {
    ///         return x;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const b = {
    ///     set foo(x) {
    ///         return x;
    ///     },
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const c = {
    ///     set foo(x) {
    ///         if (x) {
    ///             return x;
    ///         }
    ///     },
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let v;
    /// Object.defineProperty({}, "x", {
    ///     set: (value) => (v = value),
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.defineProperties({}, {
    ///     x: {
    ///         set(value) {
    ///             return value;
    ///         },
    ///     },
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // early-return
    /// class A {
    ///     set foo(x) {
    ///         if (x) {
    ///             return;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// // not a setter
    /// class B {
    ///   set(x) {
    ///     return x;
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// let v;
    /// Object.defineProperty({}, "x", {
    ///     set(value) {
    ///         v = value;
    ///     },
    /// });
    /// ```
    ///
    /// ```js
    /// // `set` is an ordinary property here, not a descriptor setter.
    /// const obj = {
    ///     set: (value) => value,
    /// };
    /// ```
    pub NoSetterReturn {
        version: "1.0.0",
        name: "noSetterReturn",
        language: "js",
        sources: &[RuleSource::Eslint("no-setter-return").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub NoSetterReturnQuery = JsReturnStatement | JsArrowFunctionExpression
}

declare_node_union! {
    pub SetterLocation =
        JsSetterClassMember
        | JsSetterObjectMember
        | JsPropertyObjectMember
        | JsMethodObjectMember
}

impl Rule for NoSetterReturn {
    type Query = Semantic<NoSetterReturnQuery>;
    type State = SetterLocation;
    type Signals = Option<Self::State>;
    type Options = NoSetterReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        match ctx.query() {
            NoSetterReturnQuery::JsReturnStatement(ret) => {
                // A bare `return` is control flow. Only a returned value is ignored.
                ret.argument()?;
                let root = ret.syntax().ancestors().skip(1).find(|ancestor| {
                    AnyJsControlFlowRoot::can_cast(ancestor.kind())
                })?;
                setter_for_control_flow_root(&root, model)
            }
            NoSetterReturnQuery::JsArrowFunctionExpression(arrow) => {
                // `{ set: (value) => value }` has no return statement.
                // The expression body is the implicit return.
                arrow.body().ok()?.as_any_js_expression()?;
                descriptor_property_setter(arrow.syntax(), model)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, setter: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reported_range(ctx.query())?,
                markup! {
                    "The setter should not "<Emphasis>"return"</Emphasis>" a value."
                },
            )
            .detail(setter.range(), "The setter is here:")
            .note("Returning a value from a setter is ignored."),
        )
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        reported_range(ctx.query())
    }
}

fn reported_range(query: &NoSetterReturnQuery) -> Option<TextRange> {
    match query {
        NoSetterReturnQuery::JsReturnStatement(ret) => Some(ret.range()),
        NoSetterReturnQuery::JsArrowFunctionExpression(arrow) => {
            Some(arrow.body().ok()?.as_any_js_expression()?.range())
        }
    }
}

/// The nearest function-like root of a `return`, when that root is a setter.
fn setter_for_control_flow_root(
    root: &JsSyntaxNode,
    model: &SemanticModel,
) -> Option<SetterLocation> {
    if let Some(setter) = JsSetterClassMember::cast_ref(root) {
        return Some(setter.clone().into());
    }
    if let Some(setter) = JsSetterObjectMember::cast_ref(root) {
        return Some(setter.clone().into());
    }
    if let Some(method) = JsMethodObjectMember::cast_ref(root) {
        return descriptor_method_setter(&method, model);
    }
    if JsArrowFunctionExpression::can_cast(root.kind()) || JsFunctionExpression::can_cast(root.kind())
    {
        return descriptor_property_setter(root, model);
    }
    None
}

/// `set() { return value; }` inside a property descriptor.
fn descriptor_method_setter(
    method: &JsMethodObjectMember,
    model: &SemanticModel,
) -> Option<SetterLocation> {
    if !member_name_is_set(&method.name().ok()?) {
        return None;
    }
    let object = enclosing_object(method.syntax())?;
    is_property_descriptor(object.syntax(), model).then(|| method.clone().into())
}

/// `set: function () { return value; }` or `set: (value) => value` in a descriptor.
fn descriptor_property_setter(function: &JsSyntaxNode, model: &SemanticModel) -> Option<SetterLocation> {
    let property = set_property_value(function)?;
    let object = enclosing_object(property.syntax())?;
    is_property_descriptor(object.syntax(), model).then(|| property.into())
}

fn set_property_value(function: &JsSyntaxNode) -> Option<JsPropertyObjectMember> {
    let mut current = function.clone();
    loop {
        let parent = current.parent()?;
        if JsParenthesizedExpression::can_cast(parent.kind()) {
            current = parent;
            continue;
        }
        let property = JsPropertyObjectMember::cast(parent)?;
        let value = property.value().ok()?.omit_parentheses();
        if value.syntax() != function {
            return None;
        }
        return member_name_is_set(&property.name().ok()?).then_some(property);
    }
}

fn member_name_is_set(name: &AnyJsObjectMemberName) -> bool {
    name.name().is_some_and(|text| text.text() == "set")
}

fn enclosing_object(member: &JsSyntaxNode) -> Option<JsObjectExpression> {
    let parent = member.parent()?;
    let object = if JsObjectMemberList::can_cast(parent.kind()) {
        parent.parent()?
    } else {
        parent
    };
    JsObjectExpression::cast(object)
}

/// A descriptor object passed to `Object.defineProperty` / `Reflect.defineProperty`,
/// or a property value of the map passed to `Object.defineProperties` / `Object.create`.
fn is_property_descriptor(object: &JsSyntaxNode, model: &SemanticModel) -> bool {
    if is_global_method_argument(object, model, "Object", "defineProperty", 2)
        || is_global_method_argument(object, model, "Reflect", "defineProperty", 2)
    {
        return true;
    }
    let Some(property) = property_value_owner(object) else {
        return false;
    };
    let Some(outer) = enclosing_object(property.syntax()) else {
        return false;
    };
    is_global_method_argument(outer.syntax(), model, "Object", "defineProperties", 1)
        || is_global_method_argument(outer.syntax(), model, "Object", "create", 1)
}

fn property_value_owner(object: &JsSyntaxNode) -> Option<JsPropertyObjectMember> {
    let mut current = object.clone();
    loop {
        let parent = current.parent()?;
        if JsParenthesizedExpression::can_cast(parent.kind()) {
            current = parent;
            continue;
        }
        let property = JsPropertyObjectMember::cast(parent)?;
        let value = property.value().ok()?.omit_parentheses();
        if value.syntax() == object {
            return Some(property);
        }
        return None;
    }
}

fn is_global_method_argument(
    node: &JsSyntaxNode,
    model: &SemanticModel,
    object_name: &str,
    method_name: &str,
    index: usize,
) -> bool {
    global_method_argument(node, model, object_name, method_name, index).unwrap_or(false)
}

fn global_method_argument(
    node: &JsSyntaxNode,
    model: &SemanticModel,
    object_name: &str,
    method_name: &str,
    index: usize,
) -> Option<bool> {
    let call = enclosing_call(node)?;
    let argument = call.arguments().ok()?.args().iter().nth(index)?.ok()?;
    let argument = argument.as_any_js_expression()?;
    if argument.clone().omit_parentheses().syntax() != node {
        return Some(false);
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let member = AnyJsMemberExpression::cast_ref(callee.syntax())?;
    if member.member_name()?.text() != method_name {
        return Some(false);
    }
    let object = member.object().ok()?.omit_parentheses();
    let reference = object.as_js_reference_identifier()?;
    Some(reference.has_name(object_name) && model.binding(&reference).is_none())
}

fn enclosing_call(node: &JsSyntaxNode) -> Option<JsCallExpression> {
    let mut current = node.clone();
    loop {
        let parent = current.parent()?;
        if let Some(call) = JsCallExpression::cast_ref(&parent) {
            return Some(call.clone());
        }
        if JsParenthesizedExpression::can_cast(parent.kind())
            || JsCallArgumentList::can_cast(parent.kind())
            || JsCallArguments::can_cast(parent.kind())
        {
            current = parent;
            continue;
        }
        return None;
    }
}
