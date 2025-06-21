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
        Some(ty) => match resolved_unit_type(ident, ty) {
            Some(resolved_ty) => quote! { Self::#ident(ty) => #resolved_ty },
            None => quote! { Self::#ident(_) => None },
        },
        None => quote! { Self::#ident => None },
    });

    let update_all_references_variants =
        variants.iter().map(|VariantData { ident, ty }| match ty {
            Some(ty) => {
                let update = update_all_references_for_type(quote! { ty }, ty);
                quote! { Self::#ident(ty) => { #update } }
            }
            None => quote! { Self::#ident => {} },
        });

    quote! {
        impl crate::Resolvable for #ident {
            fn resolved(&self, resolver: &mut dyn crate::TypeResolver) -> Option<Self> {
                match self {
                    #( #resolved_variants ),*
                }
            }

            fn update_all_references(
                &mut self,
                updater: impl Copy + Fn(&mut crate::TypeReference),
            ) {
                match self {
                    #( #update_all_references_variants ),*
                }
            }
        }
    }
}

pub(crate) fn generate_resolvable_struct(ident: Ident, fields: Vec<FieldData>) -> TokenStream {
    let resolved_fields = fields.iter().map(|FieldData { ident, ty }| {
        let resolved_ty = resolved_type(IdentOrZero::Ident(ident), ty);
        quote! { let #ident = #resolved_ty; }
    });
    let resolved_fields_test = fields.iter().map(|FieldData { ident, ty }| {
        if is_option(ty) {
            quote! { #ident.as_ref().is_some_and(Option::is_some) }
        } else {
            quote! { #ident.is_some() }
        }
    });
    let resolved_result_fields = fields.iter().map(|FieldData { ident, ty }| {
        if is_option(ty) {
            quote! { #ident: match (#ident, self.#ident.as_ref()) {
                (Some(new), Some(existing)) => Some(new.unwrap_or_else(|| existing.clone())),
                (_, existing) => existing.cloned()
            } }
        } else {
            quote! { #ident: #ident.unwrap_or_else(|| self.#ident.clone()) }
        }
    });

    let update_all_references_fields = fields.iter().filter_map(|FieldData { ident, ty }| {
        update_all_references_for_type(quote! { self.#ident }, ty)
    });

    quote! {
        impl crate::Resolvable for #ident {
            fn resolved(&self, resolver: &mut dyn crate::TypeResolver) -> Option<Self> {
                #( #resolved_fields )*
                if #( #resolved_fields_test )||* {
                    Some(Self {
                        #( #resolved_result_fields ),*
                    })
                } else {
                    None
                }
            }

            fn update_all_references(
                &mut self,
                updater: impl Copy + Fn(&mut crate::TypeReference),
            ) {
                #( #update_all_references_fields )*
            }
        }
    }
}

fn generate_resolvable_unit_type(ident: Ident, ty: Type) -> TokenStream {
    let resolved_field = resolved_type(IdentOrZero::Zero, &ty);

    let update_all_references = update_all_references_for_type(quote! { self.0 }, &ty);

    quote! {
        impl crate::Resolvable for #ident {
            fn resolved(&self, resolver: &mut dyn crate::TypeResolver) -> Option<Self> {
                #resolved_field.map(Self)
            }

            fn update_all_references(
                &mut self,
                updater: impl Copy + Fn(&mut crate::TypeReference),
            ) {
                #update_all_references
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
            quote! { None }
        }
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => {
                            quote! { None }
                        }
                        Type::Path(_) => quote! {{
                            let mut some_resolved = false;
                            let result: Vec<_> = self.#ident
                                .iter()
                                .map(|elem| {
                                    let resolved_elem = elem.resolved(resolver);
                                    if resolved_elem.is_some() {
                                        some_resolved = true;
                                    }
                                    resolved_elem
                                })
                                .collect();
                            some_resolved.then(|| {
                                result
                                    .into_iter()
                                    .enumerate()
                                    .map(|(i, elem)| elem.unwrap_or_else(|| self.#ident[i].clone()))
                                    .collect()
                            })
                        }},
                        _ => abort!(slice, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { None }
                        } else {
                            quote! {
                                self.#ident.resolved(resolver).map(Box::new)
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
                            quote! { None }
                        } else {
                            quote! { self.#ident.as_ref().map(|field| field.resolved(resolver)) }
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

fn resolved_unit_type(ident: &Ident, ty: &Type) -> Option<TokenStream> {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Text" => None,
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => None,
                        _ => abort!(args, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            None
                        } else {
                            Some(quote! { ty.resolved(resolver).map(Box::new).map(Self::#ident) })
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
                            None
                        } else {
                            Some(quote! { match ty {
                                Some(ty) => ty.resolved(resolver).map(Self::#ident).map(Some)
                                None => None
                            } })
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        _ => Some(quote! { ty.resolved(resolver).map(Self::#ident) }),
    }
}

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(path) => {
            matches!(path.path.segments.last(), Some(segment) if segment.ident == "Option")
        }
        _ => false,
    }
}

fn update_all_references_for_type(instance: TokenStream, ty: &Type) -> Option<TokenStream> {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Text" => None,
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => None,
                        Type::Path(_) => Some(quote! {
                            for elem in &mut #instance {
                                elem.update_all_references(updater);
                            }
                        }),
                        _ => abort!(slice, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            None
                        } else {
                            Some(quote! {
                                #instance.update_all_references(updater);
                            })
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
                            None
                        } else {
                            Some(quote! {
                                if let Some(ty) = &mut #instance {
                                    ty.update_all_references(updater);
                                }
                            })
                        }
                    }
                    _ => abort!(args, "Unsupported arguments"),
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        _ => Some(quote! {
            #instance.update_all_references(updater);
        }),
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
