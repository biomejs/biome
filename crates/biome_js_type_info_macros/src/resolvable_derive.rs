use proc_macro_error2::*;
use proc_macro2::{Ident, TokenStream};
use quote::{ToTokens, quote};
use syn::{Data, Fields, GenericArgument, PathArguments, Type};

pub(crate) enum DeriveInput {
    Enum {
        ident: Ident,
        variants: Vec<VariantData>,
    },
    Struct {
        ident: Ident,
        fields: Vec<FieldData>,
    },
    Unit {
        ident: Ident,
        ty: Box<Type>,
    },
}

pub(crate) struct FieldData {
    ident: Ident,
    ty: Type,
}

pub(crate) struct VariantData {
    ident: Ident,
    ty: Option<Type>,
}

impl DeriveInput {
    pub fn parse(input: syn::DeriveInput) -> Self {
        let ident = input.ident.clone();

        match input.data {
            Data::Struct(data)
                if data.fields.len() == 1
                    && data
                        .fields
                        .iter()
                        .next()
                        .is_some_and(|field| field.ident.is_none()) =>
            {
                Self::Unit {
                    ident,
                    ty: Box::new(data.fields.into_iter().next().unwrap().ty),
                }
            }
            Data::Struct(data) => {
                let fields = data
                    .fields
                    .into_iter()
                    .map(|field| {
                        let Some(ident) = field.ident else {
                            abort!(
                                ident,
                                "Resolvable derive requires either named struct fields or a struct with a single unnamed field"
                            );
                        };
                        let ty = field.ty;

                        FieldData { ident, ty }
                    })
                    .collect();

                Self::Struct { ident, fields }
            }
            Data::Enum(data) => {
                let variants = data
                    .variants
                    .into_iter()
                    .map(|variant| {
                        let ident = variant.ident;
                        let ty = match variant.fields {
                            Fields::Unnamed(fields)
                                if fields.unnamed.len() == 1 => Some(
                                    fields.unnamed.into_iter().next().unwrap().ty
                                ),
                            Fields::Unit => None,
                            fields => abort!(
                                fields,
                                "Resolvable derive requires enum variants with either a single unnamed field or no field at all"
                            )
                        };

                        VariantData { ident, ty }
                    })
                    .collect();

                Self::Enum { ident, variants }
            }
            _ => abort!(
                input,
                "Resolvable can only be derived for structs and enums"
            ),
        }
    }
}

pub(crate) fn generate_resolvable(input: DeriveInput) -> TokenStream {
    match input {
        DeriveInput::Enum { ident, variants } => generate_resolvable_enum(ident, variants),
        DeriveInput::Struct { ident, fields } => generate_resolvable_struct(ident, fields),
        DeriveInput::Unit { ident, ty } => generate_resolvable_unit_type(ident, *ty),
    }
}

pub(crate) fn generate_resolvable_enum(ident: Ident, variants: Vec<VariantData>) -> TokenStream {
    let resolved_variants = variants.iter().map(|VariantData { ident, ty }| match ty {
        Some(ty) => {
            let resolved_ty = resolved_unit_type(ty);
            quote! { Self::#ident(ty) => Self::#ident(#resolved_ty) }
        }
        None => quote! { Self::#ident => Self::#ident },
    });

    let resolved_variants_with_mapped_references =
        variants.iter().map(|VariantData { ident, ty }| match ty {
            Some(ty) => {
                let resolved_ty = resolved_unit_type_with_mapped_references(ty);
                quote! { Self::#ident(ty) => Self::#ident(#resolved_ty) }
            }
            None => quote! { Self::#ident => Self::#ident },
        });

    let variants_with_module_id = variants.iter().map(|VariantData { ident, ty }| match ty {
        Some(ty) => {
            let ty_with_module_id = unit_type_with_module_id(ty);
            quote! { Self::#ident(ty) => Self::#ident(#ty_with_module_id) }
        }
        None => quote! { Self::#ident => Self::#ident },
    });

    quote! {
        impl crate::Resolvable for #ident {
            fn resolved(&self, resolver: &mut dyn crate::TypeResolver) -> Self {
                match self {
                    #( #resolved_variants ),*
                }
            }

            fn resolved_with_mapped_references(
                &self,
                map: impl Copy + Fn(crate::TypeReference, &mut dyn crate::TypeResolver) -> crate::TypeReference,
                resolver: &mut dyn crate::TypeResolver
            ) -> Self {
                match self {
                    #( #resolved_variants_with_mapped_references ),*
                }
            }

            fn with_module_id(self, module_id: crate::ModuleId) -> Self {
                match self {
                    #( #variants_with_module_id ),*
                }
            }
        }
    }
}

pub(crate) fn generate_resolvable_struct(ident: Ident, fields: Vec<FieldData>) -> TokenStream {
    let resolved_fields = fields.iter().map(|FieldData { ident, ty }| {
        let resolved_ty = resolved_type(IdentOrZero::Ident(ident), ty);
        quote! { #ident: #resolved_ty }
    });

    let resolved_fields_with_mapped_references = fields.iter().map(|FieldData { ident, ty }| {
        let resolved_ty = resolved_type_with_mapped_references(IdentOrZero::Ident(ident), ty);
        quote! { #ident: #resolved_ty }
    });

    let fields_with_module_id = fields.iter().map(|FieldData { ident, ty }| {
        let ty_with_module_id = type_with_module_id(IdentOrZero::Ident(ident), ty);
        quote! { #ident: #ty_with_module_id }
    });

    quote! {
        impl crate::Resolvable for #ident {
            fn resolved(&self, resolver: &mut dyn crate::TypeResolver) -> Self {
                Self {
                    #( #resolved_fields ),*
                }
            }

            fn resolved_with_mapped_references(
                &self,
                map: impl Copy + Fn(crate::TypeReference, &mut dyn crate::TypeResolver) -> crate::TypeReference,
                resolver: &mut dyn crate::TypeResolver
            ) -> Self {
                Self {
                    #( #resolved_fields_with_mapped_references ),*
                }
            }

            fn with_module_id(self, module_id: crate::ModuleId) -> Self {
                Self {
                    #( #fields_with_module_id ),*
                }
            }
        }
    }
}

fn generate_resolvable_unit_type(ident: Ident, ty: Type) -> TokenStream {
    let resolved_field = resolved_type(IdentOrZero::Zero, &ty);

    let resolved_field_with_mapped_references =
        resolved_type_with_mapped_references(IdentOrZero::Zero, &ty);

    let field_with_module_id = type_with_module_id(IdentOrZero::Zero, &ty);

    quote! {
        impl crate::Resolvable for #ident {
            fn resolved(&self, resolver: &mut dyn crate::TypeResolver) -> Self {
                Self(#resolved_field)
            }

            fn resolved_with_mapped_references(
                &self,
                map: impl Copy + Fn(crate::TypeReference, &mut dyn crate::TypeResolver) -> crate::TypeReference,
                resolver: &mut dyn crate::TypeResolver
            ) -> Self {
                Self(#resolved_field_with_mapped_references)
            }

            fn with_module_id(self, module_id: crate::ModuleId) -> Self {
                Self(#field_with_module_id)
            }
        }
    }
}

fn resolved_type(ident: IdentOrZero, ty: &Type) -> TokenStream {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Text" => {
            quote! { self.#ident.clone() }
        }
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => {
                            quote! {
                                self.#ident.clone()
                            }
                        }
                        Type::Path(_) => quote! {
                            self.#ident.iter().map(|elem| elem.resolved(resolver)).collect()
                        },
                        _ => abort!(slice, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! {
                                self.#ident.clone()
                            }
                        } else {
                            quote! {
                                Box::new(self.#ident.resolved(resolver))
                            }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        Some(segment) if segment.ident == "Option" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Option is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { self.#ident.clone() }
                        } else {
                            quote! { self.#ident.as_ref().map(|f| f.resolved(resolver)) }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        _ => {
            quote! { self.#ident.resolved(resolver) }
        }
    }
}

fn resolved_type_with_mapped_references(ident: IdentOrZero, ty: &Type) -> TokenStream {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Text" => {
            quote! { self.#ident.clone() }
        }
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => {
                            quote! {
                                self.#ident.clone()
                            }
                        }
                        Type::Path(_) => quote! {
                            self.#ident.iter()
                                .map(|elem| elem.resolved_with_mapped_references(map, resolver))
                                .collect()
                        },
                        _ => abort!(slice, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! {
                                self.#ident.clone()
                            }
                        } else {
                            quote! {
                                Box::new(self.#ident.resolved_with_mapped_references(map, resolver))
                            }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        Some(segment) if segment.ident == "Option" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Option is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { self.#ident.clone() }
                        } else {
                            quote! {
                                self.#ident
                                    .as_ref()
                                    .map(|f| f.resolved_with_mapped_references(map, resolver))
                            }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        _ => {
            quote! { self.#ident.resolved_with_mapped_references(map, resolver) }
        }
    }
}

enum IdentOrZero<'a> {
    Ident(&'a Ident),
    Zero,
}

impl ToTokens for IdentOrZero<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Ident(ident) => ident.to_tokens(tokens),
            Self::Zero => quote! { 0 }.to_tokens(tokens),
        }
    }
}

fn resolved_unit_type(ty: &Type) -> TokenStream {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Text" => {
            quote! { ty.clone() }
        }
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => quote! { ty.clone() },
                        _ => abort!(args, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { ty.clone() }
                        } else {
                            quote! { Box::new(ty.resolved(resolver)) }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        Some(segment) if segment.ident == "Option" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Option is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { ty.clone() }
                        } else {
                            quote! { ty.as_ref().map(|f| f.resolved(resolver)) }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        _ => {
            quote! { ty.resolved(resolver) }
        }
    }
}

fn resolved_unit_type_with_mapped_references(ty: &Type) -> TokenStream {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Text" => {
            quote! { ty.clone() }
        }
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => quote! { ty.clone() },
                        _ => abort!(args, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { ty.clone() }
                        } else {
                            quote! { Box::new(ty.resolved_with_mapped_references(map, resolver)) }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        Some(segment) if segment.ident == "Option" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Option is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { ty.clone() }
                        } else {
                            quote! {
                                ty
                                    .as_ref()
                                    .map(|f| f.resolved_with_mapped_references(map, resolver))
                            }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        _ => {
            quote! { ty.resolved_with_mapped_references(map, resolver) }
        }
    }
}

fn type_with_module_id(ident: IdentOrZero, ty: &Type) -> TokenStream {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Text" => {
            quote! { self.#ident }
        }
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => {
                            quote! { self.#ident }
                        }
                        Type::Path(_) => quote! {
                            self.#ident.into_iter().map(|elem| elem.with_module_id(module_id)).collect()
                        },
                        _ => abort!(slice, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { self.#ident }
                        } else {
                            quote! {
                                Box::new(self.#ident.with_module_id(module_id))
                            }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        Some(segment) if segment.ident == "Option" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Option is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { self.#ident }
                        } else {
                            quote! { self.#ident.map(|f| f.with_module_id(module_id)) }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        _ => {
            quote! { self.#ident.with_module_id(module_id) }
        }
    }
}

fn unit_type_with_module_id(ty: &Type) -> TokenStream {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Text" => {
            quote! { ty }
        }
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => quote! { ty.clone() },
                        _ => abort!(args, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { ty }
                        } else {
                            quote! { Box::new(ty.with_module_id(module_id)) }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        Some(segment) if segment.ident == "Option" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Option is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { ty }
                        } else {
                            quote! { ty.map(|f| f.with_module_id(module_id)) }
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        _ => {
            quote! { ty.with_module_id(module_id) }
        }
    }
}
