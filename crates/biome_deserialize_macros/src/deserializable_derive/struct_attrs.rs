use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Meta};

use crate::util::parse_meta_list;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StructAttrs {
    pub with_validator: bool,
}

impl TryFrom<&Vec<Attribute>> for StructAttrs {
    type Error = Error;

    fn try_from(attrs: &Vec<Attribute>) -> std::prelude::v1::Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("deserializable") {
                parse_meta_list(&attr.parse_meta()?, |meta| {
                    match meta {
                        Meta::Path(path) if path.is_ident("with_validator") => {
                            opts.with_validator = true
                        }
                        val => {
                            let val_str = val.to_token_stream().to_string();
                            return Err(Error::new(
                                val.span(),
                                format!("Unexpected attribute: {val_str}"),
                            ));
                        }
                    }
                    Ok(())
                })?;
            }
        }
        Ok(opts)
    }
}
