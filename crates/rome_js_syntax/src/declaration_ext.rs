use rome_rowan::AstNode;

use crate::{JsClassDeclaration, JsSyntaxKind, JsSyntaxNode, TsEnumDeclaration};

impl TsEnumDeclaration {
    /// Returns `true` if this enum is an ambient enum or in an ambient context.
    pub fn is_ambient(&self) -> bool {
        is_in_ambient_context(self.syntax())
    }
}

impl JsClassDeclaration {
    /// Returns `true` if this class is an ambient class or in an ambient context.
    pub fn is_ambient(&self) -> bool {
        is_in_ambient_context(self.syntax())
    }
}

/// Returns `true` if `syntax` is in an ambient context.
fn is_in_ambient_context(syntax: &JsSyntaxNode) -> bool {
    syntax.ancestors().any(|x| {
        matches!(
            x.kind(),
            JsSyntaxKind::TS_DECLARE_STATEMENT | JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE
        )
    })
}
