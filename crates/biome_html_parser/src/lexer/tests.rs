#![cfg(test)]
#![allow(unused_mut, unused_variables, unused_assignments)]

use super::{HtmlLexer, TextSize};
use biome_html_syntax::HtmlSyntaxKind::{self, *};
use biome_parser::lexer::Lexer;
use biome_rowan::TextRange;

pub struct Token {
    kind: HtmlSyntaxKind,
    range: TextRange,
}

impl Iterator for HtmlLexer<'_> {
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
        let mut lexer = HtmlLexer::from_str($src);
        let mut idx = 0;
        let mut tok_idx = TextSize::default();

        let mut new_str = String::with_capacity($src.len());
        let tokens: Vec<_> = lexer.collect();

        $(
            assert_eq!(
                tokens[idx].kind,
                HtmlSyntaxKind::$kind,
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

// TODO: to fix
#[test]
#[ignore = "currently not handled"]
fn doctype() {
    assert_lex! {
        "doctype",
        ERROR_TOKEN:1,
    }
}
