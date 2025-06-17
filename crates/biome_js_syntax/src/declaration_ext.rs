use biome_rowan::AstNode;

use crate::{
    AnyJsDeclaration, AnyJsDeclarationClause, JsClassDeclaration, JsSyntaxKind, JsSyntaxNode,
    TsEnumDeclaration,
};

impl AnyJsDeclarationClause {
    pub fn into_declaration(self) -> Option<AnyJsDeclaration> {
        match self {
            Self::JsClassDeclaration(decl) => Some(AnyJsDeclaration::JsClassDeclaration(decl)),
            Self::JsFunctionDeclaration(decl) => {
                Some(AnyJsDeclaration::JsFunctionDeclaration(decl))
            }
            Self::JsVariableDeclarationClause(decl) => Some(
                AnyJsDeclaration::JsVariableDeclaration(decl.declaration().ok()?),
            ),
            Self::TsDeclareFunctionDeclaration(decl) => {
                Some(AnyJsDeclaration::TsDeclareFunctionDeclaration(decl))
            }
            Self::TsEnumDeclaration(decl) => Some(AnyJsDeclaration::TsEnumDeclaration(decl)),
            Self::TsExternalModuleDeclaration(decl) => {
                Some(AnyJsDeclaration::TsExternalModuleDeclaration(decl))
            }
            Self::TsGlobalDeclaration(decl) => Some(AnyJsDeclaration::TsGlobalDeclaration(decl)),
            Self::TsImportEqualsDeclaration(decl) => {
                Some(AnyJsDeclaration::TsImportEqualsDeclaration(decl))
            }
            Self::TsInterfaceDeclaration(decl) => {
                Some(AnyJsDeclaration::TsInterfaceDeclaration(decl))
            }
            Self::TsModuleDeclaration(decl) => Some(AnyJsDeclaration::TsModuleDeclaration(decl)),
            Self::TsTypeAliasDeclaration(decl) => {
                Some(AnyJsDeclaration::TsTypeAliasDeclaration(decl))
            }
        }
    }
}

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
pub fn is_in_ambient_context(syntax: &JsSyntaxNode) -> bool {
    syntax.ancestors().any(|x| {
        matches!(
            x.kind(),
            JsSyntaxKind::TS_DECLARE_STATEMENT | JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE
        )
    })
}
