use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, JsCallExpression,
};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_identical_test_title::NoIdenticalTestTitleOptions;

use crate::frameworks::unit_tests::{
    AnyTestScope, get_unit_test_scope_calls, is_describe_call, is_unit_test,
};

declare_lint_rule! {
    /// Disallow identical titles in test suites and test cases.
    ///
    /// Having identical titles for two different tests or test suites at the same level may create confusion.
    /// For example, when a test fails it is hard to tell which test exactly failed based on its title alone.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   it('should do bar', () => {});
    ///   it('should do bar', () => {});
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   describe('baz', () => {});
    ///   describe('baz', () => {});
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// describe('foo', () => {
    ///   it('should do foo', () => {});
    ///   it('should do bar', () => {});
    /// });
    /// ```
    ///
    /// ```js
    /// describe('foo', () => {
    ///   describe('baz', () => {
    ///     it('should work', () => {});
    ///   });
    ///   describe('bar', () => {
    ///     it('should work', () => {});
    ///   });
    /// });
    /// ```
    ///
    pub NoIdenticalTestTitle {
        version: "next",
        name: "noIdenticalTestTitle",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("no-identical-title").same(),
            RuleSource::EslintVitest("no-identical-title").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

impl Rule for NoIdenticalTestTitle {
    type Query = Ast<AnyTestScope>;
    type State = JsCallExpression;
    type Signals = Vec<Self::State>;
    type Options = NoIdenticalTestTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let calls = get_unit_test_scope_calls(ctx.query());
        check_direct_children(calls)
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let kind = if is_describe_call(node) {
            "describe"
        } else {
            "test"
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Duplicate "<Emphasis>{kind}</Emphasis>" title found."
                },
            )
            .note(markup! {
                "A "<Emphasis>{kind}</Emphasis>" with this title already exists in the same scope."
            })
            .note(markup! {
                "Rename the "<Emphasis>{kind}</Emphasis>" to give it a unique, descriptive title."
            }),
        )
    }
}

/// Checks a flat list of statements for duplicate test/describe titles.
///
/// Only looks at the **direct children** of the current scope — does not
/// recurse. Recursion is handled by the query firing again for each nested
/// `describe` call.
///
/// Returns every call expression that is a duplicate (i.e. the second or later
/// occurrence of a given title at this level).
fn check_direct_children(
    calls: impl IntoIterator<Item = JsCallExpression>,
) -> Vec<JsCallExpression> {
    let mut describe_titles: Vec<TokenText> = vec![];
    let mut test_titles: Vec<TokenText> = vec![];
    let mut duplicates: Vec<JsCallExpression> = vec![];

    for call in calls {
        if is_describe_call(&call) {
            if let Some(title) = extract_call_title(&call) {
                if describe_titles.contains(&title) {
                    duplicates.push(call);
                } else {
                    describe_titles.push(title);
                }
            }
        } else if is_unit_test(&call)
            && let Some(title) = extract_call_title(&call)
        {
            if test_titles.contains(&title) {
                duplicates.push(call);
            } else {
                test_titles.push(title);
            }
        }
    }

    duplicates
}

/// Extracts the static title (first argument) from a test/describe call.
///
/// Returns `None` for dynamic titles (template literals with substitutions,
/// non-string first arguments, etc.). Only static string literals and
/// no-substitution template literals are compared.
fn extract_call_title(call: &JsCallExpression) -> Option<TokenText> {
    let args = call.arguments().ok()?;
    let first_arg = args.args().into_iter().next()?.ok()?;
    let expr = first_arg.as_any_js_expression()?;

    match expr {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(s),
        ) => s.inner_string_text().ok(),
        // No-substitution template literals (e.g. `same title`) can be
        // statically compared. Templates with substitutions cannot.
        AnyJsExpression::JsTemplateExpression(t) => {
            // Tagged templates are excluded — the tag may transform the value.
            if t.tag().is_some() {
                return None;
            }
            let mut elements = t.elements().into_iter();
            let first = elements.next()?;
            // Must have exactly one element and it must be a chunk (no substitutions).
            if elements.next().is_some() {
                return None;
            }
            match first {
                AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                    Some(chunk.template_chunk_token().ok()?.token_text())
                }
                AnyJsTemplateElement::JsTemplateElement(_) => None,
            }
        }
        _ => None,
    }
}
