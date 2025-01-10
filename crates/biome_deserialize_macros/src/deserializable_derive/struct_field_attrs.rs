use quote::ToTokens;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Lit, Meta, MetaNameValue, Path};

use crate::util::parse_meta_list;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StructFieldAttrs {
    /// If `true`, bails on deserializing the entire struct if validation for
    /// this field fails.
    ///
    /// Note the struct may still be deserialized if the field is not present in
    /// the serialized representation at all. In that case `Default::default()`
    /// will be filled in.
    pub bail_on_error: bool,

    /// If set, this provides information about the deprecated of the field.
    pub deprecated: Option<DeprecatedField>,

    /// Optional name to use in the serialized format.
    ///
    /// See also: <https://serde.rs/field-attrs.html#rename>
    pub rename: Option<String>,

    /// If `true`, presence of this field is required for successful
    /// deserialization of the struct.
    ///
    /// Implies `bail_on_error`.
    pub required: bool,

    /// If `true`, this field will not be deserialized at all.
    pub skip: bool,

    /// Optional validation function to be called on the field value.
    pub validate: Option<Path>,

    /// Uses the field as a catch-all for unknown entries in the map
    pub rest: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeprecatedField {
    /// A generic message that explains what to do or why the field is deprecated.
    Message(String),

    /// Provides the path for a new field to use instead.
    UseInstead(String),
}

impl TryFrom<&Vec<Attribute>> for StructFieldAttrs {
    type Error = Error;

    fn try_from(attrs: &Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("deserializable") {
                parse_meta_list(&attr.parse_meta()?, |meta| {
                    match meta {
                        Meta::Path(path) => {
                            if path.is_ident("required") {
                                opts.required = true;
                            } else if path.is_ident("bail_on_error") {
                                opts.bail_on_error = true;
                            } else if path.is_ident("rest") {
                                opts.rest = true;
                            } else if path.is_ident("skip") {
                                opts.skip = true;
                            } else {
                                let path_str = path.to_token_stream().to_string();
                                return Err(Error::new(
                                    path.span(),
                                    format_args!("Unexpected attribute: {path_str}"),
                                ));
                            }
                        }
                        Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Str(s),
                            ..
                        }) => {
                            if path.is_ident("rename") {
                                opts.rename = Some(s.value())
                            } else if path.is_ident("validate") {
                                opts.validate = Some(s.parse()?)
                            }
                        }
                        Meta::List(_) if meta.path().is_ident("deprecated") => {
                            let mut deprecated = None;
                            parse_meta_list(meta, |meta| {
                                let Meta::NameValue(MetaNameValue {
                                    path,
                                    lit: Lit::Str(s),
                                    ..
                                }) = meta
                                else {
                                    let meta_text = meta.to_token_stream().to_string();
                                    return Err(Error::new(
                                        meta.span(),
                                        format_args!("Unexpected attribute: {meta_text}"),
                                    ));
                                };
                                deprecated = if deprecated.is_some() {
                                    return Err(Error::new(
                                        meta.span(),
                                        "Only one attribute expected inside deprecated()",
                                    ));
                                } else if path.is_ident("message") {
                                    Some(DeprecatedField::Message(s.value()))
                                } else if path.is_ident("use_instead") {
                                    Some(DeprecatedField::UseInstead(s.value()))
                                } else {
                                    let path_text = path.to_token_stream().to_string();
                                    return Err(Error::new(
                                        path.span(),
                                        format_args!(
                                            "Unexpected attribute inside deprecated(): {path_text}"
                                        ),
                                    ));
                                };
                                Ok(())
                            })?;
                            opts.deprecated = deprecated;
                        }
                        _ => {
                            let meta_text = meta.to_token_stream().to_string();
                            return Err(Error::new(
                                meta.span(),
                                format_args!("Unexpected attribute: {meta_text}"),
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
