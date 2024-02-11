use proc_macro2::Ident;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, Attribute, Error, Token};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StructAttrs {
    pub with_validator: bool,
}

impl StructAttrs {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("deserializable") {
                opts.merge_with(
                    syn::parse2::<Self>(attr.tokens.clone())
                        .expect("Could not parse field attributes"),
                );
            }
        }
        opts
    }

    fn merge_with(&mut self, other: Self) {
        if other.with_validator {
            self.with_validator = other.with_validator;
        }
    }
}

impl Parse for StructAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);

        let mut result = Self::default();
        loop {
            let key: Ident = content.call(IdentExt::parse_any)?;
            match key.to_string().as_ref() {
                "with_validator" => result.with_validator = true,
                other => {
                    return Err(Error::new(
                        content.span(),
                        format!("Unexpected field attribute: {other}"),
                    ))
                }
            }

            if content.is_empty() {
                break;
            }

            content.parse::<Token![,]>()?;
        }

        Ok(result)
    }
}
