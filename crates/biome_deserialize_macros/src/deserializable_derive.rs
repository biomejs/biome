mod enum_variant_attrs;
mod struct_attrs;
mod struct_field_attrs;

use self::struct_attrs::StructAttrs;
use self::struct_field_attrs::DeprecatedField;
use crate::deserializable_derive::enum_variant_attrs::EnumVariantAttrs;
use crate::deserializable_derive::struct_field_attrs::StructFieldAttrs;
use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::*;
use quote::quote;
use syn::{Data, GenericParam, Generics, Path, PathSegment, Type};

pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub generics: Generics,
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
            }
            Data::Struct(data) => {
                if data.fields.iter().all(|field| field.ident.is_some()) {
                    let attrs = StructAttrs::from_attrs(&input.attrs);

                    let fields = data
                        .fields
                        .clone()
                        .into_iter()
                        .filter_map(|field| field.ident.map(|ident| (ident, field.attrs, field.ty)))
                        .map(|(ident, attrs, ty)| {
                            let attrs = StructFieldAttrs::from_attrs(&attrs);
                            let key = attrs
                                .rename
                                .unwrap_or_else(|| ident.to_string().to_case(Case::Camel));

                            DeserializableFieldData {
                                bail_on_error: attrs.bail_on_error,
                                deprecated: attrs.deprecated,
                                ident,
                                key,
                                passthrough_name: attrs.passthrough_name,
                                required: attrs.required,
                                ty,
                                validate: attrs.validate,
                            }
                        })
                        .collect();

                    DeserializableData::Struct(DeserializableStructData {
                        fields,
                        with_validator: attrs.with_validator,
                    })
                } else if data.fields.len() == 1 {
                    let attrs = StructAttrs::from_attrs(&input.attrs);

                    DeserializableData::Newtype(DeserializableNewtypeData {
                        with_validator: attrs.with_validator,
                    })
                } else {
                    abort!(
                        data.fields,
                        "Deserializable derive requires structs to have named fields or a single unnamed one -- you may need a custom Deserializable implementation"
                    )
                }
            }
            _ => abort!(
                input,
                "Deserializable can only be derived for enums and structs"
            ),
        };

        Self {
            ident: input.ident,
            generics: input.generics,
            data,
        }
    }
}

#[derive(Debug)]
pub enum DeserializableData {
    Enum(Vec<DeserializableVariantData>),
    Newtype(DeserializableNewtypeData),
    Struct(DeserializableStructData),
}

#[derive(Debug)]
pub struct DeserializableNewtypeData {
    with_validator: bool,
}

#[derive(Debug)]
pub struct DeserializableStructData {
    fields: Vec<DeserializableFieldData>,
    with_validator: bool,
}

#[derive(Clone, Debug)]
pub struct DeserializableFieldData {
    bail_on_error: bool,
    deprecated: Option<DeprecatedField>,
    ident: Ident,
    key: String,
    passthrough_name: bool,
    required: bool,
    ty: Type,
    validate: Option<String>,
}

#[derive(Debug)]
pub struct DeserializableVariantData {
    ident: Ident,
    key: String,
}

pub(crate) fn generate_deserializable(input: DeriveInput) -> TokenStream {
    match input.data {
        DeserializableData::Enum(variants) => {
            generate_deserializable_enum(input.ident, input.generics, variants)
        }
        DeserializableData::Newtype(data) => {
            generate_deserializable_newtype(input.ident, input.generics, data)
        }
        DeserializableData::Struct(data) => {
            generate_deserializable_struct(input.ident, input.generics, data)
        }
    }
}

fn generate_deserializable_enum(
    ident: Ident,
    generics: Generics,
    variants: Vec<DeserializableVariantData>,
) -> TokenStream {
    let allowed_variants: Vec<_> = variants
        .iter()
        .map(|DeserializableVariantData { key, .. }| quote! { #key })
        .collect();

    let deserialize_variants: Vec<_> = variants
        .iter()
        .map(
            |DeserializableVariantData {
                 ident: variant_ident,
                 key,
             }| {
                quote! { #key => Some(Self::#variant_ident) }
            },
        )
        .collect();

    let trait_bounds = generate_trait_bounds(&generics);

    quote! {
        impl #generics biome_deserialize::Deserializable for #ident #generics #trait_bounds{
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

fn generate_deserializable_newtype(
    ident: Ident,
    generics: Generics,
    data: DeserializableNewtypeData,
) -> TokenStream {
    let validator = if data.with_validator {
        quote! {
            if !biome_deserialize::DeserializableValidator::validate(&result, name, value.range(), diagnostics) {
                return None;
            }
        }
    } else {
        quote! {}
    };

    let trait_bounds = generate_trait_bounds(&generics);

    quote! {
        impl #generics biome_deserialize::Deserializable for #ident #generics #trait_bounds {
            fn deserialize(
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
                diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
            ) -> Option<Self> {
                let result = biome_deserialize::Deserializable::deserialize(value, name, diagnostics).map(Self)?;
                #validator
                Some(result)
            }
        }
    }
}

fn generate_deserializable_struct(
    ident: Ident,
    generics: Generics,
    data: DeserializableStructData,
) -> TokenStream {
    let allowed_keys: Vec<_> = data
        .fields
        .iter()
        // It's not helpful to report deprecated keys as valid alternative.
        .filter(|data| data.deprecated.is_none())
        .map(|DeserializableFieldData { key, .. }| quote! { #key })
        .collect();

    let required_fields: Vec<_> = data
        .fields
        .iter()
        .filter(|data| data.required)
        .cloned()
        .collect();

    let deserialize_fields: Vec<_> = data
        .fields
        .into_iter()
        .map(|field_data| {
            let DeserializableFieldData { ident: field_ident, key, ty, .. } = field_data;

            let is_optional = matches!(
                &ty,
                Type::Path(path) if path
                    .path
                    .segments
                    .last()
                    .is_some_and(|segment| segment.ident == "Option")
            );

            let deprecation_notice = field_data.deprecated.map(|deprecated| {
                match deprecated {
                    DeprecatedField::Message(message) => quote! {
                        diagnostics.push(DeserializationDiagnostic::new_deprecated(
                            key_text.text(),
                            value.range()
                        ).with_note(#message));
                    },
                    DeprecatedField::UseInstead(path) => quote! {
                        diagnostics.push(DeserializationDiagnostic::new_deprecated_use_instead(
                            &key_text,
                            key.range(),
                            #path,
                        ));
                    },
                }
            });

            let name = match field_data.passthrough_name {
                true => quote! { name },
                false => quote! { &key_text }
            };

            let validate = field_data.validate.map(|validate| {
                let path = Path {
                    leading_colon: None,
                    segments: validate
                        .split("::")
                        .map(|segment| {
                            PathSegment::from(Ident::new(segment, Span::call_site()))
                        })
                        .collect(),
                };

                quote! {
                    .filter(|v| #path(v, #key, value.range(), diagnostics))
                }
            });

            if is_optional {
                let error_result = if field_data.bail_on_error || field_data.required {
                    quote! { return None }
                } else {
                    quote! { None }
                };

                quote! {
                    #key => {
                        result.#field_ident = match Deserializable::deserialize(&value, #name, diagnostics)#validate {
                            Some(value) => {
                                #deprecation_notice
                                Some(value)
                            }
                            None => #error_result,
                        };
                    }
                }
            } else {
                let error_result = if field_data.bail_on_error || field_data.required {
                    quote! { return None, }
                } else {
                    quote! { {} }
                };

                quote! {
                    #key => {
                        match Deserializable::deserialize(&value, #name, diagnostics)#validate {
                            Some(value) => {
                                #deprecation_notice
                                result.#field_ident = value;
                            }
                            None => #error_result
                        }
                    }
                }
            }
        })
        .collect();

    let trait_bounds = generate_trait_bounds(&generics);

    let validator = if required_fields.is_empty() {
        quote! {}
    } else {
        let required_keys: Vec<_> = required_fields
            .iter()
            .map(|field_data| &field_data.key)
            .collect();
        let required_fields = required_fields.iter().map(|field_data| {
            let DeserializableFieldData {
                ident: field_ident,
                key,
                ty,
                ..
            } = field_data;
            quote! {
                if result.#field_ident == #ty::default() {
                    diagnostics.push(DeserializationDiagnostic::new_missing_key(
                        #key,
                        range,
                        REQUIRED_KEYS,
                    ))
                }
            }
        });
        quote! {
            const REQUIRED_KEYS: &[&str] = &[#(#required_keys),*];
            #(#required_fields)*
        }
    };
    let validator = if data.with_validator {
        quote! {
            #validator
            if !biome_deserialize::DeserializableValidator::validate(&result, name, range, diagnostics) {
                return None;
            }
        }
    } else {
        validator
    };

    let visitor_ident = Ident::new(&format!("{ident}Visitor"), Span::call_site());

    quote! {
        impl #generics biome_deserialize::Deserializable for #ident #generics #trait_bounds {
            fn deserialize(
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
                diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
            ) -> Option<Self> {
                value.deserialize(#visitor_ident, name, diagnostics)
            }
        }

        struct #visitor_ident #generics;
        impl #generics biome_deserialize::DeserializationVisitor for #visitor_ident #generics {
            type Output = #ident;

            const EXPECTED_TYPE: biome_deserialize::VisitableType = biome_deserialize::VisitableType::MAP;

            fn visit_map(
                self,
                members: impl Iterator<Item = Option<(impl biome_deserialize::DeserializableValue, impl biome_deserialize::DeserializableValue)>>,
                range: biome_deserialize::TextRange,
                name: &str,
                diagnostics: &mut Vec<biome_deserialize::DeserializationDiagnostic>,
            ) -> Option<Self::Output> {
                use biome_deserialize::{Deserializable, DeserializationDiagnostic, Text};
                let mut result: Self::Output = Self::Output::default();
                for (key, value) in members.flatten() {
                    let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                        continue;
                    };
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
                #validator
                Some(result)
            }
        }
    }
}

fn generate_trait_bounds(generics: &Generics) -> TokenStream {
    if generics.params.is_empty() {
        quote! {}
    } else {
        let params = generics.params.iter().map(|param| match param {
            GenericParam::Type(ty) => {
                let ident = &ty.ident;
                quote! { #ident: biome_deserialize::Deserializable }
            }
            _ => abort!(generics, "Unsupported generic parameter"),
        });
        quote! {
            where #(#params),*
        }
    }
}
