use super::fixed_literal;
use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};
use biome_markdown_syntax::{MarkdownSyntaxKind, MarkdownSyntaxToken};
use biome_rowan::Direction;

fn code_literal(source: &str) -> MarkdownSyntaxToken {
    parse_markdown(source, MarkdownParserOptions::default())
        .syntax()
        .descendants_tokens(Direction::Next)
        .find(|token| token.kind() == MarkdownSyntaxKind::MD_CODE_LITERAL)
        .expect("the source has a fenced code block")
}

#[test]
fn keeps_the_trivia_of_the_fixed_code() {
    let token = code_literal("```js   \ndebugger;\nconst value = 1;\n```\n");

    let fixed = fixed_literal(&token, "   \nconst value = 1;\n").unwrap();

    assert_eq!(fixed.text(), "   \nconst value = 1;\n");
    assert_eq!(fixed.leading_trivia().text(), "   ");
}

#[test]
fn rejects_code_that_joins_the_opening_line() {
    let token = code_literal("```js\ndebugger;\n```\n");

    assert!(fixed_literal(&token, "debugger;\n").is_none());
}

#[test]
fn rejects_code_that_joins_the_closing_line() {
    let token = code_literal("```js\ndebugger;\n```\n");

    assert!(fixed_literal(&token, "\ndebugger;").is_none());
}

#[test]
fn rejects_code_that_changes_the_trivia() {
    let token = code_literal("```js   \ndebugger;\n```\n");

    assert!(fixed_literal(&token, "\ndebugger;\n").is_none());
}
