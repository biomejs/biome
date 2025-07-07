use std::borrow::Cow;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsModule, JsSyntaxToken};
use biome_jsdoc_comment::JsdocComment;
use biome_rowan::{
    AstNode, BatchMutationExt, Direction, TextLen, TextRange, TextSize, TriviaPiece,
};
use biome_rule_options::use_single_js_doc_asterisk::UseSingleJsDocAsteriskOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce JSDoc comment lines to start with a single asterisk, except for the first one.
    ///
    /// This rule ensures that every line in a JSDoc block, except the opening one, starts with exactly one asterisk (`*`).
    /// Extra asterisks are unnecessary in JSDoc comments and are often introduced by mistake.
    ///
    /// Double asterisks (`**`) are still allowed, because they mark the start of bold text.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// /**
    /// ** Description
    /// */
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /**
    /// * Description
    /// * */
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /** @ts-ignore **/
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// /**
    ///  * Description
    ///  * @public
    ///  */
    /// ```
    ///
    /// ```js
    /// /** @ts-ignore */
    /// ```
    ///
    /// ```js
    /// /**
    ///  * **Bold** text
    ///  */
    /// ```
    ///
    pub UseSingleJsDocAsterisk {
        version: "2.0.0",
        name: "useSingleJsDocAsterisk",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintJsDoc("no-multi-asterisks").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseSingleJsDocAsterisk {
    type Query = Ast<JsModule>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = UseSingleJsDocAsteriskOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module = ctx.query();

        let mut tokens = vec![];
        for token in module.syntax().descendants_tokens(Direction::Next) {
            let leading_trivia = token.leading_trivia();
            let comments: Vec<_> = leading_trivia
                .pieces()
                .filter_map(|trivia| {
                    let comment = trivia.as_comments()?;
                    let text = comment.text();

                    let invalid_line = get_invalid_jsdoc_line(text)?;
                    let line_offset = get_line_char_start_index(text, invalid_line.line_index);

                    let start_size =
                        comment.text_range().start() + TextSize::from(line_offset as u32);
                    let range_modified = TextRange::new(
                        start_size,
                        start_size
                            + TextSize::from(
                                text.lines().nth(invalid_line.line_index)?.len() as u32
                            ),
                    );

                    Some(RuleState {
                        token: token.clone(),
                        range: range_modified,
                        is_end_line: invalid_line.is_end_line,
                        char_start: line_offset + invalid_line.char_start,
                        char_end: line_offset + invalid_line.char_end,
                        comment_text: text.into(),
                    })
                })
                .collect();

            tokens.extend(comments);
        }

        tokens
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let position = if state.is_end_line { "end" } else { "start" };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "JSDoc comment line should " {position} " with a single asterisk."
                },
            ).note(markup! {
                "In JSDoc comments, extra asterisks beyond the first are unnecessary and are often added by mistake."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let token = state.token.clone();
        let mut mutation = ctx.root().begin();
        let mut new_trivia = vec![];
        let mut text = String::new();
        for trivia in token.clone().leading_trivia().pieces() {
            let kind = trivia.kind();
            if let Some(comment) = trivia.as_comments() {
                let mut comment_text = Cow::Borrowed(comment.text());

                if comment.text() == state.comment_text.as_ref() {
                    let new_comment_text = format!(
                        "{}{}",
                        &comment_text[..state.char_start],
                        &comment_text[state.char_end..]
                    );
                    comment_text = Cow::Owned(new_comment_text);
                }

                new_trivia.push(TriviaPiece::new(kind, comment_text.text_len()));
                text.push_str(comment_text.as_ref());
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
            markup! { "Remove additional asterisks." }.to_owned(),
            mutation,
        ))
    }
}

pub struct RuleState {
    token: JsSyntaxToken,
    range: TextRange,
    is_end_line: bool,
    char_start: usize,
    char_end: usize,
    comment_text: Box<str>,
}

pub struct InvalidJsDocLineIndexes {
    is_end_line: bool,
    line_index: usize,
    char_start: usize,
    char_end: usize,
}

fn char_is_whitespace(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

fn char_is_asterisk(c: u8) -> bool {
    c == b'*'
}

fn get_first_asterisk_index(line: &str) -> usize {
    line.as_bytes()
        .iter()
        .take_while(|&&b| !char_is_asterisk(b))
        .count()
        + 1
}

fn get_invalid_jsdoc_line_start(text: &str) -> Option<InvalidJsDocLineIndexes> {
    let lines = text.lines();
    let lines_count = lines.clone().count();

    let (invalid_line_index, invalid_line_char_end) = lines
        // Skip first/last lines
        .take(lines_count - 1)
        .enumerate()
        .skip(1)
        .find_map(|(line_index, line)| {
            let line = line.trim_start();
            let bytes = line.as_bytes();

            // If first symbol is not an asterisk, we can skip the line
            if bytes.first().is_some_and(|&b| !char_is_asterisk(b)) {
                return None;
            }

            let mut invalid_asterisk_index = None;

            let mut byte_it = bytes.iter().skip(1).enumerate().peekable();
            while let Some((char_index, &b)) = byte_it.next() {
                if char_is_asterisk(b) {
                    // double asterisk is valid
                    if byte_it
                        .peek()
                        .is_some_and(|&(_, &next_b)| char_is_asterisk(next_b))
                    {
                        return invalid_asterisk_index.map(|_| (line_index, char_index));
                    }

                    invalid_asterisk_index = Some(char_index);
                    continue;
                }

                if !char_is_whitespace(b) {
                    break;
                }
            }

            invalid_asterisk_index.map(|char_index| (line_index, char_index + 1))
        })?;

    let start = get_first_asterisk_index(text.lines().nth(invalid_line_index)?);

    Some(InvalidJsDocLineIndexes {
        is_end_line: false,
        line_index: invalid_line_index,
        char_start: start,
        char_end: start + invalid_line_char_end,
    })
}

fn get_invalid_jsdoc_last_line(text: &str) -> Option<InvalidJsDocLineIndexes> {
    let ending_marker = 2; // "*/"
    let last_line = text.lines().last()?.trim_end();

    let mut last_invalid_asterisk_index = None;

    let byte_it = last_line
        .as_bytes()
        .iter()
        .enumerate()
        .rev()
        .skip(ending_marker);

    for (char_index, &b) in byte_it {
        if char_is_asterisk(b) {
            last_invalid_asterisk_index = Some(char_index);
            continue;
        }

        if !char_is_whitespace(b) {
            break;
        }
    }

    Some(InvalidJsDocLineIndexes {
        is_end_line: true,
        line_index: text.lines().count() - 1,
        char_start: last_invalid_asterisk_index?,
        char_end: last_line.len() - ending_marker,
    })
}

fn get_invalid_jsdoc_line(text: &str) -> Option<InvalidJsDocLineIndexes> {
    if !JsdocComment::text_is_jsdoc_comment(text) {
        return None;
    }

    get_invalid_jsdoc_line_start(text).or_else(|| get_invalid_jsdoc_last_line(text))
}

fn get_line_char_start_index(text: &str, line_index: usize) -> usize {
    // Use split() instead of lines() to properly handle windows CRLF line endings
    text.split('\n').take(line_index).fold(0, |acc, line| {
        acc + line.len() + 1 // +1 for the newline character
    })
}
