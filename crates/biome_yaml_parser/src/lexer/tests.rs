#![cfg(test)]
#![expect(unused_mut)]

use super::{TextSize, YamlLexContext};
use crate::lexer::YamlLexer;
use biome_parser::lexer::TokenFlags;
use biome_yaml_syntax::YamlSyntaxKind;
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Creates a new lexer from the given string
fn init_yaml_lexer(s: &str) -> YamlLexer {
    YamlLexer {
        source: s,
        position: 0,
        after_newline: false,
        unicode_bom_length: 0,
        current_start: TextSize::from(0),
        current_kind: YamlSyntaxKind::EOF,
        current_flags: TokenFlags::empty(),
        diagnostics: vec![],
        context: YamlLexContext::Regular,
    }
}

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = init_yaml_lexer($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let tokens: Vec<_> = lexer.collect();

        $(
            assert_eq!(
                tokens[idx].kind,
                biome_yaml_syntax::YamlSyntaxKind::$kind,
                "expected token kind {}, but found {:?} when lexing {:?}",
                stringify!($kind),
                tokens[idx].kind,
                &$src[tokens[idx].range]
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
        let mut lexer = init_yaml_lexer(&cloned);
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
fn lex_booleans() {
    assert_lex!(
        "true",
        YAML_BOOLEAN_VALUE:4,
    );

    assert_lex!(
        "false",
        YAML_BOOLEAN_VALUE:5,
    );
}

#[test]
fn lex_null() {
    assert_lex!(
        "null",
        YAML_NULL_VALUE:4,
    );
}

#[test]
fn lex_float() {
    assert_lex!(
        "123.456",
        YAML_NUMBER_VALUE:7,
    );
}

#[test]
fn lex_invalid_float_as_string() {
    assert_lex!(
        "123.456.789",
        YAML_STRING_VALUE:11,
    );
}

#[test]
fn lex_quoted_string() {
    assert_lex!(
        "\"hello world\"",
        YAML_STRING_VALUE:13,
    );
}

#[test]
fn lex_key_value_pair() {
    assert_lex!(
        "key: value",
        YAML_IDENTIFIER:3,
        COLON:1,
        WHITESPACE:1,
        YAML_STRING_VALUE:5,
    );
}

#[test]
fn lex_invalid_key_value_pair() {
    assert_lex!(
        "key:value",
        YAML_STRING_VALUE:9,
    );
}

#[test]
fn lex_kinda_invalid_key_value_pair() {
    assert_lex!(
        "foo:bar: baz",
        YAML_IDENTIFIER:7,
        COLON:1,
        WHITESPACE:1,
        YAML_STRING_VALUE:3,
    );
}

#[test]
fn lex_object_nested() {
    assert_lex!(
        r#"
foo:
    bar: baz"#,
        NEWLINE:1,
        YAML_IDENTIFIER:3,
        COLON:1,
        NEWLINE:1,
        WHITESPACE:4,
        YAML_IDENTIFIER:3,
        COLON:1,
        WHITESPACE:1,
        YAML_STRING_VALUE:3,
    );
}

#[test]
fn lex_comment() {
    assert_lex!(
        "# this is a comment",
        COMMENT:19,
    );
}

#[test]
fn lex_list() {
    assert_lex!(
        "- foo",
        DASH:1,
        WHITESPACE:1,
        YAML_STRING_VALUE:3,
    );
}

#[test]
fn lex_list_object() {
    assert_lex!(
        "- foo: bar",
        DASH:1,
        WHITESPACE:1,
        YAML_IDENTIFIER:3,
        COLON:1,
        WHITESPACE:1,
        YAML_STRING_VALUE:3,
    );
}

#[test]
fn lex_list_invalid() {
    assert_lex!(
        "-foo",
        YAML_STRING_VALUE:4,
    );
}

#[test]
fn lex_nested_list() {
    assert_lex!(
        "- - bar",
        DASH:1,
        WHITESPACE:1,
        DASH:1,
        WHITESPACE:1,
        YAML_STRING_VALUE:3,
    );
}

#[test]
fn lex_array_inline() {
    assert_lex!(
        "[1]",
        L_BRACK:1,
        YAML_NUMBER_VALUE:1,
        R_BRACK:1,
    );
}
#[test]
fn lex_array_inline_2() {
    assert_lex!(
        "[1,2]",
        L_BRACK:1,
        YAML_NUMBER_VALUE:1,
        COMMA:1,
        YAML_NUMBER_VALUE:1,
        R_BRACK:1,
    );
}

#[test]
fn lex_array_inline_invalid() {
    assert_lex!(
        "1]",
        YAML_STRING_VALUE:2,
    );
}
