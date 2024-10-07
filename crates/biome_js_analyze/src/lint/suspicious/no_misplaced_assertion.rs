use crate::services::semantic::Semantic;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsIdentifierBinding, JsImport};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Checks that the assertion function, for example `expect`, is placed inside an `it()` function call.
    ///
    /// Placing (and using) the `expect` assertion function can result in unexpected behaviors when executing your testing suite.
    ///
    /// The rule will check for the following assertion calls:
    /// - `expect`
    /// - `assert`
    /// - `assertEquals`
    ///
    /// However, the rule will ignore the following assertion calls:
    /// - `expect.any`
    /// - `expect.anything`
    /// - `expect.closeTo`
    /// - `expect.arrayContaining`
    /// - `expect.objectContaining`
    /// - `expect.stringContaining`
    /// - `expect.stringMatching`
    /// - `expect.extend`
    /// - `expect.addEqualityTesters`
    /// - `expect.addSnapshotSerializer`
    ///
    /// If the assertion function is imported, the rule will check if they are imported from:
    /// - `"chai"`
    /// - `"node:assert"`
    /// - `"node:assert/strict"`
    /// - `"bun:test"`
    /// - `"vitest"`
    /// - Deno assertion module URL
    ///
    /// Check the [options](#options) if you need to change the defaults.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe("describe", () => {
    ///     expect()
    /// })
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import assert from "node:assert";
    /// describe("describe", () => {
    ///     assert.equal()
    /// })
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import {test, expect} from "bun:test";
    /// expect(1, 2)
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import {assertEquals} from "https://deno.land/std@0.220.0/assert/mod.ts";
    ///
    /// assertEquals(url.href, "https://deno.land/foo.js");
    /// Deno.test("url test", () => {
    ///     const url = new URL("./foo.js", "https://deno.land/");
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import assert from "node:assert";
    /// describe("describe", () => {
    ///     it("it", () => {
    ///         assert.equal()
    ///     })
    /// })
    /// ```
    ///
    /// ```js
    /// describe("describe", () => {
    ///     it("it", () => {
    ///         expect()
    ///     })
    /// })
    /// ```
    ///
    /// ```js
    /// test.each([1, 2, 3])('test', (a, b, expected) => {
    ///     expect(a + b).toBe(expected)
    /// })
    /// ```
    ///
    /// ```js
    /// import { waitFor } from '@testing-library/react';
    /// await waitFor(() => {
    ///   expect(111).toBe(222);
    /// });
    /// ```
    ///
    pub NoMisplacedAssertion {
        version: "1.8.0",
        name: "noMisplacedAssertion",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintJest("no-standalone-expect")],
        source_kind: RuleSourceKind::Inspired,
    }
}

const ASSERTION_FUNCTION_NAMES: [&str; 3] = ["assert", "assertEquals", "expect"];
const SPECIFIERS: [&str; 6] = [
    "chai",
    "node:assert",
    "node:assert/strict",
    "bun:test",
    "vitest",
    "/assert/mod.ts",
];
const EXCEPTION_MEMBERS_FOR_EXPECT: [&str; 10] = [
    "any",
    "anything",
    "closeTo",
    "arrayContaining",
    "objectContaining",
    "stringContaining",
    "stringMatching",
    "extend",
    "addEqualityTesters",
    "addSnapshotSerializer",
];

impl Rule for NoMisplacedAssertion {
    type Query = Semantic<AnyJsExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        if let Some(call_text) = node.to_assertion_call() {
            let ancestor_is_test_call = {
                node.syntax()
                    .ancestors()
                    .filter_map(JsCallExpression::cast)
                    .find_map(|call_expression| {
                        let callee = call_expression.callee().ok()?;
                        let callee = may_extract_nested_expr(callee)?;
                        callee.contains_it_call().then_some(true)
                    })
                    .unwrap_or_default()
            };

            let ancestor_is_describe_call = {
                node.syntax()
                    .ancestors()
                    .filter_map(JsCallExpression::cast)
                    .find_map(|call_expression| {
                        let callee = call_expression.callee().ok()?;
                        let callee = may_extract_nested_expr(callee)?;
                        callee.contains_describe_call().then_some(true)
                    })
            };
            let assertion_call = node.get_callee_object_identifier()?;

            if let Some(ancestor_is_describe_call) = ancestor_is_describe_call {
                if ancestor_is_describe_call && ancestor_is_test_call {
                    return None;
                }
            } else if ancestor_is_test_call {
                return None;
            }

            let is_exception = is_exception_for_expect(node)?;
            let binding = model.binding(&assertion_call);
            if let Some(binding) = binding {
                let ident = JsIdentifierBinding::cast_ref(binding.syntax())?;
                let import = ident.syntax().ancestors().find_map(JsImport::cast)?;
                let source_text = import.source_text().ok()?;
                if (ASSERTION_FUNCTION_NAMES.contains(&call_text.text()))
                    && (SPECIFIERS.iter().any(|specifier| {
                        // Deno is a particular case
                        if *specifier == "/assert/mod.ts" {
                            source_text.text().ends_with("/assert/mod.ts")
                                && source_text.text().starts_with("https://deno.land/std")
                        } else {
                            *specifier == source_text.text()
                        }
                    }))
                    && !is_exception
                {
                    return Some(assertion_call.range());
                }
            } else if ASSERTION_FUNCTION_NAMES.contains(&call_text.text()) && !is_exception {
                return Some(assertion_call.range());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "The assertion isn't inside a "<Emphasis>"it()"</Emphasis>", "<Emphasis>"test()"</Emphasis>" or "<Emphasis>"Deno.test()"</Emphasis>" function call."
                },
            )
            .note(markup! {
                "This will result in unexpected behaviours from your test suite."
            })
            .note(markup! {
                "Move the assertion inside a "<Emphasis>"it()"</Emphasis>", "<Emphasis>"test()"</Emphasis>" or "<Emphasis>"Deno.test()"</Emphasis>" function call."
            }),
        )
    }
}

/// Returns the nested expression if the callee is a call expression or a template expression.
fn may_extract_nested_expr(callee: AnyJsExpression) -> Option<AnyJsExpression> {
    match callee {
        AnyJsExpression::JsCallExpression(call_expr) => call_expr.callee().ok(),
        AnyJsExpression::JsTemplateExpression(template_expr) => template_expr.tag(),
        _ => Some(callee),
    }
}

/// Returns whether the assertion call is an exception for the `expect` assertion function.
fn is_exception_for_expect(node: &AnyJsExpression) -> Option<bool> {
    let assertion_call = node.get_callee_object_identifier()?;
    let callee_text = assertion_call.syntax().parent()?.text_trimmed();

    let parent = node.syntax().parent()?;
    let last_token = parent.last_token()?;
    let last_token_text = last_token.text();

    let member = node.get_callee_member_name()?;
    let member_text = member.text_trimmed();

    Some(if callee_text == "expect" {
        EXCEPTION_MEMBERS_FOR_EXPECT.contains(&member_text)
            || EXCEPTION_MEMBERS_FOR_EXPECT.contains(&last_token_text)
    } else {
        false
    })
}
