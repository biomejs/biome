use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsTryStatement, JsStatementList, TextRange};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

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
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessCatch {
    type Query = Ast<AnyJsTryStatement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let catch_clause = node.catch_clause()?;
        let catch_body = catch_clause.body().ok()?;
        let catch_body_statements = catch_body.statements();

        // We need guarantees that body_statements has only one `throw` statement.
        if catch_body_statements.len() > 1 {
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

        let first_statement = catch_body_statements.first()?;
        let js_throw_statement = first_statement.as_js_throw_statement()?;
        let throw_ident = js_throw_statement
            .argument()
            .ok()?
            .as_js_identifier_expression()?
            .to_trimmed_string();

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
                markup!("The "<Emphasis>"catch"</Emphasis>" clause that only rethrows the original error is useless."),
            )
            .note(markup!(
                "An unnecessary "<Emphasis>"catch"</Emphasis>" clause can be confusing."
            )),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let note = if node.finally_clause().is_some() {
            let catch_clause = node.catch_clause()?;
            mutation.remove_node(catch_clause);
            "catch"
        } else {
            let try_stmts = node.body().ok()?.statements();
            let stmts_list = node.parent::<JsStatementList>()?;
            let node = node.syntax();
            let try_pos = stmts_list.iter().position(|x| x.syntax() == node)?;
            let prev_stmts = stmts_list.iter().take(try_pos);
            let next_stmts = stmts_list.iter().skip(try_pos + 1);
            let new_stmts = prev_stmts
                .chain(try_stmts)
                .chain(next_stmts)
                .collect::<Vec<_>>();
            let new_stmts_list = make::js_statement_list(new_stmts);
            mutation.replace_node_discard_trivia(stmts_list, new_stmts_list);
            "try/catch"
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup!("Remove the "<Emphasis>{note}</Emphasis>" clause.").to_owned(),
            mutation,
        ))
    }
}
