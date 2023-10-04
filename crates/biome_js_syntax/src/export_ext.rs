use crate::{AnyJsExportNamedSpecifier, JsSyntaxToken};

impl AnyJsExportNamedSpecifier {
    pub fn type_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsExportNamedShorthandSpecifier(specifier) => specifier.type_token(),
            Self::JsExportNamedSpecifier(specifier) => specifier.type_token(),
        }
    }
}
