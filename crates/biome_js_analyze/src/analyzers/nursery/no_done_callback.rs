use biome_analyze::{
    context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsExpression, JsCallExpression, JsParameters,
    JsTemplateExpression,
};
use biome_rowan::{AstNode, TextRange};

declare_rule! {
    /// This rule checks the function parameter of hooks & tests for use of the done argument, suggesting you return a promise instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// beforeEach(done => {
    ///     // ...
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test('myFunction()', done => {
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
    /// test('myFunction()', () => {
    ///     expect(myFunction()).toBeTruthy();
    /// });
    /// ```
    ///
    pub NoDoneCallback {
        version: "next",
        name: "noDoneCallback",
        recommended: true,
        source: RuleSource::EslintJest("no-done-callback"),
        source_kind: RuleSourceKind::Inspired,
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

        let mut is_test_each = false;
        let callee_member = callee.get_callee_member_name();
        if let Some(member_name) = callee_member {
            if member_name.text_trimmed() == "each" {
                is_test_each = true;
            }
        }

        if is_test_each && !JsTemplateExpression::can_cast(callee.syntax().kind()) {
            return None;
        }

        let arguments = &node.arguments().ok()?;
        let callee_name = callee.get_callee_object_name()?;

        let argument = match callee_name.text_trimmed() {
            "after" | "afterAll" | "afterEach" | "before" | "beforeAll" | "beforeEach" => {
                let argument_by_index = arguments.get_arguments_by_index([0]);
                let top_arg = argument_by_index.first()?;
                top_arg.clone()
            }
            "it" | "test" => {
                // for test.each() and test() we want the second argument
                let argument_by_index = arguments.get_arguments_by_index([1]);
                let top_arg = argument_by_index.first()?;
                top_arg.clone()
            }
            _ => None,
        };

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
                        if let Some(text_range) =
                            analyze_js_parameters(&js_parameters, is_test_each)
                        {
                            return Some(text_range);
                        }
                    }
                }
            }
            AnyJsExpression::JsFunctionExpression(js_function) => {
                let js_parameters = js_function.parameters().ok()?;
                if let Some(text_range) = analyze_js_parameters(&js_parameters, is_test_each) {
                    return Some(text_range);
                }
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
