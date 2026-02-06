use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Derive macro for ProvenanceTrackable
///
/// Generates an implementation that:
/// 1. Traverses the source node's fields
/// 2. For each field, pushes the field name, deserializes recursively, and captures provenance
/// 3. Returns None if any required field fails to deserialize
///
/// # Example
///
/// ```ignore
/// #[derive(ProvenanceTrackable)]
/// struct Config {
///     name: String,
///     count: u32,
/// }
/// ```
#[proc_macro_derive(ProvenanceTrackable)]
pub fn derive_provenance_trackable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let expanded = match input.data {
        Data::Struct(data_struct) => generate_struct_impl(&input.ident, &data_struct.fields),
        Data::Enum(_) => {
            panic!("ProvenanceTrackable derive macro does not support enums yet")
        }
        Data::Union(_) => {
            panic!("ProvenanceTrackable derive macro does not support unions")
        }
    };

    TokenStream::from(expanded)
}

fn generate_struct_impl(name: &syn::Ident, fields: &Fields) -> proc_macro2::TokenStream {
    match fields {
        Fields::Named(fields_named) => generate_named_struct_impl(name, fields_named),
        Fields::Unnamed(_) => {
            panic!("ProvenanceTrackable derive macro does not support tuple structs")
        }
        Fields::Unit => {
            panic!("ProvenanceTrackable derive macro does not support unit structs")
        }
    }
}

fn generate_named_struct_impl(
    name: &syn::Ident,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let field_handlers: Vec<_> = fields
        .named
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_name_str = field_name.to_string();
            let field_ty = &field.ty;

            // Generate code for each field:
            // 1. Push field name
            // 2. Try to deserialize
            // 3. If successful, assign to result
            // 4. If failed, set error flag
            // 5. Pop field name
            quote! {
                #field_name_str => {
                    ctx.push_field(#field_name_str);
                    match <#field_ty as biome_provenance::ProvenanceTrackable>::from_source_with_provenance(
                        value_node,
                        ctx,
                    ) {
                        Some(parsed_value) => {
                            result.#field_name = parsed_value;
                        }
                        None => {
                            // Required field failed to parse
                            has_error.set(true);
                        }
                    }
                    ctx.pop();
                }
            }
        })
        .collect();

    quote! {
        impl biome_provenance::ProvenanceTrackable for #name {
            fn from_source_with_provenance(
                source: &impl biome_provenance::ProvenanceSourceNode,
                ctx: &mut biome_provenance::ProvenanceContext,
            ) -> Option<Self> {
                use std::cell::Cell;

                // Must be an object
                if !source.is_object() {
                    return None;
                }

                // Start with default values
                let mut result = Self::default();
                let has_error = Cell::new(false);

                // Traverse all fields in the source
                source.traverse_fields(&mut |key, value_node| {
                    match key {
                        #(#field_handlers)*
                        // Unknown fields are ignored
                        _ => {}
                    }
                });

                if has_error.get() {
                    return None;
                }

                Some(result)
            }
        }
    }
}
