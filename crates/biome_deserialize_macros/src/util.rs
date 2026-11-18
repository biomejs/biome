use syn::{
    Error, Lit, MacroDelimiter, Meta, Token, parse::ParseStream, punctuated::Punctuated,
    spanned::Spanned,
};

pub(crate) fn parse_meta_list(
    meta: &Meta,
    mut consume: impl FnMut(&Meta) -> Result<(), Error>,
) -> Result<(), Error> {
    let Meta::List(list) = meta else {
        return Err(Error::new(meta.span(), "A list of attribute is expected"));
    };
    if !matches!(list.delimiter, MacroDelimiter::Paren(_)) {
        return Err(Error::new(meta.span(), "A list of attribute is expected"));
    }
    let nested = list.parse_args_with(|input: ParseStream| {
        Punctuated::<Meta, Token![,]>::parse_terminated_with(input, |input| {
            if input.peek(Lit) {
                return Err(input.error("Literals are not allowed"));
            }
            input.parse()
        })
    })?;
    for nested_meta in &nested {
        consume(nested_meta)?;
    }
    Ok(())
}
