use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsFunction, AnyJsObjectMember, JsParameters, JsThisExpression,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_this_outside_of_class::NoThisOutsideOfClassOptions;

declare_lint_rule! {
    /// Disallow `this` outside of classes.
    ///
    /// `this` can make its value difficult to understand. The rule allows `this` in class members and
    /// in TypeScript functions with an explicit `this` parameter.
    ///
    /// An arrow function uses `this` from the code around it. Therefore, `this` is allowed in an
    /// arrow function inside a class member or a TypeScript function with an explicit `this`
    /// parameter.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function Person(name) {
    ///     this.name = name;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const getName = function () {
    ///     return this.name;
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (function () {
    ///     this.initialize();
    /// })();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const person = {
    ///     getName() {
    ///         return this.name;
    ///     },
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class Person {
    ///     constructor(name) {
    ///         this.name = name;
    ///     }
    ///
    ///     getName = () => this.name;
    /// }
    /// ```
    ///
    /// ```ts
    /// function getName(this: Person) {
    ///     return this.name;
    /// }
    /// ```
    ///
    pub NoThisOutsideOfClass {
        version: "next",
        name: "noThisOutsideOfClass",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-this-outside-of-class").same()],
        // not recommended because it can be a bit broad. for example, every vue component that uses the options API uses `this` outside of classes frequently. so its not the best fit for all codebases.
        recommended: false,
    }
}

impl Rule for NoThisOutsideOfClass {
    type Query = Ast<JsThisExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoThisOutsideOfClassOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        (!has_allowed_binding(ctx.query())).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Do not use "<Emphasis>"this"</Emphasis>" outside of a class."
                },
            )
            .note(markup! {
                <Emphasis>"this"</Emphasis>" depends on how a function is called and may refer to an unexpected object."
            })
            .note(markup! {
                "Use a class member, or declare an explicit "<Emphasis>"this"</Emphasis>" parameter in a TypeScript function."
            }),
        )
    }
}

/// Describes how a syntax construct affects the search for an allowed `this`.
enum ThisBinding {
    /// Continue searching the surrounding code.
    Inherit,
    /// Stop searching and allow `this`.
    Allowed,
    /// Stop searching and reject `this`.
    Disallowed,
}

/// Returns whether `expression` resolves to a class or explicitly declared `this` binding.
///
/// Arrow functions and computed member names inherit `this` from their surrounding context, so
/// they do not end the search. A non-arrow function stops the search and requires an explicit
/// TypeScript `this` parameter. Class member execution contexts allow `this`; object accessors
/// reject it, while object methods require an explicit `this` parameter.
fn has_allowed_binding(expression: &JsThisExpression) -> bool {
    for node in expression.syntax().ancestors().skip(1) {
        let binding = if let Some(function) = AnyJsFunction::cast_ref(&node) {
            function_binding(&function)
        } else if let Some(member) = AnyJsClassMember::cast_ref(&node) {
            class_member_binding(&member, expression)
        } else if let Some(member) = AnyJsObjectMember::cast_ref(&node) {
            object_member_binding(&member, expression)
        } else {
            ThisBinding::Inherit
        };

        match binding {
            ThisBinding::Inherit => {}
            ThisBinding::Allowed => return true,
            ThisBinding::Disallowed => return false,
        }
    }

    false
}

fn function_binding(function: &AnyJsFunction) -> ThisBinding {
    if function.as_js_arrow_function_expression().is_some() {
        return ThisBinding::Inherit;
    }

    if function
        .parameters()
        .ok()
        .and_then(|parameters| parameters.as_js_parameters().cloned())
        .is_some_and(|parameters| has_this_parameter(&parameters))
    {
        ThisBinding::Allowed
    } else {
        ThisBinding::Disallowed
    }
}

fn class_member_binding(
    member: &AnyJsClassMember,
    expression: &JsThisExpression,
) -> ThisBinding {
    let expression_range = expression.range();
    let is_execution_context = match member {
        AnyJsClassMember::JsMethodClassMember(method) => {
            method
                .parameters()
                .is_ok_and(|parameters| parameters.range().contains_range(expression_range))
                || method
                    .body()
                    .is_ok_and(|body| body.range().contains_range(expression_range))
        }
        AnyJsClassMember::JsConstructorClassMember(constructor) => {
            constructor
                .parameters()
                .is_ok_and(|parameters| parameters.range().contains_range(expression_range))
                || constructor
                    .body()
                    .is_ok_and(|body| body.range().contains_range(expression_range))
        }
        AnyJsClassMember::JsGetterClassMember(getter) => getter
            .body()
            .is_ok_and(|body| body.range().contains_range(expression_range)),
        AnyJsClassMember::JsSetterClassMember(setter) => {
            setter
                .parameter()
                .is_ok_and(|parameter| parameter.range().contains_range(expression_range))
                || setter
                    .body()
                    .is_ok_and(|body| body.range().contains_range(expression_range))
        }
        AnyJsClassMember::JsPropertyClassMember(property) => property
            .value()
            .is_some_and(|value| value.range().contains_range(expression_range)),
        AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => true,
        _ => false,
    };

    if is_execution_context {
        ThisBinding::Allowed
    } else {
        ThisBinding::Inherit
    }
}

fn object_member_binding(
    member: &AnyJsObjectMember,
    expression: &JsThisExpression,
) -> ThisBinding {
    let expression_range = expression.range();

    match member {
        AnyJsObjectMember::JsMethodObjectMember(method)
            if method
                .parameters()
                .is_ok_and(|parameters| parameters.range().contains_range(expression_range))
                || method
                    .body()
                    .is_ok_and(|body| body.range().contains_range(expression_range)) =>
        {
            if method
                .parameters()
                .is_ok_and(|parameters| has_this_parameter(&parameters))
            {
                ThisBinding::Allowed
            } else {
                ThisBinding::Disallowed
            }
        }
        AnyJsObjectMember::JsGetterObjectMember(getter)
            if getter
                .body()
                .is_ok_and(|body| body.range().contains_range(expression_range)) =>
        {
            ThisBinding::Disallowed
        }
        AnyJsObjectMember::JsSetterObjectMember(setter)
            if setter
                .parameter()
                .is_ok_and(|parameter| parameter.range().contains_range(expression_range))
                || setter
                    .body()
                    .is_ok_and(|body| body.range().contains_range(expression_range)) =>
        {
            ThisBinding::Disallowed
        }
        _ => ThisBinding::Inherit,
    }
}

fn has_this_parameter(parameters: &JsParameters) -> bool {
    parameters
        .items()
        .iter()
        .flatten()
        .any(|parameter| parameter.as_ts_this_parameter().is_some())
}
