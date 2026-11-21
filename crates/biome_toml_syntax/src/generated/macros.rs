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
                $crate::TomlSyntaxKind::TOML_ARRAY_TABLE_HEADER => {
                    let $pattern = unsafe { $crate::TomlArrayTableHeader::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BARE_KEY => {
                    let $pattern = unsafe { $crate::TomlBareKey::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BASIC_STRING => {
                    let $pattern = unsafe { $crate::TomlBasicString::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOOLEAN => {
                    let $pattern = unsafe { $crate::TomlBoolean::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_DATE => {
                    let $pattern = unsafe { $crate::TomlDate::new_unchecked(node) };
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
                $crate::TomlSyntaxKind::TOML_LITERAL_STRING => {
                    let $pattern = unsafe { $crate::TomlLiteralString::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_MEMBER => {
                    let $pattern = unsafe { $crate::TomlMember::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_MULTILINE_BASIC_STRING => {
                    let $pattern = unsafe { $crate::TomlMultilineBasicString::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_MULTILINE_LITERAL_STRING => {
                    let $pattern =
                        unsafe { $crate::TomlMultilineLiteralString::new_unchecked(node) };
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
                $crate::TomlSyntaxKind::TOML_TABLE => {
                    let $pattern = unsafe { $crate::TomlTable::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_TABLE_HEADER => {
                    let $pattern = unsafe { $crate::TomlTableHeader::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_TIME => {
                    let $pattern = unsafe { $crate::TomlTime::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOGUS => {
                    let $pattern = unsafe { $crate::TomlBogus::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOGUS_KEY => {
                    let $pattern = unsafe { $crate::TomlBogusKey::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOGUS_KEY_FRAGMENT => {
                    let $pattern = unsafe { $crate::TomlBogusKeyFragment::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOGUS_MEMBER => {
                    let $pattern = unsafe { $crate::TomlBogusMember::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_BOGUS_TABLE_HEADER => {
                    let $pattern = unsafe { $crate::TomlBogusTableHeader::new_unchecked(node) };
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
                $crate::TomlSyntaxKind::TOML_INLINE_TABLE_MEMBER_LIST => {
                    let $pattern =
                        unsafe { $crate::TomlInlineTableMemberList::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_KEY_FRAGMENT_LIST => {
                    let $pattern = unsafe { $crate::TomlKeyFragmentList::new_unchecked(node) };
                    $body
                }
                $crate::TomlSyntaxKind::TOML_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::TomlMemberList::new_unchecked(node) };
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
