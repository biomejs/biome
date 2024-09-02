//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::MarkdownSyntaxNode::kind(&node) {
                $crate::MarkdownSyntaxKind::ANY_VALUE => {
                    let $pattern = unsafe { $crate::AnyValue::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::NUMBER_VALUE => {
                    let $pattern = unsafe { $crate::NumberValue::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::ROOT => {
                    let $pattern = unsafe { $crate::Root::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::BOGUS => {
                    let $pattern = unsafe { $crate::Bogus::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::NUMBER_VALUE_LIST => {
                    let $pattern = unsafe { $crate::NumberValueList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
