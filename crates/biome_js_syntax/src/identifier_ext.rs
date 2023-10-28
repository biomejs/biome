use crate::{
    AnyJsName, JsIdentifierAssignment, JsLiteralExportName, JsReferenceIdentifier, JsSyntaxToken,
    JsxReferenceIdentifier,
};
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

impl JsLiteralExportName {
    pub fn is_default(&self) -> SyntaxResult<bool> {
        Ok(self.value()?.text_trimmed() == "default")
    }
}

impl AnyJsName {
    /// Retrieves the value_token for a given `AnyJsName`.
    /// JsName or JsPrivateName
    /// ```
    /// use biome_js_syntax::{AnyJsName, JsName, JsPrivateName};
    /// use biome_js_factory::make;
    ///
    /// let js_name = AnyJsName::JsName(make::js_name(make::ident("request")));
    /// assert!(js_name.value_token().is_ok());
    /// assert_eq!(js_name.value_token().expect("value token text").text(), "request");
    /// ```
    pub fn value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsName::JsName(name) => name.value_token(),
            AnyJsName::JsPrivateName(name) => name.value_token(),
        }
    }
}
