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
            node => match $crate::TomlSyntaxNode::kind(&node) {
                $crate::TomlSyntaxKind::TOML_ARRAY => {
                    let $pattern = unsafe { $crate::TomlArray::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_ARRAY_TABLE => {
                    let $pattern = unsafe { $crate::TomlArrayTable::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BARE_KEY => {
                    let $pattern = unsafe { $crate::TomlBareKey::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOOLEAN => {
                    let $pattern = unsafe { $crate::TomlBoolean::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_DATE_TIME => {
                    let $pattern = unsafe { $crate::TomlDateTime::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_FLOAT => {
                    let $pattern = unsafe { $crate::TomlFloat::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_INLINE_TABLE => {
                    let $pattern = unsafe { $crate::TomlInlineTable::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_INTEGER => {
                    let $pattern = unsafe { $crate::TomlInteger::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY => {
                    let $pattern = unsafe { $crate::TomlKey::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY_VALUE => {
                    let $pattern = unsafe { $crate::TomlKeyValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_QUOTED_KEY => {
                    let $pattern = unsafe { $crate::TomlQuotedKey::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_ROOT => {
                    let $pattern = unsafe { $crate::TomlRoot::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_STRING => {
                    let $pattern = unsafe { $crate::TomlString::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_TABLE => {
                    let $pattern = unsafe { $crate::TomlTable::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOGUS => {
                    let $pattern = unsafe { $crate::TomlBogus::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOGUS_VALUE => {
                    let $pattern = unsafe { $crate::TomlBogusValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_ARRAY_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::TomlArrayElementList::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_INLINE_TABLE_ENTRY_LIST => {
                    let $pattern = unsafe { $crate::TomlInlineTableEntryList::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY_PART_LIST => {
                    let $pattern = unsafe { $crate::TomlKeyPartList::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY_VALUE_LIST => {
                    let $pattern = unsafe { $crate::TomlKeyValueList::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_ROOT_ITEM_LIST => {
                    let $pattern = unsafe { $crate::TomlRootItemList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
