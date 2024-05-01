use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashSet;
use syn::{parse_quote, AttrStyle, Attribute, MetaNameValue, Path, Type};

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
    type Error = syn::Error;

    fn try_from(attrs: &Vec<syn::Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path().is_ident("partial") {
                let nested = attr.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )?;
                for meta in nested {
                    if meta.path().is_ident("derive") {
                        let meta = meta.require_list()?;
                        meta.parse_nested_meta(|meta| {
                            opts.derives.insert(meta.path);
                            Ok(())
                        })?;
                    } else {
                        opts.nested_attrs.push(meta.into_token_stream());
                    }
                }
            } else if attr.style == AttrStyle::Outer && attr.path().is_ident("doc") {
                opts.doc_lines.push(attr.into_token_stream());
            }
        }
        Ok(opts)
    }
}

#[derive(Clone, Debug, Default)]
pub struct FieldAttrs {
    pub use_type: Option<PartialType>,
    pub doc_lines: Vec<TokenStream>,
    pub nested_attrs: Vec<TokenStream>,
}

impl TryFrom<&Vec<Attribute>> for FieldAttrs {
    type Error = syn::Error;

    fn try_from(attrs: &Vec<syn::Attribute>) -> Result<Self, Self::Error> {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path().is_ident("partial") {
                let nested = attr.parse_args_with(
                    syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
                )?;
                for meta in nested {
                    match meta {
                        syn::Meta::Path(path)
                            if opts.use_type.is_none() && path.is_ident("use_type") =>
                        {
                            opts.use_type = Some(PartialType::Prefixed);
                        }
                        syn::Meta::NameValue(MetaNameValue {
                            path,
                            value:
                                syn::Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Str(s),
                                    ..
                                }),
                            ..
                        }) if opts.use_type.is_none() && path.is_ident("use_type") => {
                            opts.use_type = Some(PartialType::Literal(s.parse()?));
                        }
                        _ => {
                            opts.nested_attrs.push(meta.into_token_stream());
                        }
                    }
                }
            } else if attr.style == AttrStyle::Outer && attr.path().is_ident("doc") {
                opts.doc_lines.push(attr.into_token_stream());
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
