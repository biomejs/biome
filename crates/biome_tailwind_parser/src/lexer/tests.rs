#![cfg(test)]

use super::{TailwindLexer, TextSize};
use crate::token_source::TailwindLexContext;
use biome_parser::lexer::Lexer;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// This is for testing if the lexer is truly lossless
// It parses random strings and puts them back together with the produced tokens and compares
#[quickcheck]
fn losslessness(string: String) -> bool {
    // using an mpsc channel allows us to spawn a thread and spawn the lexer there, then if
    // it takes more than 2 seconds we panic because it is 100% infinite recursion
    let cloned = string.clone();
    let (sender, receiver) = channel();
    thread::spawn(move || {
        let mut lexer = TailwindLexer::from_str(&cloned);
        let mut tokens = vec![];

        while lexer.next_token(TailwindLexContext::default()) != EOF {
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

// Assert the result of lexing a piece of source code,
// and make sure the tokens yielded are fully lossless and the source can be reconstructed from only the tokens
macro_rules! assert_lex {
    ($context:expr, $src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = TailwindLexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token($context) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                TailwindSyntaxKind::$kind,
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
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {
        assert_lex!(TailwindLexContext::default(), $src, $($kind:$len,)*);
    };
}

#[test]
fn basic() {
    assert_lex!(
        "block",
        TW_BASE:5,
    );
}

#[test]
fn basic_value() {
    assert_lex!(
        "text-sm",
        TW_BASE:4,
        DASH:1,
        TW_VALUE:2,
    );
}

#[test]
fn basic_multiple() {
    assert_lex!(
        "text-sm font-bold",
        TW_BASE:4,
        DASH:1,
        TW_VALUE:2,
        WHITESPACE:1,
        TW_BASE:4,
        DASH:1,
        TW_VALUE:4,
    );
}

#[test]
fn basic_modifier() {
    assert_lex!(
        "bg-primary/10",
        TW_BASE:2,
        DASH:1,
        TW_VALUE:7,
        SLASH:1,
        TW_VALUE:2,
    );
}

#[test]
fn basic_modifier_important() {
    assert_lex!(
        "bg-primary/10!",
        TW_BASE:2,
        DASH:1,
        TW_VALUE:7,
        SLASH:1,
        TW_VALUE:2,
        BANG:1,
    );
}

#[test]
fn variant() {
    assert_lex!(
        "hover:",
        TW_BASE:5,
        COLON:1,
    );
}

#[test]
fn variant_multiple() {
    assert_lex!(
        "hover:focus:",
        TW_BASE:5,
        COLON:1,
        TW_BASE:5,
        COLON:1,
    );
}

#[test]
fn variant_multiple_full() {
    assert_lex!(
        "hover:focus:bg-red-500",
        TW_BASE:5,
        COLON:1,
        TW_BASE:5,
        COLON:1,
        TW_BASE:2,
        DASH:1,
        TW_VALUE:7
    );
}
