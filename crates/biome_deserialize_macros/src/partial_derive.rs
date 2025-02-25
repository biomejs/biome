mod attrs;

use crate::partial_derive::attrs::PartialType;
use attrs::{Attrs, FieldAttrs};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error2::*;
use quote::quote;
use syn::{Data, Field, GenericArgument, PathArguments, Type};

pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub partial_ident: Ident,
    pub attrs: Attrs,
    pub fields: Vec<FieldData>,
}

pub(crate) struct FieldData {
    ident: Ident,
    ty: Type,
    should_wrap: bool,
    attrs: FieldAttrs,
}

impl DeriveInput {
    pub fn parse(input: syn::DeriveInput) -> Self {
        let ident = input.ident.clone();
        let partial_ident = Ident::new(&format!("Partial{}", input.ident), Span::call_site());

        let attrs = Attrs::try_from(&input.attrs).expect("Could not parse attributes");

        let fields = match input.data {
            Data::Struct(data) => data
                .fields
                .iter()
                .map(
                    |Field {
                         ident, ty, attrs, ..
                     }| {
                        let Some(ident) = ident else {
                            abort!(data.fields, "Partial derive requires named fields");
                        };

                        let (ty, should_wrap) = match ty {
                            Type::Path(path) => match path.path.segments.last() {
                                Some(segment) if segment.ident == "Option" => {
                                    match &segment.arguments {
                                        PathArguments::AngleBracketed(args) => {
                                            match args.args.first() {
                                                Some(GenericArgument::Type(ty)) => (ty, false),
                                                _ => abort!(
                                                    segment,
                                                    "Expected type argument in Option"
                                                ),
                                            }
                                        }
                                        _ => abort!(segment, "Expected argument in Option type"),
                                    }
                                }
                                _ => (ty, true),
                            },
                            _ => (ty, true),
                        };

                        FieldData {
                            ident: ident.clone(),
                            ty: ty.clone(),
                            should_wrap,
                            attrs: FieldAttrs::try_from(attrs)
                                .expect("Could not parse field attributes"),
                        }
                    },
                )
                .collect(),
            _ => abort!(input, "Partial can only be derived for structs"),
        };

        Self {
            ident,
            partial_ident,
            attrs,
            fields,
        }
    }
}

pub(crate) fn generate_partial(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    let partial_ident = input.partial_ident;

    let derives = input.attrs.derives.iter();

    let doc_lines = input.attrs.doc_lines.iter().map(|tokens| {
        quote! { #[doc #tokens] }
    });

    let attrs = input.attrs.nested_attrs.iter().map(|nested| {
        quote! {
            #[#nested]
        }
    });

    let fields = input.fields.iter().map(
        |FieldData {
             ident, attrs, ty, ..
         }| {
            let doc_lines = attrs.doc_lines.iter().map(|tokens| {
                quote! { #[doc #tokens] }
            });

            let ty = match attrs.ty.as_ref() {
                Some(PartialType::Literal(ty)) => ty.clone(),
                Some(PartialType::Prefixed) => {
                    let mut ty = ty.clone();
                    if let Type::Path(type_path) = &mut ty {
                        if let Some(segment) = type_path.path.segments.first_mut() {
                            segment.ident =
                                Ident::new(&format!("Partial{}", segment.ident), Span::call_site())
                        }
                    }
                    ty
                }
                None => ty.clone(),
            };

            let attrs = attrs.nested_attrs.iter().map(|nested| {
                quote! {
                    #[#nested]
                }
            });

            quote! {
                #( #doc_lines )*
                #( #attrs )*
                #[serde(skip_serializing_if = "Option::is_none")]
                pub #ident: Option<#ty>
            }
        },
    );

    let from_partial_fields = input.fields.iter().map(
        |FieldData {
             ident,
             ty,
             should_wrap,
             ..
         }| {
            if *should_wrap {
                quote! {
                    #ident: partial.#ident.map(<#ty>::from).unwrap_or(default.#ident)
                }
            } else {
                quote! {
                    #ident: partial.#ident.map(<#ty>::from)
                }
            }
        },
    );

    quote! {
        #( #doc_lines )*
        #[derive(#(#derives),*)]
        #( #attrs )*
        pub struct #partial_ident {
            #( #fields ),*
        }

        impl From<#partial_ident> for #ident {
            fn from(partial: #partial_ident) -> Self {
                let default = Self::default();
                Self {
                    #( #from_partial_fields ),*
                }
            }
        }
    }
}
