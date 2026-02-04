use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsSyntaxKind};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

use biome_rule_options::no_playwright_skipped_test::NoPlaywrightSkippedTestOptions;

declare_lint_rule! {
    /// Disallow usage of `.skip` and `.fixme` annotations.
    ///
    /// The `.skip` and `.fixme` annotations in Playwright prevent tests from running.
    /// While these are useful during development, they should not be committed to
    /// version control as they can lead to forgotten test cases and reduced test coverage.
    ///
    /// The rule can be configured with `allowConditional: true` to permit conditional
    /// skipping inside test bodies (e.g., `test.skip(browserName === "webkit")`).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test.skip("skipped test", async ({ page }) => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.describe.skip("skipped suite", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.fixme("needs fixing", async ({ page }) => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// test("working test", async ({ page }) => {});
    /// ```
    ///
    /// ```js
    /// test.describe("suite", () => {
    ///     test("test", async ({ page }) => {});
    /// });
    /// ```
    ///
    pub NoPlaywrightSkippedTest {
        version: "next",
        name: "noPlaywrightSkippedTest",
        language: "js",
        sources: &[RuleSource::EslintPlaywright("no-skipped-test").same()],
        recommended: false,
        domains: &[RuleDomain::Playwright],
        fix_kind: FixKind::Unsafe,
    }
}

/// The type of skipped annotation detected
pub enum SkippedType {
    Skip,
    Fixme,
}

impl SkippedType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Skip => "skip",
            Self::Fixme => "fixme",
        }
    }
}

/// Collects member names from an expression in "outside-in" order.
/// For example, `test.describe.skip` returns `["test", "describe", "skip"]`.
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
        AnyJsExpression::JsComputedMemberExpression(member) => {
            let obj = member.object().ok()?;
            collect_member_names_rec(&obj, names)?;
            // Handle bracket notation like test["skip"]
            let member_expr = member.member().ok()?;
            if let AnyJsExpression::AnyJsLiteralExpression(lit) = member_expr
                && let Some(string_lit) = lit.as_js_string_literal_expression()
            {
                let token = string_lit.value_token().ok()?;
                let text = token.text_trimmed();
                // Remove quotes from string literal
                let unquoted = text.trim_matches(|c| c == '"' || c == '\'');
                names.push(unquoted.to_string());
                return Some(());
            }
            None
        }
        _ => None,
    }
}

/// Checks if this is a skipped test/describe/step call.
/// Returns the SkippedType if it matches.
fn is_playwright_skipped_call(callee: &AnyJsExpression) -> Option<SkippedType> {
    let names = collect_member_names(callee)?;
    let names_ref: Vec<&str> = names.iter().map(String::as_str).collect();

    match names_ref.as_slice() {
        // test.skip(...) / it.skip(...)
        ["test" | "it", "skip"] => Some(SkippedType::Skip),
        // test.describe.skip(...) / describe.skip(...)
        ["test", "describe", "skip"] | ["describe", "skip"] => Some(SkippedType::Skip),
        // test.step.skip(...)
        ["test", "step", "skip"] => Some(SkippedType::Skip),
        // test.fixme(...) / it.fixme(...)
        ["test" | "it", "fixme"] => Some(SkippedType::Fixme),
        // test.describe.fixme(...)
        ["test", "describe", "fixme"] => Some(SkippedType::Fixme),
        // test.describe.parallel.skip(...) / test.describe.serial.skip(...)
        ["test", "describe", mode, "skip"] if is_describe_mode(mode) => Some(SkippedType::Skip),
        // test.describe.parallel.fixme(...) / test.describe.serial.fixme(...)
        ["test", "describe", mode, "fixme"] if is_describe_mode(mode) => Some(SkippedType::Fixme),
        _ => None,
    }
}

fn is_describe_mode(s: &str) -> bool {
    matches!(s, "parallel" | "serial")
}

/// Checks if this is a conditional skip call (test.skip() inside test body with args or in if block).
fn is_conditional_skip(call_expr: &JsCallExpression, names: &[String]) -> bool {
    let names_ref: Vec<&str> = names.iter().map(String::as_str).collect();

    // Only test.skip(...) and it.skip(...) can be conditional
    if !matches!(names_ref.as_slice(), ["test" | "it", "skip"]) {
        return false;
    }

    // First check if inside an if statement (this is always conditional)
    for ancestor in call_expr.syntax().ancestors() {
        if ancestor.kind() == JsSyntaxKind::JS_IF_STATEMENT {
            return true;
        }
        // Stop at function boundaries
        if matches!(
            ancestor.kind(),
            JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_FUNCTION_DECLARATION
        ) {
            break;
        }
    }

    // Check if call has arguments (conditional skip like test.skip(condition))
    if let Ok(args) = call_expr.arguments() {
        let arg_count = args.args().iter().count();

        // test.skip(condition) - one argument is a conditional skip
        // test.skip(condition, "reason") - two arguments is also conditional
        // test.skip("name", callback) - two arguments with string first is not conditional
        if arg_count == 1 || arg_count == 2 {
            // Check if first arg is NOT a string literal (would be test name)
            if let Some(first_arg) = args.args().iter().next()
                && let Ok(first_arg) = first_arg
                && let Some(expr) = first_arg.as_any_js_expression()
            {
                // If first arg is a string literal, it's test.skip("name", fn)
                // which is not conditional
                if matches!(
                    expr,
                    AnyJsExpression::AnyJsLiteralExpression(lit)
                        if lit.as_js_string_literal_expression().is_some()
                ) {
                    return false;
                }
                // If first arg is a template literal, also not conditional
                if matches!(expr, AnyJsExpression::JsTemplateExpression(_)) {
                    return false;
                }
                // Otherwise it's likely a conditional expression
                return true;
            }
        }
    }

    false
}

impl Rule for NoPlaywrightSkippedTest {
    type Query = Ast<JsCallExpression>;
    type State = SkippedType;
    type Signals = Option<Self::State>;
    type Options = NoPlaywrightSkippedTestOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;

        // Check if this is a skipped call
        let skipped_type = is_playwright_skipped_call(&callee)?;

        // If allowConditional is enabled, check if this is a conditional skip
        let options = ctx.options();
        if options.allow_conditional() {
            let names = collect_member_names(&callee)?;
            if is_conditional_skip(call_expr, &names) {
                return None;
            }
        }

        Some(skipped_type)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let annotation = state.as_str();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of "<Emphasis>"."{annotation}"()"</Emphasis>" annotation."
                },
            )
            .note(markup! {
                "Skipped tests are discouraged because they might be forgotten and remain disabled permanently."
            })
            .note(markup! {
                "Remove the "<Emphasis>"."{annotation}"()"</Emphasis>" annotation or address the underlying issue causing the test to be skipped."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?;
        let mut mutation = ctx.root().begin();

        let annotation = state.as_str();

        // Find the outermost member expression that contains .skip or .fixme
        // We need to remove just that segment from the chain
        find_and_remove_skip_member(&callee, annotation, &mut mutation)?;

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the ."<Emphasis>{annotation}</Emphasis>"() annotation." }.to_owned(),
            mutation,
        ))
    }
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
                // This is the .skip or .fixme member - replace the entire expression with object
                let object = member.object().ok()?;

                // Replace the entire static member expression with just the object
                mutation.replace_node(
                    AnyJsExpression::JsStaticMemberExpression(member.clone()),
                    object,
                );
                return Some(());
            }

            // Check nested expression
            let obj = member.object().ok()?;
            find_and_remove_skip_member(&obj, target, mutation)
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            let member_expr = member.member().ok()?;
            if let AnyJsExpression::AnyJsLiteralExpression(lit) = &member_expr
                && let Some(string_lit) = lit.as_js_string_literal_expression()
            {
                let token = string_lit.value_token().ok()?;
                let text = token.text_trimmed();
                let unquoted = text.trim_matches(|c| c == '"' || c == '\'');

                if unquoted == target {
                    // This is ["skip"] or ["fixme"] - replace with object
                    let object = member.object().ok()?;
                    mutation.replace_node(
                        AnyJsExpression::JsComputedMemberExpression(member.clone()),
                        object,
                    );
                    return Some(());
                }
            }

            // Check nested expression
            let obj = member.object().ok()?;
            find_and_remove_skip_member(&obj, target, mutation)
        }
        _ => None,
    }
}
