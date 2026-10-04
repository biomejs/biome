use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsBinding, AnyJsCallArgument, AnyJsClass, AnyJsExpression, AnyJsFunction,
    AnyJsMemberExpression, AnyJsRoot, JsArrowFunctionExpression, JsAssignmentExpression,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsComputedMemberAssignment,
    JsConstructorClassMember, JsConstructorParameters, JsDirectiveList, JsFormalParameter,
    JsFunctionBody, JsGetterClassMember, JsGetterObjectMember, JsIdentifierAssignment,
    JsIdentifierBinding, JsInitializerClause, JsLanguage, JsMethodClassMember,
    JsMethodObjectMember, JsModule, JsParameters, JsPropertyClassMember, JsScript,
    JsSetterClassMember, JsSetterObjectMember, JsStatementList,
    JsStaticInitializationBlockClassMember, JsStaticMemberAssignment, JsSyntaxKind, JsSyntaxNode,
    JsThisExpression, JsUnaryOperator, TsTypeAnnotation,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet, declare_node_union};
use biome_rule_options::no_invalid_this::NoInvalidThisOptions;

declare_lint_rule! {
    /// Disallow `this` in places where it is `undefined`.
    ///
    /// Inside a function, the value of `this` depends on how the function is called. When the
    /// function is called as a method, as in `obj.foo()`, `this` is `obj`. When it's called on its
    /// own, as in `foo()`, `this` is `undefined` in modules and classes. At the top level of a
    /// module, `this` is always `undefined`. Reading a property of `undefined` throws an error, so
    /// using `this` in these places is usually a mistake.
    ///
    /// This rule reports `this` at the top level of a module and in functions that are unlikely to
    /// be called as a method. It allows `this` in:
    ///
    /// - class members and object methods;
    /// - functions assigned to an object property, such as `obj.foo = function () {}`;
    /// - functions whose name starts with an uppercase letter, and functions without a name that
    ///   are assigned to a variable whose name starts with an uppercase letter, because they're
    ///   likely constructors called with `new`;
    /// - functions called with `.call()`, `.apply()`, or `.bind()` and an object to use as `this`,
    ///   such as `(function () {}).call(obj)`;
    /// - functions passed to `Reflect.apply()`, `Array.from()`, or array methods such as
    ///   `.forEach()` together with an object to use as `this`;
    /// - functions that declare the type of `this` with a TypeScript `this` parameter, such as
    ///   `function foo(this: Obj) {}`;
    /// - functions in CommonJS files (`.cjs`) that don't start with `"use strict"`, because `this`
    ///   is the global object there.
    ///
    /// Arrow functions don't have their own `this`. They use the `this` of the code around them,
    /// so the rule checks them against that code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// this.a = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     this.a = 0;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo(function () {
    ///     this.a = 0;
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = () => {
    ///     this.a = 0;
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function Foo() {
    ///     this.a = 0;
    /// }
    ///
    /// const Bar = function () {
    ///     this.a = 0;
    /// };
    ///
    /// const obj = {
    ///     foo() {
    ///         this.a = 0;
    ///     },
    ///     bar: function () {
    ///         this.a = 0;
    ///     },
    /// };
    ///
    /// obj.baz = function () {
    ///     this.a = 0;
    /// };
    ///
    /// class Baz {
    ///     a = this;
    ///
    ///     constructor() {
    ///         this.a = 0;
    ///     }
    /// }
    ///
    /// list.forEach(function () {
    ///     this.a = 0;
    /// }, obj);
    ///
    /// (function () {
    ///     this.a = 0;
    /// }).call(obj);
    ///
    /// function qux(this: Obj) {
    ///     this.a = 0;
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - [`noThisOutsideOfClass`](https://biomejs.dev/linter/rules/no-this-outside-of-class/)
    ///
    pub NoInvalidThis {
        version: "next",
        name: "noInvalidThis",
        language: "js",
        sources: &[RuleSource::Eslint("no-invalid-this").same()],
        recommended: false,
    }
}

impl Rule for NoInvalidThis {
    type Query = Ast<JsThisExpression>;
    type State = InvalidThisContext;
    type Signals = Option<Self::State>;
    type Options = NoInvalidThisOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let this = ctx.query();
        let mut child = this.syntax().clone();
        for node in this.syntax().ancestors().skip(1) {
            if THIS_BINDING_MEMBERS.matches(node.kind()) {
                // Computed keys and decorators are evaluated in the surrounding context.
                if MEMBER_EXECUTION_CONTEXTS.matches(child.kind()) {
                    return None;
                }
            } else if let Some(function) = AnyJsFunction::cast_ref(&node) {
                // Arrow functions inherit `this` from the surrounding context.
                if function.as_js_arrow_function_expression().is_none() {
                    return (is_strict_mode(&ctx.root(), &function)
                        && is_default_this_binding(&function))
                    .then_some(InvalidThisContext::Function);
                }
            } else if JsModule::can_cast(node.kind()) {
                return Some(InvalidThisContext::Module);
            }
            child = node;
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().range();
        let diagnostic = match state {
            InvalidThisContext::Module => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Unexpected "<Emphasis>"this"</Emphasis>" at the top level of a module."
                },
            )
            .note(markup! {
                <Emphasis>"this"</Emphasis>" is always "<Emphasis>"undefined"</Emphasis>" at the top level of a module."
            })
            .note(markup! {
                "Use "<Emphasis>"globalThis"</Emphasis>" to refer to the global object."
            }),
            InvalidThisContext::Function => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Unexpected "<Emphasis>"this"</Emphasis>" in a function that isn't a method or a constructor."
                },
            )
            .note(markup! {
                "When a function is called on its own instead of as a method, "<Emphasis>"this"</Emphasis>" is "<Emphasis>"undefined"</Emphasis>", and reading its properties throws an error."
            })
            .note(markup! {
                "Pass the value as a parameter instead, or turn the function into a method. In TypeScript, if callers provide "<Emphasis>"this"</Emphasis>", declare its type with a "<Emphasis>"this"</Emphasis>" parameter."
            }),
        };
        Some(diagnostic)
    }
}

/// The context in which `this` is `undefined`.
pub enum InvalidThisContext {
    /// The top level of a module.
    Module,
    /// A function that isn't called with a `this` value.
    Function,
}

/// Class and object members that have their own `this`.
const THIS_BINDING_MEMBERS: SyntaxKindSet<JsLanguage> = JsMethodClassMember::KIND_SET
    .union(JsConstructorClassMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET)
    .union(JsPropertyClassMember::KIND_SET)
    .union(JsStaticInitializationBlockClassMember::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// Children of [THIS_BINDING_MEMBERS] that are evaluated with the member's `this`.
const MEMBER_EXECUTION_CONTEXTS: SyntaxKindSet<JsLanguage> = JsFunctionBody::KIND_SET
    .union(JsParameters::KIND_SET)
    .union(JsConstructorParameters::KIND_SET)
    .union(JsFormalParameter::KIND_SET)
    .union(JsInitializerClause::KIND_SET)
    .union(JsStatementList::KIND_SET);

/// Functions and members whose bodies can contain a `return` statement.
const FUNCTION_LIKE: SyntaxKindSet<JsLanguage> =
    AnyJsFunction::KIND_SET.union(THIS_BINDING_MEMBERS);

/// Array methods that accept a `thisArg` after the callback.
const ARRAY_METHODS_WITH_THIS_ARG: &[&str] = &[
    "every",
    "filter",
    "find",
    "findIndex",
    "findLast",
    "findLastIndex",
    "flatMap",
    "forEach",
    "map",
    "some",
];

/// Returns whether `function` is strict mode code.
fn is_strict_mode(root: &AnyJsRoot, function: &AnyJsFunction) -> bool {
    if root.as_js_module().is_some() {
        return true;
    }

    let has_own_directive = function.body().is_ok_and(|body| {
        body.as_js_function_body()
            .is_some_and(|body| has_use_strict_directive(&body.directives()))
    });

    has_own_directive
        || function.syntax().ancestors().skip(1).any(|node| {
            AnyJsClass::can_cast(node.kind())
                || JsFunctionBody::cast_ref(&node)
                    .is_some_and(|body| has_use_strict_directive(&body.directives()))
                || JsScript::cast_ref(&node)
                    .is_some_and(|script| has_use_strict_directive(&script.directives()))
        })
}

fn has_use_strict_directive(directives: &JsDirectiveList) -> bool {
    directives.iter().any(|directive| {
        directive
            .inner_string_text()
            .is_ok_and(|text| text == "use strict")
    })
}

/// Returns whether `function` is likely called without a `this` value.
fn is_default_this_binding(function: &AnyJsFunction) -> bool {
    !(has_this_parameter(function)
        || has_uppercase_name(function)
        || receives_this(function).unwrap_or(false))
}

/// Returns whether `function` ends up in a place that provides `this`, such as an object property
/// or the object of a `.call()`.
///
/// Logical and conditional expressions, and the return values of immediately called functions,
/// are followed to find where the function ends up. Returns `None` for function declarations,
/// which can't be passed anywhere.
fn receives_this(function: &AnyJsFunction) -> Option<bool> {
    let is_anonymous = function_name(function).is_none();
    let mut current = AnyJsExpression::cast_ref(function.syntax())?.outer_expression()?;
    for parent in function.syntax().ancestors().skip(1) {
        // Skip the nodes between `current` and its parent, such as parentheses, or the body of the
        // function whose call returns `current`.
        if current.syntax().parent().as_ref() != Some(&parent) {
            continue;
        }

        match parent.kind() {
            JsSyntaxKind::JS_LOGICAL_EXPRESSION | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                current = AnyJsExpression::unwrap_cast(parent).outer_expression()?;
            }
            // e.g. `obj.foo = (function () { return function () {}; })();`
            JsSyntaxKind::JS_RETURN_STATEMENT => {
                let returning_function = parent
                    .ancestors()
                    .find(|node| FUNCTION_LIKE.matches(node.kind()))
                    .and_then(AnyJsExpression::cast)?;
                let call = call_with_callee(&returning_function)?;
                current = AnyJsExpression::from(call).outer_expression()?;
            }
            // e.g. `obj.foo = (() => function () {})();`
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                let arrow = JsArrowFunctionExpression::unwrap_cast(parent);
                if arrow.body().ok()?.syntax() != current.syntax() {
                    return Some(false);
                }
                let call = call_with_callee(&arrow.into())?;
                current = AnyJsExpression::from(call).outer_expression()?;
            }
            // Computed keys are wrapped in their own node, so `current` is the property value.
            JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => return Some(true),
            JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                let initializer = JsInitializerClause::unwrap_cast(parent);
                return Some(is_initialized_with_this(&initializer, is_anonymous));
            }
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let assignment = JsAssignmentExpression::unwrap_cast(parent);
                let target = AnyJsFunctionTarget::cast(assignment.left().ok()?.into_syntax())?;
                return Some(is_target_with_this(&target, is_anonymous));
            }
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                let member = AnyJsMemberExpression::unwrap_cast(parent);
                return Some(is_bound_to_this_arg(&member, &current));
            }
            JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                let arguments = JsCallArgumentList::unwrap_cast(parent);
                return Some(is_callback_with_this_arg(&arguments, &current));
            }
            _ => return Some(false),
        }
    }

    Some(false)
}

fn has_this_parameter(function: &AnyJsFunction) -> bool {
    function
        .parenthesized_parameters()
        .is_some_and(|parameters| {
            parameters
                .items()
                .iter()
                .flatten()
                .any(|parameter| parameter.as_ts_this_parameter().is_some())
        })
}

/// Returns whether `function` is named like a constructor, such as `function Foo() {}`.
fn has_uppercase_name(function: &AnyJsFunction) -> bool {
    function_name(function)
        .and_then(|binding| binding.as_js_identifier_binding()?.name_token().ok())
        .is_some_and(|name| starts_with_uppercase(name.text_trimmed()))
}

/// Returns the function's own name, unlike [AnyJsFunction::binding], which also returns the
/// variable that a function expression is assigned to.
fn function_name(function: &AnyJsFunction) -> Option<AnyJsBinding> {
    match function {
        AnyJsFunction::JsFunctionDeclaration(declaration) => declaration.id().ok(),
        AnyJsFunction::JsFunctionExportDefaultDeclaration(declaration) => declaration.id(),
        AnyJsFunction::JsFunctionExpression(expression) => expression.id(),
        AnyJsFunction::JsArrowFunctionExpression(_) => None,
    }
}

fn starts_with_uppercase(name: &str) -> bool {
    name.chars().next().is_some_and(char::is_uppercase)
}

/// Returns whether `initializer` gives the function a `this` value, such as
/// `class A { foo = function () {} }`, `const Foo = function () {}`, or
/// `[obj.foo = function () {}] = list`.
fn is_initialized_with_this(initializer: &JsInitializerClause, is_anonymous: bool) -> bool {
    if initializer.parent::<JsPropertyClassMember>().is_some() {
        return true;
    }

    // The initialized variable or assignment target precedes the initializer, possibly separated
    // by a type annotation.
    std::iter::successors(
        initializer.syntax().prev_sibling(),
        JsSyntaxNode::prev_sibling,
    )
    .find(|node| !TsTypeAnnotation::can_cast(node.kind()))
    .and_then(AnyJsFunctionTarget::cast)
    .is_some_and(|target| is_target_with_this(&target, is_anonymous))
}

/// Returns whether assigning a function to `target` gives it a `this` value: `target` is an
/// object property, or an uppercase name and the function is anonymous.
fn is_target_with_this(target: &AnyJsFunctionTarget, is_anonymous: bool) -> bool {
    let name = match target {
        AnyJsFunctionTarget::JsStaticMemberAssignment(_)
        | AnyJsFunctionTarget::JsComputedMemberAssignment(_) => return true,
        AnyJsFunctionTarget::JsIdentifierBinding(binding) => binding.name_token(),
        AnyJsFunctionTarget::JsIdentifierAssignment(assignment) => assignment.name_token(),
    };
    is_anonymous && name.is_ok_and(|name| starts_with_uppercase(name.text_trimmed()))
}

/// Returns whether `function` is bound by `member`, such as `function () {}.bind(obj)` or
/// `(function () {}).call(obj)`.
fn is_bound_to_this_arg(member: &AnyJsMemberExpression, function: &AnyJsExpression) -> bool {
    if !member.object().is_ok_and(|object| &object == function) {
        return false;
    }
    if !member
        .member_name()
        .is_some_and(|name| matches!(name.text(), "bind" | "call" | "apply"))
    {
        return false;
    }

    call_with_callee(&member.clone().into())
        .and_then(|call| call.arguments().ok())
        .and_then(|arguments| arguments.args().first())
        .is_some_and(|this_arg| this_arg.is_ok_and(|this_arg| !is_nullish(&this_arg)))
}

/// Returns whether `function` is a callback whose `this` value is passed as another argument, such
/// as `list.forEach(function () {}, obj)`.
fn is_callback_with_this_arg(arguments: &JsCallArgumentList, function: &AnyJsExpression) -> bool {
    let Some(call) = arguments
        .parent::<JsCallArguments>()
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
    else {
        return false;
    };
    let Some(callee) = call
        .callee()
        .ok()
        .and_then(|callee| AnyJsMemberExpression::cast(callee.omit_parentheses().into_syntax()))
    else {
        return false;
    };
    let Some(method) = callee.member_name() else {
        return false;
    };
    let object = callee
        .object()
        .ok()
        .and_then(|object| object.as_js_reference_identifier()?.value_token().ok());
    let object = object.as_ref().map(|object| object.text_trimmed());

    let (callback_index, this_arg_index, argument_count) = match method.text() {
        "apply" if object == Some("Reflect") => (0, 1, 3),
        "from" if object.is_some_and(|object| object.ends_with("Array")) => (1, 2, 3),
        "fromAsync" if object == Some("Array") => (1, 2, 3),
        name if ARRAY_METHODS_WITH_THIS_ARG.binary_search(&name).is_ok() => (0, 1, 2),
        _ => return false,
    };

    arguments.len() == argument_count
        && arguments
            .iter()
            .nth(callback_index)
            .is_some_and(|callback| {
                callback.is_ok_and(|callback| callback.syntax() == function.syntax())
            })
        && arguments
            .iter()
            .nth(this_arg_index)
            .is_some_and(|this_arg| this_arg.is_ok_and(|this_arg| !is_nullish(&this_arg)))
}

/// Returns whether `argument` is `null`, `undefined`, or a `void` expression.
fn is_nullish(argument: &AnyJsCallArgument) -> bool {
    let Some(expression) = argument.as_any_js_expression() else {
        return false;
    };
    let expression = expression.clone().omit_parentheses();
    expression
        .as_static_value()
        .is_some_and(|value| value.is_null_or_undefined())
        || expression.as_js_unary_expression().is_some_and(|unary| {
            unary
                .operator()
                .is_ok_and(|operator| operator == JsUnaryOperator::Void)
        })
}

/// Returns the call expression that directly calls `expression`, such as `(function () {})()`.
fn call_with_callee(expression: &AnyJsExpression) -> Option<JsCallExpression> {
    let callee = expression.outer_expression()?;
    let call = callee.parent::<JsCallExpression>()?;
    (call.callee().ok()? == callee).then_some(call)
}

declare_node_union! {
    /// Assignment targets and variables that a function can be assigned to.
    pub AnyJsFunctionTarget =
        JsStaticMemberAssignment
        | JsComputedMemberAssignment
        | JsIdentifierBinding
        | JsIdentifierAssignment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_methods_with_this_arg_are_sorted() {
        for items in ARRAY_METHODS_WITH_THIS_ARG.windows(2) {
            assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
        }
    }
}
