use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsStatement, AnyJsSwitchClause, TriviaPieceKind, T};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforces switch clauses have a single statement, emits a quick fix wrapping the statements in a block.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case true:
    ///     case false:
    ///         let foo = '';
    ///         foo;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (foo) {
    ///     case true:
    ///     case false: {
    ///         let foo = '';
    ///         foo;
    ///     }
    /// }
    /// ```
    pub UseSingleCaseStatement {
        version: "1.0.0",
        name: "useSingleCaseStatement",
        language: "js",
        recommended: false,
        deprecated: "Use the rule noSwitchDeclarations instead",
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseSingleCaseStatement {
    type Query = Ast<AnyJsSwitchClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let switch_clause = ctx.query();
        let count = switch_clause
            .consequent()
            .iter()
            .filter(|stmt| !matches!(stmt, AnyJsStatement::JsBreakStatement(_)))
            .count();
        if count > 1 {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let switch_clause = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            switch_clause.consequent().range(),
            markup! {
                "A "<Emphasis>"switch clause"</Emphasis>" should only have a single statement."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let switch_clause = ctx.query();
        let clause_token = switch_clause.clause_token().ok()?;
        let colon_token = switch_clause.colon_token().ok()?;
        let consequent = switch_clause.consequent();
        let new_colon_token = colon_token.with_trailing_trivia([]);
        let new_consequent = make::js_statement_list(Some(AnyJsStatement::JsBlockStatement(
            make::js_block_statement(
                make::token(T!['{'])
                    .with_leading_trivia([(TriviaPieceKind::Whitespace, " ")])
                    .with_trailing_trivia_pieces(colon_token.trailing_trivia().pieces()),
                consequent.clone(),
                make::token(T!['}'])
                    .with_leading_trivia_pieces(clause_token.indentation_trivia_pieces()),
            ),
        )));
        let mut mutation = ctx.root().begin();
        mutation.replace_token_discard_trivia(colon_token, new_colon_token);
        mutation.replace_node_discard_trivia(consequent, new_consequent);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Wrap the statements in a block." }.to_owned(),
            mutation,
        ))
    }
}
