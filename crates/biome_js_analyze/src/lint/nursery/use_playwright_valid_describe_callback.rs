use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_playwright_valid_describe_callback::UsePlaywrightValidDescribeCallbackOptions;

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
    pub UsePlaywrightValidDescribeCallback {
        version: "next",
        name: "usePlaywrightValidDescribeCallback",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("valid-describe-callback").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
    }
}

/// Reasons why a describe callback is invalid.
pub enum InvalidReason {
    /// The callback is async, which is not allowed for describe blocks.
    Async,
    /// The callback has parameters, which describe callbacks should not have.
    HasParameters,
    /// No callback function was provided.
    MissingCallback,
    /// The provided argument is not a function.
    NotFunction,
}

/// Checks if the callee is a Playwright describe call.
/// Matches: describe, test.describe, test.describe.only, test.describe.skip,
/// test.describe.parallel, test.describe.serial, test.describe.parallel.only, etc.
fn is_playwright_describe_call(callee: &AnyJsExpression) -> Option<bool> {
    // Collect member names in "outside-in" order
    let names = collect_member_names(callee)?;

    // Convert to &str slice for pattern matching
    let names_ref: Vec<&str> = names.iter().map(String::as_str).collect();

    match names_ref.as_slice() {
        // describe()
        ["describe"] => Some(true),
        // test.describe()
        ["test", "describe"] => Some(true),
        // test.describe.only() / test.describe.skip()
        ["test", "describe", modifier] if is_describe_modifier(modifier) => Some(true),
        // test.describe.parallel() / test.describe.serial()
        ["test", "describe", mode] if is_describe_mode(mode) => Some(true),
        // test.describe.parallel.only() / test.describe.serial.only()
        ["test", "describe", mode, modifier]
            if is_describe_mode(mode) && is_describe_modifier(modifier) =>
        {
            Some(true)
        }
        _ => Some(false),
    }
}

fn collect_member_names(expr: &AnyJsExpression) -> Option<Vec<String>> {
    let mut names = Vec::new();
    collect_member_names_rec(expr, &mut names)?;
    Some(names)
}

fn collect_member_names_rec(expr: &AnyJsExpression, names: &mut Vec<String>) -> Option<()> {
    match expr {
        AnyJsExpression::JsIdentifierExpression(id) => {
            let name = id.name().ok()?;
            let token = name.value_token().ok()?;
            names.push(token.text_trimmed().to_string());
            Some(())
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let obj = member.object().ok()?;
            collect_member_names_rec(&obj, names)?;
            let m = member.member().ok()?;
            let n = m.as_js_name()?;
            let t = n.value_token().ok()?;
            names.push(t.text_trimmed().to_string());
            Some(())
        }
        _ => None,
    }
}

fn is_describe_modifier(s: &str) -> bool {
    matches!(s, "only" | "skip")
}

fn is_describe_mode(s: &str) -> bool {
    matches!(s, "parallel" | "serial")
}

impl Rule for UsePlaywrightValidDescribeCallback {
    type Query = Ast<JsCallExpression>;
    type State = InvalidReason;
    type Signals = Option<Self::State>;
    type Options = UsePlaywrightValidDescribeCallbackOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a describe call
        if !is_playwright_describe_call(&callee)? {
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
                            !p.items().is_empty()
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
