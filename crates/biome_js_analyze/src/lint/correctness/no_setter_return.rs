use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsMemberExpression, JsArrowFunctionExpression, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsMethodObjectMember, JsObjectExpression, JsObjectMemberList,
    JsPropertyObjectMember, JsReturnStatement, JsSetterClassMember, JsSetterObjectMember,
    JsSyntaxKind, JsSyntaxNode, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};
use biome_rule_options::no_setter_return::NoSetterReturnOptions;

use crate::services::control_flow::AnyJsControlFlowRoot;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow returning a value from a setter
    ///
    /// While returning a value from a setter does not produce an error, the returned value is being ignored. Therefore, returning a value from a setter is either unnecessary or a possible error.
    ///
    /// Only returning without a value is allowed, as it’s a control flow statement.
    ///
    /// The rule also checks the `set` function of a property descriptor, the object that describes a property when it is passed to
    /// `Object.defineProperty()`, `Reflect.defineProperty()`, `Object.defineProperties()`, or `Object.create()`.
    /// An arrow function with an expression body, such as `(x) => (value = x)`, returns that expression, so it is reported as well.
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
    /// let value;
    /// Object.defineProperty(globalThis, "foo", {
    ///     set: (x) => (value = x),
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
    /// // the arrow function body is a block that doesn't return a value
    /// let value;
    /// Object.defineProperty(globalThis, "foo", {
    ///     set: (x) => {
    ///         value = x;
    ///     },
    /// });
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
    /// The member that defines a setter: a class or object setter, or the `set`
    /// member of a property descriptor.
    pub JsSetterMember =
        JsSetterClassMember
        | JsSetterObjectMember
        | JsMethodObjectMember
        | JsPropertyObjectMember
}

impl Rule for NoSetterReturn {
    type Query = Semantic<NoSetterReturnQuery>;
    type State = JsSetterMember;
    type Signals = Option<Self::State>;
    type Options = NoSetterReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        match ctx.query() {
            NoSetterReturnQuery::JsReturnStatement(ret) => {
                // Do not take arg-less returns into account
                ret.argument()?;
                let root = ret
                    .syntax()
                    .ancestors()
                    .skip(1)
                    .find_map(AnyJsControlFlowRoot::cast)?;
                match root {
                    AnyJsControlFlowRoot::JsSetterClassMember(setter) => Some(setter.into()),
                    AnyJsControlFlowRoot::JsSetterObjectMember(setter) => Some(setter.into()),
                    root => descriptor_setter(root.syntax(), model),
                }
            }
            NoSetterReturnQuery::JsArrowFunctionExpression(arrow) => {
                // An expression body is the value the arrow function returns.
                arrow.body().ok()?.as_any_js_expression()?;
                descriptor_setter(arrow.syntax(), model)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, setter: &Self::State) -> Option<RuleDiagnostic> {
        let range = match ctx.query() {
            NoSetterReturnQuery::JsReturnStatement(ret) => ret.range(),
            NoSetterReturnQuery::JsArrowFunctionExpression(arrow) => arrow.body().ok()?.range(),
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The setter should not "<Emphasis>"return"</Emphasis>" a value."
                },
            )
            .detail(setter.range(), "The setter is here:")
            .note("Returning a value from a setter is ignored."),
        )
    }
}

/// Returns the `set` member of a property descriptor if `function` is the
/// setter it defines, as in `{ set(x) {} }` or `{ set: (x) => {} }`.
fn descriptor_setter(function: &JsSyntaxNode, model: &SemanticModel) -> Option<JsSetterMember> {
    let (setter, name) = if let Some(method) = JsMethodObjectMember::cast_ref(function) {
        let name = method.name().ok()?;
        (JsSetterMember::from(method), name)
    } else {
        let property =
            parent_skipping_parentheses(function).and_then(JsPropertyObjectMember::cast)?;
        let name = property.name().ok()?;
        (JsSetterMember::from(property), name)
    };
    if name.name()?.text() != "set" {
        return None;
    }
    let descriptor = setter
        .parent::<JsObjectMemberList>()?
        .parent::<JsObjectExpression>()?;
    is_property_descriptor(&descriptor, model).then_some(setter)
}

/// Returns `true` if `object` is passed as a property descriptor to one of the
/// global methods that define properties.
fn is_property_descriptor(object: &JsObjectExpression, model: &SemanticModel) -> bool {
    // Object.defineProperty(target, key, descriptor)
    // Reflect.defineProperty(target, key, descriptor)
    let descriptor_methods = [("Object", "defineProperty"), ("Reflect", "defineProperty")];
    if global_method_call(object.syntax(), 2, &descriptor_methods, model).is_some() {
        return true;
    }
    // Object.defineProperties(target, { key: descriptor })
    // Object.create(prototype, { key: descriptor })
    let properties_methods = [("Object", "defineProperties"), ("Object", "create")];
    parent_skipping_parentheses(object.syntax())
        .and_then(JsPropertyObjectMember::cast)
        .and_then(|property| {
            property
                .parent::<JsObjectMemberList>()?
                .parent::<JsObjectExpression>()
        })
        .and_then(|properties| {
            global_method_call(properties.syntax(), 1, &properties_methods, model)
        })
        .is_some()
}

/// Returns the call that receives `argument` at position `index`, if the callee
/// is one of `methods`, given as `(object, method)` pairs, and the object is the
/// global one rather than a local variable with the same name.
fn global_method_call(
    argument: &JsSyntaxNode,
    index: usize,
    methods: &[(&str, &str)],
    model: &SemanticModel,
) -> Option<JsCallExpression> {
    let arguments = parent_skipping_parentheses(argument).and_then(JsCallArgumentList::cast)?;
    let argument_at_index = arguments.iter().nth(index)?.ok()?;
    let argument_at_index = argument_at_index.as_any_js_expression()?.clone();
    if argument_at_index.omit_parentheses().syntax() != argument {
        return None;
    }
    let call = arguments
        .parent::<JsCallArguments>()?
        .parent::<JsCallExpression>()?;
    let callee = call.callee().ok()?.omit_parentheses();
    let callee = AnyJsMemberExpression::cast(callee.into_syntax())?;
    let method_name = callee.member_name()?;
    let object = callee.object().ok()?.omit_parentheses();
    let (reference, object_name) =
        global_identifier(&object.as_any_global_identifier_expression()?)?;
    let is_method = methods
        .iter()
        .any(|&(object, method)| object_name.text() == object && method_name.text() == method);
    (is_method && model.binding(&reference).is_none()).then_some(call)
}

fn parent_skipping_parentheses(node: &JsSyntaxNode) -> Option<JsSyntaxNode> {
    node.ancestors()
        .skip(1)
        .find(|ancestor| ancestor.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION)
}
