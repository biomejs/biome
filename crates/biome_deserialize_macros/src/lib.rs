mod mergeable_derive;
mod none_state_derive;

use proc_macro::TokenStream;
use proc_macro_error::*;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Mergeable)]
#[proc_macro_error]
pub fn derive_mergeable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input = mergeable_derive::DeriveInput::parse(input);

    let tokens = mergeable_derive::generate_mergeable(input);

    if false {
        panic!("{tokens}");
    }

    TokenStream::from(tokens)
}

#[proc_macro_derive(NoneState)]
#[proc_macro_error]
pub fn derive_none_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input = none_state_derive::DeriveInput::parse(input);

    let tokens = none_state_derive::generate_none_state(input);

    if false {
        panic!("{tokens}");
    }

    TokenStream::from(tokens)
}
