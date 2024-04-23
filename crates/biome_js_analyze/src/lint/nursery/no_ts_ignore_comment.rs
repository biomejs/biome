use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsModuleItem, JsLanguage};
use biome_rowan::TriviaPieceKind::SingleLineComment;
use biome_rowan::{AstNode, SyntaxTriviaPiece};

const TS_IGNORE_COMMENT: &str = "// @ts-ignore";

declare_rule! {
    /// Disallow the use of '@ts-ignore' to suppress TypeScript errors.
    ///
    /// The `@ts-ignore` directive in TypeScript is used to suppress compiler errors, allowing
    /// potentially unsafe changes to exist in the code undetected. This rule is intended to
    /// prevent its use, thereby enforcing error handling and promoting safer, more reliable code.
    ///
    /// Suppressing compiler errors can lead to significant issues in both development
    /// and production environments by hiding underlying problems that need attention.
    ///
    /// Consider using `@ts-expect-error` with an explanation instead, which is a more explicit
    /// way to suppress errors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// // @ts-ignore
    /// const a: string = 123;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// // Not a ts-ignore comment
    /// const a: string = "123";
    /// ```
    ///
    pub NoTsIgnoreComment {
        version: "next",
        name: "noTsIgnoreComment",
        recommended: false,
    }
}

impl Rule for NoTsIgnoreComment {
    type Query = Ast<AnyJsModuleItem>;
    type State = SyntaxTriviaPiece<JsLanguage>;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();

        if query.syntax().has_leading_comments() {
            let ts_ignore_comment = query
                .syntax()
                .first_leading_trivia()?
                .pieces()
                .find(|piece| {
                    piece.kind() == SingleLineComment && piece.text().starts_with(TS_IGNORE_COMMENT)
                });

            if let Some(trivia) = ts_ignore_comment {
                return Some(trivia.clone());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.text_range(),
                "Do not use @ts-ignore.",
            )
            .note(markup! {
                "Using @ts-ignore suppresses all errors for the following line. \
                Consider fixing the issue or use `@ts-expect-error` instead."
            }),
        )
    }
}
