use crate::{AnyJsExportNamedSpecifier, JsSyntaxToken};

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
}
