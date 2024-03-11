//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsDeclarationClause;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsDeclarationClause;
impl FormatRule<AnyJsDeclarationClause> for FormatAnyJsDeclarationClause {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsDeclarationClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsDeclarationClause::JsClassDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::JsFunctionDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::JsVariableDeclarationClause(node) => node.format().fmt(f),
            AnyJsDeclarationClause::TsDeclareFunctionDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::TsEnumDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::TsExternalModuleDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::TsGlobalDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::TsImportEqualsDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::TsInterfaceDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::TsModuleDeclaration(node) => node.format().fmt(f),
            AnyJsDeclarationClause::TsTypeAliasDeclaration(node) => node.format().fmt(f),
        }
    }
}
