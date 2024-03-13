use std::str::FromStr;

use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Lit, Meta, MetaNameValue, Path};

use crate::util::parse_meta_list;

/// Attributes for struct and enum.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ContainerAttrs {
    pub with_validator: bool,
    /// Deserialize the given `from` type, then convert to the annotated type
    ///
    /// See also: <https://serde.rs/container-attrs.html#from>
    pub from: Option<Path>,
    /// Deserialize the given `try_from` type, then try converting to the annotated type
    ///
    /// See also: <https://serde.rs/container-attrs.html#try_from>
    pub try_from: Option<Path>,
    /// Ignore unknown fields in a struct upon deserialization.
    pub unknown_fields: Option<UnknownFields>,
}

/// Attributes for struct that control how unkinown fields are handled.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) enum UnknownFields {
    #[default]
    Warn,
    Deny,
    Allow,
}
impl FromStr for UnknownFields {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "warn" => Ok(Self::Warn),
            "deny" => Ok(Self::Deny),
            "allow" => Ok(Self::Allow),
            _ => Err("unknown_fields takes a value amomg `deny`, `warn`, and `allow`."),
        }
    }
}

impl TryFrom<&Vec<Attribute>> for ContainerAttrs {
    type Error = Error;

    fn try_from(attrs: &Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("deserializable") {
                parse_meta_list(&attr.parse_meta()?, |meta| {
                    match meta {
                        Meta::Path(path) if path.is_ident("with_validator") => {
                            opts.with_validator = true
                        }
                        Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Str(s),
                            ..
                        }) if path.is_ident("from") => opts.from = Some(s.parse()?),
                        Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Str(s),
                            ..
                        }) if path.is_ident("try_from") => opts.try_from = Some(s.parse()?),
                        Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Str(s),
                            ..
                        }) if path.is_ident("unknown_fields") => {
                            match UnknownFields::from_str(&s.value()) {
                                Ok(value) => opts.unknown_fields = Some(value),
                                Err(error) => return Err(Error::new(meta.span(), error)),
                            }
                        }
                        _ => {
                            let meta_str = meta.to_token_stream().to_string();
                            return Err(Error::new(
                                meta.span(),
                                format_args!("Unexpected attribute: {meta_str}"),
                            ));
                        }
                    }
                    if opts.from.is_some() && opts.try_from.is_some() {
                        return Err(Error::new(
                            meta.span(),
                            "You cannot specify both `from` and `try_from`",
                        ));
                    }
                    Ok(())
                })?;
            } else if attr.path.is_ident("serde") {
                parse_meta_list(&attr.parse_meta()?, |meta| {
                    match meta {
                        Meta::Path(path) if path.is_ident("deny_unknown_fields") => {
                            if opts.unknown_fields.is_none() {
                                opts.unknown_fields = Some(UnknownFields::Deny);
                            }
                        }
                        Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Str(s),
                            ..
                        }) => {
                            if opts.from.is_none() && path.is_ident("from") {
                                opts.from = Some(s.parse()?);
                            } else if opts.try_from.is_none() && path.is_ident("try_from") {
                                opts.try_from = Some(s.parse()?);
                            } else {
                                // Don't fail on unrecognized Serde attrs
                            }
                        }
                        _ => {
                            // Don't fail on unrecognized Serde attrs
                        }
                    }
                    Ok(())
                })
                .ok();
            }
        }
        Ok(opts)
    }
}
