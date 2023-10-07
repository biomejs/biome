use biome_rowan::AstNode;

use crate::{AnyJsExportNamedSpecifier, JsExportNamedClause, JsSyntaxToken};

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

    // Returns the export clause that includes this specifier.
    pub fn export_named_clause(&self) -> Option<JsExportNamedClause> {
        JsExportNamedClause::cast(self.syntax().grand_parent()?)
    }

    /// Returns `true` if this specifier or its export clause has a type modifier.
    pub fn is_type_only(&self) -> bool {
        self.type_token().is_some()
            || self
                .export_named_clause()
                .and_then(|x| x.type_token())
                .is_some()
    }
}
