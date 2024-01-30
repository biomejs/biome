use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use std::collections::{BTreeMap, HashSet};
use syn::buffer::Cursor;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream, Result};
use syn::{
    parenthesized, parse_quote, parse_str, token, AttrStyle, Attribute, Error, LitStr, Token, Type,
};

#[derive(Clone, Debug)]
pub struct Attrs {
    pub derives: HashSet<Type>,
    pub doc_lines: Vec<TokenStream>,
    pub nested_attrs: BTreeMap<Ident, NestedAttrs>,
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

impl Attrs {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("partial") {
                opts.merge_with(
                    syn::parse2::<Self>(attr.tokens.clone()).expect("Could not parse attributes"),
                );
            } else if attr.style == AttrStyle::Outer && attr.path.is_ident("doc") {
                opts.doc_lines.push(attr.tokens.clone());
            }
        }
        opts
    }

    fn merge_with(&mut self, other: Self) {
        self.derives.extend(other.derives);
        self.nested_attrs.extend(other.nested_attrs);
    }
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);

        let mut result = Self::default();
        loop {
            let key: Ident = content.call(IdentExt::parse_any)?;
            match key.to_string().as_ref() {
                "derive" => result.derives = Derives::parse(&content)?.into_set(),
                _ => {
                    if content.peek(token::Paren) {
                        result
                            .nested_attrs
                            .insert(key, NestedAttrs::parse(&content)?);
                    } else {
                        return Err(Error::new(
                            content.span(),
                            "Attributes for other macros must be wrapped in parentheses.",
                        ));
                    }
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

#[derive(Clone, Debug, Default)]
pub struct Derives(HashSet<Type>);

impl Derives {
    fn into_set(self) -> HashSet<Type> {
        self.0
    }
}

impl Parse for Derives {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        parenthesized!(content in input);

        let mut result = HashSet::default();
        loop {
            let ty: Type = content.call(Type::parse)?;
            result.insert(ty);

            if content.is_empty() {
                break;
            }

            content.parse::<Token![,]>()?;
        }

        Ok(Self(result))
    }
}

#[derive(Clone, Debug, Default)]
pub struct FieldAttrs {
    pub ty: Option<PartialType>,
    pub doc_lines: Vec<TokenStream>,
    pub nested_attrs: BTreeMap<Ident, NestedAttrs>,
}

impl FieldAttrs {
    pub fn from_attrs(attrs: &[Attribute]) -> Self {
        let mut opts = Self::default();
        for attr in attrs {
            if attr.path.is_ident("partial") {
                opts.merge_with(
                    syn::parse2::<Self>(attr.tokens.clone())
                        .expect("Could not parse field attributes"),
                );
            } else if attr.style == AttrStyle::Outer && attr.path.is_ident("doc") {
                opts.doc_lines.push(attr.tokens.clone());
            }
        }
        opts
    }

    fn merge_with(&mut self, other: Self) {
        if other.ty.is_some() {
            self.ty = other.ty;
        }
        self.nested_attrs.extend(other.nested_attrs);
    }
}

impl Parse for FieldAttrs {
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
            match key.to_string().as_str() {
                "type" => {
                    result.ty = if content.peek(Token![=]) {
                        Some(PartialType::Literal(parse_str::<Type>(&parse_value()?)?))
                    } else {
                        Some(PartialType::Prefixed)
                    };
                }
                _ => {
                    if content.peek(token::Paren) {
                        result
                            .nested_attrs
                            .insert(key, NestedAttrs::parse(&content)?);
                    } else {
                        return Err(Error::new(
                            content.span(),
                            "Attributes for other macros must be wrapped in parentheses.",
                        ));
                    }
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

#[derive(Clone, Debug, Default)]
pub struct NestedAttrs(TokenStream);

impl Parse for NestedAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let tokens = input.step(|cursor| match cursor.token_tree() {
            Some((tt, next)) => Ok((tt.into_token_stream(), next)),
            None => Ok((TokenStream::default(), Cursor::empty())),
        })?;

        Ok(Self(tokens))
    }
}

impl ToTokens for NestedAttrs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.0.clone())
    }
}

#[derive(Clone, Debug, Default)]
pub enum PartialType {
    Literal(Type),
    #[default]
    Prefixed,
}
