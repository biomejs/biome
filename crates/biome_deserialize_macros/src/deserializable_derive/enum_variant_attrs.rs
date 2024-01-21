use proc_macro2::Ident;
use quote::ToTokens;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, Attribute, Error, LitStr, Token};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EnumVariantAttrs {
    /// Optional name to use in the serialized format.
    ///
    /// See also: <https://serde.rs/variant-attrs.html#rename>
    pub rename: Option<String>,
}

impl EnumVariantAttrs {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("deserializable") || attr.path.is_ident("serde") {
                opts.merge_with(
                    syn::parse2::<Self>(attr.tokens.clone())
                        .expect("Could not parse variant attributes"),
                );
            }
        }
        opts
    }

    fn merge_with(&mut self, other: Self) {
        if other.rename.is_some() {
            self.rename = other.rename;
        }
    }
}

impl Parse for EnumVariantAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);

        let parse_value = || -> Result<String> {
            content.parse::<Token![=]>()?;
            Ok(content
                .parse::<LitStr>()?
                .to_token_stream()
                .to_string()
                .trim_matches('"')
                .to_owned())
        };

        let mut result = Self::default();
        loop {
            let key: Ident = content.call(IdentExt::parse_any)?;
            match key.to_string().as_ref() {
                "rename" => result.rename = Some(parse_value()?),
                other => {
                    return Err(Error::new(
                        content.span(),
                        format!("Unexpected variant attribute: {other}"),
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
