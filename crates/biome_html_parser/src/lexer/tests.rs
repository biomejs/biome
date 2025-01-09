#![cfg(test)]

use super::{HtmlLexer, TextSize};
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{self, *};
use biome_parser::lexer::Lexer;
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
        let mut lexer = HtmlLexer::from_str(&cloned);
        let mut tokens = vec![];

        while lexer.next_token(HtmlLexContext::default()) != EOF {
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
        let mut lexer = HtmlLexer::from_str($src);
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
                HtmlSyntaxKind::$kind,
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
        assert_lex!(HtmlLexContext::default(), $src, $($kind:$len,)*);
    };
}

#[test]
fn doctype_key() {
    assert_lex! {
        HtmlLexContext::Doctype,
        "doctype",
        DOCTYPE_KW: 7,
    }
}

#[test]
fn doctype_upper_key() {
    assert_lex! {
            HtmlLexContext::Doctype,
        "DOCTYPE",
        DOCTYPE_KW: 7,
    }
}

#[test]
fn string_literal() {
    assert_lex! {
        "\"data-attribute\"",
        HTML_STRING_LITERAL: 16,
    }
}

#[test]
fn self_closing() {
    assert_lex! {
        "<div />",
        L_ANGLE: 1,
        HTML_LITERAL: 3,
        WHITESPACE: 1,
        SLASH: 1
        R_ANGLE: 1
    }
}

#[test]
fn element() {
    assert_lex! {
        "<div></div>",
        L_ANGLE: 1,
        HTML_LITERAL: 3,
        R_ANGLE: 1,
        L_ANGLE: 1,
        SLASH: 1,
        HTML_LITERAL: 3,
        R_ANGLE: 1,
    }
}

#[test]
fn html_text() {
    assert_lex! {
        HtmlLexContext::OutsideTag,
        "abcdefghijklmnopqrstuvwxyz!@_-:;",
        HTML_LITERAL: 32,
    }
}

#[test]
fn doctype_with_quirk() {
    assert_lex! {
        HtmlLexContext::Doctype,
        "<!DOCTYPE HTML>",
        L_ANGLE: 1,
        BANG: 1,
        DOCTYPE_KW: 7,
        WHITESPACE: 1,
        HTML_KW: 4,
        R_ANGLE: 1,
    }
}

#[test]
fn doctype_with_quirk_and_system() {
    assert_lex! {
        HtmlLexContext::Doctype,
        "<!DOCTYPE HTML \"+//silmaril//dtd html pro v0r11 19970101//\">",
        L_ANGLE: 1,
        BANG: 1,
        DOCTYPE_KW: 7,
        WHITESPACE: 1,
        HTML_KW: 4,
        WHITESPACE: 1,
        HTML_STRING_LITERAL: 44,
        R_ANGLE: 1,
    }
}

#[test]
fn element_with_attributes() {
    assert_lex! {
        "<div class='joy and happiness'>",
        L_ANGLE: 1,
        HTML_LITERAL: 3,
        WHITESPACE: 1,
        HTML_LITERAL: 5,
        EQ:1,
        HTML_STRING_LITERAL: 19,
        R_ANGLE: 1,
    }
}

#[test]
fn element_with_dashed_attributes() {
    assert_lex! {
        "<div aria-hidden>",
        L_ANGLE: 1,
        HTML_LITERAL: 3,
        WHITESPACE: 1,
        HTML_LITERAL: 11,
        R_ANGLE: 1,
    }
}

#[test]
fn html_element() {
    assert_lex! {
        "<html></html>",
        L_ANGLE: 1,
        HTML_LITERAL: 4,
        R_ANGLE: 1,
        L_ANGLE: 1,
        SLASH: 1,
        HTML_LITERAL: 4,
        R_ANGLE: 1,
    }
}

#[test]
fn html_text_spaces() {
    assert_lex! {
        HtmlLexContext::OutsideTag,
        "Lorem ipsum dolor sit amet, consectetur.",
        HTML_LITERAL: 40,
    }
}

#[test]
fn html_text_spaces_with_lines() {
    assert_lex! {
        HtmlLexContext::OutsideTag,
        "Lorem ipsum dolor sit
        amet, consectetur.",
        HTML_LITERAL: 21,
        NEWLINE: 1,
        WHITESPACE: 8,
        HTML_LITERAL: 18,
    }
}

#[test]
fn long_text() {
    assert_lex! {
        HtmlLexContext::OutsideTag,
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed dapibus velit non justo",
        HTML_LITERAL: 84,
    }
}

#[test]
fn text_trailing_whitespace() {
    assert_lex! {
        HtmlLexContext::OutsideTag,
        "Lorem ipsum dolor <a",
        HTML_LITERAL: 17,
        WHITESPACE: 1,
        L_ANGLE: 1,
        HTML_LITERAL: 1,
    }
}

#[test]
fn text_trailing_whitespace_multiple() {
    assert_lex! {
        HtmlLexContext::OutsideTag,
        "Lorem ipsum dolor  <a",
        HTML_LITERAL: 17,
        WHITESPACE: 2,
        L_ANGLE: 1,
        HTML_LITERAL: 1,
    }
}

#[test]
fn unquoted_attribute_value_1() {
    assert_lex! {
        HtmlLexContext::AttributeValue,
        "value",
        HTML_STRING_LITERAL: 5,
    }
}

#[test]
fn unquoted_attribute_value_2() {
    assert_lex! {
        HtmlLexContext::AttributeValue,
        "value value\tvalue\n",
        HTML_STRING_LITERAL: 5,
        WHITESPACE: 1,
        HTML_STRING_LITERAL: 5,
        WHITESPACE: 1,
        HTML_STRING_LITERAL: 5,
        NEWLINE: 1,
    }
}

#[test]
fn unquoted_attribute_value_invalid_chars() {
    assert_lex! {
        HtmlLexContext::AttributeValue,
        "?<='\"`",
        ERROR_TOKEN: 1,
        L_ANGLE: 1,
        ERROR_TOKEN: 1,
        ERROR_TOKEN: 3,
    }
}

#[test]
fn comment_start() {
    assert_lex! {
        "<!--",
        COMMENT_START: 4,
    }
}

#[test]
fn comment_end() {
    assert_lex! {
        HtmlLexContext::Comment,
        "-->",
        COMMENT_END: 3,
    }
}

#[test]
fn comment_full() {
    assert_lex! {
        HtmlLexContext::Comment,
        "<!-- foo -->",
        COMMENT_START: 4,
        HTML_LITERAL: 5,
        COMMENT_END: 3,
    }
}

#[test]
fn cdata_full() {
    assert_lex! {
        HtmlLexContext::CdataSection,
        "<![CDATA[1]]>",
        CDATA_START: 9,
        HTML_LITERAL: 1,
        CDATA_END: 3,
    }
}
