use biome_analyze::{
    AddVisitor, Phases, QueryMatch, Queryable, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    ServiceBag, Visitor, VisitorContext, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsLanguage};
use biome_rowan::{AstNode, Language, SyntaxKindSet, SyntaxNode, TextRange, WalkEvent};
use biome_rule_options::use_top_level_describe::UseTopLevelDescribeOptions;

use crate::frameworks::unit_tests::{LifecycleHook, TestBlockKind};

declare_lint_rule! {
    /// Require tests and hooks to be inside a `describe` block.
    ///
    /// A `describe` block gives a group of tests a shared name and keeps their
    /// hooks next to them. A hook is a function such as `beforeEach` or
    /// `afterAll` that runs before or after tests. Tests and hooks written
    /// outside of any `describe` block have no such group, which makes a test
    /// file harder to navigate as it grows.
    ///
    /// This rule reports calls that declare a test, such as `test` and `it`,
    /// and calls that declare a hook, when they are not inside a `describe`
    /// block. It can also limit how many `describe` blocks a file has at its
    /// top level.
    ///
    /// Tests and hooks are recognized by the name of the function, such as
    /// `test`, `it`, `beforeEach`, or `after`. The rule doesn't check where
    /// that function comes from, so an unrelated function with one of these
    /// names is reported as well.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test("adds two numbers", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// beforeEach(() => {});
    ///
    /// describe("calculator", () => {
    ///     test("adds two numbers", () => {});
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// describe("calculator", () => {
    ///     beforeEach(() => {});
    ///
    ///     test("adds two numbers", () => {});
    ///
    ///     describe("division", () => {
    ///         test("divides two numbers", () => {});
    ///     });
    /// });
    /// ```
    ///
    /// ## Options
    ///
    /// ### `maxTopLevelDescribes`
    ///
    /// Default: no limit
    ///
    /// The maximum number of `describe` blocks allowed at the top level of a file.
    /// A `describe` block that is inside another `describe` block doesn't count
    /// towards this limit.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "maxTopLevelDescribes": 1
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// describe("addition", () => {
    ///     test("adds two numbers", () => {});
    /// });
    ///
    /// describe("division", () => {
    ///     test("divides two numbers", () => {});
    /// });
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// describe("calculator", () => {
    ///     describe("addition", () => {
    ///         test("adds two numbers", () => {});
    ///     });
    ///
    ///     describe("division", () => {
    ///         test("divides two numbers", () => {});
    ///     });
    /// });
    /// ```
    ///
    pub UseTopLevelDescribe {
        version: "next",
        name: "useTopLevelDescribe",
        language: "js",
        recommended: false,
        severity: Severity::Information,
        sources: &[
            RuleSource::EslintJest("require-top-level-describe").same(),
            RuleSource::EslintVitest("require-top-level-describe").same(),
            RuleSource::EslintPlaywright("require-top-level-describe").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

/// A test, hook, or `describe` call that is not inside any `describe` call.
#[derive(Clone)]
pub struct TopLevelTestCall {
    call: JsCallExpression,
    kind: TopLevelTestCallKind,
}

impl Rule for UseTopLevelDescribe {
    type Query = TopLevelTestCall;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseTopLevelDescribeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query().kind {
            TopLevelTestCallKind::Test | TopLevelTestCallKind::Hook => Some(()),
            TopLevelTestCallKind::Describe { position } => {
                let max = ctx.options().max_top_level_describes?;
                (position > u32::from(max.get())).then_some(())
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let TopLevelTestCall { call, kind } = ctx.query();
        let range = test_function_range(call)?;
        Some(match kind {
            TopLevelTestCallKind::Test => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This test is not inside a "<Emphasis>"describe"</Emphasis>" block."
                },
            )
            .note(markup! {
                "A test outside of a "<Emphasis>"describe"</Emphasis>" block isn't grouped with related tests under a shared name, which makes the file harder to navigate as it grows."
            })
            .note(markup! {
                "Move this test into a "<Emphasis>"describe"</Emphasis>" block."
            }),
            TopLevelTestCallKind::Hook => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This hook is not inside a "<Emphasis>"describe"</Emphasis>" block."
                },
            )
            .note(markup! {
                "A hook outside of a "<Emphasis>"describe"</Emphasis>" block isn't tied to a named group of tests, which makes it harder to tell which tests depend on it."
            })
            .note(markup! {
                "Move this hook into the "<Emphasis>"describe"</Emphasis>" block that contains the tests it applies to."
            }),
            TopLevelTestCallKind::Describe { .. } => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This top-level "<Emphasis>"describe"</Emphasis>" block exceeds the maximum of "{ctx.options().max_top_level_describes?}" allowed in a file."
                },
            )
            .note(markup! {
                "A file with many top-level "<Emphasis>"describe"</Emphasis>" blocks is harder to navigate than a file that groups them under a shared name."
            })
            .note(markup! {
                "Move this block into another "<Emphasis>"describe"</Emphasis>" block, or move it to a separate file."
            }),
        })
    }
}

#[derive(Clone, Copy)]
enum TopLevelTestCallKind {
    Test,
    Hook,
    Describe {
        /// The 1-based position of this call among the `describe` calls of the
        /// file that are not inside another `describe` call.
        position: u32,
    },
}

impl QueryMatch for TopLevelTestCall {
    fn text_range(&self) -> TextRange {
        self.call.range()
    }
}

impl Queryable for TopLevelTestCall {
    type Input = Self;
    type Language = JsLanguage;
    type Output = Self;
    type Services = ();

    fn build_visitor(
        analyzer: &mut impl AddVisitor<Self::Language>,
        _: &<Self::Language as Language>::Root,
    ) {
        analyzer.add_visitor(Phases::Syntax, TopLevelTestCallVisitor::default);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        query.clone()
    }
}

#[derive(Default)]
struct TopLevelTestCallVisitor {
    /// The number of `describe` calls that contain the node being visited.
    describe_depth: u32,
    /// The number of `describe` calls entered so far that are not inside
    /// another `describe` call.
    top_level_describes: u32,
}

impl Visitor for TopLevelTestCallVisitor {
    type Language = JsLanguage;

    fn visited_kinds(
        &self,
        _: Option<SyntaxKindSet<JsLanguage>>,
    ) -> Option<SyntaxKindSet<JsLanguage>> {
        Some(JsCallExpression::KIND_SET)
    }

    fn visit(
        &mut self,
        event: &WalkEvent<SyntaxNode<Self::Language>>,
        mut ctx: VisitorContext<Self::Language>,
    ) {
        match event {
            WalkEvent::Enter(node) => {
                let Some(call) = JsCallExpression::cast_ref(node) else {
                    return;
                };
                let kind = match TestBlockKind::from_call_expression(&call) {
                    Some(TestBlockKind::Describe) => {
                        self.describe_depth += 1;
                        if self.describe_depth > 1 {
                            return;
                        }
                        self.top_level_describes = self.top_level_describes.saturating_add(1);
                        TopLevelTestCallKind::Describe {
                            position: self.top_level_describes,
                        }
                    }
                    _ if self.describe_depth > 0 => return,
                    Some(TestBlockKind::Test) if declares_test(&call) => TopLevelTestCallKind::Test,
                    None if LifecycleHook::from_call_expression(&call).is_some() => {
                        TopLevelTestCallKind::Hook
                    }
                    _ => return,
                };
                ctx.match_query(TopLevelTestCall { call, kind });
            }
            WalkEvent::Leave(node) => {
                if let Some(call) = JsCallExpression::cast_ref(node)
                    && TestBlockKind::from_call_expression(&call) == Some(TestBlockKind::Describe)
                {
                    self.describe_depth -= 1;
                }
            }
        }
    }
}

/// Returns the range of the function that declares a test, hook, or `describe`
/// block, leaving out the table or condition that some of them take first:
///
/// - `test` in `test("title", () => {})`
/// - `test.each` in `test.each(table)("title", () => {})`
/// - `describe.skipIf` in `describe.skipIf(condition)("title", () => {})`
fn test_function_range(call: &JsCallExpression) -> Option<TextRange> {
    let function = match call.callee().ok()?.omit_parentheses() {
        AnyJsExpression::JsCallExpression(table) => table.callee().ok()?,
        AnyJsExpression::JsTemplateExpression(table) => table.tag()?,
        callee => callee,
    };
    Some(function.range())
}

/// Returns `false` for the calls that [`TestBlockKind`] identifies as tests but
/// that don't declare a test to move into a `describe` block:
///
/// - Playwright steps (`test.step(...)`, `test.step.skip(...)`), which split up
///   the body of a test and are often written in helper functions.
/// - Playwright annotations (`test.skip()`, `test.fixme(condition, "reason")`),
///   which change how the tests around them run.
/// - `Deno.test(...)`, which belongs to Deno's built-in test runner. That
///   runner has no `describe` function.
/// - Calls to a function named `skip`. The Node.js test runner exports one
///   that declares a test, but the name is more often an unrelated function,
///   such as the RxJS operator.
fn declares_test(call: &JsCallExpression) -> bool {
    let Ok(mut callee) = call.callee() else {
        return false;
    };
    // An annotation is told apart from a skipped test by the last name of the
    // callee only: `test.skip.each(table)` always declares tests.
    let mut is_last_name = true;
    loop {
        match callee.omit_parentheses() {
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let name = member
                    .member()
                    .ok()
                    .and_then(|name| name.as_js_name()?.value_token().ok());
                match name.as_ref().map(|name| name.text_trimmed()) {
                    Some("step") => return false,
                    Some("skip" | "fixme") if is_last_name && !has_callback_after_title(call) => {
                        return false;
                    }
                    _ => {}
                }
                is_last_name = false;
                let Ok(object) = member.object() else {
                    return false;
                };
                callee = object;
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                return identifier
                    .name()
                    .and_then(|name| name.value_token())
                    .is_ok_and(|name| !matches!(name.text_trimmed(), "Deno" | "skip"));
            }
            // The function is the result of another call, as in
            // `test.each(table)("title", () => {})`.
            _ => return true,
        }
    }
}

/// Returns `true` if an argument after the first one is a function, as in
/// `test.skip("title", () => {})`.
///
/// A Playwright annotation takes no function there: its arguments are an
/// optional condition and an optional description.
fn has_callback_after_title(call: &JsCallExpression) -> bool {
    call.arguments().is_ok_and(|arguments| {
        arguments.args().into_iter().skip(1).any(|argument| {
            argument
                .ok()
                .and_then(|argument| argument.as_any_js_expression().cloned())
                .is_some_and(|argument| {
                    matches!(
                        argument.omit_parentheses(),
                        AnyJsExpression::JsArrowFunctionExpression(_)
                            | AnyJsExpression::JsFunctionExpression(_)
                    )
                })
        })
    })
}
