use crate::services::semantic::Semantic;
use biome_analyze::{
    context::RuleContext, declare_rule, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsIdentifierBinding, JsImport};
use biome_rowan::AstNode;

declare_rule! {
    /// Checks that the assertion function, for example `expect`, is placed inside an `it()` function call.
    ///
    /// Placing (and using) the `expect` assertion function can result in unexpected behaviors when executing your testing suite.
    ///
    /// The rule will check for the following assertion calls:
    /// - `expect`
    /// - `assert`
    /// - `assertEquals`
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
    pub NoMisplacedAssertion {
        version: "next",
        name: "noMisplacedAssertion",
        recommended: false,
        source: RuleSource::EslintJest("no-standalone-expect"),
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
                {
                    return Some(assertion_call.range());
                }
            } else if ASSERTION_FUNCTION_NAMES.contains(&call_text.text()) {
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
