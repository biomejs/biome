use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsModuleItem, AnyJsStatement,
    AnyJsTemplateElement, JsCallExpression, JsModule, JsScript,
};
use biome_rowan::{AstNode, TokenText, declare_node_union};
use biome_rule_options::no_identical_test_title::NoIdenticalTestTitleOptions;

use crate::frameworks::unit_tests::{is_describe_call, is_unit_test};

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
    /// it('should do bar', () => {});
    /// it('should do bar', () => {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {
    ///   it('should do bar', () => {});
    ///   it('should do bar', () => {});
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// describe('foo', () => {});
    /// describe('foo', () => {});
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
    ///
    /// describe('bar', () => {});
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

declare_node_union! {
    /// A node that represents a scope containing test/describe calls:
    /// either the top-level module/script, or a `describe(...)` call.
    pub AnyTestScope = JsModule | JsScript | JsCallExpression
}

impl Rule for NoIdenticalTestTitle {
    type Query = Ast<AnyTestScope>;
    type State = JsCallExpression;
    type Signals = Vec<Self::State>;
    type Options = NoIdenticalTestTitleOptions;

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
            return check_direct_children(&stmts);
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
        check_direct_children(&stmts)
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
fn check_direct_children(statements: &[AnyJsStatement]) -> Vec<JsCallExpression> {
    let mut describe_titles = vec![];
    let mut test_titles = vec![];
    let mut duplicates = vec![];

    for stmt in statements {
        let Some(call) = call_from_statement(stmt) else {
            continue;
        };

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

/// Returns the list of direct-child statements from the callback passed to a
/// `describe(...)` call, or `None` if the callback is not a recognisable
/// function literal with a block body.
fn describe_body_statements(call: &JsCallExpression) -> Option<Vec<AnyJsStatement>> {
    let args = call.arguments().ok()?;
    let [_, callback_arg] = args.get_arguments_by_index([0, 1]);
    let callback_arg = callback_arg?;
    let expr = callback_arg.as_any_js_expression()?;

    let body = match expr {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.body().ok()?,
        AnyJsExpression::JsFunctionExpression(func) => {
            AnyJsFunctionBody::JsFunctionBody(func.body().ok()?)
        }
        _ => return None,
    };

    let AnyJsFunctionBody::JsFunctionBody(block) = body else {
        return None;
    };

    Some(block.statements().into_iter().collect())
}

/// Extracts the (single) `JsCallExpression` from an expression statement, if
/// the statement is of the form `call_expr(...)`.
fn call_from_statement(stmt: &AnyJsStatement) -> Option<JsCallExpression> {
    let expr_stmt = stmt.as_js_expression_statement()?;
    let expr = expr_stmt.expression().ok()?;
    expr.as_js_call_expression().cloned()
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
