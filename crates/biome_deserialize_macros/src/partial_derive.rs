mod attrs;

use crate::partial_derive::attrs::PartialType;
use attrs::{Attrs, FieldAttrs};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::*;
use quote::quote;
use syn::{Data, Field, Fields, Type};

pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub partial_ident: Ident,
    pub attrs: Attrs,
    pub fields: Fields,
}

impl DeriveInput {
    pub fn parse(input: syn::DeriveInput) -> Self {
        let ident = input.ident.clone();
        let partial_ident = Ident::new(&format!("Partial{}", input.ident), Span::call_site());

        let attrs = Attrs::from_attrs(&input.attrs);

        let fields = match input.data {
            Data::Struct(data) => data.fields,
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

    let attrs = input.attrs.nested_attrs.iter().map(|(ident, nested)| {
        quote! {
            #[#ident #nested]
        }
    });

    let fields = input.fields.iter().map(
        |Field {
             ident, attrs, ty, ..
         }| {
            let attrs = FieldAttrs::from_attrs(attrs);

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

            let attrs = attrs.nested_attrs.iter().map(|(ident, nested)| {
                quote! {
                    #[#ident #nested]
                }
            });

            quote! {
                #( #attrs )*
                #[serde(skip_serializing_if = "Option::is_none")]
                pub #ident: Option<#ty>
            }
        },
    );

    let from_fields = input.fields.iter().map(|Field { ident, ty, .. }| {
        quote! {
            #ident: partial.#ident.map(#ty::from).unwrap_or(default.#ident)
        }
    });

    quote! {
        #[derive(#(#derives),*)]
        #( #attrs )*
        pub struct #partial_ident {
            #( #fields ),*
        }

        impl From<#partial_ident> for #ident {
            fn from(partial: #partial_ident) -> Self {
                let default = Self::default();
                Self {
                    #( #from_fields ),*
                }
            }
        }
    }
}
