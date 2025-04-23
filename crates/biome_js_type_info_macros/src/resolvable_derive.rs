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
        ty: Type,
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
                    ty: data.fields.into_iter().next().unwrap().ty,
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
                            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Some(fields.unnamed.into_iter().next().unwrap().ty),
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
        DeriveInput::Unit { ident, ty } => generate_resolvable_unit_type(ident, ty),
    }
}

pub(crate) fn generate_resolvable_enum(ident: Ident, variants: Vec<VariantData>) -> TokenStream {
    let variants_need_resolving = variants.iter().map(|VariantData { ident, ty }| match ty {
        Some(ty) => {
            let needs_resolving = unit_type_needs_resolving(ty);
            quote! { Self::#ident(ty) => #needs_resolving }
        }
        None => quote! { Self::#ident => false },
    });

    let resolved_variants = variants.iter().map(|VariantData { ident, ty }| match ty {
        Some(ty) => {
            let resolved_ty = resolved_unit_type(ty);
            quote! { Self::#ident(ty) => Self::#ident(#resolved_ty) }
        }
        None => quote! { Self::#ident => Self::#ident },
    });

    quote! {
        impl crate::Resolvable for #ident {
            fn needs_resolving(&self, resolver: &dyn crate::TypeResolver, stack: &[&crate::TypeInner]) -> bool {
                match self {
                    #( #variants_need_resolving ),*
                }
            }

            fn resolved(&self, resolver: &dyn crate::TypeResolver, stack: &[&crate::TypeInner]) -> Self {
                match self {
                    #( #resolved_variants ),*
                }
            }
        }
    }
}

pub(crate) fn generate_resolvable_struct(ident: Ident, fields: Vec<FieldData>) -> TokenStream {
    let needs_resolving = fields
        .iter()
        .filter_map(|FieldData { ident, ty }| type_needs_resolving(IdentOrZero::Ident(ident), ty));

    let resolved_fields = fields.iter().map(|FieldData { ident, ty }| {
        let resolved_ty = resolved_type(IdentOrZero::Ident(ident), ty);
        quote! { #ident: #resolved_ty }
    });

    quote! {
        impl crate::Resolvable for #ident {
            fn needs_resolving(&self, resolver: &dyn crate::TypeResolver, stack: &[&crate::TypeInner]) -> bool {
                #( #needs_resolving )||*
            }

            fn resolved(&self, resolver: &dyn crate::TypeResolver, stack: &[&crate::TypeInner]) -> Self {
                Self {
                    #( #resolved_fields ),*
                }
            }
        }
    }
}

fn generate_resolvable_unit_type(ident: Ident, ty: Type) -> TokenStream {
    let needs_resolving =
        type_needs_resolving(IdentOrZero::Zero, &ty).unwrap_or_else(|| quote! { false });
    let resolved_field = resolved_type(IdentOrZero::Zero, &ty);

    quote! {
        impl crate::Resolvable for #ident {
            fn needs_resolving(&self, resolver: &dyn crate::TypeResolver, stack: &[&crate::TypeInner]) -> bool {
                #needs_resolving
            }

            fn resolved(&self, resolver: &dyn crate::TypeResolver, stack: &[&crate::TypeInner]) -> Self {
                Self(#resolved_field)
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
        Some(segment) if segment.ident == "Arc" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Arc is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                quote! {
                    self.#ident.clone()
                }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
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
                            self.#ident.iter().map(|elem| elem.resolved(resolver, stack)).collect()
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
                                Box::new(self.#ident.resolved(resolver, stack))
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
                            quote! { self.#ident.as_ref().map(|f| f.resolved(resolver, stack)) }
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
            quote! { self.#ident.resolved(resolver, stack) }
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

fn type_needs_resolving(ident: IdentOrZero, ty: &Type) -> Option<TokenStream> {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Arc" || segment.ident == "Text" => None,
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => None,
                        Type::Path(_) => Some(quote! {
                            self.#ident.iter().any(|elem| elem.needs_resolving(resolver, stack))
                        }),
                        _ => None,
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            None
                        } else {
                            Some(quote! {
                                self.#ident.needs_resolving(resolver, stack)
                            })
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        Some(segment) if segment.ident == "Option" => match &segment.arguments {
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            None
                        } else {
                            Some(quote! {
                                self.#ident
                                    .as_ref()
                                    .map_or(false, |f| f.needs_resolving(resolver, stack))
                            })
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        _ => Some(quote! { self.#ident.needs_resolving(resolver, stack) }),
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
        Some(segment) if segment.ident == "Arc" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Arc is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                quote! { ty.clone() }
            }
            PathArguments::AngleBracketed(_) | PathArguments::Parenthesized(_) => {
                abort!(path, "Unsupported type arguments in path")
            }
        },
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::None => abort!(segment, "Box is missing argument"),
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => quote! { ty.clone() },
                        Type::Path(_) => quote! {
                            ty.iter().any(|elem| elem.needs_resolving(resolver, stack))
                        },
                        _ => abort!(args, "Unsupported arguments"),
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            quote! { ty.clone() }
                        } else {
                            quote! { Box::new(ty.resolved(resolver, stack)) }
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
                            quote! { ty.as_ref().map(|f| f.resolved(resolver, stack)) }
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
            quote! { ty.resolved(resolver, stack) }
        }
    }
}

fn unit_type_needs_resolving(ty: &Type) -> Option<TokenStream> {
    let Type::Path(path) = ty else {
        abort!(ty, "Resolvable derive requires plain path types");
    };

    match path.path.segments.last() {
        Some(segment) if segment.ident == "Arc" || segment.ident == "Text" => {
            Some(quote! { false })
        }
        Some(segment) if segment.ident == "Box" => match &segment.arguments {
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Slice(slice)) => match slice.elem.as_ref() {
                        Type::Path(ty) if ty.path.is_ident("Text") => Some(quote! { false }),
                        Type::Path(_) => Some(quote! {
                            ty.iter().any(|elem| elem.needs_resolving(resolver, stack))
                        }),
                        _ => None,
                    },
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            Some(quote! { false })
                        } else {
                            Some(quote! { ty.needs_resolving(resolver, stack) })
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        Some(segment) if segment.ident == "Option" => match &segment.arguments {
            PathArguments::AngleBracketed(args) if args.args.len() == 1 => {
                match args.args.iter().next().unwrap() {
                    GenericArgument::Type(Type::Path(ty)) => {
                        if ty.path.is_ident("Text") {
                            Some(quote! { false })
                        } else {
                            Some(quote! {
                                ty
                                    .as_ref()
                                    .map_or(false, |f| f.needs_resolving(resolver, stack))
                            })
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        _ => Some(quote! { ty.needs_resolving(resolver, stack) }),
    }
}
