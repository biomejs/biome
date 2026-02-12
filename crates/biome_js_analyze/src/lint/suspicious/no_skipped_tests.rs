use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};
use biome_rowan::{AstSeparatedList, BatchMutationExt, TextRange};
use biome_rule_options::no_skipped_tests::NoSkippedTestsOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow disabled tests.
    ///
    /// Disabled tests are useful when developing and debugging, although they should not be committed in production.
    ///
    /// The rule detects the following patterns:
    /// - `describe.skip`, `it.skip`, `test.skip`
    /// - `describe.fixme`, `it.fixme`, `test.fixme`
    /// - `test.describe.skip`, `test.describe.fixme`
    /// - `test.describe.parallel.skip`, `test.describe.serial.skip`
    /// - `test.describe.parallel.fixme`, `test.describe.serial.fixme`
    /// - `test.step.skip`, `test.step.fixme`
    /// - `xdescribe`, `xit`, `xtest`
    /// - Bracket notation: `test["skip"]`, `test["fixme"]`
    /// - Bare `test.skip()` / `test.fixme()` calls (0 arguments)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe.skip("test", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.skip("test", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.fixme("needs fixing", async () => {});
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// test.only("test", () => {});
    /// test("test", () => {});
    /// ```
    ///
    pub NoSkippedTests {
        version: "1.6.0",
        name: "noSkippedTests",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("no-disabled-tests").inspired(),
            RuleSource::EslintVitest("no-disabled-tests").inspired(),
            RuleSource::EslintPlaywright("no-skipped-test").inspired(),
        ],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Test],
    }
}

const FUNCTION_NAMES: [&str; 5] = ["skip", "fixme", "xdescribe", "xit", "xtest"];

impl Rule for NoSkippedTests {
    type Query = Ast<JsCallExpression>;
    type State = SkipState;
    type Signals = Option<Self::State>;
    type Options = NoSkippedTestsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        // Path 1: Standard test call with skip/fixme/x-prefix (static member expressions)
        if node.is_test_call_expression().ok()?
            && callee.contains_a_test_pattern()
            && let Some(function_name) = callee.get_callee_member_name()
        {
            let name_text = function_name.text_trimmed();
            if FUNCTION_NAMES.contains(&name_text) {
                let annotation = if name_text == "fixme" {
                    "fixme"
                } else {
                    "skip"
                };
                return Some(SkipState {
                    range: function_name.text_trimmed_range(),
                    annotation,
                });
            }
        }

        // Path 2: Bare skip/fixme calls (e.g., test.skip() or test.fixme() with 0 args),
        // bracket notation (test["skip"]), and Playwright describe/step patterns
        if let Some(skip_state) = detect_bare_skip_call(node, &callee) {
            return Some(skip_state);
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "Don't disable tests."
                },
            )
            .note("Disabling tests is useful when debugging or creating placeholder while working.")
            .note("If this is intentional, and you want to commit a disabled test, add a suppression comment.")
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        let mut mutation = ctx.root().begin();

        if let Some(function_name) = callee.get_callee_member_name() {
            let replaced_function;
            match function_name.text_trimmed() {
                "skip" | "fixme" => {
                    find_and_remove_skip_member(
                        &callee,
                        function_name.text_trimmed(),
                        &mut mutation,
                    )?;
                }
                "xdescribe" => {
                    replaced_function = make::js_reference_identifier(make::ident("describe"));
                    mutation.replace_element(function_name.into(), replaced_function.into());
                }
                "xit" => {
                    replaced_function = make::js_reference_identifier(make::ident("it"));
                    mutation.replace_element(function_name.into(), replaced_function.into());
                }
                "xtest" => {
                    replaced_function = make::js_reference_identifier(make::ident("test"));
                    mutation.replace_element(function_name.into(), replaced_function.into());
                }
                _ => {}
            }
        } else {
            // Computed member expression (bracket notation) or other Path 2 patterns
            find_and_remove_skip_member(&callee, state.annotation, &mut mutation)?;
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Enable the test." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug)]
pub struct SkipState {
    range: TextRange,
    /// The type of annotation: "skip" or "fixme".
    annotation: &'static str,
}

/// Detects bare `test.skip()` / `test.fixme()` calls that don't pass `is_test_call_expression()`
/// because they have 0, 1, or 2 non-string arguments.
///
/// Also detects Playwright-specific patterns:
/// - `test.describe.skip(...)`, `test.describe.fixme(...)`
/// - `test.describe.parallel.skip(...)`, `test.describe.serial.skip(...)`
/// - `test.step.skip(...)`, `test.step.fixme(...)`
fn detect_bare_skip_call(
    call_expr: &JsCallExpression,
    callee: &AnyJsExpression,
) -> Option<SkipState> {
    // Collect the member names from the callee chain
    let names = collect_callee_names(callee)?;

    // Determine if this is a skip/fixme pattern
    let (annotation, name_range) = match names.as_slice() {
        // test.skip() / it.skip() / describe.skip() / test.fixme() / it.fixme() / describe.fixme()
        [(_, _root), (range, name)]
            if matches!(_root.text(), "test" | "it" | "describe")
                && (name.text() == "skip" || name.text() == "fixme") =>
        {
            (annotation_for(name.text()), *range)
        }
        // test.describe.skip() / test.describe.fixme() / test.step.skip() / test.step.fixme()
        [(_, root), (_, middle), (range, name)]
            if root.text() == "test"
                && (middle.text() == "describe" || middle.text() == "step")
                && (name.text() == "skip" || name.text() == "fixme") =>
        {
            (annotation_for(name.text()), *range)
        }
        // test.describe.parallel.skip() / test.describe.serial.skip() / etc.
        [(_, root), (_, desc), (_, mode), (range, name)]
            if root.text() == "test"
                && desc.text() == "describe"
                && (mode.text() == "parallel" || mode.text() == "serial")
                && (name.text() == "skip" || name.text() == "fixme") =>
        {
            (annotation_for(name.text()), *range)
        }
        _ => return None,
    };

    // Check if this is already handled by Path 1 (is_test_call_expression).
    // For standard test calls like test.skip("name", fn), Path 1 handles it.
    // We only handle:
    //   - Bare calls with 0 args
    //   - Conditional calls (1-2 non-string args)
    //   - Playwright describe/step patterns
    let arg_count = call_expr
        .arguments()
        .ok()
        .map_or(0, |a| a.args().iter().count());

    // For 2-name chains (test.skip / it.skip / describe.skip):
    // - Standard test calls (2+ args with string first) are caught by Path 1,
    //   UNLESS the callee uses bracket notation (JsComputedMemberExpression),
    //   which get_callee_member_name() can't handle.
    // - We handle bare calls (0 args) and conditional calls (1-2 non-string args)
    let is_computed = matches!(callee, AnyJsExpression::JsComputedMemberExpression(_));

    if names.len() == 2 {
        let root_name = names[0].1.text();
        if root_name == "test" || root_name == "it" {
            // Standard test.skip("name", fn) with string first arg → handled by Path 1
            // (but not bracket notation, which Path 1 can't fix)
            if arg_count >= 2 && has_string_first_arg(call_expr) && !is_computed {
                return None;
            }
            // 0-arg bare skip/fixme
            if arg_count == 0 {
                return Some(SkipState {
                    range: name_range,
                    annotation,
                });
            }
            // 1-2 arg conditional skip
            if (arg_count == 1 || arg_count == 2) && !has_string_first_arg(call_expr) {
                return Some(SkipState {
                    range: name_range,
                    annotation,
                });
            }
            // Bracket notation with standard test args: test["skip"]("name", fn)
            if is_computed {
                return Some(SkipState {
                    range: name_range,
                    annotation,
                });
            }
            // Standard static member: handled by Path 1
            return None;
        }
        if root_name == "describe" {
            // describe.skip("name", fn) handled by Path 1 (unless bracket notation)
            if arg_count >= 1 && has_string_first_arg(call_expr) && !is_computed {
                return None;
            }
            return Some(SkipState {
                range: name_range,
                annotation,
            });
        }
    }

    // For 3+ name chains (Playwright patterns): always report
    // test.describe.skip("name", fn), test.step.skip("name", fn), etc.
    Some(SkipState {
        range: name_range,
        annotation,
    })
}

fn annotation_for(name: &str) -> &'static str {
    if name == "fixme" { "fixme" } else { "skip" }
}

/// Collects member names with their text ranges from a callee expression.
/// Returns names in "outside-in" order: `["test", "describe", "skip"]`
fn collect_callee_names(
    expr: &AnyJsExpression,
) -> Option<Vec<(TextRange, biome_rowan::TokenText)>> {
    let mut names = Vec::new();
    collect_callee_names_rec(expr, &mut names)?;
    Some(names)
}

fn collect_callee_names_rec(
    expr: &AnyJsExpression,
    names: &mut Vec<(TextRange, biome_rowan::TokenText)>,
) -> Option<()> {
    match expr {
        AnyJsExpression::JsIdentifierExpression(id) => {
            let name = id.name().ok()?;
            let token = name.value_token().ok()?;
            names.push((token.text_trimmed_range(), token.token_text_trimmed()));
            Some(())
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let object = member.object().ok()?;
            collect_callee_names_rec(&object, names)?;
            let m = member.member().ok()?;
            let n = m.as_js_name()?;
            let t = n.value_token().ok()?;
            names.push((t.text_trimmed_range(), t.token_text_trimmed()));
            Some(())
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            let object = member.object().ok()?;
            collect_callee_names_rec(&object, names)?;
            if let Ok(expr) = member.member()
                && let Some(literal) = expr.as_any_js_literal_expression()
                && let Some(string_lit) = literal.as_js_string_literal_expression()
                && let Ok(inner) = string_lit.inner_string_text()
            {
                let token = string_lit.value_token().ok()?;
                names.push((token.text_trimmed_range(), inner));
                return Some(());
            }
            None
        }
        _ => None,
    }
}

/// Checks if the first argument of a call expression is a string literal or template literal.
fn has_string_first_arg(call_expr: &JsCallExpression) -> bool {
    let Ok(args) = call_expr.arguments() else {
        return false;
    };
    let Some(first_arg) = args.args().iter().next() else {
        return false;
    };
    let Ok(first_arg) = first_arg else {
        return false;
    };
    let Some(expr) = first_arg.as_any_js_expression() else {
        return false;
    };
    matches!(
        expr,
        AnyJsExpression::AnyJsLiteralExpression(lit)
            if lit.as_js_string_literal_expression().is_some()
    ) || matches!(expr, AnyJsExpression::JsTemplateExpression(_))
}

/// Recursively find and remove the .skip or .fixme member from the expression chain
fn find_and_remove_skip_member(
    expr: &AnyJsExpression,
    target: &str,
    mutation: &mut biome_rowan::BatchMutation<biome_js_syntax::JsLanguage>,
) -> Option<()> {
    match expr {
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let member_name = member.member().ok()?;
            let name_node = member_name.as_js_name()?;
            let name_text = name_node.value_token().ok()?;

            if name_text.text_trimmed() == target {
                let object = member.object().ok()?;
                mutation.replace_node(
                    AnyJsExpression::JsStaticMemberExpression(member.clone()),
                    object,
                );
                return Some(());
            }

            let obj = member.object().ok()?;
            find_and_remove_skip_member(&obj, target, mutation)
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            let member_expr = member.member().ok()?;
            if let AnyJsExpression::AnyJsLiteralExpression(lit) = &member_expr
                && let Some(string_lit) = lit.as_js_string_literal_expression()
                && let Ok(inner) = string_lit.inner_string_text()
                && inner == target
            {
                let object = member.object().ok()?;
                mutation.replace_node(
                    AnyJsExpression::JsComputedMemberExpression(member.clone()),
                    object,
                );
                return Some(());
            }

            let obj = member.object().ok()?;
            find_and_remove_skip_member(&obj, target, mutation)
        }
        _ => None,
    }
}
