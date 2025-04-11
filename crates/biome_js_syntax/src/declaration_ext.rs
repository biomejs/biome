use biome_rowan::AstNode;

use crate::{
    AnyJsDeclaration, AnyJsDeclarationClause, JsClassDeclaration, JsSyntaxKind, JsSyntaxNode,
    TsEnumDeclaration,
};

impl AnyJsDeclarationClause {
    pub fn into_declaration(self) -> Option<AnyJsDeclaration> {
        match self {
            AnyJsDeclarationClause::JsClassDeclaration(decl) => {
                Some(AnyJsDeclaration::JsClassDeclaration(decl))
            }
            AnyJsDeclarationClause::JsFunctionDeclaration(decl) => {
                Some(AnyJsDeclaration::JsFunctionDeclaration(decl))
            }
            AnyJsDeclarationClause::JsVariableDeclarationClause(decl) => Some(
                AnyJsDeclaration::JsVariableDeclaration(decl.declaration().ok()?),
            ),
            AnyJsDeclarationClause::TsDeclareFunctionDeclaration(decl) => {
                Some(AnyJsDeclaration::TsDeclareFunctionDeclaration(decl))
            }
            AnyJsDeclarationClause::TsEnumDeclaration(decl) => {
                Some(AnyJsDeclaration::TsEnumDeclaration(decl))
            }
            AnyJsDeclarationClause::TsExternalModuleDeclaration(decl) => {
                Some(AnyJsDeclaration::TsExternalModuleDeclaration(decl))
            }
            AnyJsDeclarationClause::TsGlobalDeclaration(decl) => {
                Some(AnyJsDeclaration::TsGlobalDeclaration(decl))
            }
            AnyJsDeclarationClause::TsImportEqualsDeclaration(decl) => {
                Some(AnyJsDeclaration::TsImportEqualsDeclaration(decl))
            }
            AnyJsDeclarationClause::TsInterfaceDeclaration(decl) => {
                Some(AnyJsDeclaration::TsInterfaceDeclaration(decl))
            }
            AnyJsDeclarationClause::TsModuleDeclaration(decl) => {
                Some(AnyJsDeclaration::TsModuleDeclaration(decl))
            }
            AnyJsDeclarationClause::TsTypeAliasDeclaration(decl) => {
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
