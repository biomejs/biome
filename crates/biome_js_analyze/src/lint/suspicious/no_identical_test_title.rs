use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsModuleItem, AnyJsStatement,
    AnyJsTemplateElement, JsCallExpression, JsModule, JsScript,
};
use biome_rowan::{AstNode, TextRange, TokenText, declare_node_union};
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
        version: "2.4.12",
        name: "noIdenticalTestTitle",
        language: "js",
        recommended: true,
        severity: Severity::Warning,
        sources: &[
            RuleSource::EslintJest("no-identical-title").same(),
            RuleSource::EslintVitest("no-identical-title").same(),
            RuleSource::EslintPlaywright("no-identical-title").same(),
        ],
        domains: &[RuleDomain::Test],
    }
}

impl Rule for NoIdenticalTestTitle {
    type Query = Ast<AnyTestScopeQuery>;
    type State = DuplicateTitle;
    type Signals = Vec<Self::State>;
    type Options = NoIdenticalTestTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let scope = ctx.query();

        // For a JsCallExpression, only act when it is a describe block.
        if let AnyTestScopeQuery::JsCallExpression(call) = scope {
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
            AnyTestScopeQuery::JsModule(module) => module
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
            AnyTestScopeQuery::JsScript(script) => script.statements().into_iter().collect(),
            AnyTestScopeQuery::JsCallExpression(_) => unreachable!(),
        };
        check_direct_children(&stmts)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let DuplicateTitle {
            kind,
            title,
            first,
            duplicates,
        } = state;
        let kind = kind.as_str();
        let title = title.text();

        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            first,
            markup! {
                "The "<Emphasis>{kind}</Emphasis>" title \""<Emphasis>{title}</Emphasis>"\" is used more than once in the same scope."
            },
        );

        for range in duplicates {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The "<Emphasis>{kind}</Emphasis>" title \""<Emphasis>{title}</Emphasis>"\" is repeated here."
                },
            );
        }

        Some(diagnostic.note(markup! {
            "Rename the duplicated "<Emphasis>{kind}</Emphasis>" calls to give each a unique, descriptive title."
        }))
    }
}

declare_node_union! {
    /// A node that represents a scope containing test/describe calls:
    /// either the top-level module/script, or a `describe(...)` call.
    pub AnyTestScopeQuery = JsModule | JsScript | JsCallExpression
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestTitleKind {
    Describe,
    Test,
}

impl TestTitleKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Describe => "describe",
            Self::Test => "test",
        }
    }
}

/// A title used by more than one sibling `describe` or test call.
#[derive(Debug)]
pub struct DuplicateTitle {
    pub(crate) kind: TestTitleKind,
    pub(crate) title: TokenText,
    /// The title at its first occurrence.
    pub(crate) first: TextRange,
    /// The title at every later occurrence, in source order. Never empty.
    pub(crate) duplicates: Vec<TextRange>,
}

/// Checks a flat list of statements for duplicate test/describe titles.
///
/// Only looks at the **direct children** of the current scope — does not
/// recurse. Recursion is handled by the query firing again for each nested
/// `describe` call.
///
/// Returns one entry per title used more than once, ordered by first
/// occurrence. `describe` and test titles are tracked separately.
fn check_direct_children(statements: &[AnyJsStatement]) -> Vec<DuplicateTitle> {
    let mut groups: Vec<DuplicateTitle> = vec![];

    for stmt in statements {
        let Some(call) = call_from_statement(stmt) else {
            continue;
        };

        let kind = if is_describe_call(&call) {
            TestTitleKind::Describe
        } else if is_unit_test(&call) {
            TestTitleKind::Test
        } else {
            continue;
        };
        let Some((title, range)) = extract_call_title(&call) else {
            continue;
        };

        match groups
            .iter_mut()
            .find(|group| group.kind == kind && group.title == title)
        {
            Some(group) => group.duplicates.push(range),
            None => groups.push(DuplicateTitle {
                kind,
                title,
                first: range,
                duplicates: vec![],
            }),
        }
    }

    groups.retain(|group| !group.duplicates.is_empty());
    groups
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

/// Extracts the static title (first argument) from a test/describe call,
/// along with the range of the title expression.
///
/// Returns `None` for dynamic titles (template literals with substitutions,
/// non-string first arguments, etc.). Only static string literals and
/// no-substitution template literals are compared.
fn extract_call_title(call: &JsCallExpression) -> Option<(TokenText, TextRange)> {
    let args = call.arguments().ok()?;
    let first_arg = args.args().into_iter().next()?.ok()?;
    let expr = first_arg.as_any_js_expression()?;
    let title = static_title_text(expr)?;
    Some((title, expr.range()))
}

fn static_title_text(expr: &AnyJsExpression) -> Option<TokenText> {
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
