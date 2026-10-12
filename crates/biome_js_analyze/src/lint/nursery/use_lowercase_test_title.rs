use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, JsCallExpression, JsSyntaxKind,
    JsSyntaxToken,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_lowercase_test_title::UseLowercaseTestTitleOptions;

use crate::{JsRuleAction, frameworks::unit_tests::TestBlockKind};

declare_lint_rule! {
    /// Require test and test suite titles to start with a lowercase letter.
    ///
    /// Titles of `describe`, `test`, `it`, and `bench` blocks often read as a sentence,
    /// such as `it("returns the cached value")`.
    /// Starting every title with a lowercase letter keeps test reports consistent.
    ///
    /// Only the first character of the title is checked, so names and acronyms later in the title can keep their capitalization.
    /// Titles built from a template literal with `${}` placeholders are not checked.
    ///
    /// The fix is unsafe because test runners use titles to name stored snapshots and to select which tests to run.
    /// Renaming a test can leave its stored snapshot orphaned.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// it("Returns the cached value", () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// describe(`Cache`, () => {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// it("returns the cached value", () => {});
    /// describe("cache", () => {});
    /// test("parses JSON input", () => {});
    /// test("123 is a number", () => {});
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowedPrefixes`
    ///
    /// Type: `string[]`
    ///
    /// Default: `[]`
    ///
    /// A list of prefixes. Titles that start with one of these prefixes are not checked.
    /// Uppercase and lowercase letters are treated as different, so the prefix `GET` doesn't match a title that starts with `Get`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowedPrefixes": ["GET", "POST"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// test("Returns the list of users", () => {});
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// test("GET /users returns the list of users", () => {});
    /// describe("POST /users", () => {});
    /// ```
    ///
    pub UseLowercaseTestTitle {
        version: "next",
        name: "useLowercaseTestTitle",
        language: "js",
        recommended: false,
        sources: &[
            RuleSource::EslintJest("prefer-lowercase-title").same(),
            RuleSource::EslintVitest("prefer-lowercase-title").same(),
            RuleSource::EslintPlaywright("prefer-lowercase-title").same(),
        ],
        domains: &[RuleDomain::Test],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseLowercaseTestTitle {
    type Query = Ast<JsCallExpression>;
    /// The string literal token or template chunk token that holds the title text.
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = UseLowercaseTestTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if !is_titled_test_call(call) {
            return None;
        }

        let arguments = call.arguments().ok()?;
        let first_arg = arguments.args().into_iter().next()?.ok()?;
        let title = first_arg.as_any_js_expression()?;

        let (token, text) = match title {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(string),
            ) => {
                let token = string.value_token().ok()?;
                let text = string.inner_string_text().ok()?;
                (token, text)
            }
            AnyJsExpression::JsTemplateExpression(template) => {
                if template.tag().is_some() {
                    return None;
                }
                let mut elements = template.elements().into_iter();
                let AnyJsTemplateElement::JsTemplateChunkElement(chunk) = elements.next()? else {
                    return None;
                };
                if elements.next().is_some() {
                    return None;
                }
                let token = chunk.template_chunk_token().ok()?;
                let text = token.token_text_trimmed();
                (token, text)
            }
            _ => return None,
        };

        let text = text.text();
        if !starts_with_uppercase(text) {
            return None;
        }
        let is_allowed = ctx
            .options()
            .allowed_prefixes()
            .iter()
            .any(|prefix| text.starts_with(prefix.as_ref()));
        (!is_allowed).then_some(token)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _token: &Self::State) -> Option<RuleDiagnostic> {
        let title = ctx.query().arguments().ok()?.args().first()?.ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                title.range(),
                markup! {
                    "This test title starts with an uppercase letter."
                },
            )
            .note(markup! {
                "Test titles that consistently start with a lowercase letter make test reports easier to read."
            })
            .note(markup! {
                "Change the first letter of the title to lowercase."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, token: &Self::State) -> Option<JsRuleAction> {
        let text = token.text_trimmed();
        // String literal tokens include their opening quote.
        let start = if token.kind() == JsSyntaxKind::JS_STRING_LITERAL {
            1
        } else {
            0
        };
        let first = text[start..].chars().next()?;
        let rest = start + first.len_utf8();

        let mut new_text = String::with_capacity(text.len());
        new_text.push_str(&text[..start]);
        new_text.extend(first.to_lowercase());
        new_text.push_str(&text[rest..]);

        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(
            token.clone(),
            JsSyntaxToken::new_detached(token.kind(), &new_text, [], []),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Lowercase the first letter of the title." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if the first argument of `call` is the title of a test,
/// test suite, or benchmark.
fn is_titled_test_call(call: &JsCallExpression) -> bool {
    if TestBlockKind::from_call_expression(call).is_some() {
        return true;
    }
    let Ok(callee) = call.callee() else {
        return false;
    };
    match callee.omit_parentheses() {
        // test.each`table`("title", ...)
        AnyJsExpression::JsTemplateExpression(template) => template
            .tag()
            .is_some_and(|tag| tag.omit_parentheses().contains_a_test_each_pattern()),
        // test.skipIf(condition)("title", ...)
        AnyJsExpression::JsCallExpression(inner_call) => inner_call
            .callee()
            .is_ok_and(|inner_callee| is_conditional_test_callee(&inner_callee.omit_parentheses())),
        callee => is_bench_callee(callee),
    }
}

/// Returns `true` for a Vitest conditional test or suite such as
/// `test.skipIf` or `describe.runIf`.
fn is_conditional_test_callee(callee: &AnyJsExpression) -> bool {
    let AnyJsExpression::JsStaticMemberExpression(member) = callee else {
        return false;
    };
    let is_conditional = member
        .member()
        .ok()
        .and_then(|member| member.as_js_name()?.value_token().ok())
        .is_some_and(|name| matches!(name.text_trimmed(), "skipIf" | "runIf"));
    is_conditional
        && member
            .object()
            .is_ok_and(|object| object.omit_parentheses().contains_a_test_pattern())
}

/// Returns `true` for the callee of a Vitest benchmark: `bench`, `bench.only`,
/// `bench.skip`, or `bench.todo`.
fn is_bench_callee(callee: AnyJsExpression) -> bool {
    let callee = match callee {
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let is_modifier = member
                .member()
                .ok()
                .and_then(|member| member.as_js_name()?.value_token().ok())
                .is_some_and(|name| matches!(name.text_trimmed(), "only" | "skip" | "todo"));
            if !is_modifier {
                return false;
            }
            let Ok(object) = member.object() else {
                return false;
            };
            object.omit_parentheses()
        }
        callee => callee,
    };
    callee
        .as_js_identifier_expression()
        .and_then(|ident| ident.name().ok()?.value_token().ok())
        .is_some_and(|name| name.text_trimmed() == "bench")
}

/// Mirrors JavaScript's `c !== c.toLowerCase()` check, which also catches
/// titlecase letters such as `ǅ` that `char::is_uppercase` does not.
fn starts_with_uppercase(text: &str) -> bool {
    text.chars()
        .next()
        .is_some_and(|first| first.to_lowercase().ne([first]))
}
