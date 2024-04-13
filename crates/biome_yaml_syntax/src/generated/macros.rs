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
            node => match $crate::YamlSyntaxNode::kind(&node) {
                $crate::YamlSyntaxKind::YAML_DOCUMENT => {
                    let $pattern = unsafe { $crate::YamlDocument::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_ROOT => {
                    let $pattern = unsafe { $crate::YamlRoot::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_SCALAR => {
                    let $pattern = unsafe { $crate::YamlScalar::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BOGUS => {
                    let $pattern = unsafe { $crate::YamlBogus::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BOGUS_VALUE => {
                    let $pattern = unsafe { $crate::YamlBogusValue::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_CONTENT_LIST => {
                    let $pattern = unsafe { $crate::YamlContentList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_DOCUMENT_LIST => {
                    let $pattern = unsafe { $crate::YamlDocumentList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
