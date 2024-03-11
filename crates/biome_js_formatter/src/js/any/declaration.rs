//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsDeclaration;
impl FormatRule<AnyJsDeclaration> for FormatAnyJsDeclaration {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsDeclaration::JsClassDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::JsFunctionDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::JsVariableDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::TsDeclareFunctionDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::TsEnumDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::TsExternalModuleDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::TsGlobalDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::TsImportEqualsDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::TsInterfaceDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::TsModuleDeclaration(node) => node.format().fmt(f),
            AnyJsDeclaration::TsTypeAliasDeclaration(node) => node.format().fmt(f),
        }
    }
}
