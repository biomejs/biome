use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsModuleItem, AnyJsStatement, JsBlockStatement, JsCatchClause, JsExpressionStatement,
    JsModule, JsSyntaxToken, JsTryFinallyStatement, JsTryStatement, TextRange,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unnecessary `catch` clauses.
    ///
    /// A `catch` clause that only rethrows the original error is redundant,
    /// and has no effect on the runtime behavior of the program.
    /// These redundant clauses can be a source of confusion and code bloat,
    /// so it’s better to disallow these unnecessary `catch` clauses.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     doSomething();
    /// } catch(e) {
    ///     throw e;
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// try {
    ///     doSomething();
    /// } catch(e) {
    ///     throw e;
    /// } finally {
    ///     doCleanUp();
    /// }
    /// ```
    /// ### Valid
    ///
    /// ```js
    /// try {
    ///     doSomething();
    /// } catch(e) {
    ///     doSomethingWhenCatch();
    ///     throw e;
    /// }
    /// ```
    ///
    /// ```js
    /// try {
    ///     doSomething();
    /// } catch(e) {
    ///     handleError(e);
    /// }
    /// ```
    ///
    /// ```js
    /// try {
    ///     doSomething();
    /// } finally {
    ///     doCleanUp();
    /// }
    /// ```
    ///
    pub NoUselessCatch {
        version: "1.0.0",
        name: "noUselessCatch",
        language: "js",
        sources: &[RuleSource::Eslint("no-useless-catch")],
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessCatch {
    type Query = Ast<AnyJsStatementAndModule>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();

        let catch_clause = binding.catch_clause()?;

        let catch_body = catch_clause.body().ok()?;
        let body_statements = catch_body.statements();

        // We need guarantees that body_statements has only one `throw` statement.
        if body_statements.len() != 1 {
            return None;
        }

        let catch_declaration = catch_clause.declaration()?;
        let catch_binding_err = catch_declaration
            .binding()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?;
        let catch_err_name = catch_binding_err.text();

        let first_statement = body_statements.first()?;
        let js_throw_statement = first_statement.as_js_throw_statement()?;
        let throw_ident = js_throw_statement
            .argument()
            .ok()?
            .as_js_identifier_expression()?
            .text();

        if throw_ident.eq(catch_err_name) {
            Some(js_throw_statement.syntax().text_trimmed_range())
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, text_range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                text_range,
                markup!("The "<Emphasis>"catch"</Emphasis>" clause that only rethrows the original error is redundant."),
            )
            .note(markup!(
                "These unnecessary "<Emphasis>"catch"</Emphasis>" clauses can be confusing. It is recommended to remove them."
            )),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let body = node.body()?;
        let stmts = body.statements();
        let expressions: Vec<JsExpressionStatement> = stmts
            .iter()
            .map(|stmt| stmt.as_js_expression_statement().unwrap().clone())
            .collect();

        let catch_clause = node.catch_clause()?;
        let finally_clause = node.finally_clause();

        let mut mutation = ctx.root().begin();

        let note = if finally_clause.is_some() {
            mutation.remove_node(catch_clause);
            "catch"
        } else {
            let module = make::js_module(
                make::js_directive_list(None),
                make::js_module_item_list(expressions.into_iter().map(|expression| {
                    AnyJsModuleItem::AnyJsStatement(AnyJsStatement::JsExpressionStatement(
                        expression.clone(),
                    ))
                })),
                make::eof(),
            )
            .build();
            mutation.replace_node_discard_trivia(
                node.clone(),
                AnyJsStatementAndModule::JsModule(module),
            );
            "try/catch"
        };

        return Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup!("Remove the "<Emphasis>{note}</Emphasis>" clause.").to_owned(),
            mutation,
        ));
    }
}

declare_node_union! {
    pub AnyJsStatementAndModule = JsTryStatement | JsTryFinallyStatement | JsModule
}

impl AnyJsStatementAndModule {
    pub fn catch_clause(&self) -> Option<JsCatchClause> {
        match self {
            AnyJsStatementAndModule::JsTryStatement(node) => node.catch_clause().ok(),
            AnyJsStatementAndModule::JsTryFinallyStatement(node) => node.catch_clause(),
            AnyJsStatementAndModule::JsModule(_) => None,
        }
    }

    pub fn finally_clause(&self) -> Option<JsTryFinallyStatement> {
        match self {
            AnyJsStatementAndModule::JsTryStatement(_) => None,
            AnyJsStatementAndModule::JsTryFinallyStatement(node) => Some(node.clone()),
            AnyJsStatementAndModule::JsModule(_) => None,
        }
    }

    pub fn body(&self) -> Option<JsBlockStatement> {
        match self {
            AnyJsStatementAndModule::JsTryStatement(node) => node.body().ok(),
            AnyJsStatementAndModule::JsTryFinallyStatement(node) => node.body().ok(),
            AnyJsStatementAndModule::JsModule(_) => None,
        }
    }

    pub fn try_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsStatementAndModule::JsTryStatement(node) => node.try_token().ok(),
            AnyJsStatementAndModule::JsTryFinallyStatement(node) => node.try_token().ok(),
            AnyJsStatementAndModule::JsModule(_) => None,
        }
    }
}
