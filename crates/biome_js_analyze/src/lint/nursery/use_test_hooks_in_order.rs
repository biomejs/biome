use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsModuleItem, AnyJsStatement, JsCallExpression, JsModule, JsScript};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::use_test_hooks_in_order::UseTestHooksInOrderOptions;

use crate::frameworks::unit_tests::{LifecycleHook, describe_body_statements, is_describe_call};

declare_lint_rule! {
    /// Enforce that test lifecycle hooks are declared in the order they execute.
    ///
    /// Jest and Vitest always execute lifecycle hooks in the following order,
    /// regardless of how they are written in the file:
    ///
    /// 1. `beforeAll`
    /// 2. `beforeEach`
    /// 3. `afterEach`
    /// 4. `afterAll`
    ///
    /// Writing the hooks in a different order creates a discrepancy between
    /// the visual order in the source and the actual execution order, which
    /// makes test code harder to reason about.
    ///
    /// This rule flags any hook that appears after a hook that runs later in the
    /// execution order. Only consecutive groups of hooks in the same block are
    /// compared — test cases and other statements between hooks are allowed and
    /// reset the comparison baseline.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   beforeEach(() => {});
    ///   beforeAll(() => {});
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   afterEach(() => {});
    ///   afterAll(() => {});
    ///   beforeAll(() => {});
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// describe('foo', () => {
    ///   beforeAll(() => {});
    ///   beforeEach(() => {});
    ///   afterEach(() => {});
    ///   afterAll(() => {});
    /// });
    /// ```
    ///
    /// ```js
    /// // Hooks separated by test cases are treated independently.
    /// describe('foo', () => {
    ///   beforeEach(() => {});
    ///   it('a test', () => {});
    ///   afterAll(() => {});
    /// });
    /// ```
    ///
    /// See [`useTestHooksOnTop`](https://biomejs.dev/linter/rules/use-test-hooks-on-top) if you want to group all the hooks at the top of the block, before any test cases.
    ///
    pub UseTestHooksInOrder {
        version: "next",
        name: "useTestHooksInOrder",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("prefer-hooks-in-order").same(),
            RuleSource::EslintVitest("prefer-hooks-in-order").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

declare_node_union! {
    /// A node that represents a scope containing test/hook calls:
    /// either the top-level module/script, or a `describe(...)` call.
    pub AnyTestScope = JsModule | JsScript | JsCallExpression
}

/// A lifecycle hook that is out of order.
pub struct HookOrderViolation {
    /// The out-of-order hook.
    pub hook: JsCallExpression,
    /// The hook that precedes it but executes later, causing the violation.
    pub preceding_hook: JsCallExpression,
    /// The range of the preceding hook, for the detail span.
    pub preceding_hook_range: TextRange,
}

impl Rule for UseTestHooksInOrder {
    type Query = Ast<AnyTestScope>;
    type State = HookOrderViolation;
    type Signals = Vec<Self::State>;
    type Options = UseTestHooksInOrderOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let scope = ctx.query();

        // For a JsCallExpression, only act when it is a describe block.
        if let AnyTestScope::JsCallExpression(call) = scope {
            if !is_describe_call(call) {
                return vec![];
            }
            let Some(stmts) = describe_body_statements(call) else {
                return vec![];
            };
            return check_hooks_order(&stmts);
        }

        // For the top-level module or script, gather the top-level statements.
        let stmts: Vec<AnyJsStatement> = match scope {
            AnyTestScope::JsModule(module) => module
                .items()
                .into_iter()
                .filter_map(|item| {
                    if let AnyJsModuleItem::AnyJsStatement(stmt) = item {
                        Some(stmt)
                    } else {
                        None
                    }
                })
                .collect(),
            AnyTestScope::JsScript(script) => script.statements().into_iter().collect(),
            AnyTestScope::JsCallExpression(_) => unreachable!(),
        };
        check_hooks_order(&stmts)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let current_name =
            LifecycleHook::from_call_expression(&state.hook).map_or("hook", |h| h.as_str());
        let preceding_name = LifecycleHook::from_call_expression(&state.preceding_hook)
            .map_or("hook", |h| h.as_str());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.hook.range(),
                markup! {
                    <Emphasis>{current_name}</Emphasis>" should come after "<Emphasis>{preceding_name}</Emphasis>"."
                },
            )
            .detail(
                state.preceding_hook_range,
                markup! {
                    <Emphasis>{preceding_name}</Emphasis>" is declared here but executes after "<Emphasis>{current_name}</Emphasis>"."
                },
            )
            .note(markup! {
                "Hooks should appear in the order they execute: "<Emphasis>"beforeAll"</Emphasis>", "<Emphasis>"beforeEach"</Emphasis>", "<Emphasis>"afterEach"</Emphasis>", "<Emphasis>"afterAll"</Emphasis>"."
            }),
        )
    }
}

/// Scans a flat list of statements for lifecycle hooks that are declared out of
/// their canonical execution order.
///
/// Hooks are compared within contiguous groups: any non-hook statement (test
/// cases, variable declarations, describe blocks, etc.) resets the comparison
/// window. This mirrors the ESLint rule's behaviour — only adjacent hook runs
/// are checked against each other.
///
/// Only looks at the **direct children** of the current scope — does not
/// recurse. Recursion is handled by the query firing again for each nested
/// `describe` call.
fn check_hooks_order(statements: &[AnyJsStatement]) -> Vec<HookOrderViolation> {
    let mut violations: Vec<HookOrderViolation> = vec![];
    // Tracks the last hook seen in the current contiguous run.
    let mut last_hook: Option<(JsCallExpression, LifecycleHook)> = None;

    for stmt in statements {
        let Some(call) = stmt
            .as_js_expression_statement()
            .and_then(|e| e.expression().ok())
            .and_then(|e| e.as_js_call_expression().cloned())
        else {
            // A non-call statement breaks the contiguous hook run.
            last_hook = None;
            continue;
        };

        if let Some(hook) = LifecycleHook::from_call_expression(&call) {
            if let Some((ref prev_call, prev_hook)) = last_hook
                && hook < prev_hook
            {
                // The current hook executes *before* the previous one — violation.
                violations.push(HookOrderViolation {
                    hook: call.clone(),
                    preceding_hook: prev_call.clone(),
                    preceding_hook_range: prev_call.range(),
                });
            }

            last_hook = Some((call, hook));
        } else {
            // A non-hook call (test case, describe, utility call) breaks the run.
            last_hook = None;
        }
    }

    violations
}
