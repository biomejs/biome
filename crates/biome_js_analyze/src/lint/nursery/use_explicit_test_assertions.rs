use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSourceKind, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsCallExpression, JsFunctionBody};
use biome_rowan::{AstNode, AstNodeList};

declare_lint_rule! {
    /// Require each test function (`test()`, `it()`) to have an assertion (`expect()`, `assert()`, etc.).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test('myLogic', () => {
    ///   console.log('myLogic');
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test('myLogic', () => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,expect_diagnostic
    /// test('myLogic', () => {
    ///   const actual = myLogic();
    ///   expect(actual).toBe(true);
    /// });
    /// ```
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///   "useExplicitTestAssertions": {
    ///     "level": "error",
    ///     "options": {
    ///       "assertFunctionNames": ["expect"],
    ///       "additionalTestBlockFunctions": []
    ///     }
    ///   ]
    /// }
    /// ```
    ///
    /// ### `assertFunctionNames`
    ///
    /// This array option specifies the names of functions that should be considered to
    /// be asserting functions. Function names can use wildcards i.e `request.*.expect`,
    /// `request.**.expect`, `request.*.expect*`
    ///
    /// Examples of **incorrect** code for the `{ "assertFunctionNames": ["expect"] }`
    /// option:
    ///
    /// ```js
    /// /* useExplicitTestAssertions: { "level": "error", "options": { "assertFunctionNames": ["expect"] } } */
    ///
    /// import { expectSaga } from 'redux-saga-test-plan';
    /// import { addSaga } from '../src/sagas';
    ///
    /// test('returns sum', () => {
    ///   expectSaga(addSaga, 1, 1).returns(2).run();
    /// });
    /// ```
    ///
    /// Examples of **correct** code for the
    /// `{ "assertFunctionNames": ["expect", "expectSaga"] }` option:
    ///
    /// ```js
    /// /* useExplicitTestAssertions: { "level": "error", "options": { "assertFunctionNames": ["expect", "expectSaga"] } } */
    ///
    /// import { expectSaga } from 'redux-saga-test-plan';
    /// import { addSaga } from '../src/sagas';
    ///
    /// test('returns sum', () => {
    ///   expectSaga(addSaga, 1, 1).returns(2).run();
    /// });
    /// ```
    ///
    /// _Note: wildcards and RegExp not currently supported._
    ///
    /// Default: `["expect", "assert"]`
    ///
    /// ### `additionalTestBlockFunctions`
    ///
    /// This array can be used to specify the names of functions that should also be
    /// treated as test blocks:
    ///
    /// ```json
    /// {
    ///   "useExplicitTestAssertions": [
    ///     "error",
    ///     { "additionalTestBlockFunctions": ["theoretically"] }
    ///   ]
    /// }
    /// ```
    ///
    /// The following is _correct_ when using the above configuration:
    ///
    /// ```js
    /// import theoretically from 'jest-theories';
    ///
    /// describe('NumberToLongString', () => {
    ///   const theories = [
    ///     { input: 100, expected: 'One hundred' },
    ///     { input: 1000, expected: 'One thousand' },
    ///     { input: 10000, expected: 'Ten thousand' },
    ///     { input: 100000, expected: 'One hundred thousand' },
    ///   ];
    ///
    ///   theoretically(
    ///     'the number {input} is correctly translated to string',
    ///     theories,
    ///     (theory) => {
    ///       const output = NumberToLongString(theory.input);
    ///       expect(output).toBe(theory.expected);
    ///     },
    ///   );
    /// });
    /// ```
    ///
    /// Default: `["test", "it"]`
    pub UseExplicitTestAssertions {
        version: "next",
        name: "useExplicitTestAssertions",
        language: "js",
        sources: &[RuleSource::EslintJest("expect-expect")],
        recommended: false,
        source_kind: RuleSourceKind::Inspired,
        domains: &[RuleDomain::Test],
    }
}

/// Rule's options
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UseExplicitTestAssertionsOptions {
    /// Specify assert function names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assert_function_names: Option<Vec<String>>,
    /// Specify assert function names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_test_block_function_names: Option<Vec<String>>,
}

impl Default for UseExplicitTestAssertionsOptions {
    fn default() -> Self {
        Self {
            assert_function_names: default_assert_function_names(),
            additional_test_block_function_names: default_additional_test_block_function_names(),
        }
    }
}

fn default_assert_function_names() -> Option<Vec<String>> {
    Some(vec!["expect".to_string(), "assert".to_string()])
}

fn default_additional_test_block_function_names() -> Option<Vec<String>> {
    Some(vec!["test".to_string(), "it".to_string()])
}

impl Rule for UseExplicitTestAssertions {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseExplicitTestAssertionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();

        // Is this function named it() or test()?
        if !is_matching_fn(node, &options.additional_test_block_function_names) {
            return None;
        }

        let [Some(second)] = node.arguments().ok()?.get_arguments_by_index([1]) else {
            return None;
        };
        let Some(second_expr) = second.as_any_js_expression() else {
            return None;
        };
        let try_function_body = match second_expr {
            AnyJsExpression::JsArrowFunctionExpression(function) => function.body().ok(),
            // TODO: support function
            _ => None,
        };
        if let Some(function_body) = try_function_body {
            if let Some(js_function_body) = function_body.as_js_function_body() {
                if fn_body_contains_call(js_function_body, &options.assert_function_names) {
                    Some(());
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Missing assertion in test body."
                },
            )
            .note(markup! {
                "This prevents false positives where a test always passes but isn’t actually testing anything."
            }).note(markup! {
                "Add an expect() (Vitest/Jest) or assert() (node:assert) assertion to this test."
            }),
        )
    }
}

/// Filter functions by name
fn is_matching_fn(node: &JsCallExpression, names: &Option<Vec<String>>) -> bool {
    let Some(callee) = node.callee().ok() else {
        return false;
    };
    if let Some(function_name) = callee.get_callee_member_name() {
        match names {
            Some(names) => names.iter().any(|s| s == &function_name.to_string()),
            None => false,
        };
    }

    false
}

/// Detect if a function body has a call expression in given names list
fn fn_body_contains_call(node: &JsFunctionBody, names: &Option<Vec<String>>) -> bool {
    node.statements().iter().any(|statement| match statement {
        AnyJsStatement::JsExpressionStatement(try_expression) => {
            if let Some(expression) = try_expression.expression().ok() {
                if let Some(call_expression) = expression.as_js_call_expression() {
                    return is_matching_fn(&call_expression, names);
                }
                false;
            }
            false
        }
        _ => false,
    })
}
