use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsModule, JsSyntaxToken, TextLen};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TextSize, TriviaPiece};
use biome_rule_options::no_ts_ignore::NoTsIgnoreOptions;

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
        version: "2.0.0",
        name: "noTsIgnore",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("ban-ts-comment").inspired()],
        recommended: true,
        fix_kind: FixKind::Safe,
        severity: Severity::Warning,
    }
}

const TS_IGNORE_DIRECTIVE: &str = "@ts-ignore";
const TS_EXPECT_ERROR_DIRECTIVE: &str = "@ts-expect-error";

/// We track the token that has the trivia, and the range when the incorrect comment is the document
type RuleState = (JsSyntaxToken, TextRange);

impl Rule for NoTsIgnore {
    type Query = Ast<JsModule>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = NoTsIgnoreOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module = ctx.query().syntax();

        let mut tokens = vec![];
        // Borrowed tokens avoid allocating a cursor for each token of the
        // file; the token for the state is looked up only for a match.
        for token in module.borrowed_tokens() {
            for trivia in token.leading_trivia_pieces() {
                if !trivia.kind().is_comment() {
                    continue;
                }
                let Some(offset) = ts_ignore_directive_offset(trivia.text()) else {
                    continue;
                };
                let start = trivia.text_range().start() + TextSize::from(offset as u32);
                let range = TextRange::at(start, TS_IGNORE_DIRECTIVE.text_len());
                // The range lies inside the token's leading trivia, so the
                // token is the deepest element that covers it.
                if let Some(token) = module.covering_element(range).into_token() {
                    tokens.push((token, range));
                }
            }
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
        let (token, range) = state;
        let mut mutation = ctx.root().begin();
        let mut new_trivia = vec![];
        let mut text = String::new();
        for trivia in token.leading_trivia().pieces() {
            let kind = trivia.kind();
            let trivia_range = trivia.text_range();
            if trivia_range.contains_range(*range) {
                let start = usize::from(range.start() - trivia_range.start());
                let end = usize::from(range.end() - trivia_range.start());
                let comment = trivia.text();
                let new_comment = format!(
                    "{}{TS_EXPECT_ERROR_DIRECTIVE}{}",
                    &comment[..start],
                    &comment[end..]
                );
                new_trivia.push(TriviaPiece::new(kind, new_comment.text_len()));
                text.push_str(&new_comment);
            } else {
                new_trivia.push(TriviaPiece::new(kind, trivia.text_len()));
                text.push_str(trivia.text());
            }
        }
        text.push_str(token.text_trimmed());
        let new_token = JsSyntaxToken::new_detached(token.kind(), text.as_str(), new_trivia, [])
            .with_trailing_trivia_pieces(token.trailing_trivia().pieces());

        mutation.replace_token_discard_trivia(token.clone(), new_token);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the "<Emphasis>"@ts-expect-error"</Emphasis>" directive instead." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Returns the byte offset of `@ts-ignore` in `comment` when TypeScript
/// treats the comment as a `@ts-ignore` directive.
///
/// TypeScript requires the directive to be the leading content of a line
/// comment, after `//` or `///`. In a block comment, only the last line is
/// considered, and leading `/` and `*` characters are skipped, so
/// `/* @ts-ignore */` is a directive but a JSDoc line ` * @ts-ignore`
/// followed by more lines is not.
fn ts_ignore_directive_offset(comment: &str) -> Option<usize> {
    let rest = if let Some(rest) = comment.strip_prefix("//") {
        rest.strip_prefix('/').unwrap_or(rest)
    } else {
        let last_line = comment
            .rsplit(['\n', '\r', '\u{2028}', '\u{2029}'])
            .next()?;
        trim_js_whitespace_start(last_line).trim_start_matches(['/', '*'])
    };
    let rest = trim_js_whitespace_start(rest);
    rest.starts_with(TS_IGNORE_DIRECTIVE)
        .then(|| comment.len() - rest.len())
}

/// Trims the characters matched by `\s` in a JavaScript regular expression.
fn trim_js_whitespace_start(text: &str) -> &str {
    // Unicode `White_Space` differs from `\s` only by U+0085 and U+FEFF.
    text.trim_start_matches(|c: char| (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}')
}
