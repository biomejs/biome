use super::js_kinds_src::AstSrc;
use crate::language_kind::LanguageKind;
use crate::Result;
use biome_string_case::Case;
use quote::{format_ident, quote};

pub fn generate_macros(ast: &AstSrc, language_kind: LanguageKind) -> Result<String> {
    let syntax_kind = language_kind.syntax_kind();
    let syntax_node = language_kind.syntax_node();

    let match_arms: Vec<_> = ast
        .nodes
        .iter()
        .map(|node| {
            let name = format_ident!("{}", node.name);
            let node_kind = format_ident!("{}", Case::Constant.convert(&node.name));
            (name, node_kind)
        })
        .chain(ast.bogus.iter().map(|node_name| {
            let name = format_ident!("{}", node_name);
            let node_kind = format_ident!("{}", Case::Constant.convert(node_name));
            (name, node_kind)
        }))
        .chain(ast.lists().map(|(node_name, _)| {
            let name = format_ident!("{}", node_name);
            let node_kind = format_ident!("{}", Case::Constant.convert(node_name));
            (name, node_kind)
        }))
        .map(|(name, node_kind)| {
            quote! {
                $crate::#syntax_kind::#node_kind => {
                    // SAFETY: The call to new_unchecked is guarded by matching on node.kind()
                    let $pattern = unsafe { $crate::#name::new_unchecked(node) };
                    $body
                }
            }
        })
        .collect();

    let ast = quote! {
        /// Reconstruct an AstNode from a SyntaxNode
        ///
        /// This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)
        /// of the provided [biome_rowan::SyntaxNode] and constructs the appropriate
        /// AstNode type for it, then execute the provided expression over it.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// map_syntax_node!(syntax_node, node => node.format())
        /// ```
        #[macro_export]
        macro_rules! map_syntax_node {
            ($node:expr, $pattern:pat => $body:expr) => {
                match $node {
                    node => match $crate::#syntax_node::kind(&node) {
                        #( #match_arms, )*
                        _ => unreachable!()
                    }
                }
            };
        }

        pub(crate) use map_syntax_node;
    };

    xtask::reformat(ast)
}
