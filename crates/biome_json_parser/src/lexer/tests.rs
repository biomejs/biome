#![cfg(test)]
#![expect(unused_mut)]

use super::{Lexer, TextSize};
use biome_json_syntax::JsonSyntaxKind::{self, EOF};
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = Lexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let tokens: Vec<_> = lexer.collect();

        $(
            assert_eq!(
                tokens[idx].kind,
                biome_json_syntax::JsonSyntaxKind::$kind,
                "expected token kind {}, but found {:?}",
                stringify!($kind),
                tokens[idx].kind,
            );

            assert_eq!(
                tokens[idx].range.len(),
                TextSize::from($len),
                "expected token length of {}, but found {:?} for token {:?}",
                $len,
                tokens[idx].range.len(),
                tokens[idx].kind,
            );

            new_str.push_str(&$src[tokens[idx].range]);
            tok_idx += tokens[idx].range.len();

            idx += 1;
        )*

        if idx < tokens.len() {
            panic!(
                "expected {} tokens but lexer returned {}, first unexpected token is '{:?}'",
                idx,
                tokens.len(),
                tokens[idx].kind
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
        let mut lexer = Lexer::from_str(&cloned);
        let tokens: Vec<_> = lexer.map(|token| token.range).collect();

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
        EOF:0
    }
}

#[test]
fn int() {
    assert_lex! {
        "5098382",
        JSON_NUMBER_LITERAL:7,
        EOF:0
    }
}

#[test]
fn float() {
    assert_lex! {
        "345.893872",
        JSON_NUMBER_LITERAL:10,
        EOF:0
    }
}

#[test]
fn float_invalid() {
    assert_lex! {
        "345.893872.43322",
        ERROR_TOKEN:16,
        EOF:0
    }
}

#[test]
fn negative() {
    assert_lex! {
        "-5098382",
        JSON_NUMBER_LITERAL:8,
        EOF:0
    }
}

#[test]
fn minus_without_number() {
    assert_lex! {
        "-",
        ERROR_TOKEN:1,
        EOF:0
    }
}

#[test]
fn exponent() {
    assert_lex! {
        "-493e+534",
        JSON_NUMBER_LITERAL:9,
        EOF:0
    }

    assert_lex! {
        "-493E-534",
        JSON_NUMBER_LITERAL:9,
        EOF:0
    }
}

#[test]
fn multiple_exponent() {
    assert_lex! {
        "-493e5E3",
        ERROR_TOKEN:8,
        EOF:0
    }

    assert_lex! {
        "-493e4E45",
        ERROR_TOKEN:9,
        EOF:0
    }
}

#[test]
fn array() {
    assert_lex! {
        "[1, 2, 3, 4]",
        L_BRACK:1,
        JSON_NUMBER_LITERAL:1,
        COMMA:1
        WHITESPACE:1,
        JSON_NUMBER_LITERAL:1,
        COMMA:1,
        WHITESPACE:1,
        JSON_NUMBER_LITERAL:1,
        COMMA:1,
        WHITESPACE:1,
        JSON_NUMBER_LITERAL:1,
        R_BRACK:1,
        EOF:0,
    }
}

#[test]
fn object() {
    assert_lex! {
        r#"{ "key": "value", "other": 4 }"#,
        L_CURLY:1,
        WHITESPACE:1,

        JSON_STRING_LITERAL:5,
        COLON:1,
        WHITESPACE:1,
        JSON_STRING_LITERAL:7,
        COMMA:1,

        WHITESPACE:1,
        JSON_STRING_LITERAL:7,
        COLON:1,
        WHITESPACE:1,
        JSON_NUMBER_LITERAL:1,
        WHITESPACE:1,
        R_CURLY:1,
        EOF:0,
    }
}

#[test]
fn basic_string() {
    assert_lex! {
        r#""A string consisting of ASCII characters only""#,
        JSON_STRING_LITERAL:46,
        EOF:0
    }
}

#[test]
fn single_quote_string() {
    assert_lex! {
        r#"'A string token using single quotes that are not supported in JSON'"#,
        ERROR_TOKEN:67,
        EOF:0
    }
}

#[test]
fn unterminated_string() {
    assert_lex! {
        r#""A string without the closing quote"#,
        ERROR_TOKEN:35,
        EOF:0
    }
}

#[test]
fn simple_escape_sequences() {
    assert_lex! {
        r#""Escaped \t""#,
        JSON_STRING_LITERAL:12,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \"""#,
        JSON_STRING_LITERAL:12,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \\""#,
        JSON_STRING_LITERAL:12,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \/""#,
        JSON_STRING_LITERAL:12,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \b""#,
        JSON_STRING_LITERAL:12,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \f""#,
        JSON_STRING_LITERAL:12,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \n""#,
        JSON_STRING_LITERAL:12,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \r""#,
        JSON_STRING_LITERAL:12,
        EOF:0
    }
}

#[test]
fn unicode_escape() {
    assert_lex! {
        r#""Escaped \u002F""#,
        JSON_STRING_LITERAL:16,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \u002f""#,
        JSON_STRING_LITERAL:16,
        EOF:0
    }
}

#[test]
fn invalid_unicode_escape() {
    assert_lex! {
        r#""Escaped \u0""#,
        ERROR_TOKEN:13,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \u002G""#,
        ERROR_TOKEN:16,
        EOF:0
    }
}

#[test]
fn invalid_escape() {
    assert_lex! {
        r#""\"#,
        ERROR_TOKEN:2,
        EOF:0
    }

    assert_lex! {
        r#""Invalid escape \'""#,
        ERROR_TOKEN:19,
        EOF:0
    }
}

#[test]
fn single_quote_escape_in_single_quote_string() {
    assert_lex! {
        r"'A single \' escape'",
        ERROR_TOKEN:20,
        EOF:0
    }
}

#[test]
fn identifiers() {
    assert_lex! {
        r#"asciiIdentifier"#,
        IDENT:15,
        EOF:0
    }

    assert_lex! {
        r#"with_underscore_here"#,
        IDENT:20,
        EOF:0
    }

    assert_lex! {
        r#"with_unicodeàçᨀ"#,
        IDENT:19,
        EOF:0
    }

    assert_lex! {
        r#"ᨀwith_unicodeàç"#,
        IDENT:19,
        EOF:0
    }
}

#[test]
fn single_line_comments() {
    assert_lex! {
        "//abc
    ",
        COMMENT:5,
        NEWLINE:1,
        WHITESPACE:4,
        EOF:0
    }

    assert_lex! {
        "//a",
        COMMENT:3,
        EOF:0
    }
}

#[test]
fn block_comment() {
    assert_lex! {
        "/*
        */",
        MULTILINE_COMMENT:13,
        EOF:0
    }

    assert_lex! {
        "/* */",
        COMMENT:5,
        EOF:0
    }

    assert_lex! {
        "/* *",
        COMMENT:4,
        EOF:0
    }
}

#[test]
fn keywords() {
    let keywords = vec!["true", "false", "null"];

    for keyword in keywords {
        let kind = JsonSyntaxKind::from_keyword(keyword).expect(
            "Expected `JsonSyntaxKind::from_keyword` to return a kind for keyword {keyword}.",
        );

        let mut lexer = Lexer::from_str(keyword);
        let current = lexer.next_token().expect("To have lexed keyword");

        assert_eq!(
            current.kind, kind,
            "Expected token '{keyword}' to be of kind {:?} but is {:?}.",
            kind, current.kind
        );

        assert_eq!(
            current.range.len(),
            TextSize::from(keyword.len() as u32),
            "Expected lexed keyword to be of len {} but has length {:?}",
            keyword.len(),
            current.range.len()
        );

        assert_eq!(lexer.next_token().expect("Expected EOF token").kind, EOF);
    }
}
