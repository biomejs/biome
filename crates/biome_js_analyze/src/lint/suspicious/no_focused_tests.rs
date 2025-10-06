use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsLanguage, TextRange};
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, NodeOrToken, TokenText};
use biome_rule_options::no_focused_tests::NoFocusedTestsOptions;

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
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("no-focused-tests").inspired(),
            RuleSource::EslintVitest("no-focused-tests").inspired(),
        ],
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
const CALLEE_NAMES: [&str; 3] = ["describe", "it", "test"];

impl Rule for NoFocusedTests {
    type Query = Ast<JsCallExpression>;
    type State = FunctionNameAndRange;
    type Signals = Option<Self::State>;
    type Options = NoFocusedTestsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        // Case Focused Test via computed member expression with ["only"]
        // not covered by contains_focused_test()
        let computed_member_match = match_computed_only_member(&callee);
        if computed_member_match.is_some() {
            return computed_member_match;
        }

        // Quick check for other patterns - early exit if not a focused test
        if !callee.contains_focused_test().ok()? && !callee.contains_only_each_pattern().ok()? {
            return None;
        }

        // Check all focused test patterns in order:
        // 1. Function name patterns (e.g. `fdescribe`, `fit`)
        // 2. `only` as part of a chained call (e.g. `test.only.each()`)
        // 3. `only` as static member expression (e.g. `test.only()`)
        if callee.contains_only_each_pattern().ok()? {
            match_only_each_pattern(&callee)
        } else {
            match_function_name_pattern(&callee).or_else(|| match_static_only_member(&callee))
        }
    }

    fn diagnostic(
        _: &RuleContext<Self>,
        FunctionNameAndRange { name, text_range }: &Self::State,
    ) -> Option<RuleDiagnostic> {
        // if invoked in square brackets such as it["only"] computed member expression, strip quotes
        let name_str = name.text().replace('"', "");
        let secondary_note_suffix = "to ensure all tests are executed.";
        let secondary_note = {
            if name_str == ONLY_KEYWORD {
                format!("Consider removing '{name_str}' {secondary_note_suffix}",)
            } else {
                format!("Consider removing 'f' prefix from '{name_str}' {secondary_note_suffix}",)
            }
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                text_range,
                markup! {
                    "Don't focus the test."
                },
            )
                .note(markup! {"The '"{name_str}"' method is often used for debugging or during implementation."})
                .note(secondary_note)
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let mut mutation = ctx.root().begin();

        // Apply mutations for each pattern type, matching the order we use in run()
        // 1. Check for computed member expressions first - they're handled separately
        // 2. Check for function patterns (fdescribe, fit)
        // 3. Check for only.each patterns
        // 4. Check for static .only patterns
        fix_computed_only_member(&callee, &mut mutation)
            .or_else(|| fix_function_name_pattern(&callee, &mut mutation))
            .or_else(|| fix_only_each_pattern(&callee, &mut mutation))
            .or_else(|| fix_static_only_member(&callee, &mut mutation))
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

pub struct FunctionNameAndRange {
    name: TokenText,
    text_range: TextRange,
}

// Pattern detection functions

/// Find the `.only.each` pattern in test calls
fn match_only_each_pattern(callee: &AnyJsExpression) -> Option<FunctionNameAndRange> {
    callee
        .as_js_static_member_expression()
        .and_then(|static_member| {
            static_member
                .object()
                .ok()
                .and_then(|obj| match_static_only_member(&obj))
        })
        .or_else(|| {
            // Fallback to providing any reasonable range
            callee
                .get_callee_member_name()
                .map(|name| FunctionNameAndRange {
                    name: name.token_text_trimmed(),
                    text_range: name.text_trimmed_range(),
                })
        })
}

/// Find focused test function names like fdescribe, fit
fn match_function_name_pattern(callee: &AnyJsExpression) -> Option<FunctionNameAndRange> {
    // Get the specific token for the diagnostic range
    callee
        .get_callee_member_name()
        .filter(|function_name| {
            let name = function_name.text_trimmed();
            // Detect f-prefixed functions like fdescribe, fit
            name.starts_with('f')
                && (name == FDESCRIBE_KEYWORD || name == FIT_KEYWORD || name == "ftest")
        })
        .map(|function_name| FunctionNameAndRange {
            name: function_name.token_text_trimmed(),
            text_range: function_name.text_trimmed_range(),
        })
}

/// Find static member expressions with `.only` member
fn match_static_only_member(callee: &AnyJsExpression) -> Option<FunctionNameAndRange> {
    // Find the specific 'only' member to highlight in the diagnostic
    callee
        .as_js_static_member_expression()
        .and_then(|static_member| {
            static_member
                .member()
                .ok()
                .filter(|member| member.syntax().text_trimmed() == ONLY_KEYWORD)
                .and_then(|member| {
                    member
                        .value_token()
                        .ok()
                        .map(|value_token| FunctionNameAndRange {
                            name: value_token.token_text_trimmed(),
                            text_range: member.syntax().text_trimmed_range(),
                        })
                })
        })
}

/// Find computed member expressions with `["only"]` syntax
fn match_computed_only_member(callee: &AnyJsExpression) -> Option<FunctionNameAndRange> {
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
    if expression.l_brack_token().is_err() || expression.r_brack_token().is_err() {
        return None;
    }

    // Check for "only" string literal
    let member = expression.member().ok()?;
    let literal = member.as_any_js_literal_expression()?;

    if literal.as_js_string_literal_expression().is_some()
        && literal.syntax().text_trimmed() == "\"only\""
    {
        Some(FunctionNameAndRange {
            name: literal.value_token().ok()?.token_text_trimmed(),
            text_range: expression.syntax().text_trimmed_range(),
        })
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
    if !callee.contains_only_each_pattern().ok()? {
        return None;
    }

    // Remove the '.only' part from the member chain
    callee
        .as_js_static_member_expression()
        .and_then(|static_member| {
            static_member
                .object()
                .ok()
                .and_then(|obj| fix_static_only_member(&obj, mutation))
        })
}

/// Apply fix for function name patterns (fdescribe, fit, ftest)
fn fix_function_name_pattern(
    callee: &AnyJsExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    // Skip if not a focused test
    if !matches!(callee.contains_focused_test().ok(), Some(true)) {
        return None;
    }

    let function_name = callee.get_callee_member_name()?;
    let name = function_name.text_trimmed();

    // Only handle f-prefixed functions here (fdescribe, fit, ftest)
    if !name.starts_with('f') {
        return None;
    }

    match name {
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
        "ftest" => {
            // Replace ftest with test
            let replaced_function = make::js_reference_identifier(make::ident("test"));
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
    let static_member = callee.as_js_static_member_expression()?;

    let member = static_member.member().ok()?;
    if member.syntax().text_trimmed() != ONLY_KEYWORD {
        return None;
    }

    // Get the operator token (the '.' in test.only)
    let operator = static_member.operator_token().ok()?;

    // Remove both the 'only' member and the '.' operator
    mutation.remove_element(member.into());
    mutation.remove_element(operator.into());

    Some(())
}

/// Apply fix for computed `["only"]` member expressions
fn fix_computed_only_member(
    callee: &AnyJsExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    let expression = callee.as_js_computed_member_expression()?;

    let object = expression.as_fields().object.ok()?;
    let identifier = object.as_js_identifier_expression()?;
    let value_token = identifier.name().ok()?.value_token().ok()?;

    if !CALLEE_NAMES.contains(&value_token.text_trimmed()) {
        return None;
    }

    let member = expression.member().ok()?;
    let literal = member.as_any_js_literal_expression()?;

    if literal.as_js_string_literal_expression().is_none()
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
