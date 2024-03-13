use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Lit, Meta, MetaNameValue};

use crate::util::parse_meta_list;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EnumVariantAttrs {
    /// Optional name to use in the serialized format.
    ///
    /// See also: <https://serde.rs/variant-attrs.html#rename>
    pub rename: Option<String>,
}

impl TryFrom<&Vec<Attribute>> for EnumVariantAttrs {
    type Error = Error;

    fn try_from(attrs: &Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("deserializable") {
                parse_meta_list(&attr.parse_meta()?, |meta| {
                    match meta {
                        Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Str(s),
                            ..
                        }) if path.is_ident("rename") => opts.rename = Some(s.value()),
                        val => {
                            let val_str = val.to_token_stream().to_string();
                            return Err(Error::new(
                                val.span(),
                                format_args!("Unexpected attribute: {val_str}"),
                            ));
                        }
                    }
                    Ok(())
                })?;
            } else if attr.path.is_ident("serde") {
                parse_meta_list(&attr.parse_meta()?, |meta| {
                    match meta {
                        Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Str(s),
                            ..
                        }) if opts.rename.is_none() && path.is_ident("rename") => {
                            opts.rename = Some(s.value())
                        }
                        _ => {} // Don't fail on unrecognized Serde attrs
                    }
                    Ok(())
                })
                .ok();
            }
        }
        Ok(opts)
    }
}
