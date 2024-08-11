use crate::{
    inner_string_text, AnyJsExportNamedSpecifier, AnyJsName, JsIdentifierAssignment,
    JsLiteralExportName, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxToken,
    JsxReferenceIdentifier,
};
use biome_rowan::{
    declare_node_union, AstNode, SyntaxError, SyntaxNodeOptionExt, SyntaxResult, TokenText,
};

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

    /// returns `true` if the identifier is only used as a type.
    pub fn is_only_type(&self) -> bool {
        match self {
            AnyJsIdentifierUsage::JsReferenceIdentifier(_) => {
                self.parent::<AnyJsExportNamedSpecifier>()
                    .is_some_and(|specifier| specifier.exports_only_types())
                    || matches!(
                        self.syntax()
                            .ancestors()
                            .skip(1)
                            .find(|x| x.kind() != JsSyntaxKind::TS_QUALIFIED_NAME)
                            .kind(),
                        Some(JsSyntaxKind::TS_REFERENCE_TYPE | JsSyntaxKind::TS_TYPEOF_TYPE)
                    )
            }
            AnyJsIdentifierUsage::JsxReferenceIdentifier(_)
            | AnyJsIdentifierUsage::JsIdentifierAssignment(_) => false,
        }
    }
}

pub enum IdentifierUsageKind {
    TypeValue,
    Type,
    Value,
}

impl JsLiteralExportName {
    /// get the exported name, stripping the quotes if it is a string.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let export_name = make::js_literal_export_name(make::js_string_literal("foo")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(export_name.inner_string_text().unwrap().text(), "foo");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value()?))
    }

    /// Returns `true` if the export name is `default`.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let export_name = make::js_literal_export_name(make::js_string_literal("default"));
    /// assert!(export_name.is_default());
    ///
    /// let export_name = make::js_literal_export_name(make::js_string_literal("foo"));
    /// assert!(!export_name.is_default());
    /// ```
    pub fn is_default(&self) -> bool {
        self.inner_string_text()
            .map(|x| x.text() == "default")
            .unwrap_or(false)
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
            AnyJsName::JsMetavariable(_) => Err(SyntaxError::UnexpectedMetavariable),
        }
    }
}
