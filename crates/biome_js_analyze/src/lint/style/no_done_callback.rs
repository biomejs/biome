use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsExpression, JsCallExpression, JsParameters,
    JsTemplateExpression,
};
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Disallow using a callback in asynchronous tests and hooks.
    ///
    /// This rule checks the function parameter of hooks and tests for use of the `done` argument, suggesting you return a promise instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// beforeEach((done) => {
    ///     // ...
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test('tets-name', (done) => {
    ///     // ...
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// beforeEach(async () => {
    ///     // ...
    /// });
    /// ```
    ///
    /// ```js
    /// test('test-name', () => {
    ///     expect(myFunction()).toBeTruthy();
    /// });
    /// ```
    ///
    pub NoDoneCallback {
        version: "1.6.1",
        name: "noDoneCallback",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintJest("no-done-callback")],
    }
}

impl Rule for NoDoneCallback {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let callee = node.callee().ok()?;

        let is_test_each = callee
            .get_callee_member_name()
            .is_some_and(|m| m.text_trimmed() == "each");

        if is_test_each && !JsTemplateExpression::can_cast(callee.syntax().kind()) {
            return None;
        }

        let arguments = &node.arguments().ok()?;
        let callee_name = callee.get_callee_object_name()?;

        let argument_index = match callee_name.text_trimmed() {
            "after" | "afterAll" | "afterEach" | "before" | "beforeAll" | "beforeEach" => 0,
            "it" | "test" => 1, // for test.each() and test() we want the second argument
            _ => return None,
        };
        let argument = arguments
            .get_arguments_by_index([argument_index])
            .first()?
            .clone();

        let callback = argument?;
        let callback = callback.as_any_js_expression()?;

        match callback {
            AnyJsExpression::JsArrowFunctionExpression(arrow_function) => {
                let parameter = arrow_function.parameters().ok()?;
                match parameter {
                    AnyJsArrowFunctionParameters::AnyJsBinding(binding) => {
                        let param = binding.as_js_identifier_binding()?;
                        let text_range = param.name_token().ok()?;
                        let text_range = text_range.text_trimmed_range();
                        return Some(text_range);
                    }
                    AnyJsArrowFunctionParameters::JsParameters(js_parameters) => {
                        return analyze_js_parameters(&js_parameters, is_test_each)
                    }
                }
            }
            AnyJsExpression::JsFunctionExpression(js_function) => {
                let js_parameters = js_function.parameters().ok()?;
                return analyze_js_parameters(&js_parameters, is_test_each);
            }
            _ => {}
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Disallow using a callback in asynchronous tests and hooks."
                },
            )
            .note(markup! {
                "Return a Promise instead of relying on callback parameter."
            }),
        )
    }
}

fn analyze_js_parameters(js_parameters: &JsParameters, is_test_each: bool) -> Option<TextRange> {
    let items = js_parameters.items();

    let param = match is_test_each {
        true => items.into_iter().nth(1)?,
        false => items.into_iter().next()?,
    };

    let param = param.ok()?;
    let formal_parameter = param.as_any_js_formal_parameter()?;
    let formal_parameter = formal_parameter.as_js_formal_parameter()?;

    let binding = formal_parameter.binding().ok()?;
    let binding = binding.as_any_js_binding()?;
    let text_range = get_js_binding_range(binding)?;

    Some(text_range)
}

fn get_js_binding_range(binding: &AnyJsBinding) -> Option<TextRange> {
    let param = binding.as_js_identifier_binding()?;
    let text_range = param.name_token().ok()?;
    let text_range = text_range.text_trimmed_range();
    Some(text_range)
}
