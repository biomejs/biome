use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashSet;
use syn::{parse_quote, AttrStyle, Attribute, Error, Lit, Meta, MetaNameValue, Path, Type};

use crate::util::parse_meta_list;

#[derive(Clone, Debug)]
pub struct Attrs {
    pub derives: HashSet<Path>,
    pub doc_lines: Vec<TokenStream>,
    pub nested_attrs: Vec<TokenStream>,
}

impl Default for Attrs {
    fn default() -> Self {
        Self {
            derives: HashSet::from([
                parse_quote!(Debug),
                parse_quote!(Default),
                parse_quote!(serde::Deserialize),
                parse_quote!(serde::Serialize),
            ]),
            doc_lines: Default::default(),
            nested_attrs: Default::default(),
        }
    }
}

impl TryFrom<&Vec<Attribute>> for Attrs {
    type Error = Error;

    fn try_from(attrs: &Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("partial") {
                parse_meta_list(&attr.parse_meta()?, |meta| {
                    match meta {
                        Meta::List(_) if meta.path().is_ident("derive") => {
                            parse_meta_list(meta, |meta| {
                                opts.derives.insert(meta.path().clone());
                                Ok(())
                            })?;
                        }
                        _ => {
                            opts.nested_attrs.push(meta.into_token_stream());
                        }
                    }
                    Ok(())
                })?;
            } else if attr.style == AttrStyle::Outer && attr.path.is_ident("doc") {
                opts.doc_lines.push(attr.tokens.clone());
            }
        }
        Ok(opts)
    }
}

#[derive(Clone, Debug, Default)]
pub struct FieldAttrs {
    pub ty: Option<PartialType>,
    pub doc_lines: Vec<TokenStream>,
    pub nested_attrs: Vec<TokenStream>,
}

impl TryFrom<&Vec<Attribute>> for FieldAttrs {
    type Error = Error;

    fn try_from(attrs: &Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("partial") {
                parse_meta_list(&attr.parse_meta()?, |meta| {
                    match meta {
                        syn::Meta::Path(path) if opts.ty.is_none() && path.is_ident("type") => {
                            opts.ty = Some(PartialType::Prefixed);
                        }
                        syn::Meta::NameValue(MetaNameValue {
                            path,
                            lit: Lit::Str(s),
                            ..
                        }) if opts.ty.is_none() && path.is_ident("type") => {
                            opts.ty = Some(PartialType::Literal(s.parse()?));
                        }
                        _ => {
                            opts.nested_attrs.push(meta.into_token_stream());
                        }
                    }
                    Ok(())
                })?;
            } else if attr.style == AttrStyle::Outer && attr.path.is_ident("doc") {
                opts.doc_lines.push(attr.tokens.clone());
            }
        }
        Ok(opts)
    }
}

#[derive(Clone, Debug, Default)]
pub enum PartialType {
    #[default]
    Prefixed,
    Literal(Type),
}
