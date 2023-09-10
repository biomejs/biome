use super::kinds_src::AstSrc;
use crate::generate_nodes::token_kind_to_code;
use crate::kinds_src::TokenKind;
use crate::{kinds_src::Field, to_upper_snake_case, LanguageKind};
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
    };
    let normal_node_arms = ast.nodes.iter().map(|node| {
        let kind = format_ident!("{}", to_upper_snake_case(&node.name));
        let expected_len = node.fields.len();

        let fields = node.fields.iter().map(|field| {
            let field_predicate = match field {
                Field::Node { ty, .. } => {
                    let ast_type_name = format_ident!("{}", ty);

                    quote! {
                        #ast_type_name::can_cast(element.kind())
                    }
                }
                Field::Token { kind, .. } => match kind {
                    TokenKind::Single(expected) => {
                        let expected_kind = token_kind_to_code(expected, language_kind);
                        quote! { element.kind() == #expected_kind}
                    }
                    TokenKind::Many(expected) => {
                        let expected_kinds = expected
                            .iter()
                            .map(|kind| token_kind_to_code(kind, language_kind));
                        quote! {
                            matches!(element.kind(), #(#expected_kinds)|*)
                        }
                    }
                },
            };

            quote! {
                if let Some(element) = &current_element {
                    if #field_predicate {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
            }
        });

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
