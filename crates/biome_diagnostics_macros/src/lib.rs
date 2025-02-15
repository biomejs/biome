use proc_macro::TokenStream;
use proc_macro_error2::*;
use syn::{parse_macro_input, DeriveInput};

mod generate;
mod parse;

#[proc_macro_derive(
    Diagnostic,
    attributes(
        diagnostic,
        severity,
        category,
        description,
        message,
        advice,
        verbose_advice,
        location,
        tags,
        source
    )
)]
#[proc_macro_error]
pub fn derive_diagnostic(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input = parse::DeriveInput::parse(input);

    let tokens = generate::generate_diagnostic(input);

    if false {
        panic!("{tokens}");
    }

    TokenStream::from(tokens)
}
