#![cfg(test)]
#![expect(unused_mut, unused_variables)]

use super::{CssLexer, TextSize};
use crate::lexer::CssLexContext;
use crate::CssParserOptions;
use biome_css_syntax::CssSyntaxKind::EOF;
use biome_parser::lexer::Lexer;
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let options = CssParserOptions::default().allow_wrong_line_comments().allow_css_modules();
        let mut lexer = CssLexer::from_str($src).with_options(options);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token(CssLexContext::default()) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                biome_css_syntax::CssSyntaxKind::$kind,
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
        let mut lexer = CssLexer::from_str(&cloned);
        let mut tokens = vec![];

        while lexer.next_token(CssLexContext::default()) != EOF {
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
fn string() {
    assert_lex! {
        "'5098382'",
        CSS_STRING_LITERAL:9
    }

    // double quote
    assert_lex! {
        r#"'hel"lo"'"#,
        CSS_STRING_LITERAL:9
    }

    // escaped quote
    assert_lex! {
        r"'hel\'lo\''",
        CSS_STRING_LITERAL:11
    }

    // escaped quote
    assert_lex! {
        r#""hel\"lo\"""#,
        CSS_STRING_LITERAL:11
    }

    // unicode
    assert_lex! {
        "'юникод'",
        CSS_STRING_LITERAL:14
    }

    // missing single closing quote
    assert_lex! {
        "'he",
        ERROR_TOKEN:3
    }

    // missing double closing quote
    assert_lex! {
        r#""he"#,
        ERROR_TOKEN:3
    }

    // line break
    assert_lex! {
        r#"'he
    "#,
        ERROR_TOKEN:3,
        NEWLINE:1,
        WHITESPACE:4
    }

    // line break
    assert_lex! {
        r#"'he
    '"#,
        ERROR_TOKEN:3,
        NEWLINE:1,
        WHITESPACE:4,
        ERROR_TOKEN:1
    }

    assert_lex! {
        r#""Escaped \n""#,
        CSS_STRING_LITERAL:12
    }

    assert_lex! {
        r#""Escaped \r""#,
        CSS_STRING_LITERAL:12
    }

    // invalid escape sequence
    assert_lex! {
        r"'\0'",
        ERROR_TOKEN:4
    }
}

#[test]
fn number() {
    assert_lex! {
        "5098382",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "509.382",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        ".382",
        CSS_NUMBER_LITERAL:4
    }

    assert_lex! {
        "+123",
        CSS_NUMBER_LITERAL:4
    }

    assert_lex! {
        "-123",
        CSS_NUMBER_LITERAL:4
    }

    assert_lex! {
        "+123",
        CSS_NUMBER_LITERAL:4
    }

    assert_lex! {
        "123e10",
        CSS_NUMBER_LITERAL:6
    }

    assert_lex! {
        "123e+10",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "123e-10",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "123E10",
        CSS_NUMBER_LITERAL:6
    }

    assert_lex! {
        "123E+10",
        CSS_NUMBER_LITERAL:7
    }

    assert_lex! {
        "123E-10",
        CSS_NUMBER_LITERAL:7
    }
}

#[test]
fn dimension() {
    assert_lex! {
        "5098382s",
        CSS_DIMENSION_VALUE:7,
        S_KW:1
    }
    assert_lex! {
        "10.0px",
        CSS_DIMENSION_VALUE:4,
        PX_KW:2
    }
    assert_lex! {
        "10.0e+7fr",
        CSS_DIMENSION_VALUE:7,
        FR_KW:2
    }
    assert_lex! {
        "0\\0",
        CSS_DIMENSION_VALUE:1,
        IDENT:2
    }

    // A space breaks the dimension token and leaves it as a number literal
    assert_lex! {
        "1 px",
        CSS_NUMBER_LITERAL:1,
        WHITESPACE:1,
        PX_KW:2
    }

    // Percentages aren't technically dimensions, but we treat them similarly.
    assert_lex! {
        "100%",
        CSS_PERCENTAGE_VALUE:3,
        PERCENT:1
    }

    assert_lex! {
        "100 %",
        CSS_NUMBER_LITERAL:3,
        WHITESPACE:1
        PERCENT:1
    }
}

#[test]
fn cdo_and_cdc() {
    assert_lex! {
        "<!-- -->",
        CDO:4,
        WHITESPACE:1,
        CDC:3
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
        FROM_KW:4
    }
}

#[test]
fn attribute() {
    assert_lex! {
        "$=",
        DOLLAR_EQ:2
    }
}

#[test]
fn identifier() {
    assert_lex! {
        "--",
        MINUS:1,
        MINUS:1
    }

    assert_lex! {
        "i4f5g7",
        IDENT:6
    }

    assert_lex! {
        "class",
        IDENT:5
    }

    assert_lex! {
        r"cl\aass",
        IDENT:7
    }

    assert_lex! {
        r"\ccl\aass",
        IDENT:9
    }

    assert_lex! {
        "-class",
        IDENT:6
    }

    assert_lex! {
        r"-cl\aass",
        IDENT:8
    }

    assert_lex! {
        r"-\acl\aass",
        IDENT:10
    }

    assert_lex! {
        "--property",
        IDENT:10
    }

    assert_lex! {
        r"--prop\eerty",
        IDENT:12
    }

    assert_lex! {
        r"--\pprop\eerty",
        IDENT:14
    }
}

#[test]
fn wrong_line_comments() {
    assert_lex! {
        "//abc
    ",
        COMMENT:5,
        NEWLINE:1,
        WHITESPACE:4
    }

    assert_lex! {
        "//a",
        COMMENT:3
    }
}

#[test]
fn block_comment() {
    assert_lex! {
        "/*
        */",
        MULTILINE_COMMENT:13
    }

    assert_lex! {
        "/* */",
        COMMENT:5
    }

    assert_lex! {
        "/* *",
        COMMENT:4
    }
}

#[test]
fn char() {
    assert_lex! {
        "!",
        BANG:1
    }
    assert_lex! {
        "%",
        PERCENT:1
    }
    assert_lex! {
        "/",
        SLASH:1
    }
}
