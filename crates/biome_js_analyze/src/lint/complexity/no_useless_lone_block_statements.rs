use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsStatement, AnyJsSwitchClause, JsBlockStatement, JsFileSource, JsLabeledStatement,
    JsStatementList, JsSyntaxKind, JsVariableStatement,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

declare_lint_rule! {
    /// Disallow unnecessary nested block statements.
    ///
    /// > In JavaScript, prior to ES6, standalone code blocks delimited by curly braces do not create a new scope and have no use.
    /// > In ES6, code blocks may create a new scope if a block-level binding (let and const), a class declaration or a function declaration (in strict mode) are present. A block is not considered redundant in these cases.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (foo) {
    ///   bar();
    ///   {
    ///     baz();
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// while (foo) {
    ///   bar();
    /// }
    /// ```
    ///
    pub NoUselessLoneBlockStatements {
        version: "1.3.3",
        name: "noUselessLoneBlockStatements",
        language: "js",
        sources: &[RuleSource::Eslint("no-lone-blocks")],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessLoneBlockStatements {
    type Query = Ast<JsBlockStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let block = ctx.query();
        let is_module = ctx.source_type::<JsFileSource>().is_module();

        if JsLabeledStatement::can_cast(block.syntax().parent()?.kind()) {
            return None;
        }

        if AnyJsSwitchClause::can_cast(block.syntax().grand_parent()?.kind()) {
            return None;
        }

        if in_control_structure(block) {
            return None;
        }

        if block.statements().is_empty() {
            return Some(());
        }

        if block
            .statements()
            .iter()
            .any(|statement| statement_has_block_level_declaration(&statement, is_module))
        {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let block = ctx.query();
        let block_range = block.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                block_range,
                markup! {
                    "This block statement doesn't serve any purpose and can be safely removed."
                },
            )
                .note(markup! {
                "Standalone block statements without any block-level declarations are redundant in JavaScript and can be removed to simplify the code."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let block = ctx.query();
        let stmts_list = block.parent::<JsStatementList>()?;
        let block_pos = stmts_list
            .iter()
            .position(|x| x.syntax() == block.syntax())?;

        let inner_statements: Vec<AnyJsStatement> = block.statements().iter().collect();

        let prev_stmts = stmts_list.iter().take(block_pos);

        let next_stmts = stmts_list.iter().skip(block_pos + 1);

        let new_stmts: Vec<_> = prev_stmts
            .chain(inner_statements)
            .chain(next_stmts)
            .collect();

        let new_stmts_list = make::js_statement_list(new_stmts);

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(stmts_list, new_stmts_list);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove redundant block." }.to_owned(),
            mutation,
        ))
    }
}

fn statement_has_block_level_declaration(statement: &AnyJsStatement, is_module: bool) -> bool {
    match statement {
        AnyJsStatement::JsVariableStatement(variable) => is_not_var_declaration(variable),
        AnyJsStatement::JsFunctionDeclaration(_) => is_module,
        AnyJsStatement::JsClassDeclaration(_) => true,
        _ => false,
    }
}

fn in_control_structure(block: &JsBlockStatement) -> bool {
    if let Some(node) = block.syntax().parent() {
        let syntax_kind = node.kind();

        if let JsSyntaxKind::JS_ELSE_CLAUSE
        | JsSyntaxKind::JS_CATCH_CLAUSE
        | JsSyntaxKind::JS_FINALLY_CLAUSE = syntax_kind
        {
            return true;
        }
    }

    matches!(
        block.parent(),
        Some(
            AnyJsStatement::JsIfStatement(_)
                | AnyJsStatement::JsWhileStatement(_)
                | AnyJsStatement::JsDoWhileStatement(_)
                | AnyJsStatement::JsForStatement(_)
                | AnyJsStatement::JsForInStatement(_)
                | AnyJsStatement::JsForOfStatement(_)
                | AnyJsStatement::JsWithStatement(_)
                | AnyJsStatement::JsSwitchStatement(_)
                | AnyJsStatement::JsTryStatement(_)
                | AnyJsStatement::JsTryFinallyStatement(_)
        )
    )
}

fn is_not_var_declaration(variable: &JsVariableStatement) -> bool {
    variable
        .declaration()
        .ok()
        .is_some_and(|decl| !decl.is_var())
}
