mod container_attrs;
mod enum_variant_attrs;
mod struct_field_attrs;

use self::container_attrs::{ContainerAttrs, UnknownFields};
use self::struct_field_attrs::DeprecatedField;
use crate::deserializable_derive::enum_variant_attrs::EnumVariantAttrs;
use crate::deserializable_derive::struct_field_attrs::StructFieldAttrs;
use biome_string_case::Case;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error2::*;
use quote::quote;
use syn::{Data, GenericParam, Generics, Path, Type};

pub(crate) struct DeriveInput {
    pub ident: Ident,
    pub generics: Generics,
    pub data: DeserializableData,
}

impl DeriveInput {
    pub fn parse(input: syn::DeriveInput) -> Self {
        let attrs =
            ContainerAttrs::try_from(&input.attrs).expect("Could not parse field attributes");
        let data = if let ContainerAttrs {
            with_validator,
            from: Some(from),
            ..
        } = attrs
        {
            DeserializableData::From(DeserializableFromData {
                from,
                with_validator,
            })
        } else if let ContainerAttrs {
            with_validator,
            try_from: Some(try_from),
            ..
        } = attrs
        {
            DeserializableData::TryFrom(DeserializableTryFromData {
                try_from,
                with_validator,
            })
        } else {
            match input.data {
                Data::Enum(data) => {
                    let variants = data
                    .variants
                    .into_iter()
                    .map(|variant| {
                        if !variant.fields.is_empty() {
                            abort!(
                                variant.fields,
                                "Deserializable derive cannot handle enum variants with fields -- you may need a custom Deserializable implementation"
                            )
                        }
                        let attrs = EnumVariantAttrs::try_from(&variant.attrs).expect("Could not parse enum variant attributes");
                        let ident = variant.ident;
                        let key = attrs
                            .rename
                            .unwrap_or_else(|| Case::Camel.convert(&ident.to_string()));

                        DeserializableVariantData { ident, key }
                    })
                    .collect();
                    DeserializableData::Enum(DeserializableEnumData {
                        variants,
                        with_validator: attrs.with_validator,
                    })
                }
                Data::Struct(data) => {
                    if data.fields.iter().all(|field| field.ident.is_some()) {
                        let mut rest_field = None;
                        let fields = data
                            .fields
                            .into_iter()
                            .filter_map(|field| {
                                field.ident.map(|ident| (ident, field.attrs, field.ty))
                            })
                            .filter_map(|(ident, attrs, ty)| {
                                let attrs = StructFieldAttrs::try_from(&attrs)
                                    .expect("Could not parse field attributes");
                                if attrs.skip {
                                    return None;
                                }

                                let key = attrs
                                    .rename
                                    .unwrap_or_else(|| Case::Camel.convert(&ident.to_string()));

                                if rest_field.is_some() && attrs.rest {
                                    abort!(
                                        ident,
                                        "Cannot have multiple fields with #[deserializable(rest)]"
                                    )
                                }
                                if attrs.rest {
                                    rest_field = Some(ident.clone());
                                    // If rest field, we don't return a field data, because we don't
                                    // want to deserialize into the field directly.
                                    return None;
                                }

                                Some(DeserializableFieldData {
                                    bail_on_error: attrs.bail_on_error,
                                    deprecated: attrs.deprecated,
                                    ident,
                                    key,
                                    required: attrs.required,
                                    ty,
                                    validate: attrs.validate,
                                })
                            })
                            .collect();

                        if rest_field.is_some()
                            && matches!(attrs.unknown_fields, Some(UnknownFields::Deny))
                        {
                            abort!(
                                rest_field.unwrap(),
                                "Cannot have a field with #[deserializable(rest)] and #[deserializable(unknown_fields = \"deny\")]"
                            )
                        }

                        DeserializableData::Struct(DeserializableStructData {
                            fields,
                            rest_field,
                            with_validator: attrs.with_validator,
                            unknown_fields: attrs.unknown_fields.unwrap_or_default(),
                        })
                    } else if data.fields.len() == 1 {
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
            }
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
    Enum(DeserializableEnumData),
    Newtype(DeserializableNewtypeData),
    Struct(DeserializableStructData),
    From(DeserializableFromData),
    TryFrom(DeserializableTryFromData),
}

#[derive(Debug)]
pub struct DeserializableEnumData {
    variants: Vec<DeserializableVariantData>,
    with_validator: bool,
}

#[derive(Debug)]
pub struct DeserializableNewtypeData {
    with_validator: bool,
}

#[derive(Debug)]
pub struct DeserializableStructData {
    fields: Vec<DeserializableFieldData>,
    rest_field: Option<Ident>,
    with_validator: bool,
    unknown_fields: UnknownFields,
}

#[derive(Debug)]
pub struct DeserializableFromData {
    pub from: Path,
    pub with_validator: bool,
}

#[derive(Debug)]
pub struct DeserializableTryFromData {
    pub try_from: Path,
    pub with_validator: bool,
}

#[derive(Clone, Debug)]
pub struct DeserializableFieldData {
    bail_on_error: bool,
    deprecated: Option<DeprecatedField>,
    ident: Ident,
    key: String,
    required: bool,
    ty: Type,
    validate: Option<Path>,
}

#[derive(Debug)]
pub struct DeserializableVariantData {
    ident: Ident,
    key: String,
}

pub(crate) fn generate_deserializable(input: DeriveInput) -> TokenStream {
    match input.data {
        DeserializableData::Enum(data) => {
            generate_deserializable_enum(input.ident, input.generics, data)
        }
        DeserializableData::Newtype(data) => {
            generate_deserializable_newtype(input.ident, input.generics, data)
        }
        DeserializableData::Struct(data) => {
            generate_deserializable_struct(input.ident, input.generics, data)
        }
        DeserializableData::From(data) => {
            generate_deserializable_from(input.ident, input.generics, data)
        }
        DeserializableData::TryFrom(data) => {
            generate_deserializable_try_from(input.ident, input.generics, data)
        }
    }
}

fn generate_deserializable_enum(
    ident: Ident,
    generics: Generics,
    data: DeserializableEnumData,
) -> TokenStream {
    let allowed_variants: Vec<_> = data
        .variants
        .iter()
        .map(|DeserializableVariantData { key, .. }| quote! { #key })
        .collect();

    let deserialize_variants: Vec<_> = data
        .variants
        .iter()
        .map(
            |DeserializableVariantData {
                 ident: variant_ident,
                 key,
             }| {
                quote! { #key => Self::#variant_ident }
            },
        )
        .collect();

    let validator = if data.with_validator {
        quote! {
            if !biome_deserialize::DeserializableValidator::validate(&mut result, ctx, name, value.range()) {
                return None;
            }
        }
    } else {
        quote! {}
    };

    let trait_bounds = generate_trait_bounds(&generics);

    quote! {
        impl #generics biome_deserialize::Deserializable for #ident #generics #trait_bounds{
            fn deserialize(
                ctx: &mut impl biome_deserialize::DeserializationContext,
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
            ) -> Option<Self> {
                let mut result = match biome_deserialize::Text::deserialize(ctx, value, name)?.text() {
                    #(#deserialize_variants),*,
                    unknown_variant => {
                        const ALLOWED_VARIANTS: &[&str] = &[#(#allowed_variants),*];
                        ctx.report(biome_deserialize::DeserializationDiagnostic::new_unknown_value(
                            unknown_variant,
                            value.range(),
                            ALLOWED_VARIANTS,
                        ));
                        return None;
                    }
                };
                #validator
                Some(result)
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
            if !biome_deserialize::DeserializableValidator::validate(&mut result, ctx, name, value.range()) {
                return None;
            }
        }
    } else {
        quote! {}
    };

    let trait_bounds = generate_trait_bounds(&generics);
    let generics = generate_generics_without_trait_bounds(&generics);

    quote! {
        impl #generics biome_deserialize::Deserializable for #ident #generics #trait_bounds {
            fn deserialize(
                ctx: &mut impl biome_deserialize::DeserializationContext,
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
            ) -> Option<Self> {
                let result = biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self)?;
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
            let DeserializableFieldData {
                ident: field_ident,
                key,
                ..
            } = field_data;
            let deprecation_notice = field_data.deprecated.map(|deprecated| match deprecated {
                DeprecatedField::Message(message) => quote! {
                    ctx.report(DeserializationDiagnostic::new_deprecated(
                        key_text.text(),
                        value.range()
                    ).with_note(#message));
                },
                DeprecatedField::UseInstead(path) => quote! {
                    ctx.report(DeserializationDiagnostic::new_deprecated_use_instead(
                        &key_text,
                        key.range(),
                        #path,
                    ));
                },
            });

            let validate = field_data.validate.map(|path| {
                quote! {
                    .filter(|v| #path(ctx, v, #key, value.range()))
                }
            });

            let error_result = if field_data.bail_on_error || field_data.required {
                quote! { return None, }
            } else {
                quote! { {} }
            };

            quote! {
                #key => {
                    match Deserializable::deserialize(ctx, &value, &key_text)#validate {
                        Some(value) => {
                            #deprecation_notice
                            result.#field_ident = value;
                        }
                        None => #error_result
                    }
                }
            }
        })
        .collect();

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
                    ctx.report(DeserializationDiagnostic::new_missing_key(
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
            if !biome_deserialize::DeserializableValidator::validate(&mut result, ctx, name, range) {
                return None;
            }
        }
    } else {
        validator
    };
    let unknown_key_handler = if let Some(rest_field) = data.rest_field {
        quote! {
            unknown_key => {
                let key_text = Text::deserialize(ctx, &key, "")?;
                if let Some(value) = Deserializable::deserialize(ctx, &value, key_text.text()) {
                    std::iter::Extend::extend(&mut result.#rest_field, [(key_text, value)]);
                }
            }
        }
    } else {
        match data.unknown_fields {
            UnknownFields::Warn | UnknownFields::Deny => {
                let with_customseverity = if data.unknown_fields == UnknownFields::Warn {
                    quote! { .with_custom_severity(biome_diagnostics::Severity::Warning) }
                } else {
                    quote! {}
                };
                quote! {
                    unknown_key => {
                        const ALLOWED_KEYS: &[&str] = &[#(#allowed_keys),*];
                        ctx.report(DeserializationDiagnostic::new_unknown_key(
                            unknown_key,
                            key.range(),
                            ALLOWED_KEYS,
                        )#with_customseverity)
                    }
                }
            }
            UnknownFields::Allow => quote! { _ => {} },
        }
    };

    let tuple_type = generate_generics_tuple(&generics);
    let trait_bounds = generate_trait_bounds(&generics);
    let generics = generate_generics_without_trait_bounds(&generics);

    quote! {
        impl #generics biome_deserialize::Deserializable for #ident #generics #trait_bounds {
            fn deserialize(
                ctx: &mut impl biome_deserialize::DeserializationContext,
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
            ) -> Option<Self> {
                use std::marker::PhantomData;
                struct Visitor #generics (PhantomData< #tuple_type >);
                impl #generics biome_deserialize::DeserializationVisitor for Visitor #generics #trait_bounds {
                    type Output = #ident #generics;

                    const EXPECTED_TYPE: biome_deserialize::DeserializableTypes = biome_deserialize::DeserializableTypes::MAP;

                    fn visit_map(
                        self,
                        ctx: &mut impl biome_deserialize::DeserializationContext,
                        members: impl Iterator<Item = Option<(impl biome_deserialize::DeserializableValue, impl biome_deserialize::DeserializableValue)>>,
                        range: biome_deserialize::TextRange,
                        name: &str,
                    ) -> Option<Self::Output> {
                        use biome_deserialize::{Deserializable, DeserializationDiagnostic, Text};
                        let mut result: Self::Output = Self::Output::default();
                        for (key, value) in members.flatten() {
                            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                                continue;
                            };
                            match key_text.text() {
                                #(#deserialize_fields)*
                                #unknown_key_handler
                            }
                        }
                        #validator
                        Some(result)
                    }
                }

                value.deserialize(ctx, Visitor(PhantomData), name)
            }
        }
    }
}

fn generate_deserializable_from(
    ident: Ident,
    generics: Generics,
    data: DeserializableFromData,
) -> TokenStream {
    let trait_bounds = generate_trait_bounds(&generics);
    let generics = generate_generics_without_trait_bounds(&generics);
    let from = data.from;
    let validator = if data.with_validator {
        quote! {
            if !biome_deserialize::DeserializableValidator::validate(&mut result, ctx, name, value.range()) {
                return None;
            }
        }
    } else {
        quote! {}
    };
    quote! {
        impl #generics biome_deserialize::Deserializable for #ident #generics #trait_bounds {
            fn deserialize(
                ctx: &mut impl biome_deserialize::DeserializationContext,
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
            ) -> Option<Self> {
                let result: #from = biome_deserialize::Deserializable::deserialize(ctx, value, name)?;
                let mut result: Self = result.into();
                #validator
                Some(result)
            }
        }
    }
}

fn generate_deserializable_try_from(
    ident: Ident,
    generics: Generics,
    data: DeserializableTryFromData,
) -> TokenStream {
    let trait_bounds = generate_trait_bounds(&generics);
    let generics = generate_generics_without_trait_bounds(&generics);
    let try_from = data.try_from;
    let validator = if data.with_validator {
        quote! {
            if !biome_deserialize::DeserializableValidator::validate(&mut result, ctx, name, value.range()) {
                return None;
            }
        }
    } else {
        quote! {}
    };
    quote! {
        impl #generics biome_deserialize::Deserializable for #ident #generics #trait_bounds {
            fn deserialize(
                ctx: &mut impl biome_deserialize::DeserializationContext,
                value: &impl biome_deserialize::DeserializableValue,
                name: &str,
            ) -> Option<Self> {
                let mut result: #try_from = biome_deserialize::Deserializable::deserialize(ctx, value, name)?;
                match result.try_into() {
                    Ok(result) => {
                        #validator
                        Some(result)
                    }
                    Err(err) => {
                        ctx.report(biome_deserialize::DeserializationDiagnostic::new(
                            format_args!("{}", err)
                        ).with_range(value.range()));
                        None
                    }
                }
            }
        }
    }
}

fn generate_generics_without_trait_bounds(generics: &Generics) -> TokenStream {
    if generics.params.is_empty() {
        quote! {}
    } else {
        let params = generics.params.iter().map(|param| match param {
            GenericParam::Type(ty) => {
                let attrs = ty
                    .attrs
                    .iter()
                    .fold(quote! {}, |acc, attr| quote! { #acc #attr });
                let ident = &ty.ident;
                quote! { #attrs #ident }
            }
            _ => abort!(generics, "Unsupported generic parameter"),
        });
        quote! {
            < #(#params),* >
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
                let bounds = &ty.bounds;
                if bounds.is_empty() {
                    quote! { #ident: biome_deserialize::Deserializable }
                } else {
                    quote! { #ident: #bounds + biome_deserialize::Deserializable }
                }
            }
            _ => abort!(generics, "Unsupported generic parameter"),
        });
        quote! {
            where #(#params),*
        }
    }
}

fn generate_generics_tuple(generics: &Generics) -> TokenStream {
    let params = generics.params.iter().map(|param| match param {
        GenericParam::Type(ty) => {
            let ident = &ty.ident;
            quote! { #ident }
        }
        _ => abort!(generics, "Unsupported generic parameter"),
    });
    quote! {
        ( #(#params),* )
    }
}
