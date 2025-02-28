use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsModule, JsSyntaxToken, TextLen};
use biome_rowan::{AstNode, BatchMutationExt, Direction, TextRange, TextSize, TriviaPiece};

declare_lint_rule! {
    /// Prevents the use of the TypeScript directive `@ts-ignore`.
    ///
    /// The directive `@ts-ignore` suppresses all compilation errors, even ones that could be considered bugs
    /// coming from an upstream library or the compiler itself. If you use `@ts-ignore`, it won't be possible to know
    /// when and if the bug is fixed.
    ///
    /// The rule promotes the use the directive `@ts-expect-error`, which is meant to raise an error if there aren't any errors.
    /// This means that once the bug is fixed, you can delete the directive, safely.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// // @ts-ignore
    /// let foo;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// // @ts-expect-error
    /// let foo;
    /// ```
    ///
    pub NoTsIgnore {
        version: "next",
        name: "noTsIgnore",
        language: "js",
        sources: &[RuleSource::Eslint("ban-ts-comment")],
        recommended: true,
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Safe,
        severity: Severity::Warning,
    }
}

/// We track the token that has the trivia, and the range when the incorrect comment is the document
type RuleState = (JsSyntaxToken, TextRange);

impl Rule for NoTsIgnore {
    type Query = Ast<JsModule>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module = ctx.query();

        let mut tokens = vec![];
        for token in module.syntax().descendants_tokens(Direction::Next) {
            let leading_trivia = token.leading_trivia();
            let comments: Vec<_> = leading_trivia
                .pieces()
                .filter_map(|trivia| {
                    if let Some(comment) = trivia.as_comments() {
                        if let Some((index, _)) = comment.text().match_indices("@ts-ignore").next()
                        {
                            return Some((
                                token.clone(),
                                comment.text_range().add_start(TextSize::from(index as u32)),
                            ));
                        }
                    }
                    None
                })
                .collect();

            tokens.extend(comments);
        }

        tokens
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (token, range) = state;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Unsafe use of the "<Emphasis>"@ts-ignore"</Emphasis>" directive found in this comment."
                },
            )
            .detail(
                token.text_trimmed_range(),
                markup! {
                    "The directive is applied to this line."
                },
            )
            .note(markup! {
                "The "<Emphasis>"@ts-ignore"</Emphasis>" directive suppresses any kind of error, even possible errors that might be fixed by upstream libraries or the compiler itself."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let (token, _) = state;
        let token = token.clone();
        let mut mutation = ctx.root().begin();
        let mut new_trivia = vec![];
        let mut text = String::new();
        for trivia in token.clone().leading_trivia().pieces() {
            let kind = trivia.kind();
            if let Some(comment) = trivia.as_comments() {
                if comment.text().contains("@ts-ignore") {
                    let new_comment = comment.text().replace("@ts-ignore", "@ts-expect-error");
                    new_trivia.push(TriviaPiece::new(kind, new_comment.text_len()));
                    text.push_str(new_comment.as_str());
                } else {
                    new_trivia.push(TriviaPiece::new(kind, comment.text_len()));
                    text.push_str(comment.text());
                }
            } else {
                new_trivia.push(TriviaPiece::new(kind, trivia.text_len()));
                text.push_str(trivia.text());
            }
        }
        text.push_str(token.text_trimmed());
        let new_token = JsSyntaxToken::new_detached(token.kind(), text.as_str(), new_trivia, [])
            .with_trailing_trivia_pieces(token.trailing_trivia().pieces());

        mutation.replace_token_discard_trivia(token, new_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the "<Emphasis>"@ts-expect-error"</Emphasis>" directive instead." }
                .to_owned(),
            mutation,
        ))
    }
}
