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
            node => match $crate::MiniSyntaxNode::kind(&node) {
                $crate::MiniSyntaxKind::MINI_COMPLEX_NODE => {
                    let $pattern = unsafe { $crate::MiniComplexNode::new_unchecked(node) };
                    $body
                }
                $crate::MiniSyntaxKind::MINI_ROOT => {
                    let $pattern = unsafe { $crate::MiniRoot::new_unchecked(node) };
                    $body
                }
                $crate::MiniSyntaxKind::MINI_SIMPLE_NODE => {
                    let $pattern = unsafe { $crate::MiniSimpleNode::new_unchecked(node) };
                    $body
                }
                $crate::MiniSyntaxKind::MINI_STRING => {
                    let $pattern = unsafe { $crate::MiniString::new_unchecked(node) };
                    $body
                }
                $crate::MiniSyntaxKind::MINI_BOGUS => {
                    let $pattern = unsafe { $crate::MiniBogus::new_unchecked(node) };
                    $body
                }
                $crate::MiniSyntaxKind::MINI_NODE_LIST => {
                    let $pattern = unsafe { $crate::MiniNodeList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
