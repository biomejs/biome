use syn::{spanned::Spanned, Error, Meta, MetaList, NestedMeta};

pub(crate) fn parse_meta_list(
    meta: &Meta,
    mut consume: impl FnMut(&Meta) -> Result<(), Error>,
) -> Result<(), Error> {
    let Meta::List(MetaList { nested, .. }) = meta else {
        return Err(Error::new(meta.span(), "A list of attribute is expected"));
    };
    for nested_meta in nested {
        let NestedMeta::Meta(meta) = nested_meta else {
            return Err(Error::new(nested_meta.span(), "Literals are not allowed"));
        };
        consume(meta)?;
    }
    Ok(())
}
