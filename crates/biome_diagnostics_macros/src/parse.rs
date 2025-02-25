use proc_macro2::{Ident, TokenStream};
use proc_macro_error2::*;
use quote::{quote, ToTokens};
use syn::{
    parse::{discouraged::Speculative, Error, Parse, ParseStream, Parser, Result},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren,
    Attribute, DataEnum, DataStruct, Generics, Token, Variant,
};

pub(crate) enum DeriveInput {
    DeriveStructInput(DeriveStructInput),
    DeriveEnumInput(DeriveEnumInput),
}

pub(crate) struct DeriveStructInput {
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,

    pub(crate) severity: Option<StaticOrDynamic<Ident>>,
    pub(crate) category: Option<StaticOrDynamic<syn::LitStr>>,
    pub(crate) description: Option<StaticOrDynamic<StringOrMarkup>>,
    pub(crate) message: Option<StaticOrDynamic<StringOrMarkup>>,
    pub(crate) advices: Vec<TokenStream>,
    pub(crate) verbose_advices: Vec<TokenStream>,
    pub(crate) location: Vec<(TokenStream, LocationField)>,
    pub(crate) tags: Option<StaticOrDynamic<Punctuated<Ident, Token![|]>>>,
    pub(crate) source: Option<TokenStream>,
}

pub(crate) struct DeriveEnumInput {
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,

    pub(crate) variants: Vec<Variant>,
}

impl DeriveInput {
    pub(crate) fn parse(input: syn::DeriveInput) -> Self {
        match input.data {
            syn::Data::Struct(data) => Self::DeriveStructInput(DeriveStructInput::parse(
                input.ident,
                input.generics,
                input.attrs,
                data,
            )),
            syn::Data::Enum(data) => Self::DeriveEnumInput(DeriveEnumInput::parse(
                input.ident,
                input.generics,
                input.attrs,
                data,
            )),
            syn::Data::Union(data) => abort!(
                data.union_token.span(),
                "unions are not supported by the Diagnostic derive macro"
            ),
        }
    }
}

impl DeriveStructInput {
    pub(crate) fn parse(
        ident: Ident,
        generics: Generics,
        attrs: Vec<Attribute>,
        data: DataStruct,
    ) -> Self {
        let mut result = Self {
            ident,
            generics,

            severity: None,
            category: None,
            description: None,
            message: None,
            advices: Vec::new(),
            verbose_advices: Vec::new(),
            location: Vec::new(),
            tags: None,
            source: None,
        };

        for attr in attrs {
            if attr.path.is_ident("diagnostic") {
                let tokens = attr.tokens.into();
                let attrs = match DiagnosticAttrs::parse.parse(tokens) {
                    Ok(attrs) => attrs,
                    Err(err) => abort!(
                        err.span(),
                        "failed to parse \"diagnostic\" attribute: {}",
                        err
                    ),
                };

                for item in attrs.attrs {
                    match item {
                        DiagnosticAttr::Severity(attr) => {
                            result.severity = Some(StaticOrDynamic::Static(attr.value));
                        }
                        DiagnosticAttr::Category(attr) => {
                            result.category = Some(StaticOrDynamic::Static(attr.value));
                        }
                        DiagnosticAttr::Message(MessageAttr::SingleString { value, .. }) => {
                            let value = StringOrMarkup::from(value);
                            result.description = Some(StaticOrDynamic::Static(value.clone()));
                            result.message = Some(StaticOrDynamic::Static(value));
                        }
                        DiagnosticAttr::Message(MessageAttr::SingleMarkup { markup, .. }) => {
                            let value = StringOrMarkup::from(markup);
                            result.description = Some(StaticOrDynamic::Static(value.clone()));
                            result.message = Some(StaticOrDynamic::Static(value));
                        }
                        DiagnosticAttr::Message(MessageAttr::Split(attr)) => {
                            for item in attr.attrs {
                                match item {
                                    SplitMessageAttr::Description { value, .. } => {
                                        result.description =
                                            Some(StaticOrDynamic::Static(value.into()));
                                    }
                                    SplitMessageAttr::Message { markup, .. } => {
                                        result.message =
                                            Some(StaticOrDynamic::Static(markup.into()));
                                    }
                                }
                            }
                        }
                        DiagnosticAttr::Tags(attr) => {
                            result.tags = Some(StaticOrDynamic::Static(attr.tags));
                        }
                    }
                }

                continue;
            }
        }

        for (index, field) in data.fields.into_iter().enumerate() {
            let ident = match field.ident {
                Some(ident) => quote! { #ident },
                None => quote! { #index },
            };

            for attr in field.attrs {
                if attr.path.is_ident("category") {
                    result.category = Some(StaticOrDynamic::Dynamic(ident.clone()));
                    continue;
                }

                if attr.path.is_ident("severity") {
                    result.severity = Some(StaticOrDynamic::Dynamic(ident.clone()));
                    continue;
                }

                if attr.path.is_ident("description") {
                    result.description = Some(StaticOrDynamic::Dynamic(ident.clone()));
                    continue;
                }

                if attr.path.is_ident("message") {
                    result.message = Some(StaticOrDynamic::Dynamic(ident.clone()));
                    continue;
                }

                if attr.path.is_ident("advice") {
                    result.advices.push(ident.clone());
                    continue;
                }

                if attr.path.is_ident("verbose_advice") {
                    result.verbose_advices.push(ident.clone());
                    continue;
                }

                if attr.path.is_ident("location") {
                    let tokens = attr.tokens.into();
                    let attr = match LocationAttr::parse.parse(tokens) {
                        Ok(attr) => attr,
                        Err(err) => abort!(
                            err.span(),
                            "failed to parse \"location\" attribute: {}",
                            err
                        ),
                    };

                    result.location.push((ident.clone(), attr.field));
                    continue;
                }

                if attr.path.is_ident("tags") {
                    result.tags = Some(StaticOrDynamic::Dynamic(ident.clone()));
                    continue;
                }

                if attr.path.is_ident("source") {
                    result.source = Some(ident.clone());
                    continue;
                }
            }
        }

        result
    }
}

impl DeriveEnumInput {
    pub(crate) fn parse(
        ident: Ident,
        generics: Generics,
        attrs: Vec<Attribute>,
        data: DataEnum,
    ) -> Self {
        for attr in attrs {
            if attr.path.is_ident("diagnostic") {
                abort!(
                    attr.span(),
                    "\"diagnostic\" attributes are not supported on enums"
                );
            }
        }

        Self {
            ident,
            generics,

            variants: data.variants.into_iter().collect(),
        }
    }
}

pub(crate) enum StaticOrDynamic<S> {
    Static(S),
    Dynamic(TokenStream),
}

#[derive(Clone)]
pub(crate) enum StringOrMarkup {
    String(syn::LitStr),
    Markup(TokenStream),
}

impl From<syn::LitStr> for StringOrMarkup {
    fn from(value: syn::LitStr) -> Self {
        Self::String(value)
    }
}

impl From<TokenStream> for StringOrMarkup {
    fn from(value: TokenStream) -> Self {
        Self::Markup(value)
    }
}

struct DiagnosticAttrs {
    _paren_token: Paren,
    attrs: Punctuated<DiagnosticAttr, Token![,]>,
}

impl Parse for DiagnosticAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _paren_token: syn::parenthesized!(content in input),
            attrs: content.parse_terminated(DiagnosticAttr::parse)?,
        })
    }
}

enum DiagnosticAttr {
    Severity(SeverityAttr),
    Category(CategoryAttr),
    Message(MessageAttr),
    Tags(TagsAttr),
}

impl Parse for DiagnosticAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        if name == "severity" {
            return Ok(Self::Severity(input.parse()?));
        }

        if name == "category" {
            return Ok(Self::Category(input.parse()?));
        }

        if name == "message" {
            return Ok(Self::Message(input.parse()?));
        }

        if name == "tags" {
            return Ok(Self::Tags(input.parse()?));
        }

        Err(Error::new_spanned(name, "unknown attribute"))
    }
}

struct SeverityAttr {
    _eq_token: Token![=],
    value: Ident,
}

impl Parse for SeverityAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct CategoryAttr {
    _eq_token: Token![=],
    value: syn::LitStr,
}

impl Parse for CategoryAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            _eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

enum MessageAttr {
    SingleString {
        _eq_token: Token![=],
        value: syn::LitStr,
    },
    SingleMarkup {
        _paren_token: Paren,
        markup: TokenStream,
    },
    Split(SplitMessageAttrs),
}

impl Parse for MessageAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();

        if lookahead.peek(Token![=]) {
            return Ok(Self::SingleString {
                _eq_token: input.parse()?,
                value: input.parse()?,
            });
        }

        let fork = input.fork();
        if let Ok(attr) = fork.parse() {
            input.advance_to(&fork);
            return Ok(Self::Split(attr));
        }

        let content;
        Ok(Self::SingleMarkup {
            _paren_token: syn::parenthesized!(content in input),
            markup: content.parse()?,
        })
    }
}

struct SplitMessageAttrs {
    _paren_token: Paren,
    attrs: Punctuated<SplitMessageAttr, Token![,]>,
}

impl Parse for SplitMessageAttrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _paren_token: syn::parenthesized!(content in input),
            attrs: content.parse_terminated(SplitMessageAttr::parse)?,
        })
    }
}

enum SplitMessageAttr {
    Description {
        _eq_token: Token![=],
        value: syn::LitStr,
    },
    Message {
        _paren_token: Paren,
        markup: TokenStream,
    },
}

impl Parse for SplitMessageAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        if name == "description" {
            return Ok(Self::Description {
                _eq_token: input.parse()?,
                value: input.parse()?,
            });
        }

        if name == "message" {
            let content;
            return Ok(Self::Message {
                _paren_token: syn::parenthesized!(content in input),
                markup: content.parse()?,
            });
        }

        Err(Error::new_spanned(name, "unknown attribute"))
    }
}

struct TagsAttr {
    _paren_token: Paren,
    tags: Punctuated<Ident, Token![|]>,
}

impl Parse for TagsAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Self {
            _paren_token: syn::parenthesized!(content in input),
            tags: content.parse_terminated(Ident::parse)?,
        })
    }
}

struct LocationAttr {
    _paren_token: Paren,
    field: LocationField,
}

pub(crate) enum LocationField {
    Resource(Ident),
    Span(Ident),
    SourceCode(Ident),
}

impl Parse for LocationAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _paren_token = syn::parenthesized!(content in input);
        let ident: Ident = content.parse()?;

        let field = if ident == "resource" {
            LocationField::Resource(ident)
        } else if ident == "span" {
            LocationField::Span(ident)
        } else if ident == "source_code" {
            LocationField::SourceCode(ident)
        } else {
            return Err(Error::new_spanned(ident, "unknown location field"));
        };

        Ok(Self {
            _paren_token,
            field,
        })
    }
}

impl ToTokens for LocationField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            LocationField::Resource(ident) => ident.to_tokens(tokens),
            LocationField::Span(ident) => ident.to_tokens(tokens),
            LocationField::SourceCode(ident) => ident.to_tokens(tokens),
        }
    }
}
