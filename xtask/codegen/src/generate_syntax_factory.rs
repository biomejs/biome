use super::js_kinds_src::AstSrc;
use crate::generate_nodes::{get_field_predicate, group_fields_for_ordering, token_kind_to_code};
use crate::{to_upper_snake_case, LanguageKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use xtask::Result;

pub fn generate_syntax_factory(ast: &AstSrc, language_kind: LanguageKind) -> Result<String> {
    let (syntax_crate, syntax_kind, factory_kind) = match language_kind {
        LanguageKind::Js => (
            quote! { biome_js_syntax },
            quote! { JsSyntaxKind },
            quote! { JsSyntaxFactory },
        ),
        LanguageKind::Css => (
            quote! { biome_css_syntax },
            quote! { CssSyntaxKind },
            quote! { CssSyntaxFactory },
        ),
        LanguageKind::Json => (
            quote! { biome_json_syntax },
            quote! { JsonSyntaxKind },
            quote! { JsonSyntaxFactory },
        ),
        LanguageKind::Grit => (
            quote! { biome_grit_syntax },
            quote! { GritSyntaxKind },
            quote! { GritSyntaxFactory },
        ),
        LanguageKind::Html => (
            quote! { biome_html_syntax },
            quote! { HtmlSyntaxKind },
            quote! { HtmlSyntaxFactory },
        ),
    };
    let normal_node_arms = ast.nodes.iter().map(|node| {
        let kind = format_ident!("{}", to_upper_snake_case(&node.name));
        let expected_len = node.fields.len();

        let fields = if node.dynamic {
            // Chunk the fields of the node into groups of unordered nodes that need
            // to be checked in parallel and ordered nodes that get checked one by one.
            let field_groups = group_fields_for_ordering(node);

            field_groups
                .iter()
                .map(|group| {
                    match group.len() {
                        0 => unreachable!("Somehow encountered a group of fields with no entries"),
                        // Single-field groups are assumed to act like ordered fields, so
                        // they can just check the kind and move on if there's no match.
                        1 => {
                            let field = group[0];
                            let field_predicate = get_field_predicate(field, language_kind);

                            quote! {
                                if let Some(element) = &current_element {
                                    if #field_predicate {
                                        slots.mark_present();
                                        current_element = elements.next();
                                    }
                                }
                                slots.next_slot();
                            }
                        }
                        _ => {
                            let variants = group.iter().enumerate().map(|(index, field)| {
                                let field_predicate = get_field_predicate(field, language_kind);

                                let maybe_else = if index > 0 {
                                    quote! { else }
                                } else {
                                    Default::default()
                                };

                                quote! {
                                    #maybe_else if !group_slot_map[#index] && #field_predicate {
                                        group_slot_map[#index] = true;
                                    }
                                }
                            });

                            let group_length = group.len();

                            quote! {
                                let mut unmatched_count = #group_length;
                                let mut group_slot_map = [false; #group_length];
                                for _ in 0usize..#group_length {
                                    let Some(element) = &current_element else {
                                        break;
                                    };

                                    #(#variants)* else {
                                        // If the element didn't match any of the variants, then no more
                                        // are allowed to match, so move on to the next group.
                                        break;
                                    }
                                    unmatched_count -= 1;
                                    slots.mark_present();
                                    slots.next_slot();
                                    current_element = elements.next();
                                }
                                // Advanced past all of the expected slots for the group so that
                                // they get marked as empty.
                                for _ in 0..unmatched_count {
                                    slots.next_slot();
                                }
                            }
                        }
                    }
                })
                .collect::<Vec<TokenStream>>()
        } else {
            node.fields
                .iter()
                .map(|field| {
                    let field_predicate = get_field_predicate(field, language_kind);
                    quote! {
                        if let Some(element) = &current_element {
                            if #field_predicate {
                                slots.mark_present();
                                current_element = elements.next();
                            }
                        }
                        slots.next_slot();
                    }
                })
                .collect::<Vec<TokenStream>>()
        };

        quote! {
            #kind => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<#expected_len> = RawNodeSlots::default();
                let mut current_element = elements.next();

                #(#fields)*

                // Additional unexpected elements
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        #kind.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }

                slots.into_node(#kind, children)
            }
        }
    });

    let lists = ast.lists().map(|(name, data)| {
        let element_type = format_ident!("{}", data.element_name);
        let kind = format_ident!("{}", to_upper_snake_case(name));
        if let Some(separator) = &data.separator {
            let allow_trailing = separator.allow_trailing;
            let separator_kind = token_kind_to_code(&separator.separator_token, language_kind);
            quote! {
                #kind => Self::make_separated_list_syntax(kind, children, #element_type::can_cast, #separator_kind, #allow_trailing)
            }
        } else {
            quote! {
                #kind => Self::make_node_list_syntax(kind, children, #element_type::can_cast)
            }
        }
    });

    let bogus_kinds = ast
        .bogus
        .iter()
        .map(|node| format_ident!("{}", to_upper_snake_case(node)));

    let output = quote! {
        use #syntax_crate::{*, #syntax_kind, #syntax_kind::*, T};
        use biome_rowan::{AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind};

        #[derive(Debug)]
        pub struct #factory_kind;

        impl SyntaxFactory for #factory_kind {
            type Kind = #syntax_kind;

            #[allow(unused_mut)]
            fn make_syntax(
                kind: Self::Kind,
                children: ParsedChildren<Self::Kind>,
            ) -> RawSyntaxNode<Self::Kind>
            {
                match kind {
                    #(#bogus_kinds)|* => {
                        RawSyntaxNode::new(kind, children.into_iter().map(Some))
                    },
                    #(#normal_node_arms),*,
                    #(#lists),*,
                    _ => unreachable!("Is {:?} a token?", kind),
                }
            }
        }
    };

    let pretty = xtask::reformat(output)?;
    Ok(pretty)
}
