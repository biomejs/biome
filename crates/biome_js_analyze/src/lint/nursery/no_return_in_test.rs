use std::iter;

use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsReturnStatement,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList};
use biome_rule_options::no_return_in_test::NoReturnInTestOptions;

use crate::{
    frameworks::unit_tests::{TestBlockKind, is_unit_test},
    services::semantic::Semantic,
};

declare_lint_rule! {
    /// Disallow `return` statements in tests.
    ///
    /// A test doesn't need to return a value.
    /// The only returned value that test runners make use of is a promise: the runner waits for it before it finishes the test.
    /// Using `await` inside an `async` test function does the same thing, and it makes the waiting easier to see.
    /// Unlike `return`, `await` also lets the test continue with more code afterwards.
    ///
    /// This rule reports a `return` statement that is directly inside the body of a test function, whether or not it returns a value.
    /// It also checks functions that are declared elsewhere in the file and passed to a test by name.
    ///
    /// The rule doesn't report:
    ///
    /// - `return` statements inside another statement, such as an `if` statement, or inside a nested function.
    /// - Arrow functions that consist of a single expression, such as `() => expect(1).toBe(1)`.
    /// - Functions passed to Playwright's `test.step()`, because `test.step()` gives the returned value back to the code that called it.
    /// - Functions passed to fast-check's `test.prop()`, because the returned value decides whether the test passes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test("adds two numbers", () => {
    ///     return expect(add(1, 2)).toBe(3);
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// it("loads the user", function () {
    ///     return loadUser().then((user) => expect(user.name).toBe("Ada"));
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// it("loads the user", loadsTheUser);
    ///
    /// function loadsTheUser() {
    ///     return loadUser().then((user) => expect(user.name).toBe("Ada"));
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// test("adds two numbers", () => {
    ///     expect(add(1, 2)).toBe(3);
    /// });
    ///
    /// test("subtracts two numbers", () => expect(subtract(3, 2)).toBe(1));
    ///
    /// it("loads the user", async () => {
    ///     const user = await loadUser();
    ///     expect(user.name).toBe("Ada");
    /// });
    ///
    /// it("saves the user", async () => {
    ///     if (isReadOnly) {
    ///         return;
    ///     }
    ///     await saveUser();
    /// });
    /// ```
    ///
    pub NoReturnInTest {
        version: "next",
        name: "noReturnInTest",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("no-test-return-statement").same(),
            RuleSource::EslintVitest("no-test-return-statement").same(),
            RuleSource::EslintPlaywright("no-test-return-statement").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

impl Rule for NoReturnInTest {
    type Query = Semantic<AnyJsFunction>;
    /// The first `return` statement directly inside the body of the test function.
    type State = JsReturnStatement;
    type Signals = Option<Self::State>;
    type Options = NoReturnInTestOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let function = ctx.query();
        let return_statement = function
            .body()
            .ok()?
            .as_js_function_body()?
            .statements()
            .iter()
            .find_map(|statement| statement.as_js_return_statement().cloned())?;

        let is_test_callback = if let Some(binding) = function.binding() {
            binding
                .as_js_identifier_binding()?
                .all_reads(ctx.model())
                .any(|reference| {
                    reference
                        .syntax()
                        .parent()
                        .and_then(AnyJsExpression::cast)
                        .is_some_and(|expression| is_test_callback(&expression))
                })
        } else {
            AnyJsExpression::cast_ref(function.syntax())
                .is_some_and(|expression| is_test_callback(&expression))
        };

        is_test_callback.then_some(return_statement)
    }

    fn diagnostic(_: &RuleContext<Self>, return_statement: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                return_statement.range(),
                markup! {
                    "Unexpected "<Emphasis>"return"</Emphasis>" statement in a test."
                },
            )
            .note(markup! {
                "A test doesn't need to return a value. Test runners only make use of a returned promise, by waiting for it, and "<Emphasis>"await"</Emphasis>" makes that waiting easier to see."
            })
            .note(markup! {
                "Remove the "<Emphasis>"return"</Emphasis>" statement. If it returns a promise, mark the test function as "<Emphasis>"async"</Emphasis>" and use "<Emphasis>"await"</Emphasis>" on the promise instead."
            }),
        )
    }
}

/// Returns `true` if `expression` is the function that a test runs, as in
/// `test("title", expression)`.
fn is_test_callback(expression: &AnyJsExpression) -> bool {
    let Some(expression) = expression.outer_expression() else {
        return false;
    };
    let Some(arguments) = expression.parent::<JsCallArgumentList>() else {
        return false;
    };
    // The first argument is the title of the test. Playwright's
    // `test.skip(callback, description)` takes a function there, but that
    // function has to return whether the tests are skipped.
    if arguments
        .first()
        .and_then(Result::ok)
        .is_some_and(|first| first.syntax() == expression.syntax())
    {
        return false;
    }
    arguments
        .parent::<JsCallArguments>()
        .and_then(|arguments| arguments.parent::<JsCallExpression>())
        .is_some_and(|call| is_test_call(&call))
}

/// Returns `true` if `call` declares a test, such as `it()`, `test.only()`,
/// or `test.each(table)()`.
fn is_test_call(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };
    match callee.omit_parentheses() {
        // test.each`table`("title", () => {})
        AnyJsExpression::JsTemplateExpression(template) => {
            template.tag().is_some_and(is_test_table_callee)
        }
        // test.each(table)("title", () => {})
        AnyJsExpression::JsCallExpression(table_call) => {
            table_call.callee().is_ok_and(is_test_table_callee)
        }
        callee => is_unit_test(call) && !has_non_test_name(callee),
    }
}

/// Returns `true` if `callee` declares tests from a table of cases, as
/// `test.each` and `it.only.each` do.
fn is_test_table_callee(callee: AnyJsExpression) -> bool {
    let callee = callee.omit_parentheses();
    TestBlockKind::from_each_callee(&callee) == Some(TestBlockKind::Test)
        && !has_non_test_name(callee)
}

/// Returns `true` if one of the names in `callee` shows that the function
/// passed to it isn't the body of a test:
///
/// - `describe` and `suite` declare a group of tests, as in
///   `test.describe.parallel`.
/// - `step` is Playwright's `test.step`, which hands the value that its
///   function returns back to the code that called it.
/// - `prop` is fast-check's `test.prop`, whose function returns whether the
///   checked property holds.
fn has_non_test_name(callee: AnyJsExpression) -> bool {
    iter::successors(Some(callee), |expression| {
        expression.as_js_static_member_expression()?.object().ok()
    })
    .any(|expression| {
        expression
            .get_callee_member_name()
            .is_some_and(|name| matches!(name.text_trimmed(), "describe" | "suite" | "step" | "prop"))
    })
}
