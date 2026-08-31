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
                $crate::TomlSyntaxKind::TOML_BOOLEAN_VALUE => {
                    let $pattern = unsafe { $crate::TomlBooleanValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_FLOAT_VALUE => {
                    let $pattern = unsafe { $crate::TomlFloatValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_INLINE_TABLE => {
                    let $pattern = unsafe { $crate::TomlInlineTable::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_INTEGER_VALUE => {
                    let $pattern = unsafe { $crate::TomlIntegerValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY => {
                    let $pattern = unsafe { $crate::TomlKey::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY_SEGMENT => {
                    let $pattern = unsafe { $crate::TomlKeySegment::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY_VALUE => {
                    let $pattern = unsafe { $crate::TomlKeyValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_LOCAL_DATE_TIME_VALUE => {
                    let $pattern = unsafe { $crate::TomlLocalDateTimeValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_LOCAL_DATE_VALUE => {
                    let $pattern = unsafe { $crate::TomlLocalDateValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_LOCAL_TIME_VALUE => {
                    let $pattern = unsafe { $crate::TomlLocalTimeValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_OFFSET_DATE_TIME_VALUE => {
                    let $pattern = unsafe { $crate::TomlOffsetDateTimeValue::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_ROOT => {
                    let $pattern = unsafe { $crate::TomlRoot::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_STRING_VALUE => {
                    let $pattern = unsafe { $crate::TomlStringValue::new_unchecked(node) };
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
                $crate::TomlSyntaxKind::TOML_INLINE_TABLE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::TomlInlineTableElementList::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_ITEM_LIST => {
                    let $pattern = unsafe { $crate::TomlItemList::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY_SEGMENT_LIST => {
                    let $pattern = unsafe { $crate::TomlKeySegmentList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
