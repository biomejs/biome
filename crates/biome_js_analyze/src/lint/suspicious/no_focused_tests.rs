use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSourceKind,
    context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{JsCallExpression, TextRange};
use biome_rowan::{AstNode, BatchMutationExt, NodeOrToken};

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

        // Special check for patterns like test.only.each()
        // Due to additional chaining (.only.each) we need check it first
        let callee_str = callee.to_string();
        if callee_str.contains(&format!(".{ONLY_KEYWORD}.each")) {
            // Try to find the "only" member in a chain like test.only.each
            let range = callee
                .as_js_static_member_expression()
                .and_then(|static_member| {
                    static_member.object().ok().and_then(|obj| {
                        obj.as_js_static_member_expression().and_then(|parent| {
                            parent.member().ok().and_then(|member| {
                                if member.to_string() == ONLY_KEYWORD {
                                    Some(member.syntax().text_trimmed_range())
                                } else {
                                    None
                                }
                            })
                        })
                    })
                });

            if range.is_some() {
                return range;
            }

            // Fallback to providing any reasonable range
            return callee
                .get_callee_member_name()
                .map(|name| name.text_trimmed_range());
        }

        if node.is_test_call_expression().ok()? {
            let callee = node.callee().ok()?;
            if callee.contains_a_test_pattern().ok()? {
                let function_name = callee.get_callee_member_name()?;
                if FUNCTION_NAMES.contains(&function_name.text_trimmed()) {
                    return Some(function_name.text_trimmed_range());
                }

                // Check for direct "only" member in static member expression
                let range = callee
                    .as_js_static_member_expression()
                    .and_then(|static_member| {
                        static_member.member().ok().and_then(|member| {
                            if member.to_string() == ONLY_KEYWORD {
                                Some(member.syntax().text_trimmed_range())
                            } else {
                                None
                            }
                        })
                    });

                if range.is_some() {
                    return range;
                }
            }
        } else if let Some(expression) = callee.as_js_computed_member_expression() {
            // Handle computed member expressions like test["only"]
            let value_token = expression
                .as_fields()
                .object
                .ok()?
                .as_js_identifier_expression()?
                .name()
                .ok()?
                .value_token()
                .ok()?;

            // Early return if conditions aren't met
            if !expression.l_brack_token().is_ok()
                || !expression.r_brack_token().is_ok()
                || !CALLEE_NAMES.contains(&value_token.text_trimmed())
            {
                return None;
            }

            let member = expression.member().ok()?;
            let literal = member.as_any_js_literal_expression()?;

            if literal.as_js_string_literal_expression().is_some()
                && literal.to_string() == "\"only\""
            {
                return Some(expression.syntax().text_trimmed_range());
            }
        }

        None
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

        // Special handling for test.only.each() pattern
        let callee_str = callee.to_string();
        if callee_str.contains(&format!(".{ONLY_KEYWORD}.each")) {
            // Look for and remove ".only" from pattern like "test.only.each()" using functional chain
            let action = callee
                .as_js_static_member_expression()
                .and_then(|static_member| {
                    static_member.object().ok().and_then(|obj| {
                        obj.as_js_static_member_expression().and_then(|parent| {
                            parent.member().ok().and_then(|member| {
                                if member.to_string() == ONLY_KEYWORD {
                                    parent.operator_token().ok().map(|operator| {
                                        mutation.remove_element(member.into());
                                        mutation.remove_element(operator.into());
                                    })
                                } else {
                                    None
                                }
                            })
                        })
                    })
                });

            if action.is_some() {
                return Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove focus from test." },
                    mutation,
                ));
            }
        }

        // Process different code patterns using functional chains where possible

        // Check for function name patterns (only, fdescribe, fit)
        if let Some(function_name) = callee.get_callee_member_name() {
            match function_name.text_trimmed() {
                ONLY_KEYWORD => {
                    // Remove .only from expressions like test.only() using functional chain
                    if let Some(expression) = callee.as_js_static_member_expression() {
                        expression.member().ok().and_then(|member_name| {
                            expression.operator_token().ok().map(|operator_token| {
                                mutation.remove_element(member_name.into());
                                mutation.remove_element(operator_token.into());
                            })
                        });
                    }
                }
                FDESCRIBE_KEYWORD => {
                    // Replace fdescribe with describe
                    let replaced_function = make::js_reference_identifier(make::ident("describe"));
                    mutation.replace_element(function_name.into(), replaced_function.into());
                }
                FIT_KEYWORD => {
                    // Replace fit with it
                    let replaced_function = make::js_reference_identifier(make::ident("it"));
                    mutation.replace_element(function_name.into(), replaced_function.into());
                }
                _ => {}
            };
        }
        // Check for direct .only member using functional chain
        else if let Some(static_member) = callee.as_js_static_member_expression() {
            static_member.member().ok().and_then(|member| {
                if member.to_string() == ONLY_KEYWORD {
                    static_member.operator_token().ok().map(|operator| {
                        mutation.remove_element(member.into());
                        mutation.remove_element(operator.into());
                    })
                } else {
                    None
                }
            });
        }
        // Check for computed ["only"] member using functional chain
        else if let Some(expression) = callee.as_js_computed_member_expression() {
            let maybe_action = (|| {
                let l_brack = expression.l_brack_token().ok()?;
                let r_brack = expression.r_brack_token().ok()?;
                let member = expression.member().ok()?;
                let literal = member.as_any_js_literal_expression()?;

                if literal.as_js_string_literal_expression().is_some()
                    && literal.to_string() == "\"only\""
                {
                    mutation.remove_element(NodeOrToken::Token(l_brack));
                    mutation.remove_element(NodeOrToken::Node(literal.syntax().clone()));
                    mutation.remove_element(NodeOrToken::Token(r_brack));
                    Some(())
                } else {
                    None
                }
            })();

            if maybe_action.is_none() {
                return None;
            }
        }

        // We'll just create the action - any mutations will be applied
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove focus from test." },
            mutation,
        ))
    }
}
