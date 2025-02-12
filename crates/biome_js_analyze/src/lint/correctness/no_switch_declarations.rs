use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsDeclaration, AnyJsStatement, AnyJsSwitchClause, JsVariableStatement, TriviaPieceKind, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow lexical declarations in `switch` clauses.
    ///
    /// Lexical declarations in `switch` clauses are accessible in the entire `switch`.
    /// However, it only gets initialized when it is assigned, which will only happen if the `switch` clause where it is defined is reached.
    ///
    /// To ensure that the lexical declarations only apply to the current `switch` clause wrap your declarations in a block.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case 0:
    ///         const x = 1;
    ///         break;
    ///     case 2:
    ///         x; // `x` can be used while it is not initialized
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case 0:
    ///         function f() {}
    ///         break;
    ///     case 2:
    ///         f(); // `f` can be called here
    ///         break;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// switch (foo) {
    ///     case 0:
    ///         class A {}
    ///         break;
    ///     default:
    ///         new A(); // `A` can be instantiated here
    ///         break;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// switch (foo) {
    ///     case 0: {
    ///         const x = 1;
    ///         break;
    ///     }
    ///     case 1:
    ///         // `x` is not visible here
    ///         break;
    /// }
    /// ```
    ///
    pub NoSwitchDeclarations {
        version: "1.0.0",
        name: "noSwitchDeclarations",
        language: "js",
        sources: &[RuleSource::Eslint("no-case-declarations")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoSwitchDeclarations {
    type Query = Ast<AnyJsSwitchClause>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let switch_clause = ctx.query();
        switch_clause
            .consequent()
            .syntax()
            .children()
            .filter_map(|node| {
                if JsVariableStatement::can_cast(node.kind()) {
                    Some(JsVariableStatement::cast(node)?.declaration().ok()?.range())
                } else if AnyJsDeclaration::can_cast(node.kind()) {
                    Some(node.text_trimmed_range())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, decl_range: &Self::State) -> Option<RuleDiagnostic> {
        let switch_clause = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            decl_range,
            markup! {
                "Other switch clauses can erroneously access this "<Emphasis>"declaration"</Emphasis>".\nWrap the declaration in a block to restrict its access to the switch clause."
            },
        ).detail(switch_clause.range(), markup! {
            "The declaration is defined in this "<Emphasis>"switch clause"</Emphasis>":"
        }))
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
                    .with_leading_trivia(Some((TriviaPieceKind::Whitespace, " ")))
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
            markup! { "Wrap the "<Emphasis>"declaration"</Emphasis>" in a block." }.to_owned(),
            mutation,
        ))
    }
}
