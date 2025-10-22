use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList};

declare_lint_rule! {
    /// Enforce valid `describe()` callback.
    ///
    /// Using an improper `describe()` callback function can lead to unexpected test errors.
    /// This rule validates that describe callbacks are proper synchronous functions without parameters.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test.describe('suite', async () => {
    ///     test('one', async ({ page }) => {});
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.describe('suite', (done) => {
    ///     test('one', async ({ page }) => {});
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// test.describe('suite', () => {
    ///     test('one', async ({ page }) => {});
    ///     test('two', async ({ page }) => {});
    /// });
    /// ```
    ///
    /// ```js
    /// describe('suite', function() {
    ///     test('one', async ({ page }) => {});
    /// });
    /// ```
    ///
    pub NoPlaywrightValidDescribeCallback {
        version: "next",
        name: "noPlaywrightValidDescribeCallback",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("valid-describe-callback").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

pub enum InvalidReason {
    Async,
    HasParameters,
    MissingCallback,
    NotFunction,
}

impl Rule for NoPlaywrightValidDescribeCallback {
    type Query = Ast<JsCallExpression>;
    type State = InvalidReason;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a describe call
        let is_describe = match callee {
            AnyJsExpression::JsIdentifierExpression(id) => {
                let name = id.name().ok()?;
                let token = name.value_token().ok()?;
                token.text_trimmed() == "describe"
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let member_name = member.member().ok()?;
                let member_text = member_name.as_js_name()?.value_token().ok()?;

                if member_text.text_trimmed() == "describe" {
                    // Check if object is "test"
                    let object = member.object().ok()?;
                    if let AnyJsExpression::JsIdentifierExpression(id) = object {
                        let name = id.name().ok()?;
                        let token = name.value_token().ok()?;
                        token.text_trimmed() == "test"
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            _ => false,
        };

        if !is_describe {
            return None;
        }

        // Get the callback argument (should be the second argument for describe calls)
        let args = call_expr.arguments().ok()?;
        let [_, callback_arg] = args.get_arguments_by_index([0, 1]);

        // Check if callback is missing
        let Some(callback_arg) = callback_arg else {
            return Some(InvalidReason::MissingCallback);
        };

        let Some(callback_expr) = callback_arg.as_any_js_expression() else {
            return Some(InvalidReason::NotFunction);
        };

        // Check if it's a function
        match callback_expr {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                // Check if async
                if arrow.async_token().is_some() {
                    return Some(InvalidReason::Async);
                }

                // Check if has parameters
                if let Ok(params) = arrow.parameters() {
                    let has_params = match params {
                        biome_js_syntax::AnyJsArrowFunctionParameters::AnyJsBinding(_) => true,
                        biome_js_syntax::AnyJsArrowFunctionParameters::JsParameters(p) => {
                            p.items().len() > 0
                        }
                    };
                    if has_params {
                        return Some(InvalidReason::HasParameters);
                    }
                }
            }
            AnyJsExpression::JsFunctionExpression(func) => {
                // Check if async
                if func.async_token().is_some() {
                    return Some(InvalidReason::Async);
                }

                // Check if has parameters
                if let Ok(params) = func.parameters()
                    && !params.items().is_empty()
                {
                    return Some(InvalidReason::HasParameters);
                }
            }
            _ => return Some(InvalidReason::NotFunction),
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let (message, note, explanation) = match state {
            InvalidReason::Async => (
                markup! { "Describe callback should not be "<Emphasis>"async"</Emphasis>"." },
                markup! { "Describe blocks are meant to organize tests, not contain asynchronous logic. Async operations should be placed within individual test callbacks." },
                markup! { "Remove the "<Emphasis>"async"</Emphasis>" keyword from the describe callback." },
            ),
            InvalidReason::HasParameters => (
                markup! { "Describe callback should not have parameters." },
                markup! { "Describe callbacks are invoked without arguments by the test framework." },
                markup! { "Remove parameters from the describe callback." },
            ),
            InvalidReason::MissingCallback => (
                markup! { "Describe requires a callback function." },
                markup! { "The second argument to describe must be a function that contains the test definitions." },
                markup! { "Add a callback function as the second argument to describe." },
            ),
            InvalidReason::NotFunction => (
                markup! { "Describe callback must be a function." },
                markup! { "The second argument to describe must be a function, not a "<Emphasis>"string"</Emphasis>", "<Emphasis>"number"</Emphasis>", "<Emphasis>"object"</Emphasis>", or other type." },
                markup! { "Replace the callback with a function expression or arrow function." },
            ),
        };

        Some(
            RuleDiagnostic::new(rule_category!(), node.range(), message)
                .note(note)
                .note(explanation),
        )
    }
}
