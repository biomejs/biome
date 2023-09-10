use crate::{JsIdentifierAssignment, JsReferenceIdentifier, JsSyntaxToken, JsxReferenceIdentifier};
use biome_rowan::{declare_node_union, SyntaxResult};

declare_node_union! {
    pub AnyJsIdentifierUsage = JsReferenceIdentifier | JsIdentifierAssignment | JsxReferenceIdentifier
}

impl AnyJsIdentifierUsage {
    pub fn value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsIdentifierUsage::JsReferenceIdentifier(node) => node.value_token(),
            AnyJsIdentifierUsage::JsIdentifierAssignment(node) => node.name_token(),
            AnyJsIdentifierUsage::JsxReferenceIdentifier(node) => node.value_token(),
        }
    }
}
