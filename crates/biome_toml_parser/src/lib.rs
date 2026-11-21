//! Lossless, error tolerant TOML 1.1 parser.
//!
//! The crate only contains the lexer so far, which splits a document into the
//! tokens defined by the [TOML 1.1.0 specification](https://toml.io/en/v1.1.0).

#![deny(clippy::use_self)]

// TODO: remove the `dead_code` expectations once the parse rules use the lexer
// and the token source.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "only the lexer tests use the lexer so far")
)]
mod lexer;
#[expect(dead_code, reason = "no parse rules use the token source yet")]
mod token_source;
