#![deny(clippy::use_self)]

mod resolvable_derive;

use proc_macro::TokenStream;
use proc_macro_error2::*;
use syn::{DeriveInput, parse_macro_input};

/// Implements the `Resolvable` trait for a given type.
///
/// Types with a derived `Resolvable` implementation are expected to be one of
/// the following:
/// - A struct with either named fields, or a single unnamed field, whose types
///   are either:
///   - A primitive type.
///   - Our `Text` type.
///   - A type that is itself `Resolvable`.
///   - An `Option<T>` or a `Box<[T]>`, where `T` is one of the above.
/// - An enum whose variants are either:
///   - A unit variant.
///   - A variant with a single unnamed field whose type is either `Text`,
///     another `Resolvable` type, or a `Box<T>` where `T` implements
///     `Resolvable`.
#[proc_macro_derive(Resolvable)]
#[proc_macro_error]
pub fn derive_resolvable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input = resolvable_derive::DeriveInput::parse(input);

    let tokens = resolvable_derive::generate_resolvable(input);

    if false {
        panic!("{tokens}");
    }

    TokenStream::from(tokens)
}
