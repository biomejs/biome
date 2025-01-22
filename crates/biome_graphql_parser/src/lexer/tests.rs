#![cfg(test)]
#![expect(unused_mut, unused_variables)]

use super::{GraphqlLexer, TextSize};
use biome_graphql_syntax::GraphqlSyntaxKind::{self, EOF};
use biome_parser::lexer::Lexer;
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = GraphqlLexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token(()) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                GraphqlSyntaxKind::$kind,
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
        let mut lexer = GraphqlLexer::from_str(&cloned);
        let mut tokens = vec![];

        while lexer.next_token(()) != EOF {
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
        r#""5098382""#,
        GRAPHQL_STRING_LITERAL:9
    }

    // single quote is just a normal char
    assert_lex! {
        r#""'hello""#,
        GRAPHQL_STRING_LITERAL:8
    }

    // escaped quote
    assert_lex! {
        r#""hel\"lo\"""#,
        GRAPHQL_STRING_LITERAL:11
    }

    // unicode
    assert_lex! {
        r#""юникод""#,
        GRAPHQL_STRING_LITERAL:14
    }

    // missing double closing quote
    assert_lex! {
        r#""he"#,
        ERROR_TOKEN:3
    }

    // line break
    assert_lex! {
        r#""he
    "#,
        ERROR_TOKEN:3,
        NEWLINE:1,
        WHITESPACE:4
    }

    // line break
    assert_lex! {
        r#""he
    ""#,
        ERROR_TOKEN:3,
        NEWLINE:1,
        WHITESPACE:4,
        ERROR_TOKEN:1
    }

    assert_lex! {
        r#""Escaped \n""#,
        GRAPHQL_STRING_LITERAL:12
    }

    assert_lex! {
        r#""Escaped \r""#,
        GRAPHQL_STRING_LITERAL:12
    }

    // invalid escape sequence
    assert_lex! {
        r#""\0""#,
        ERROR_TOKEN:4
    }

    // empty
    assert_lex! {
        r#""""#,
        GRAPHQL_STRING_LITERAL:2
    }

    // block string newline
    assert_lex! {
        r#""""aaa
        """"#,
        // NEWLINE + 8 WHITESPACE
        GRAPHQL_STRING_LITERAL:18
    }

    // block string unterminated
    assert_lex! {
        r#""""aaa
        """#,
        ERROR_TOKEN:17
    }

    // unterminated block string
    assert_lex! {
        r#" """" "#,
        WHITESPACE:1,
        ERROR_TOKEN:5,
    }

    // unterminated block string
    assert_lex! {
        r#""""aaa
        """#,
        ERROR_TOKEN:17
    }

    // escape sequence
    assert_lex! {
        r#" """ \""" """ "#,
        WHITESPACE:1,
        GRAPHQL_STRING_LITERAL:12,
        WHITESPACE:1,
    }

    // unescaped backslash
    assert_lex! {
        r#"""" \" \r \n \"" """"#,
        GRAPHQL_STRING_LITERAL:20,
    }

    // empty
    assert_lex! {
        r#""""""""#,
        GRAPHQL_STRING_LITERAL:6
    }
}

#[test]
fn number() {
    assert_lex! {
        "5098382",
        GRAPHQL_INT_LITERAL:7
    }

    assert_lex! {
        "509.382",
        GRAPHQL_FLOAT_LITERAL:7
    }

    assert_lex! {
        "-",
        ERROR_TOKEN:1
    }

    assert_lex! {
        "+",
        ERROR_TOKEN:1
    }

    assert_lex! {
        "-123",
        GRAPHQL_INT_LITERAL:4
    }

    assert_lex! {
        "123e10",
        GRAPHQL_FLOAT_LITERAL:6
    }

    assert_lex! {
        "123e+10",
        GRAPHQL_FLOAT_LITERAL:7
    }

    assert_lex! {
        "123e-10",
        GRAPHQL_FLOAT_LITERAL:7
    }

    assert_lex! {
        "123E10",
        GRAPHQL_FLOAT_LITERAL:6
    }

    assert_lex! {
        "123E+10",
        GRAPHQL_FLOAT_LITERAL:7
    }

    assert_lex! {
        "123E-10",
        GRAPHQL_FLOAT_LITERAL:7
    }
}

#[test]
fn comment() {
    assert_lex! {
        "# abc",
        COMMENT:5
    }

    // newline
    assert_lex! {
        r#"# abc
        "#,
        COMMENT:5,
        NEWLINE:1,
        WHITESPACE:8
    }
}

#[test]
fn name() {
    assert_lex! {
        r#"asciiIdentifier"#,
        IDENT:15,
    }

    assert_lex! {
        r#"with_underscore_here"#,
        IDENT:20,
    }

    assert_lex! {
        r#"with_unicodeà"#,
        IDENT:12,
        ERROR_TOKEN:2,
    }

    assert_lex! {
        r#"ᨀwith_unicodeàç"#,
        ERROR_TOKEN:3,
        IDENT:12,
        ERROR_TOKEN:2,
        ERROR_TOKEN:2,
    }

    assert_lex! {
        r#"field }"#,
        IDENT:5,
        WHITESPACE:1,
        R_CURLY:1,
    }

    assert_lex! {
        r#"null"#,
        NULL_KW:4,
    }
}

#[test]
fn dot() {
    assert_lex! {
        "...",
        DOT3:3
    }

    assert_lex! {
        "..",
        ERROR_TOKEN:2
    }

    assert_lex! {
        ".",
        ERROR_TOKEN:1
    }
}

#[test]
fn trivia() {
    assert_lex! {
        r#",  ,
        ,,,,,"#,
        COMMA:1,
        WHITESPACE:2,
        COMMA:1,
        NEWLINE:1,
        WHITESPACE:8,
        COMMA:5
    }

    assert_lex! {
        r#"(1,  , ,,,,,2)"#,
        L_PAREN:1,
        GRAPHQL_INT_LITERAL:1,
        COMMA:1,
        WHITESPACE:2,
        COMMA:1,
        WHITESPACE:1,
        COMMA:5,
        GRAPHQL_INT_LITERAL:1,
        R_PAREN:1
    }
}
