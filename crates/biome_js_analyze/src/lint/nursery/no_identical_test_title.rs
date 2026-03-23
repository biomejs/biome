use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsModuleItem, AnyJsStatement,
    AnyJsTemplateElement, JsCallExpression, JsExpressionStatement, JsLanguage, JsModule,
    JsModuleItemList, JsScript, JsStatementList,
};
use biome_rowan::{AstNode, AstNodeList, TokenText};
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

#[derive(Clone)]
pub struct NoIdenticalTestTitleState {
    duplicate: Option<JsCallExpression>,
    suppressed_scopes: Vec<JsCallExpression>,
}

impl Rule for NoIdenticalTestTitle {
    type Query = Ast<JsCallExpression>;
    type State = NoIdenticalTestTitleState;
    type Signals = Vec<Self::State>;
    type Options = NoIdenticalTestTitleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let Some(statements) = scope_statements_for_call(call) else {
            return vec![];
        };

        if !is_scope_owner(call, &statements) {
            return vec![];
        }

        let mut scope_result = scan_scope(&statements);
        scope_result
            .suppressed_scopes
            .retain(|suppressed| suppressed.syntax() != call.syntax());

        let mut signals: Vec<Self::State> = scope_result
            .duplicates
            .into_iter()
            .map(|duplicate| NoIdenticalTestTitleState {
                duplicate: Some(duplicate),
                suppressed_scopes: vec![],
            })
            .collect();

        if !scope_result.suppressed_scopes.is_empty() {
            signals.push(NoIdenticalTestTitleState {
                duplicate: None,
                suppressed_scopes: scope_result.suppressed_scopes,
            });
        }

        signals
    }

    fn suppressed_nodes(
        _ctx: &RuleContext<Self>,
        state: &Self::State,
        suppressions: &mut RuleSuppressions<JsLanguage>,
    ) {
        for scope in &state.suppressed_scopes {
            suppressions.suppress_node(scope.syntax().clone());
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = state.duplicate.as_ref()?;
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

struct ScopeScanResult {
    duplicates: Vec<JsCallExpression>,
    suppressed_scopes: Vec<JsCallExpression>,
}

fn scan_scope(statements: &[AnyJsStatement]) -> ScopeScanResult {
    let mut describe_titles = vec![];
    let mut test_titles = vec![];
    let mut duplicates = vec![];
    let mut suppressed_scopes = vec![];

    for stmt in statements {
        let Some(call) = call_from_statement(stmt) else {
            continue;
        };

        if is_describe_call(&call) {
            if let Some(title) = extract_call_title(&call) {
                if describe_titles.contains(&title) {
                    duplicates.push(call.clone());
                } else {
                    describe_titles.push(title);
                }
            }

            if let Some(statements) = describe_body_statements(&call) {
                suppressed_scopes.push(call.clone());

                let nested_result = scan_scope(&statements);
                duplicates.extend(nested_result.duplicates);
                suppressed_scopes.extend(nested_result.suppressed_scopes);
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

    ScopeScanResult {
        duplicates,
        suppressed_scopes,
    }
}

fn scope_statements_for_call(call: &JsCallExpression) -> Option<Vec<AnyJsStatement>> {
    let expr_stmt = call.parent::<JsExpressionStatement>()?;

    if let Some(module_items) = expr_stmt.parent::<JsModuleItemList>() {
        return module_statements(&module_items);
    }

    let statement_list = expr_stmt.parent::<JsStatementList>()?;

    statement_list.parent::<JsScript>()?;
    Some(statement_list.into_iter().collect())
}

fn module_statements(module_items: &JsModuleItemList) -> Option<Vec<AnyJsStatement>> {
    module_items.parent::<JsModule>().map(|_| {
        module_items
            .iter()
            .filter_map(|item| match item {
                AnyJsModuleItem::AnyJsStatement(statement) => Some(statement),
                _ => None,
            })
            .collect()
    })
}

fn is_scope_owner(call: &JsCallExpression, statements: &[AnyJsStatement]) -> bool {
    first_relevant_call(statements).is_some_and(|first| first.syntax() == call.syntax())
}

fn first_relevant_call(statements: &[AnyJsStatement]) -> Option<JsCallExpression> {
    statements
        .iter()
        .filter_map(call_from_statement)
        .find(|call| is_describe_call(call) || is_unit_test(call))
}

fn describe_body_statements(call: &JsCallExpression) -> Option<Vec<AnyJsStatement>> {
    let args = call.arguments().ok()?;
    let [_, callback_arg] = args.get_arguments_by_index([0, 1]);
    let callback_arg = callback_arg?;
    let expr = callback_arg.as_any_js_expression()?;

    let body = match expr {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.body().ok()?,
        AnyJsExpression::JsFunctionExpression(function) => {
            AnyJsFunctionBody::JsFunctionBody(function.body().ok()?)
        }
        _ => return None,
    };

    let AnyJsFunctionBody::JsFunctionBody(block) = body else {
        return None;
    };

    Some(block.statements().into_iter().collect())
}

fn call_from_statement(stmt: &AnyJsStatement) -> Option<JsCallExpression> {
    let expr_stmt = stmt.as_js_expression_statement()?;
    let expr = expr_stmt.expression().ok()?;
    expr.as_js_call_expression().cloned()
}

fn extract_call_title(call: &JsCallExpression) -> Option<TokenText> {
    let args = call.arguments().ok()?;
    let first_arg = args.args().into_iter().next()?.ok()?;
    let expr = first_arg.as_any_js_expression()?;

    match expr {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => string.inner_string_text().ok(),
        AnyJsExpression::JsTemplateExpression(template) => {
            if template.tag().is_some() {
                return None;
            }

            let mut elements = template.elements().into_iter();
            let first = elements.next()?;
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
