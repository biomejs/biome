use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsModuleItemList, JsStatementList, JsSyntaxToken, JsVariableDeclarationFields,
    JsVariableDeclaratorList, JsVariableStatement, JsVariableStatementFields, TextSize,
    TriviaPieceKind, T,
};
use rome_rowan::{AstNode, AstNodeExt, AstSeparatedList, BatchMutationExt, TriviaPiece};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow multiple variable declarations in the same variable statement
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let foo, bar;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (let i = 0, x = 1; i < arr.length; i++) {}
    /// ```
    pub(crate) UseSingleVarDeclarator {
        version: "1.0.0",
        name: "useSingleVarDeclarator",
        recommended: true,
    }
}

impl Rule for UseSingleVarDeclarator {
    type Query = Ast<JsVariableStatement>;
    type State = (
        Option<JsSyntaxToken>,
        JsSyntaxToken,
        JsVariableDeclaratorList,
        Option<JsSyntaxToken>,
    );
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        let JsVariableStatementFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        let JsVariableDeclarationFields {
            await_token,
            kind,
            declarators,
        } = declaration.ok()?.as_fields();

        let kind = kind.ok()?;

        if declarators.len() < 2 {
            return None;
        }

        Some((await_token, kind, declarators, semicolon_token))
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            "Declare variables separately",
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let prev_parent = node.syntax().parent()?;

        if !JsStatementList::can_cast(prev_parent.kind())
            && !JsModuleItemList::can_cast(prev_parent.kind())
        {
            return None;
        }

        let (await_token, kind, declarators, semicolon_token) = state;

        let index = prev_parent
            .children()
            .position(|slot| &slot == node.syntax())?;

        // Extract the indentation part from the leading trivia of the kind
        // token, defined as all whitespace and newline trivia pieces starting
        // from the token going backwards up to the first newline (included).
        // If the leading trivia for the token is empty, a single newline trivia
        // piece is created.  For efficiency, the trivia pieces are stored in
        // reverse order (the vector is then reversed again on iteration)
        let mut has_newline = false;
        let leading_trivia: Vec<_> = kind
            .leading_trivia()
            .pieces()
            .rev()
            .take_while(|piece| {
                let has_previous_newline = has_newline;
                has_newline |= piece.is_newline();
                !has_previous_newline
            })
            .filter_map(|piece| {
                if piece.is_whitespace() || piece.is_newline() {
                    Some((piece.kind(), piece.text().to_string()))
                } else {
                    None
                }
            })
            .collect();

        let kind_indent = if !leading_trivia.is_empty() {
            leading_trivia
        } else {
            vec![(TriviaPieceKind::Newline, String::from("\n"))]
        };

        let last_semicolon_token = semicolon_token.as_ref();
        let remaining_semicolon_token = semicolon_token.clone().map(|_| make::token(T![;]));

        let declarators_len = declarators.len();

        let next_parent = prev_parent.clone().splice_slots(
            index..=index,
            declarators
                .iter()
                .enumerate()
                .filter_map(|(index, declarator)| {
                    let mut declarator = declarator.ok()?;

                    // Remove the leading trivia for the declarators
                    let first_token = declarator.syntax().first_token()?;
                    let first_token_leading_trivia = first_token.leading_trivia();

                    declarator = declarator
                        .replace_token_discard_trivia(
                            first_token.clone(),
                            first_token.with_leading_trivia([]),
                        )
                        // SAFETY: first_token is a known child of declarator
                        .unwrap();

                    let kind = if index == 0 {
                        // Clone the kind token with its entire leading trivia
                        // for the first statement
                        kind.clone()
                    } else {
                        // For the remaining statements, clone the kind token
                        // with the leading trivia pieces previously removed
                        // from the first token of the declarator node, with
                        // the indentation fixed up to match the original kind
                        // token
                        let indent: &[(TriviaPieceKind, String)] = &kind_indent;
                        let mut trivia_pieces = Vec::new();
                        let mut token_text = String::new();

                        for piece in first_token_leading_trivia.pieces() {
                            if !piece.is_comments() {
                                continue;
                            }

                            for (kind, text) in indent.iter().rev() {
                                trivia_pieces.push(TriviaPiece::new(*kind, TextSize::of(text)));
                                token_text.push_str(text);
                            }

                            trivia_pieces.push(TriviaPiece::new(piece.kind(), piece.text_len()));
                            token_text.push_str(piece.text());
                        }

                        for (kind, text) in indent.iter().rev() {
                            trivia_pieces.push(TriviaPiece::new(*kind, TextSize::of(text)));
                            token_text.push_str(text);
                        }

                        token_text.push_str(kind.text_trimmed());

                        for piece in kind.trailing_trivia().pieces() {
                            token_text.push_str(piece.text());
                        }

                        JsSyntaxToken::new_detached(
                            kind.kind(),
                            &token_text,
                            trivia_pieces,
                            kind.trailing_trivia().pieces().map(|piece| {
                                TriviaPiece::new(piece.kind(), TextSize::of(piece.text()))
                            }),
                        )
                    };

                    let mut variable_declaration = make::js_variable_declaration(
                        kind,
                        make::js_variable_declarator_list([declarator], []),
                    );

                    if let Some(await_token) = await_token {
                        variable_declaration =
                            variable_declaration.with_await_token(await_token.clone());
                    }

                    let mut builder = make::js_variable_statement(variable_declaration.build());

                    let semicolon_token = if index + 1 == declarators_len {
                        last_semicolon_token
                    } else {
                        remaining_semicolon_token.as_ref()
                    };

                    if let Some(semicolon_token) = semicolon_token {
                        builder = builder.with_semicolon_token(semicolon_token.clone());
                    }

                    Some(Some(builder.build().into_syntax().into()))
                }),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_element(prev_parent.into(), next_parent.into());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Break out into multiple declarations" }.to_owned(),
            mutation,
        })
    }
}
