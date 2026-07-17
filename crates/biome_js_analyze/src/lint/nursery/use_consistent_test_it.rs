use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsName, JsCallExpression, JsLanguage, JsSyntaxToken, T};
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, TextRange};
use biome_rule_options::use_consistent_test_it::{TestFunctionKind, UseConsistentTestItOptions};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce consistent use of `it` or `test` for test functions.
    ///
    /// `it` and `test` are aliases for the same function in most test frameworks.
    /// This rule enforces using one over the other for consistency.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// test("foo", () => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// it("foo", () => {});
    /// ```
    ///
    /// ## Options
    ///
    /// ### `function`
    ///
    /// The function to use for top-level tests (outside `describe` blocks).
    /// Accepted values are:
    /// - `"it"` (default): Enforce using `it()` for top-level tests
    /// - `"test"`: Enforce using `test()` for top-level tests
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "function": "test"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// it("foo", () => {});
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// test("foo", () => {});
    /// ```
    ///
    /// ### `withinDescribe`
    ///
    /// The function to use for tests inside `describe` blocks.
    /// Accepted values are:
    /// - `"it"` (default): Enforce using `it()` inside describe blocks
    /// - `"test"`: Enforce using `test()` inside describe blocks
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "withinDescribe": "test"
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// describe("suite", () => {
    ///     it("foo", () => {});
    /// });
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// describe("suite", () => {
    ///     test("foo", () => {});
    /// });
    /// ```
    ///
    pub UseConsistentTestIt {
        version: "2.4.11",
        name: "useConsistentTestIt",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("consistent-test-it").inspired(),
            RuleSource::EslintVitest("consistent-test-it").inspired(),
        ],
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Test],
    }
}

pub struct ConsistentTestItState {
    /// The kind of rename to apply
    rename_kind: RenameKind,
    /// Range for the diagnostic (points to the base function name)
    range: TextRange,
}

/// Internal enum to track what kind of rename to apply
enum RenameKind {
    /// Rename `it` -> `test` (includes variants like `it.skip`, `it.only`, etc.)
    ItToTest,
    /// Rename `test` -> `it` (includes variants like `test.skip`, `test.only`, etc.)
    TestToIt,
    /// Rename `xit` -> `xtest`
    XitToXtest,
    /// Rename `xtest` -> `xit`
    XtestToXit,
    /// Rename `fit` -> `test.only`
    FitToTestOnly,
    /// Rename `fit` -> `it.only`
    FitToItOnly,
    /// Rename `test.only` -> `fit`
    TestOnlyToFit,
}

impl RenameKind {
    /// Returns `(current_display, suggested_display)` strings for use in
    /// diagnostics and action messages. Both strings are `'static` — no
    /// repetition of literals across `diagnostic` and `action`.
    const fn names(&self) -> (&'static str, &'static str) {
        match self {
            Self::ItToTest => (
                TestFunctionName::It.as_str(),
                TestFunctionName::Test.as_str(),
            ),
            Self::TestToIt => (
                TestFunctionName::Test.as_str(),
                TestFunctionName::It.as_str(),
            ),
            Self::XitToXtest => (
                TestFunctionName::Xit.as_str(),
                TestFunctionName::Xtest.as_str(),
            ),
            Self::XtestToXit => (
                TestFunctionName::Xtest.as_str(),
                TestFunctionName::Xit.as_str(),
            ),
            Self::FitToTestOnly => (TestFunctionName::Fit.as_str(), "test.only"),
            Self::FitToItOnly => (TestFunctionName::Fit.as_str(), "it.only"),
            Self::TestOnlyToFit => ("test.only", TestFunctionName::Fit.as_str()),
        }
    }
}

impl Rule for UseConsistentTestIt {
    type Query = Ast<JsCallExpression>;
    type State = ConsistentTestItState;
    type Signals = Option<Self::State>;
    type Options = UseConsistentTestItOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let options = ctx.options();

        // Get the required function kind based on context (inside/outside describe)
        let within_describe = is_within_describe(node);
        let required_kind = if within_describe {
            options.within_describe()
        } else {
            options.function()
        };

        let callee = node.callee().ok()?;

        // Get the base identifier name (it, test, xit, xtest, fit)
        let (base_name, base_token) = get_test_base_name(callee.clone())?;

        let rename_kind = match (base_name, required_kind) {
            // `it` when `test` is required
            (TestFunctionName::It, TestFunctionKind::Test) => RenameKind::ItToTest,
            // `test` when `it` is required
            (TestFunctionName::Test, TestFunctionKind::It) => RenameKind::TestToIt,
            // `xit` when `test` is required (becomes `xtest`)
            (TestFunctionName::Xit, TestFunctionKind::Test) => RenameKind::XitToXtest,
            // `xtest` when `it` is required (becomes `xit`)
            (TestFunctionName::Xtest, TestFunctionKind::It) => RenameKind::XtestToXit,
            // `fit` when `test` is required (becomes `test.only`)
            (TestFunctionName::Fit, TestFunctionKind::Test) => RenameKind::FitToTestOnly,
            // `fit` when `it` is required (becomes `it.only`)
            (TestFunctionName::Fit, TestFunctionKind::It) => RenameKind::FitToItOnly,
            // `test.only` when `it` is required (becomes `fit`)
            _ => {
                if required_kind == TestFunctionKind::It
                    && base_name == TestFunctionName::Test
                    && is_static_member_only(&callee)
                {
                    RenameKind::TestOnlyToFit
                } else {
                    return None;
                }
            }
        };

        Some(ConsistentTestItState {
            rename_kind,
            range: base_token.text_trimmed_range(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (current, suggested) = state.rename_kind.names();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "The test function "<Emphasis>{current}</Emphasis>" is inconsistent with the configured preferred function "<Emphasis>{suggested}</Emphasis>"."
                },
            )
            .note(markup! {
                "Using a consistent function for tests improves readability and makes it easier to search for tests in the codebase."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let mut mutation = ctx.root().begin();

        let (_, suggested) = state.rename_kind.names();
        match state.rename_kind {
            RenameKind::ItToTest => {
                rename_base_identifier(&callee, TestFunctionName::Test, &mut mutation)?;
            }
            RenameKind::TestToIt => {
                rename_base_identifier(&callee, TestFunctionName::It, &mut mutation)?;
            }
            RenameKind::XitToXtest => {
                rename_base_identifier(&callee, TestFunctionName::Xtest, &mut mutation)?;
            }
            RenameKind::XtestToXit => {
                rename_base_identifier(&callee, TestFunctionName::Xit, &mut mutation)?;
            }
            RenameKind::FitToTestOnly => {
                fix_fit_to_member_only(&callee, TestFunctionName::Test, &mut mutation)?;
            }
            RenameKind::FitToItOnly => {
                fix_fit_to_member_only(&callee, TestFunctionName::It, &mut mutation)?;
            }
            RenameKind::TestOnlyToFit => {
                fix_test_only_to_fit(&callee, &mut mutation)?;
            }
        };
        let message = markup! { "Use "<Emphasis>{suggested}</Emphasis>" instead." }.to_owned();

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}

/// The recognized JS test function identifier names.
///
/// A single source of truth for all string literals used to match and
/// construct test function calls (`it`, `test`, `xit`, `xtest`, `fit`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TestFunctionName {
    It,
    Test,
    Xit,
    Xtest,
    Fit,
}

impl TestFunctionName {
    /// The JavaScript identifier string for this function name.
    const fn as_str(self) -> &'static str {
        match self {
            Self::It => "it",
            Self::Test => "test",
            Self::Xit => "xit",
            Self::Xtest => "xtest",
            Self::Fit => "fit",
        }
    }

    /// Parse from a string slice. Returns `None` for unrecognized names.
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "it" => Some(Self::It),
            "test" => Some(Self::Test),
            "xit" => Some(Self::Xit),
            "xtest" => Some(Self::Xtest),
            "fit" => Some(Self::Fit),
            _ => None,
        }
    }
}

/// Get the base name token of a test callee expression.
///
/// Returns `(name, token)` where `name` is the recognized [`TestFunctionName`]
/// and `token` is the corresponding syntax token.
///
/// Examples:
/// - `it(...)` → `(It, <it token>)`
/// - `it.only(...)` → `(It, <it token>)`
/// - `it.skip.each([])()` → `(It, <it token>)`
/// - `xit(...)` → `(Xit, <xit token>)`
/// - `fit(...)` → `(Fit, <fit token>)`
fn get_test_base_name(callee: AnyJsExpression) -> Option<(TestFunctionName, JsSyntaxToken)> {
    let callee = callee.omit_parentheses();
    let base = get_base_identifier(&callee)?;
    let name = TestFunctionName::from_str(base.text_trimmed())?;
    Some((name, base))
}

/// Recursively get the base identifier token from a callee expression chain.
///
/// For `it` → returns the `it` token
/// For `it.only` → returns the `it` token
/// For `it.only.each` → returns the `it` token
fn get_base_identifier(callee: &AnyJsExpression) -> Option<JsSyntaxToken> {
    match callee {
        AnyJsExpression::JsIdentifierExpression(id) => id.name().ok()?.value_token().ok(),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let obj = member.object().ok()?;
            get_base_identifier(&obj)
        }
        AnyJsExpression::JsTemplateExpression(tmpl) => {
            // For tagged template expressions like `it.each`...``()
            let tag = tmpl.tag()?;
            get_base_identifier(&tag)
        }
        _ => None,
    }
}

/// Check if the callee is a `<base>.only` static member expression.
///
/// Used to detect `test.only` before converting to `fit`.
fn is_static_member_only(callee: &AnyJsExpression) -> bool {
    if let AnyJsExpression::JsStaticMemberExpression(member) = callee
        && let Ok(AnyJsName::JsName(name)) = member.member()
        && let Ok(token) = name.value_token()
    {
        return token.text_trimmed() == "only";
    }
    false
}

/// Rename the base identifier in a callee expression to a new name.
///
/// For `it(...)` → renames `it` to `test`
/// For `it.only(...)` → renames `it` to `test` (producing `test.only(...)`)
fn rename_base_identifier(
    callee: &AnyJsExpression,
    new_name: TestFunctionName,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    let base = get_base_identifier(callee)?;
    mutation.replace_token(base, make::ident(new_name.as_str()));
    Some(())
}

/// Fix `fit(...)` → `<base>.only(...)`
///
/// Replaces the `fit` identifier with a `<base>.only` static member expression,
/// where `base` is either [`TestFunctionName::Test`] or [`TestFunctionName::It`].
///
/// - `fit(...)` → `test.only(...)` / `it.only(...)`
/// - `fit.skip(...)` → `test.only.skip(...)` / `it.only.skip(...)`
fn fix_fit_to_member_only(
    callee: &AnyJsExpression,
    base: TestFunctionName,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    let base_ref =
        make::js_identifier_expression(make::js_reference_identifier(make::ident(base.as_str())));
    let base_only = make::js_static_member_expression(
        AnyJsExpression::JsIdentifierExpression(base_ref),
        make::token(T![.]),
        AnyJsName::JsName(make::js_name(make::ident("only"))),
    );

    match callee {
        AnyJsExpression::JsIdentifierExpression(_) => {
            // `fit(...)` → replace the whole callee with `<base>.only`
            mutation.replace_node(
                callee.clone(),
                AnyJsExpression::JsStaticMemberExpression(base_only),
            );
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            // `fit.something(...)` → replace the object `fit` with `<base>.only`
            let obj = member.object().ok()?;
            mutation.replace_node(obj, AnyJsExpression::JsStaticMemberExpression(base_only));
        }
        _ => return None,
    }
    Some(())
}

/// Fix `test.only(...)` → `fit(...)`
///
/// Replaces the static member expression `test.only` with just `fit`.
fn fix_test_only_to_fit(
    callee: &AnyJsExpression,
    mutation: &mut BatchMutation<JsLanguage>,
) -> Option<()> {
    if let AnyJsExpression::JsStaticMemberExpression(_) = callee {
        let fit_ref =
            make::js_identifier_expression(make::js_reference_identifier(make::ident("fit")));
        mutation.replace_node(
            callee.clone(),
            AnyJsExpression::JsIdentifierExpression(fit_ref),
        );
        Some(())
    } else {
        None
    }
}

/// Check if a call expression is nested inside a `describe` block.
///
/// Walks up ancestors looking for a `JsCallExpression` whose callee starts with `describe`.
fn is_within_describe(node: &JsCallExpression) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .filter_map(JsCallExpression::cast)
        .any(|ancestor| {
            ancestor
                .callee()
                .ok()
                .is_some_and(|callee| callee.contains_describe_call())
        })
}
