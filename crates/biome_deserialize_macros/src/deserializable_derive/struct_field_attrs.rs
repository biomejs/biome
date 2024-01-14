use proc_macro2::Ident;
use quote::ToTokens;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parenthesized, Attribute, Error, LitStr, Token};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StructFieldAttrs {
    /// If set, this provides information about the deprecated of the field.
    pub deprecated: Option<DeprecatedField>,

    pub disallow_empty: bool,

    /// If set, the name passed to the deserializer (which was passed by the
    /// deserializer of the parent object) is also passed through to the
    /// deserializer of the field value.
    pub passthrough_name: bool,

    /// Optional name to use in the serialized format.
    ///
    /// See also: <https://serde.rs/field-attrs.html#rename>
    pub rename: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeprecatedField {
    /// A generic message that explains what to do or why the field is deprecated.
    Message(String),

    /// Provides the path for a new field to use instead.
    UseInstead(String),
}

impl StructFieldAttrs {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("deserializable") {
                opts.merge_with(
                    syn::parse2::<Self>(attr.tokens.clone())
                        .expect("Could not parse field attributes"),
                );
            } else if attr.path.is_ident("serde") {
                opts.merge_with_serde(
                    syn::parse2::<SerdeStructFieldAttrs>(attr.tokens.clone())
                        .expect("Could not parse Serde field attributes"),
                );
            }
        }
        opts
    }

    fn merge_with(&mut self, other: Self) {
        if other.deprecated.is_some() {
            self.deprecated = other.deprecated;
        }
        if other.disallow_empty {
            self.disallow_empty = other.disallow_empty;
        }
        if other.passthrough_name {
            self.passthrough_name = other.passthrough_name;
        }
        if other.rename.is_some() {
            self.rename = other.rename;
        }
    }

    fn merge_with_serde(&mut self, other: SerdeStructFieldAttrs) {
        if other.rename.is_some() {
            self.rename = other.rename;
        }
    }
}

impl Parse for StructFieldAttrs {
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
                "deprecated" => {
                    result.deprecated = Some(content.parse::<DeprecatedField>()?);
                }
                "disallow_empty" => result.disallow_empty = true,
                "passthrough_name" => result.passthrough_name = true,
                "rename" => result.rename = Some(parse_value()?),
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

impl Parse for DeprecatedField {
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

        let key: Ident = content.call(IdentExt::parse_any)?;
        let result = match key.to_string().as_ref() {
            "message" => Self::Message(parse_value()?),
            "use_instead" => Self::UseInstead(parse_value()?),
            other => {
                return Err(Error::new(
                    content.span(),
                    format!("Unexpected field attribute inside deprecated(): {other}"),
                ))
            }
        };

        if !content.is_empty() {
            return Err(Error::new(
                content.span(),
                "Only one attribute expected inside deprecated()",
            ));
        }

        Ok(result)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct SerdeStructFieldAttrs {
    rename: Option<String>,
}

impl Parse for SerdeStructFieldAttrs {
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
                _ => {
                    // Don't fail on unrecognized Serde attrs,
                    // but consume values to advance the parser.
                    let _result = parse_value();
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
