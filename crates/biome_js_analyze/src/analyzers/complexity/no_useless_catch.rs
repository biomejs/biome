use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{JsCatchClause, TextRange};
use biome_rowan::{AstNode, AstNodeList};

declare_rule! {
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
    pub(crate) NoUselessCatch {
        version: "1.0.0",
        name: "noUselessCatch",
        source: RuleSource::Eslint("no-useless-catch"),
        recommended: true,
    }
}

impl Rule for NoUselessCatch {
    type Query = Ast<JsCatchClause>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();

        let catch_body = binding.body().ok()?;
        let body_statements = catch_body.statements();

        // We need guarantees that body_statements has only one `throw` statement.
        if body_statements.len() != 1 {
            return None;
        }

        let catch_declaration = binding.declaration()?;
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
}
