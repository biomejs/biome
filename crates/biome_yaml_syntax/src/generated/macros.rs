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
                $crate::YamlSyntaxKind::YAML_ARRAY => {
                    let $pattern = unsafe { $crate::YamlArray::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_ARRAY_INLINE => {
                    let $pattern = unsafe { $crate::YamlArrayInline::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_ARRAY_ITEM => {
                    let $pattern = unsafe { $crate::YamlArrayItem::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_FOLDED => {
                    let $pattern = unsafe { $crate::YamlBlockFolded::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_LITERAL => {
                    let $pattern = unsafe { $crate::YamlBlockLiteral::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_VALUE => {
                    let $pattern = unsafe { $crate::YamlBlockValue::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BOOLEAN_VALUE => {
                    let $pattern = unsafe { $crate::YamlBooleanValue::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_DOCUMENT => {
                    let $pattern = unsafe { $crate::YamlDocument::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_IDENTIFIER => {
                    let $pattern = unsafe { $crate::YamlIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_NULL_VALUE => {
                    let $pattern = unsafe { $crate::YamlNullValue::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_NUMBER_VALUE => {
                    let $pattern = unsafe { $crate::YamlNumberValue::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_OBJECT => {
                    let $pattern = unsafe { $crate::YamlObject::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_OBJECT_MEMBER => {
                    let $pattern = unsafe { $crate::YamlObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_ROOT => {
                    let $pattern = unsafe { $crate::YamlRoot::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_STRING_VALUE => {
                    let $pattern = unsafe { $crate::YamlStringValue::new_unchecked(node) };
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
                $crate::YamlSyntaxKind::YAML_ARRAY_INLINE_LIST => {
                    let $pattern = unsafe { $crate::YamlArrayInlineList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_ARRAY_ITEM_LIST => {
                    let $pattern = unsafe { $crate::YamlArrayItemList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_DOCUMENT_LIST => {
                    let $pattern = unsafe { $crate::YamlDocumentList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_OBJECT_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::YamlObjectMemberList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
