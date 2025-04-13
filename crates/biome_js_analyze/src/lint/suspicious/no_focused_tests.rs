use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSourceKind,
    context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsLanguage, TextRange};
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, NodeOrToken};

declare_lint_rule! {
    /// Disallow focused tests.
    ///
    /// Disabled test are useful when developing and debugging, because it forces the test suite to run only certain tests.
    ///
    /// However, in pull/merge request, you usually want to run all the test suite.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe.only("foo", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.only("foo", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// test.only.each([["a"]])("%s", (a) => {});
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// test("foo", () => {});
    /// ```
    ///
    /// ```js
    /// test.each([["a"]])("%s", (a) => {});
    /// ```
    pub NoFocusedTests {
        version: "1.6.0",
        name: "noFocusedTests",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintJest("no-focused-tests")],
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Test],
    }
}

/// Focused test keyword as used in e.g. Jest or Vitest
const ONLY_KEYWORD: &str = "only";
/// Focused test keyword as used in e.g. Jasmine or Angular
const FDESCRIBE_KEYWORD: &str = "fdescribe";
/// Focused test keyword as used in e.g. Jasmine or Angular
const FIT_KEYWORD: &str = "fit";
const FUNCTION_NAMES: [&str; 3] = [ONLY_KEYWORD, FDESCRIBE_KEYWORD, FIT_KEYWORD];
const CALLEE_NAMES: [&str; 3] = ["describe", "it", "test"];

impl Rule for NoFocusedTests {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        // Check all focused test patterns in order:
        // 1. `only` as part of a chained call (e.g. `test.only.each()`)
        // 2. Function name patterns (e.g. `fdescribe`, `fit`)
        // 3. `only` as static member expression (e.g. `test.only()`)
        // 4. `only` as computed member expression (e.g. `test["only"]`)
        match_only_each_pattern(&callee)
            .or_else(|| match_function_name_pattern(&callee))
            .or_else(|| match_static_only_member(node, &callee))
            .or_else(|| match_computed_only_member(&callee))
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Don't focus the test."
                },
            )
            .note("The 'only' method is often used for debugging or during implementation. It should be removed before deploying to production.")
            .note("Consider removing 'only' to ensure all tests are executed.")
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let mut mutation = ctx.root().begin();

        // Apply mutations for each pattern type
        fix_only_each_pattern(&callee, &mut mutation)
            .or_else(|| fix_function_name_pattern(&callee, &mut mutation))
            .or_else(|| fix_static_only_member(&callee, &mut mutation))
            .or_else(|| fix_computed_only_member(&callee, &mut mutation))
            .map(|_| {
                JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove focus from test." },
                    mutation,
                )
            })
    }
}

// Pattern detection functions

/// Find the `.only.each` pattern in test calls
fn match_only_each_pattern(callee: &AnyJsExpression) -> Option<TextRange> {
    // Check if callee contains ".only.each" pattern using string conversion
    // This is still more efficient than traversing the AST when the pattern isn't present
    let callee_str = callee.to_string();
    if !callee_str.contains(".only.each") {
        return None;
    }

    // Find the "only" member in a chain like test.only.each
    callee
        .as_js_static_member_expression()
        .and_then(|static_member| {
            static_member.object().ok().and_then(|obj| {
                obj.as_js_static_member_expression().and_then(|parent| {
                    parent
                        .member()
                        .ok()
                        .filter(|member| member.syntax().text_trimmed() == ONLY_KEYWORD)
                        .map(|member| member.syntax().text_trimmed_range())
                })
            })
        })
        .or_else(|| {
            // Fallback to providing any reasonable range
            callee
                .get_callee_member_name()
                .map(|name| name.text_trimmed_range())
        })
}

/// Find focused test function names like fdescribe, fit
fn match_function_name_pattern(callee: &AnyJsExpression) -> Option<TextRange> {
    callee
        .get_callee_member_name()
        .filter(|function_name| FUNCTION_NAMES.contains(&function_name.text_trimmed()))
        .map(|function_name| function_name.text_trimmed_range())
}

/// Find static member expressions with `.only` member
fn match_static_only_member(
    node: &JsCallExpression,
    callee: &AnyJsExpression,
) -> Option<TextRange> {
    // Skip if not a test call expression - avoids unnecessary pattern checks
    if !matches!(node.is_test_call_expression().ok(), Some(true))
        || !matches!(callee.contains_a_test_pattern().ok(), Some(true))
    {
        return None;
    }

    callee
        .as_js_static_member_expression()
        .and_then(|static_member| {
            static_member
                .member()
                .ok()
                .filter(|member| member.syntax().text_trimmed() == ONLY_KEYWORD)
                .map(|member| member.syntax().text_trimmed_range())
        })
}

/// Find computed member expressions with ["only"] syntax
fn match_computed_only_member(callee: &AnyJsExpression) -> Option<TextRange> {
    let expression = callee.as_js_computed_member_expression()?;

    // Check for a valid test function name (describe, it, test)
    let object = expression.as_fields().object.ok()?;
    let identifier = object.as_js_identifier_expression()?;
    let value_token = identifier.name().ok()?.value_token().ok()?;

    // Early return if not a test callee
    if !CALLEE_NAMES.contains(&value_token.text_trimmed()) {
        return None;
    }

    // Verify brackets exist
    if !expression.l_brack_token().is_ok() || !expression.r_brack_token().is_ok() {
        return None;
    }

    // Check for "only" string literal
    let member = expression.member().ok()?;
    let literal = member.as_any_js_literal_expression()?;

    if literal.as_js_string_literal_expression().is_some()
        && literal.syntax().text_trimmed() == "\"only\""
    {
        Some(expression.syntax().text_trimmed_range())
    } else {
        None
    }
}

// Fix application functions

/// Apply fix for test.only.each() pattern
fn fix_only_each_pattern(
    callee: &AnyJsExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    // Early check for pattern
    let callee_str = callee.to_string();
    if !callee_str.contains(".only.each") {
        return None;
    }

    // Remove the '.only' part from the member chain
    callee
        .as_js_static_member_expression()
        .and_then(|static_member| {
            static_member.object().ok().and_then(|obj| {
                obj.as_js_static_member_expression().and_then(|parent| {
                    parent
                        .member()
                        .ok()
                        .filter(|member| member.syntax().text_trimmed() == ONLY_KEYWORD)
                        .and_then(|member| {
                            parent.operator_token().ok().map(|operator| {
                                mutation.remove_element(member.into());
                                mutation.remove_element(operator.into());
                                ()
                            })
                        })
                })
            })
        })
}

/// Apply fix for function name patterns (only, fdescribe, fit)
fn fix_function_name_pattern(
    callee: &AnyJsExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    let function_name = callee.get_callee_member_name()?;

    match function_name.text_trimmed() {
        ONLY_KEYWORD => {
            // Remove .only from static member expressions
            callee
                .as_js_static_member_expression()
                .and_then(|expression| {
                    let member = expression.member().ok()?;
                    let operator = expression.operator_token().ok()?;
                    mutation.remove_element(member.into());
                    mutation.remove_element(operator.into());
                    Some(())
                })
        }
        FDESCRIBE_KEYWORD => {
            // Replace fdescribe with describe
            let replaced_function = make::js_reference_identifier(make::ident("describe"));
            mutation.replace_element(function_name.into(), replaced_function.into());
            Some(())
        }
        FIT_KEYWORD => {
            // Replace fit with it
            let replaced_function = make::js_reference_identifier(make::ident("it"));
            mutation.replace_element(function_name.into(), replaced_function.into());
            Some(())
        }
        _ => None,
    }
}

/// Apply fix for direct .only static member
fn fix_static_only_member(
    callee: &AnyJsExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    callee
        .as_js_static_member_expression()
        .and_then(|static_member| {
            static_member
                .member()
                .ok()
                .filter(|member| member.syntax().text_trimmed() == ONLY_KEYWORD)
                .and_then(|member| {
                    static_member.operator_token().ok().map(|operator| {
                        mutation.remove_element(member.into());
                        mutation.remove_element(operator.into());
                        ()
                    })
                })
        })
}

/// Apply fix for computed ["only"] member
fn fix_computed_only_member(
    callee: &AnyJsExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    let expression = callee.as_js_computed_member_expression()?;

    // Get member and validate it's "only"
    let member = expression.member().ok()?;
    let literal = member.as_any_js_literal_expression()?;

    if !literal.as_js_string_literal_expression().is_some()
        || literal.syntax().text_trimmed() != "\"only\""
    {
        return None;
    }

    // Remove the computed member brackets and the "only" string
    let l_brack = expression.l_brack_token().ok()?;
    let r_brack = expression.r_brack_token().ok()?;

    mutation.remove_element(NodeOrToken::Token(l_brack));
    mutation.remove_element(NodeOrToken::Node(literal.syntax().clone()));
    mutation.remove_element(NodeOrToken::Token(r_brack));

    Some(())
}
