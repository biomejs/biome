#![cfg(test)]
#![allow(unused_mut, unused_variables, unused_assignments)]

use super::{Lexer, TextSize};
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
                rome_css_syntax::CssSyntaxKind::$kind,
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
        .unwrap_or_else(|_| {
            panic!(
                "Lexer is infinitely recursing with this code: ->{}<-",
                string
            )
        });

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
fn string() {
    assert_lex! {
        "'5098382'",
        CSS_STRING_LITERAL:9,
        EOF:0
    }

    // double quote
    assert_lex! {
        r#"'hel"lo"'"#,
        CSS_STRING_LITERAL:9,
        EOF:0
    }

    // escaped quote
    assert_lex! {
        r"'hel\'lo\''",
        CSS_STRING_LITERAL:11,
        EOF:0
    }

    // escaped quote
    assert_lex! {
        r#""hel\"lo\"""#,
        CSS_STRING_LITERAL:11,
        EOF:0
    }

    // unicode
    assert_lex! {
        "'юникод'",
        CSS_STRING_LITERAL:14,
        EOF:0
    }

    // missing single closing quote
    assert_lex! {
        "'he",
        ERROR_TOKEN:3,
        EOF:0
    }

    // missing double closing quote
    assert_lex! {
        r#""he"#,
        ERROR_TOKEN:3,
        EOF:0
    }

    // line break
    assert_lex! {
        r#"'he
    "#,
        ERROR_TOKEN:3,
        NEWLINE:1,
        WHITESPACE:4,
        EOF:0
    }

    // line break
    assert_lex! {
        r#"'he
    '"#,
        ERROR_TOKEN:3,
        NEWLINE:1,
        WHITESPACE:4,
        ERROR_TOKEN:1,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \n""#,
        CSS_STRING_LITERAL:12,
        EOF:0
    }

    assert_lex! {
        r#""Escaped \r""#,
        CSS_STRING_LITERAL:12,
        EOF:0
    }

    // invalid escape sequence
    assert_lex! {
        r"'\0'",
        ERROR_TOKEN:4,
        EOF:0
    }
}

#[test]
fn number() {
    assert_lex! {
        "5098382",
        CSS_NUMBER_LITERAL:7,
        EOF:0
    }

    assert_lex! {
        "509.382",
        CSS_NUMBER_LITERAL:7,
        EOF:0
    }

    assert_lex! {
        ".382",
        CSS_NUMBER_LITERAL:4,
        EOF:0
    }

    assert_lex! {
        "+123",
        CSS_NUMBER_LITERAL:4,
        EOF:0
    }

    assert_lex! {
        "-123",
        CSS_NUMBER_LITERAL:4,
        EOF:0
    }

    assert_lex! {
        "+123",
        CSS_NUMBER_LITERAL:4,
        EOF:0
    }

    assert_lex! {
        "123e10",
        CSS_NUMBER_LITERAL:6,
        EOF:0
    }

    assert_lex! {
        "123e+10",
        CSS_NUMBER_LITERAL:7,
        EOF:0
    }

    assert_lex! {
        "123e-10",
        CSS_NUMBER_LITERAL:7,
        EOF:0
    }

    assert_lex! {
        "123E10",
        CSS_NUMBER_LITERAL:6,
        EOF:0
    }

    assert_lex! {
        "123E+10",
        CSS_NUMBER_LITERAL:7,
        EOF:0
    }

    assert_lex! {
        "123E-10",
        CSS_NUMBER_LITERAL:7,
        EOF:0
    }
}

#[test]
fn cdo_and_cdc() {
    assert_lex! {
        "<!-- -->",
        CDO:4,
        WHITESPACE:1,
        CDC:3
        EOF:0
    }
}

#[test]
fn dimension() {
    assert_lex! {
        "100vh",
        CSS_NUMBER_LITERAL:3,
        IDENT:2,
        EOF:0
    }
}

#[test]
fn keywords() {
    assert_lex! {
        "media keyframes important from",
        MEDIA_KW:5,
        WHITESPACE:1,
        KEYFRAMES_KW:9,
        WHITESPACE:1,
        IMPORTANT_KW:9,
        WHITESPACE:1,
        FROM_KW:4,
        EOF:0
    }
}

#[test]
fn identifier() {
    assert_lex! {
        "--",
        MINUS:1,
        MINUS:1,
        EOF:0
    }

    assert_lex! {
        "i4f5g7",
        IDENT:6,
        EOF:0
    }

    assert_lex! {
        "class",
        IDENT:5,
        EOF:0
    }

    assert_lex! {
        r"cl\aass",
        IDENT:7,
        EOF:0
    }

    assert_lex! {
        r"\ccl\aass",
        IDENT:9,
        EOF:0
    }

    assert_lex! {
        "-class",
        IDENT:6,
        EOF:0
    }

    assert_lex! {
        r"-cl\aass",
        IDENT:8,
        EOF:0
    }

    assert_lex! {
        r"-\acl\aass",
        IDENT:10,
        EOF:0
    }

    assert_lex! {
        "--property",
        CSS_CUSTOM_PROPERTY:10,
        EOF:0
    }

    assert_lex! {
        r"--prop\eerty",
        CSS_CUSTOM_PROPERTY:12,
        EOF:0
    }

    assert_lex! {
        r"--\pprop\eerty",
        CSS_CUSTOM_PROPERTY:14,
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
