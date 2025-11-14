use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, global_identifier};
use biome_rowan::{AstNode, TextRange};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow usage of `pauseTest` in tests.
    ///
    /// `pauseTest` is a debugging utility from `@ember/test-helpers` that pauses test execution.
    /// It should not be committed to the codebase as it will cause tests to hang in CI/CD pipelines.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { pauseTest } from '@ember/test-helpers';
    ///
    /// pauseTest();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { setupTest } from 'ember-qunit';
    /// import { module, test } from 'qunit';
    ///
    /// module('Example', function (hooks) {
    ///   setupTest(hooks);
    ///
    ///   test('it works', async function () {
    ///     await this.pauseTest();
    ///   });
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { setupTest } from 'ember-qunit';
    /// import { module, test } from 'qunit';
    ///
    /// module('Example', function (hooks) {
    ///   setupTest(hooks);
    ///
    ///   test('it works', async function () {
    ///     // No pauseTest call
    ///   });
    /// });
    /// ```
    ///
    pub NoEmberPauseTest {
        version: "next",
        name: "noEmberPauseTest",
        language: "js",
        sources: &[RuleSource::Eslint("ember/no-pause-test").same()],
        recommended: true,
    }
}

impl Rule for NoEmberPauseTest {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let callee = call_expression.callee().ok()?;
        let model = ctx.model();

        // Check for direct call: pauseTest()
        if let Some((reference, name)) = global_identifier(&callee) {
            if name.text() == "pauseTest" {
                // Check if it's imported (has a binding)
                if let Some(binding) = model.binding(&reference) {
                    // Check if it's imported from @ember/test-helpers
                    if binding.is_imported() {
                        // Get the import source
                        if let Some(import_source) = get_import_source(&binding) {
                            if import_source == "@ember/test-helpers" {
                                return Some(call_expression.range());
                            }
                        }
                    }
                }
            }
            return None;
        }

        // Check for this.pauseTest()
        if let AnyJsExpression::JsStaticMemberExpression(member_expr) = callee {
            let member = member_expr.member().ok()?;
            let member_name = member.as_js_name()?.value_token().ok()?;

            if member_name.text_trimmed() == "pauseTest" {
                let object = member_expr.object().ok()?;
                if let AnyJsExpression::JsThisExpression(_) = object {
                    return Some(call_expression.range());
                }
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
                    "Do not use "<Emphasis>"pauseTest"</Emphasis>" in tests."
                },
            )
            .note(markup! {
                <Emphasis>"pauseTest"</Emphasis>" is a debugging utility that pauses test execution and should not be committed to the codebase."
            })
            .note(markup! {
                "Remove the "<Emphasis>"pauseTest()"</Emphasis>" call before committing your code."
            }),
        )
    }
}

/// Helper function to get the import source from a binding
fn get_import_source(binding: &biome_js_semantic::Binding) -> Option<String> {
    use biome_js_syntax::JsImport;

    let syntax = binding.syntax();

    // Navigate up the tree to find the import statement
    let mut current = Some(syntax.clone());
    while let Some(node) = current {
        if JsImport::can_cast(node.kind()) {
            let import = JsImport::unwrap_cast(node);
            let import_clause = import.import_clause().ok()?;
            let source = import_clause.source().ok()?;
            let source_text = source.inner_string_text().ok()?;
            return Some(source_text.to_string());
        }

        current = node.parent();
    }

    None
}
