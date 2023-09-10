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
            node => match $crate::JsonSyntaxNode::kind(&node) {
                $crate::JsonSyntaxKind::JSON_ARRAY_VALUE => {
                    let $pattern = unsafe { $crate::JsonArrayValue::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_BOOLEAN_VALUE => {
                    let $pattern = unsafe { $crate::JsonBooleanValue::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_MEMBER => {
                    let $pattern = unsafe { $crate::JsonMember::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::JsonMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_NULL_VALUE => {
                    let $pattern = unsafe { $crate::JsonNullValue::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_NUMBER_VALUE => {
                    let $pattern = unsafe { $crate::JsonNumberValue::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_OBJECT_VALUE => {
                    let $pattern = unsafe { $crate::JsonObjectValue::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_ROOT => {
                    let $pattern = unsafe { $crate::JsonRoot::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_STRING_VALUE => {
                    let $pattern = unsafe { $crate::JsonStringValue::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_BOGUS => {
                    let $pattern = unsafe { $crate::JsonBogus::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_BOGUS_VALUE => {
                    let $pattern = unsafe { $crate::JsonBogusValue::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_ARRAY_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::JsonArrayElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsonSyntaxKind::JSON_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::JsonMemberList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
