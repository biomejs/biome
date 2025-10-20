#![cfg(test)]
#![expect(unused_mut, unused_variables)]

use super::{GritLexer, TextSize};
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_parser::lexer::Lexer;
use biome_rowan::TextRange;
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub struct Token {
    kind: GritSyntaxKind,
    range: TextRange,
}

impl Iterator for GritLexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.next_token(());
        if kind == EOF {
            None
        } else {
            Some(Token {
                kind,
                range: self.current_range(),
            })
        }
    }
}

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = GritLexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let tokens: Vec<_> = lexer.collect();

        $(
            assert_eq!(
                tokens[idx].kind,
                GritSyntaxKind::$kind,
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
        let mut lexer = GritLexer::from_str(&cloned);
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
    }
}

#[test]
fn int() {
    assert_lex! {
        "5098382",
        GRIT_INT:7,
    }
}

#[test]
fn float() {
    assert_lex! {
        "345.893872",
        GRIT_DOUBLE:10,
    }
}

#[test]
fn float_invalid() {
    assert_lex! {
        "345.893872.43322",
        ERROR_TOKEN:16,
    }
}

#[test]
fn negative() {
    assert_lex! {
        "-5098382",
        GRIT_NEGATIVE_INT:8,
    }
}

#[test]
fn minus_without_number() {
    assert_lex! {
        "-",
        MINUS:1,
    }
}

#[test]
fn exponent() {
    assert_lex! {
        "-493e+534",
        GRIT_DOUBLE:9,
    }

    assert_lex! {
        "-493E-534",
        GRIT_DOUBLE:9,
    }
}

#[test]
fn multiple_exponent() {
    assert_lex! {
        "-493e5E3",
        ERROR_TOKEN:8,
    }

    assert_lex! {
        "-493e4E45",
        ERROR_TOKEN:9,
    }
}

#[test]
fn array() {
    assert_lex! {
        "[1, 2, 3, 4]",
        L_BRACK:1,
        GRIT_INT:1,
        COMMA:1
        WHITESPACE:1,
        GRIT_INT:1,
        COMMA:1,
        WHITESPACE:1,
        GRIT_INT:1,
        COMMA:1,
        WHITESPACE:1,
        GRIT_INT:1,
        R_BRACK:1,
    }
}

#[test]
fn object() {
    assert_lex! {
        r#"{ key: "value", other: 4 }"#,
        L_CURLY:1,
        WHITESPACE:1,

        GRIT_NAME:3,
        COLON:1,
        WHITESPACE:1,
        GRIT_STRING:7,
        COMMA:1,

        WHITESPACE:1,
        GRIT_NAME:5,
        COLON:1,
        WHITESPACE:1,
        GRIT_INT:1,
        WHITESPACE:1,
        R_CURLY:1,
    }
}

#[test]
fn basic_string() {
    assert_lex! {
        r#""A string consisting of ASCII characters only""#,
        GRIT_STRING:46,
    }
}

#[test]
fn single_quote_string() {
    assert_lex! {
        r#"'A string token using single quotes that are not supported in GritQL'"#,
        ERROR_TOKEN:69,
    }
}

#[test]
fn unterminated_string() {
    assert_lex! {
        r#""A string without the closing quote"#,
        ERROR_TOKEN:35,
    }
}

#[test]
fn simple_escape_sequences() {
    assert_lex! {
        r#""Escaped \$""#,
        GRIT_STRING:12,
    }

    assert_lex! {
        r#""Escaped \"""#,
        GRIT_STRING:12,
    }

    assert_lex! {
        r#""Escaped \\""#,
        GRIT_STRING:12,
    }

    assert_lex! {
        r#""Escaped \n""#,
        GRIT_STRING:12,
    }
}

#[test]
fn unicode_escape() {
    assert_lex! {
        r#""Escaped \u002F""#,
        GRIT_STRING:16,
    }

    assert_lex! {
        r#""Escaped \u002f""#,
        GRIT_STRING:16,
    }
}

#[test]
fn invalid_unicode_escape() {
    assert_lex! {
        r#""Escaped \u0""#,
        ERROR_TOKEN:13,
    }

    assert_lex! {
        r#""Escaped \u002G""#,
        ERROR_TOKEN:16,
    }
}

#[test]
fn invalid_escape() {
    assert_lex! {
        r#""\"#,
        ERROR_TOKEN:2,
    }

    assert_lex! {
        r#""Invalid escape \'""#,
        ERROR_TOKEN:19,
    }
}

#[test]
fn single_quote_escape_in_single_quote_string() {
    assert_lex! {
        r"'A single \' escape'",
        ERROR_TOKEN:20,
    }
}

#[test]
fn names() {
    assert_lex! {
        r#"asciiIdentifier"#,
        GRIT_NAME:15,
    }

    assert_lex! {
        r#"with_underscore_here"#,
        GRIT_NAME:20,
    }

    assert_lex! {
        r#"with_unicodeà"#,
        GRIT_NAME:12,
        ERROR_TOKEN:2,
    }

    assert_lex! {
        r#"ᨀwith_unicodeàç"#,
        ERROR_TOKEN:3,
        GRIT_NAME:12,
        ERROR_TOKEN:2,
        ERROR_TOKEN:2,
    }
}

#[test]
fn regex() {
    assert_lex! {
        r#"r"a+b?""#,
        GRIT_REGEX:7,
    }

    assert_lex! {
        r#"r"a\\.b?""#,
        GRIT_REGEX:9,
    }

    assert_lex! {
        r#"r"a\"b?""#,
        GRIT_REGEX:8,
    }

    assert_lex! {
        r#"r"a+b?"#,
        ERROR_TOKEN: 6,
    }
}

#[test]
fn snippet_regex() {
    assert_lex! {
        r#"r`a+b?`"#,
        GRIT_SNIPPET_REGEX:7,
    }

    assert_lex! {
        r#"r`a\\.b?`"#,
        GRIT_SNIPPET_REGEX:9,
    }

    assert_lex! {
        r#"r`a\`b?`"#,
        GRIT_SNIPPET_REGEX:8,
    }

    assert_lex! {
        r#"r`a+b?"#,
        ERROR_TOKEN: 6,
    }
}

#[test]
fn snippets() {
    assert_lex! {
        r#"`console.log()`"#,
        GRIT_BACKTICK_SNIPPET:15,
    }

    assert_lex! {
        r#"`console.log($message)`"#,
        GRIT_BACKTICK_SNIPPET:23,
    }

    assert_lex! {
        r#"`console.log(\$message)`"#,
        GRIT_BACKTICK_SNIPPET:24,
    }

    assert_lex! {
        r#"`console.log(\/message)`"#,
        ERROR_TOKEN:24,
    }
}

#[test]
fn raw_snippets() {
    assert_lex! {
        r#"raw`console.log()`"#,
        GRIT_RAW_BACKTICK_SNIPPET:18,
    }

    assert_lex! {
        r#"raw`console.log($message)`"#,
        GRIT_RAW_BACKTICK_SNIPPET:26,
    }

    assert_lex! {
        r#"raw`console.log(\$message)`"#,
        GRIT_RAW_BACKTICK_SNIPPET:27,
    }

    assert_lex! {
        r#"raw`console.log(\/message)`"#,
        ERROR_TOKEN:27,
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
    }

    assert_lex! {
        "//a",
        COMMENT:3,
    }
}

#[test]
fn block_comment() {
    assert_lex! {
        "/*
        */",
        MULTILINE_COMMENT:13,
    }

    assert_lex! {
        "/* */",
        COMMENT:5,
    }

    assert_lex! {
        "/* *",
        COMMENT:4,
    }
}

#[test]
fn keywords() {
    let keywords = vec!["true", "false", "undefined", "as"];

    for keyword in keywords {
        let kind = GritSyntaxKind::from_keyword(keyword).expect(
            "Expected `GritSyntaxKind::from_keyword` to return a kind for keyword {keyword}.",
        );

        let mut lexer = GritLexer::from_str(keyword);
        let next_kind = lexer.next_token(());

        assert_eq!(
            next_kind, kind,
            "Expected token '{keyword}' to be of kind {kind:?} but is {next_kind:?}."
        );

        assert_eq!(
            lexer.current_range().len(),
            TextSize::from(keyword.len() as u32),
            "Expected lexed keyword to be of len {} but has length {:?}",
            keyword.len(),
            lexer.current_range().len()
        );

        assert_eq!(lexer.next_token(()), EOF);
    }
}
