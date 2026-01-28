#![cfg(test)]
#![expect(unused_mut, unused_variables)]

use super::{MarkdownLexer, TextSize};
use crate::lexer::MarkdownLexContext;
use biome_markdown_syntax::MarkdownSyntaxKind::*;
use biome_parser::lexer::Lexer;
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = MarkdownLexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token(MarkdownLexContext::default()) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                biome_markdown_syntax::MarkdownSyntaxKind::$kind,
                "expected token kind {}, but found {:?}",
                stringify!($kind),
                tokens[idx].0,
            );

            assert_eq!(
                tokens[idx].1.len(),
                TextSize::from($len),
                "expected token length of {}, but found {:?} for token {:?}",
                $len,
                tokens[idx].1.len(),
                tokens[idx].0,
            );

            new_str.push_str(&$src[tokens[idx].1]);
            tok_idx += tokens[idx].1.len();

            idx += 1;
        )*

        if idx < tokens.len() {
            panic!(
                "expected {} tokens but lexer returned {}, first unexpected token is '{:?}'",
                idx,
                tokens.len(),
                tokens[idx].0
            );
        } else {
            assert_eq!(idx, tokens.len());
        }

        assert_eq!($src, new_str, "Failed to reconstruct input");
    }};
}

// This is for testing if the lexer is truly lossless
// It parses random strings and puts them back together with the produced tokens and compares
#[quickcheck]
fn losslessness(string: String) -> bool {
    // using an mpsc channel allows us to spawn a thread and spawn the lexer there, then if
    // it takes more than 2 seconds we panic because it is 100% infinite recursion
    let cloned = string.clone();
    let (sender, receiver) = channel();
    thread::spawn(move || {
        let mut lexer = MarkdownLexer::from_str(&cloned);
        let mut tokens = vec![];

        while lexer.next_token(MarkdownLexContext::default()) != EOF {
            tokens.push(lexer.current_range());
        }

        sender
            .send(tokens)
            .expect("Could not send tokens to receiver");
    });
    let token_ranges = receiver
        .recv_timeout(Duration::from_secs(2))
        .unwrap_or_else(|_| panic!("Lexer is infinitely recursing with this code: ->{string}<-"));

    let mut new_str = String::with_capacity(string.len());
    let mut idx = TextSize::from(0);

    for range in token_ranges {
        new_str.push_str(&string[range]);
        idx += range.len();
    }

    string == new_str
}

#[test]
fn empty() {
    assert_lex! {
        "",
    }
}

#[test]
fn textual() {
    assert_lex! {
        "a",
       MD_TEXTUAL_LITERAL:1,
    }
}

#[test]
fn new_line() {
    assert_lex! {
        "\n\r\n\r",
        NEWLINE:1,
        NEWLINE:2,
        NEWLINE:1,
    }
}

#[test]
fn tab() {
    assert_lex! {
        "\t",
        MD_TEXTUAL_LITERAL:1,
    }
}

#[test]
fn whitespace() {
    assert_lex! {
        " ",
        MD_TEXTUAL_LITERAL:1,
    }
}

#[test]
fn thematic_break_literal() {
    assert_lex! {
        r#"---
***
___
* * *
* * * *
_ _ _ _  _ "#,
        MD_THEMATIC_BREAK_LITERAL:3,
        NEWLINE:1,
        MD_THEMATIC_BREAK_LITERAL:3,
        NEWLINE:1,
        MD_THEMATIC_BREAK_LITERAL:3,
        NEWLINE:1,
        MD_THEMATIC_BREAK_LITERAL:5,
        NEWLINE:1,
        MD_THEMATIC_BREAK_LITERAL:7,
        NEWLINE:1,
        MD_THEMATIC_BREAK_LITERAL:11,
    }
}

#[test]
fn hash_token() {
    // Single hash for ATX header
    assert_lex! {
        "#",
        HASH:1,
    }
}

#[test]
fn multiple_hashes() {
    // Multiple hashes emitted as a single token - parser determines level from length
    assert_lex! {
        "###",
        HASH:3,
    }
}

#[test]
fn backtick_token() {
    // Single backtick for inline code
    assert_lex! {
        "`",
        BACKTICK:1,
    }
}

#[test]
fn triple_backtick() {
    // Triple backtick for fenced code blocks
    assert_lex! {
        "```",
        TRIPLE_BACKTICK:3,
    }
}

#[test]
fn tilde_token() {
    // Single tilde
    assert_lex! {
        "~",
        TILDE:1,
    }
}

#[test]
fn triple_tilde() {
    // Triple tilde for fenced code blocks
    assert_lex! {
        "~~~",
        TRIPLE_TILDE:3,
    }
}

#[test]
fn greater_than_token() {
    // Greater than for block quotes
    assert_lex! {
        ">",
        R_ANGLE:1,
    }
}

#[test]
fn greater_than_with_text() {
    // Block quote with content - text is grouped into a single token
    // Mid-line whitespace is now included in textual content,
    // so " text" becomes a single token
    assert_lex! {
        "> text",
        R_ANGLE:1,
        MD_TEXTUAL_LITERAL:1, // optional space after '>'
        MD_TEXTUAL_LITERAL:4, // "text"
    }
}

#[test]
fn plus_token() {
    // Plus for bullet list marker
    assert_lex! {
        "+",
        PLUS:1,
    }
}

#[test]
fn star_token_single() {
    // Single star followed by space (not a thematic break)
    // The trailing space is now included in textual content
    // since it's not at the start of a line
    assert_lex! {
        "* ",
        STAR:1,
        MD_TEXTUAL_LITERAL:1, // trailing space as text
    }
}

#[test]
fn brackets() {
    // Brackets for links - text is grouped into single tokens
    assert_lex! {
        "[text](url)",
        L_BRACK:1,
        MD_TEXTUAL_LITERAL:4, // "text" grouped
        R_BRACK:1,
        L_PAREN:1,
        MD_TEXTUAL_LITERAL:3, // "url" grouped
        R_PAREN:1,
    }
}

#[test]
fn bang_token() {
    // Exclamation for images
    assert_lex! {
        "!",
        BANG:1,
    }
}

#[test]
fn image_syntax() {
    // Image syntax - text is grouped into single tokens
    assert_lex! {
        "![alt](src)",
        BANG:1,
        L_BRACK:1,
        MD_TEXTUAL_LITERAL:3, // "alt" grouped
        R_BRACK:1,
        L_PAREN:1,
        MD_TEXTUAL_LITERAL:3, // "src" grouped
        R_PAREN:1,
    }
}

#[test]
fn star_and_underscore_emphasis() {
    // Single star for emphasis - text is grouped
    assert_lex! {
        "*text*",
        STAR:1,
        MD_TEXTUAL_LITERAL:4, // "text" grouped
        STAR:1,
    }
}

#[test]
fn double_star_emphasis() {
    // Double star for strong emphasis - text is grouped
    assert_lex! {
        "**bold**",
        DOUBLE_STAR:2,
        MD_TEXTUAL_LITERAL:4, // "bold" grouped
        DOUBLE_STAR:2,
    }
}

#[test]
fn underscore_token() {
    // Underscore token for emphasis - text is grouped
    assert_lex! {
        "_text_",
        UNDERSCORE:1,
        MD_TEXTUAL_LITERAL:4, // "text" grouped
        UNDERSCORE:1,
    }
}

#[test]
fn double_underscore_emphasis() {
    // Double underscore for strong emphasis - text is grouped
    assert_lex! {
        "__bold__",
        DOUBLE_UNDERSCORE:2,
        MD_TEXTUAL_LITERAL:4, // "bold" grouped
        DOUBLE_UNDERSCORE:2,
    }
}

#[test]
fn minus_token_single() {
    // Single minus followed by text (not a thematic break)
    // Mid-line whitespace is now included in textual content,
    // so " item" becomes a single token
    assert_lex! {
        "- item",
        MINUS:1,
        MD_TEXTUAL_LITERAL:5, // " item" grouped (space + text)
    }
}

#[test]
fn code_fence_with_language() {
    // Code fence with language specifier - language name is grouped
    assert_lex! {
        "```rust",
        TRIPLE_BACKTICK:3,
        MD_TEXTUAL_LITERAL:4, // "rust" grouped
    }
}

#[test]
fn escape_sequences() {
    // Backslash escapes punctuation characters
    assert_lex! {
        r#"\*\[\]"#,
        MD_TEXTUAL_LITERAL:2, // \*
        MD_TEXTUAL_LITERAL:2, // \[
        MD_TEXTUAL_LITERAL:2, // \]
    }
}

#[test]
fn escape_backslash() {
    // Escaped backslash
    assert_lex! {
        r#"\\"#,
        MD_TEXTUAL_LITERAL:2, // \\
    }
}

#[test]
fn escape_non_punctuation() {
    // Backslash before non-punctuation is just backslash
    assert_lex! {
        r#"\a"#,
        MD_TEXTUAL_LITERAL:1, // \
        MD_TEXTUAL_LITERAL:1, // a
    }
}

#[test]
fn hard_line_break_trailing_spaces() {
    // Two spaces followed by newline is a hard line break
    assert_lex! {
        "text  \nmore",
        MD_TEXTUAL_LITERAL:4, // "text"
        MD_HARD_LINE_LITERAL:3, // "  \n"
        MD_TEXTUAL_LITERAL:4, // "more"
    }
}

#[test]
fn hard_line_break_many_trailing_spaces() {
    // More than two spaces followed by newline is also a hard line break
    assert_lex! {
        "text    \nmore",
        MD_TEXTUAL_LITERAL:4, // "text"
        MD_HARD_LINE_LITERAL:5, // "    \n"
        MD_TEXTUAL_LITERAL:4, // "more"
    }
}

#[test]
fn hard_line_break_backslash_newline() {
    // Backslash followed by newline is a hard line break
    assert_lex! {
        "text\\\nmore",
        MD_TEXTUAL_LITERAL:4, // "text"
        MD_HARD_LINE_LITERAL:2, // "\\\n"
        MD_TEXTUAL_LITERAL:4, // "more"
    }
}

#[test]
fn not_hard_line_break_single_space() {
    // Single space followed by newline is NOT a hard line break
    assert_lex! {
        "text \nmore",
        MD_TEXTUAL_LITERAL:5, // "text " (space included in text)
        NEWLINE:1,
        MD_TEXTUAL_LITERAL:4, // "more"
    }
}

#[test]
fn ordered_list_marker_dot() {
    // Ordered list marker with dot
    assert_lex! {
        "1. item",
        MD_ORDERED_LIST_MARKER:2, // "1."
        MD_TEXTUAL_LITERAL:5, // " item" (space + text)
    }
}

#[test]
fn ordered_list_marker_paren() {
    // Ordered list marker with parenthesis
    assert_lex! {
        "1) item",
        MD_ORDERED_LIST_MARKER:2, // "1)"
        MD_TEXTUAL_LITERAL:5, // " item"
    }
}

#[test]
fn ordered_list_marker_multi_digit() {
    // Multi-digit ordered list marker
    assert_lex! {
        "123. item",
        MD_ORDERED_LIST_MARKER:4, // "123."
        MD_TEXTUAL_LITERAL:5, // " item"
    }
}

#[test]
fn ordered_list_marker_not_at_line_start() {
    // Digits not at line start should be textual
    assert_lex! {
        "text 1. more",
        MD_TEXTUAL_LITERAL:12, // entire line is textual
    }
}

#[test]
fn ordered_list_marker_no_space() {
    // "1.text" without space is not a list marker
    assert_lex! {
        "1.text",
        MD_TEXTUAL_LITERAL:6, // "1.text"
    }
}

#[test]
fn setext_underline_equals() {
    // Setext underline with equals at line start
    assert_lex! {
        "===",
        MD_SETEXT_UNDERLINE_LITERAL:3,
    }

    // Setext underline after newline
    assert_lex! {
        "text\n===",
        MD_TEXTUAL_LITERAL:4, // "text"
        NEWLINE:1,
        MD_SETEXT_UNDERLINE_LITERAL:3, // "==="
    }

    // Longer setext underline (9 equals)
    assert_lex! {
        "Heading 1\n=========",
        MD_TEXTUAL_LITERAL:9, // "Heading 1"
        NEWLINE:1,
        MD_SETEXT_UNDERLINE_LITERAL:9, // "========="
    }
}

#[test]
fn setext_underline_dashes() {
    // Single dash at line start is a setext underline (not enough for thematic break)
    assert_lex! {
        "-\n",
        MD_SETEXT_UNDERLINE_LITERAL:1,
        NEWLINE:1,
    }

    // Two dashes at line start is a setext underline
    assert_lex! {
        "--\n",
        MD_SETEXT_UNDERLINE_LITERAL:2,
        NEWLINE:1,
    }

    // Three+ dashes is a thematic break at lexer level
    // (parser will convert to setext if preceded by paragraph)
    assert_lex! {
        "---\n",
        MD_THEMATIC_BREAK_LITERAL:3,
        NEWLINE:1,
    }
}

#[test]
fn link_reference_definition_tokens() {
    // Test tokenizing a link reference definition
    // Whitespace after colon is included in textual (not at line start)
    assert_lex! {
        "[label]: https://example.com",
        L_BRACK:1,
        MD_TEXTUAL_LITERAL:5, // "label"
        R_BRACK:1,
        COLON:1,
        MD_TEXTUAL_LITERAL:20, // " https://example.com" (with leading space)
    }
}

#[test]
fn block_quote_with_header() {
    // Block quote with header inside
    assert_lex! {
        "> # Foo\n",
        R_ANGLE:1,
        MD_TEXTUAL_LITERAL:1, // " "
        HASH:1, // "#"
        MD_TEXTUAL_LITERAL:4, // " Foo"
        NEWLINE:1,
    }
}

#[test]
fn block_quote_simple() {
    // Simple block quote without header
    assert_lex! {
        "> This is a quote\n",
        R_ANGLE:1,
        MD_TEXTUAL_LITERAL:1, // " "
        MD_TEXTUAL_LITERAL:15, // "This is a quote"
        NEWLINE:1,
    }
}
