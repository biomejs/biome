use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    JsSyntaxKind, JsSyntaxToken, JsVariableDeclarationFields, JsVariableStatement,
    JsVariableStatementFields, TextSize, TriviaPieceKind, T,
};
use biome_rowan::{
    trim_leading_trivia_pieces, AstNode, AstSeparatedList, BatchMutationExt, TriviaPiece,
};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow multiple variable declarations in the same variable statement
    ///
    /// In JavaScript, multiple variables can be declared within a single `var`, `const` or `let` declaration.
    /// It is often considered a best practice to declare every variable separately.
    /// That is what this rule enforces.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let foo = 0, bar, baz;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = 0;
    /// let bar;
    /// let baz;
    /// ```
    ///
    /// ```js
    /// for (let i = 0, x = 1; i < arr.length; i++) {}
    /// ```
    pub UseSingleVarDeclarator {
        version: "1.0.0",
        name: "useSingleVarDeclarator",
        language: "js",
        sources: &[RuleSource::Eslint("one-var")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseSingleVarDeclarator {
    type Query = Ast<JsVariableStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        (ctx.query().declaration().ok()?.declarators().len() > 1).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            "Declare variables separately",
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let prev_parent = node.syntax().parent()?;
        if !matches!(
            prev_parent.kind(),
            JsSyntaxKind::JS_STATEMENT_LIST | JsSyntaxKind::JS_MODULE_ITEM_LIST
        ) {
            return None;
        }

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
        let index = prev_parent
            .children()
            .position(|slot| &slot == node.syntax())?;

        let kind_indent = kind
            .indentation_trivia_pieces()
            .map(|piece| (piece.kind(), piece.text().to_string()))
            .collect::<Vec<_>>();
        let kind_indent = if kind_indent.is_empty() {
            vec![(TriviaPieceKind::Newline, String::from("\n"))]
        } else {
            kind_indent
        };

        let declarators_len = declarators.len();
        let mut separators = declarators.separators();
        let last_semicolon_token = semicolon_token;
        let next_parent = prev_parent.clone().splice_slots(
            index..=index,
            declarators
                .iter()
                .enumerate()
                .filter_map(|(index, declarator)| {
                    // Remove the leading trivia for the declarators
                    let declarator = declarator.ok()?;
                    let declarator_leading_trivia = declarator.syntax().first_leading_trivia()?;
                    let declarator = declarator.with_leading_trivia_pieces([])?;

                    let kind = if index == 0 {
                        let kind = kind.clone();
                        // Clone the kind token with its entire leading trivia
                        // for the first statement
                        if let Some(last_piece) = kind.trailing_trivia().last() {
                            if last_piece.kind().is_single_line_comment() {
                                kind.prepend_trivia_pieces(trim_leading_trivia_pieces(
                                    kind.trailing_trivia().pieces(),
                                ))
                                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
                            } else {
                                kind
                            }
                        } else {
                            // Add a trailing space if the kind has no trailing trivia.
                            kind.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])
                        }
                    } else {
                        // For the remaining statements, clone the kind token
                        // with the leading trivia pieces previously removed
                        // from the declarator node, with the indentation
                        // fixed up to match the original kind token
                        let indent: &[(TriviaPieceKind, String)] = &kind_indent;
                        let mut trivia_pieces = Vec::new();
                        let mut token_text = String::new();
                        for piece in declarator_leading_trivia.pieces() {
                            if !piece.is_comments() {
                                continue;
                            }
                            for (kind, text) in indent {
                                trivia_pieces.push(TriviaPiece::new(*kind, TextSize::of(text)));
                                token_text.push_str(text);
                            }
                            trivia_pieces.push(TriviaPiece::new(piece.kind(), piece.text_len()));
                            token_text.push_str(piece.text());
                        }
                        for (kind, text) in indent {
                            trivia_pieces.push(TriviaPiece::new(*kind, TextSize::of(text)));
                            token_text.push_str(text);
                        }
                        token_text.push_str(kind.text_trimmed());
                        token_text.push(' ');
                        JsSyntaxToken::new_detached(
                            kind.kind(),
                            &token_text,
                            trivia_pieces,
                            [TriviaPiece::new(
                                TriviaPieceKind::Whitespace,
                                TextSize::from(1),
                            )],
                        )
                    };

                    let mut variable_declaration = make::js_variable_declaration(
                        kind,
                        make::js_variable_declarator_list([declarator], []),
                    );
                    if let Some(await_token) = await_token.clone() {
                        variable_declaration = variable_declaration.with_await_token(await_token);
                    }

                    let mut builder = make::js_variable_statement(variable_declaration.build());
                    if let Some(last_semicolon_token) = last_semicolon_token.as_ref() {
                        let semicolon_token = if index + 1 == declarators_len {
                            last_semicolon_token.clone()
                        } else {
                            make::token(T![;])
                        };
                        builder = builder.with_semicolon_token(semicolon_token)
                    }
                    let mut result = builder.build();
                    if let Some(Ok(separator)) = separators.next() {
                        if separator.has_trailing_comments() {
                            result = result
                                .append_trivia_pieces(separator.trailing_trivia().pieces())?;
                        }
                    }

                    Some(Some(result.into_syntax().into()))
                }),
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_element(prev_parent.into(), next_parent.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Break out into multiple declarations" }.to_owned(),
            mutation,
        ))
    }
}
