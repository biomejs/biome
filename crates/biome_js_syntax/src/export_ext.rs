use biome_rowan::{AstNode, SyntaxResult};

use crate::{AnyJsExportNamedSpecifier, JsExportNamedClause, JsReferenceIdentifier, JsSyntaxToken};

impl AnyJsExportNamedSpecifier {
    /// Type token of the export specifier.
    ///
    /// ```ts
    /// export { type X }
    ///          ^^^^
    /// ```
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsExportNamedShorthandSpecifier(specifier) => specifier.type_token(),
            Self::JsExportNamedSpecifier(specifier) => specifier.type_token(),
        }
    }

    /// Returns the export clause that includes this specifier.
    pub fn export_named_clause(&self) -> Option<JsExportNamedClause> {
        JsExportNamedClause::cast(self.syntax().grand_parent()?)
    }

    /// Returns `true` if this specifier or its export clause has **only** a type modifier.
    pub fn exports_only_types(&self) -> bool {
        self.type_token().is_some()
            || self
                .export_named_clause()
                .and_then(|x| x.type_token())
                .is_some()
    }

    /// Returns the local name of the export.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsExportNamedSpecifier, T};
    /// use biome_js_factory::make;
    ///
    /// let specifier = make::js_export_named_shorthand_specifier(
    ///     make::js_reference_identifier(make::ident("a"))
    /// ).with_type_token(make::token(T![type])).build();
    /// let export = AnyJsExportNamedSpecifier::from(specifier.clone());
    ///
    /// assert_eq!(export.local_name(), specifier.name());
    ///
    /// let specifier = make::js_export_named_specifier(
    ///     make::js_reference_identifier(make::ident("a")),
    ///     make::token(T![as]),
    ///     make::js_literal_export_name(make::ident("b")),
    /// ).build();
    /// let export = AnyJsExportNamedSpecifier::from(specifier.clone());
    ///
    /// assert_eq!(export.local_name(), specifier.local_name());
    /// ```
    pub fn local_name(&self) -> SyntaxResult<JsReferenceIdentifier> {
        match self {
            Self::JsExportNamedShorthandSpecifier(specifier) => specifier.name(),
            Self::JsExportNamedSpecifier(specifier) => specifier.local_name(),
        }
    }

    pub fn with_type_token(self, type_token: Option<JsSyntaxToken>) -> Self {
        match self {
            Self::JsExportNamedShorthandSpecifier(specifier) => {
                specifier.with_type_token(type_token).into()
            }
            Self::JsExportNamedSpecifier(specifier) => specifier.with_type_token(type_token).into(),
        }
    }
}
