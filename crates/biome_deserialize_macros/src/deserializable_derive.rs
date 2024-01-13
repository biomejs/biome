mod enum_variant_attrs;
mod struct_attrs;
mod struct_field_attrs;

use crate::deserializable_derive::enum_variant_attrs::EnumVariantAttrs;
use crate::deserializable_derive::struct_field_attrs::StructFieldAttrs;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::*;
use quote::quote;
use syn::{Data, Type};
use self::struct_attrs::StructAttrs;

pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub data: DeserializableData,
}

impl DeriveInput {
    pub fn parse(input: syn::DeriveInput) -> Self {
        let data = match input.data {
            Data::Enum(data) => {
                let data = data
                    .variants
                    .into_iter()
                    .filter(|variant| {
                        if variant.fields.is_empty() {
                            true
                        } else {
                            abort!(
                                variant.fields,
                                "Deserializable derive cannot handle enum variants with fields -- you may need a custom Deserializable implementation"
                            )
                        }
                    })
                    .map(|variant| {
                        let attrs = EnumVariantAttrs::from_attrs(&variant.attrs);
            
                        let ident = variant.ident;
                        let key = attrs
                            .rename
                            .unwrap_or_else(|| ident.to_string().to_case(Case::Camel));
            
                        DeserializableVariantData { ident, key }
                    })
                    .collect();
                DeserializableData::Enum(data)
            },
            Data::Struct(data) => {
                if data.fields.iter().all(|field| field.ident.is_some()) {
                    let attrs = StructAttrs::from_attrs(&input.attrs);

                    let fields = data
                        .fields
                        .into_iter()
                        .filter_map(|field| field.ident.map(|ident| (ident, field.attrs, field.ty)))
                        .map(|(ident, attrs, ty)| {
                            let attrs = StructFieldAttrs::from_attrs(&attrs);
                            
                            let disallow_empty = attrs.disallow_empty;
                            let key = attrs
                                .rename
                                .unwrap_or_else(|| ident.to_string().to_case(Case::Camel));
                
                            DeserializableFieldData { disallow_empty, ident, key, ty }
                        })
                        .collect();
                    let from_none = attrs.from_none;
                    DeserializableData::Struct(DeserializableStructData { fields, from_none })
                } else if data.fields.len() == 1 {
                    DeserializableData::Newtype
                } else {
                    abort!(
                        data.fields,
                        "Deserializable derive requires structs to have named fields or a single unnamed one -- you may need a custom Deserializable implementation"
                    )
                }
            }
            _ => abort!(input, "Deserializable can only be derived for enums and structs"),
        };

        Self {
            ident: input.ident,
            data,
        }
    }
}

pub enum DeserializableData {
    Enum(Vec<DeserializableVariantData>),
    Newtype,
    Struct(DeserializableStructData),
}

pub struct DeserializableStructData {
    fields: Vec<DeserializableFieldData>,
    from_none: bool,
}

pub struct DeserializableFieldData {
    disallow_empty: bool,
    ident: Ident,
    key: String,
    ty: Type,
}

pub struct DeserializableVariantData {
    ident: Ident,
    key: String,
}

pub(crate) fn generate_deserializable(input: DeriveInput) -> TokenStream {
    match input.data {
        DeserializableData::Enum(variants) => generate_deserializable_enum(input.ident, variants),
        DeserializableData::Newtype => generate_deserializable_newtype(input.ident),
        DeserializableData::Struct(data) => generate_deserializable_struct(input.ident, data),
    }
}

fn generate_deserializable_enum(ident: Ident, variants: Vec<DeserializableVariantData>) -> TokenStream {
    let allowed_variants: Vec<_> = variants
        .iter()
        .map(|DeserializableVariantData { key, .. }| quote! { #key })
        .collect();

    let deserialize_variants: Vec<_> = variants
        .iter()
        .map(|DeserializableVariantData { ident: variant_ident, key }| {
            quote! { #key => Some(#ident::#variant_ident) }
        })
        .collect();

    quote! {
        impl biome_deserialize::Deserializable for #ident {
            fn deserialize(
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
                diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
            ) -> Option<Self> {
                match biome_deserialize::Text::deserialize(value, name, diagnostics)?.text() {
                    #(#deserialize_variants),*,
                    unknown_variant => {
                        const ALLOWED_VARIANTS: &[&str] = &[#(#allowed_variants),*];
                        diagnostics.push(biome_deserialize::DeserializationDiagnostic::new_unknown_value(
                            unknown_variant,
                            value.range(),
                            ALLOWED_VARIANTS,
                        ));
                        None
                    }
                }
            }
        }
    }
}

fn generate_deserializable_newtype(ident: Ident) -> TokenStream {
    quote! {
        impl biome_deserialize::Deserializable for #ident {
            fn deserialize(
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
                diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
            ) -> Option<Self> {
                biome_deserialize::Deserializable::deserialize(value, name, diagnostics).map(#ident)
            }
        }
    }
}

fn generate_deserializable_struct(ident: Ident, data: DeserializableStructData) -> TokenStream {
    let allowed_keys: Vec<_> = data
        .fields
        .iter()
        .map(|DeserializableFieldData { key, .. }| quote! { #key })
        .collect();

    let deserialize_fields: Vec<_> = data
        .fields
        .into_iter()
        .map(|DeserializableFieldData { disallow_empty, ident: field_ident, key, ty }| {
            let is_optional = matches!(
                &ty,
                Type::Path(path) if path
                    .path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == "Option")
            );

            let is_empty_check = disallow_empty.then(|| {
                let test = if is_optional {
                    quote! { value.as_ref().is_some_and(|v| v.is_empty()) }
                } else {
                    quote! { value.is_empty() }
                };
                quote! {
                    if #test {
                        diagnostics.push(
                            DeserializationDiagnostic::new(markup!(
                                <Emphasis>#key</Emphasis>" may not be empty"
                            ))
                                .with_range(value_range)
                        );
                        continue;
                    }
                }
            });

            if is_optional {
                quote! {
                    #key => {
                        let value: #ty = Deserializable::deserialize(&value, &key_text, diagnostics);
                        #is_empty_check
                        result.#field_ident = value;
                    }
                }
            } else {
                quote! {
                    #key => {
                        let deserialize_result: Option<#ty> =
                            Deserializable::deserialize(&value, &key_text, diagnostics);
                        if let Some(value) = deserialize_result {
                            #is_empty_check
                            result.#field_ident = value;
                        }
                    }
                }
            }
        })
        .collect();

    let visitor_ident = Ident::new(&format!("{ident}Visitor"), Span::call_site());
    let result_init = if data.from_none {
        quote! { biome_deserialize::NoneState::none() }
    } else {
        quote! { Self::Output::default() }
    };

    quote! {
        impl biome_deserialize::Deserializable for #ident {
            fn deserialize(
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
                diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
            ) -> Option<Self> {
                value.deserialize(#visitor_ident, name, diagnostics)
            }
        }

        struct #visitor_ident;
        impl biome_deserialize::DeserializationVisitor for #visitor_ident {
            type Output = #ident;

            const EXPECTED_TYPE: biome_deserialize::VisitableType = biome_deserialize::VisitableType::MAP;

            fn visit_map(
                self,
                members: impl Iterator<Item = Option<(impl biome_deserialize::DeserializableValue, impl biome_deserialize::DeserializableValue)>>,
                _range: biome_deserialize::TextRange,
                _name: &str,
                diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                use biome_deserialize::{Deserializable, DeserializationDiagnostic, Text};
                let mut result: Self::Output = #result_init;
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
                    let value_range = value.range();
                    match key_text.text() {
                        #(#deserialize_fields)*
                        unknown_key => {
                            const ALLOWED_KEYS: &[&str] = &[#(#allowed_keys),*];
                            diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                                unknown_key,
                                key.range(),
                                ALLOWED_KEYS,
                            ))
                        }
                    }
                }
                Some(result)
            }
        }
    }
}
