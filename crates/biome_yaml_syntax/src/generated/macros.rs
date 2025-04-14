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
                $crate::YamlSyntaxKind::YAML_ALIAS_NODE => {
                    let $pattern = unsafe { $crate::YamlAliasNode::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_ANCHOR_PROPERTY => {
                    let $pattern = unsafe { $crate::YamlAnchorProperty::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_COLLECTION => {
                    let $pattern = unsafe { $crate::YamlBlockCollection::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_ENTRY => {
                    let $pattern =
                        unsafe { $crate::YamlBlockMapExplicitEntry::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_KEY => {
                    let $pattern = unsafe { $crate::YamlBlockMapExplicitKey::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_VALUE => {
                    let $pattern =
                        unsafe { $crate::YamlBlockMapExplicitValue::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY => {
                    let $pattern =
                        unsafe { $crate::YamlBlockMapImplicitEntry::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_VALUE => {
                    let $pattern =
                        unsafe { $crate::YamlBlockMapImplicitValue::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_MAPPING => {
                    let $pattern = unsafe { $crate::YamlBlockMapping::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_SEQUENCE => {
                    let $pattern = unsafe { $crate::YamlBlockSequence::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_SEQUENCE_ENTRY => {
                    let $pattern = unsafe { $crate::YamlBlockSequenceEntry::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_COMPACT_MAPPING => {
                    let $pattern = unsafe { $crate::YamlCompactMapping::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_COMPACT_SEQUENCE => {
                    let $pattern = unsafe { $crate::YamlCompactSequence::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_DIRECTIVE => {
                    let $pattern = unsafe { $crate::YamlDirective::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_DOCUMENT => {
                    let $pattern = unsafe { $crate::YamlDocument::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_DOUBLE_QUOTED_SCALAR => {
                    let $pattern = unsafe { $crate::YamlDoubleQuotedScalar::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FLOW_JSON_NODE => {
                    let $pattern = unsafe { $crate::YamlFlowJsonNode::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FLOW_MAP_EXPLICIT_ENTRY => {
                    let $pattern = unsafe { $crate::YamlFlowMapExplicitEntry::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FLOW_MAP_IMPLICIT_ENTRY => {
                    let $pattern = unsafe { $crate::YamlFlowMapImplicitEntry::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FLOW_MAPPING => {
                    let $pattern = unsafe { $crate::YamlFlowMapping::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FLOW_SEQUENCE => {
                    let $pattern = unsafe { $crate::YamlFlowSequence::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FLOW_YAML_NODE => {
                    let $pattern = unsafe { $crate::YamlFlowYamlNode::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FOLDED_SCALAR => {
                    let $pattern = unsafe { $crate::YamlFoldedScalar::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_LITERAL_SCALAR => {
                    let $pattern = unsafe { $crate::YamlLiteralScalar::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_PLAIN_SCALAR => {
                    let $pattern = unsafe { $crate::YamlPlainScalar::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_PROPERTY_LIST => {
                    let $pattern = unsafe { $crate::YamlPropertyList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_SINGLE_QUOTED_SCALAR => {
                    let $pattern = unsafe { $crate::YamlSingleQuotedScalar::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_STREAM => {
                    let $pattern = unsafe { $crate::YamlStream::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_TAG_PROPERTY => {
                    let $pattern = unsafe { $crate::YamlTagProperty::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BOGUS => {
                    let $pattern = unsafe { $crate::YamlBogus::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BOGUS_NODE => {
                    let $pattern = unsafe { $crate::YamlBogusNode::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_MAP_ENTRY_LIST => {
                    let $pattern = unsafe { $crate::YamlBlockMapEntryList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_BLOCK_SEQUENCE_ENTRY_LIST => {
                    let $pattern =
                        unsafe { $crate::YamlBlockSequenceEntryList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_DIRECTIVE_LIST => {
                    let $pattern = unsafe { $crate::YamlDirectiveList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_DOCUMENT_LIST => {
                    let $pattern = unsafe { $crate::YamlDocumentList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FLOW_MAP_ENTRY_LIST => {
                    let $pattern = unsafe { $crate::YamlFlowMapEntryList::new_unchecked(node) };
                    $body
                }
                $crate::YamlSyntaxKind::YAML_FLOW_SEQUENCE_ENTRY_LIST => {
                    let $pattern =
                        unsafe { $crate::YamlFlowSequenceEntryList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
