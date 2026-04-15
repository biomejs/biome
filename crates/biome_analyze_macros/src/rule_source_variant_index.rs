use proc_macro::TokenStream;
use proc_macro2::{Literal, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn rule_source_variant_index_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match generate_variant_index_impl(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn generate_variant_index_impl(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let Data::Enum(data_enum) = &input.data else {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "RuleSourceVariantIndex can only be derived for enums",
        ));
    };

    if data_enum.variants.len() > usize::from(u16::MAX) + 1 {
        return Err(syn::Error::new_spanned(
            &input.ident,
            "RuleSourceVariantIndex supports at most u16::MAX + 1 variants",
        ));
    }

    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let arms = data_enum
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let variant_ident = &variant.ident;
            let index = Literal::u16_unsuffixed(index as u16);
            let pattern = match &variant.fields {
                Fields::Unit => quote!(Self::#variant_ident),
                Fields::Unnamed(_) => quote!(Self::#variant_ident(..)),
                Fields::Named(_) => quote!(Self::#variant_ident { .. }),
            };

            quote!(#pattern => #index,)
        });

    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub const fn variant_index(&self) -> u16 {
                match self {
                    #( #arms )*
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::generate_variant_index_impl;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn generates_indices_for_all_variant_shapes() {
        let input = parse_quote! {
            enum Example<'a> {
                Unit,
                Tuple(&'a str),
                Struct { name: &'a str },
            }
        };

        let tokens = generate_variant_index_impl(&input).unwrap();

        assert_eq!(
            tokens.to_string(),
            quote! {
                impl<'a> Example<'a> {
                    pub const fn variant_index(&self) -> u16 {
                        match self {
                            Self::Unit => 0,
                            Self::Tuple(..) => 1,
                            Self::Struct { .. } => 2,
                        }
                    }
                }
            }
            .to_string()
        );
    }

    #[test]
    fn rejects_non_enum_input() {
        let input = parse_quote! {
            struct Example;
        };

        let error = generate_variant_index_impl(&input).unwrap_err();
        assert_eq!(
            error.to_string(),
            "RuleSourceVariantIndex can only be derived for enums"
        );
    }
}
