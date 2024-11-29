use crate::ast::{AstSrc, Field};
use crate::LanguageSrc;
use anyhow::Result;
use quote::{format_ident, quote};

pub fn generate_nodes_mut<K>(ast: &AstSrc, kind_source: &K) -> Result<String>
where
    K: LanguageSrc,
{
    let node_boilerplate_impls: Vec<_> = ast
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);

            let methods: Vec<_> = node
                .fields
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let method_name = format_ident!("with_{}", field.method_name(kind_source));
                    let type_name = field.ty();

                    let element = match field {
                        Field::Token { .. } => {
                            quote! { element }
                        }
                        Field::Node { .. } => {
                            quote! { element.into_syntax() }
                        }
                    };

                    let element = quote! { #element.into() };

                    let (arg_type, element) = if field.is_optional() {
                        (
                            quote! { Option<#type_name> },
                            quote! { element.map(|element| #element) },
                        )
                    } else {
                        (quote! { #type_name }, quote! { Some(#element) })
                    };

                    // Dynamic nodes also track a `slot_map` that has to be calculated
                    // every time the SyntaxNode gets cast back into an AstNode, so we
                    // want to avoid that cast as much as possible. We also need to
                    // update the `slot_map` accordingly based on what we're given.
                    if node.dynamic {
                        quote! {
                            pub fn #method_name(self, element: #arg_type, slot_index: u8) -> Self {
                                // TODO: Implement range checking for the slot index to ensure other
                                // tokens can't accidentally be overridden.
                                let mut updated_slot_map = self.slot_map;
                                updated_slot_map[#index] = slot_index;
                                Self {
                                    syntax: self.syntax.splice_slots((slot_index as usize)..=(slot_index as usize), once(#element)),
                                    slot_map: updated_slot_map,
                                }
                            }
                        }
                    } else {
                        quote! {
                            pub fn #method_name(self, element: #arg_type) -> Self {
                                Self::unwrap_cast(self.syntax.splice_slots(#index..=#index, once(#element)))
                            }
                        }
                    }
                })
                .collect();

            quote! {
                impl #name {
                    #(#methods)*
                }
            }
        })
        .collect();

    let syntax_token = kind_source.syntax_token();

    let ast = quote! {
        use std::iter::once;
        use biome_rowan::AstNode;
        use crate::{generated::nodes::*, #syntax_token as SyntaxToken};

        #(#node_boilerplate_impls)*
    };

    let ast = ast
        .to_string()
        .replace("T ! [ ", "T![")
        .replace(" ] )", "])");

    Ok(ast)
}
