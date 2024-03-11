//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsExportDefaultDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsExportDefaultDeclaration;
impl FormatRule<AnyJsExportDefaultDeclaration> for FormatAnyJsExportDefaultDeclaration {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsExportDefaultDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsExportDefaultDeclaration::JsClassExportDefaultDeclaration(node) => {
                node.format().fmt(f)
            }
            AnyJsExportDefaultDeclaration::JsFunctionExportDefaultDeclaration(node) => {
                node.format().fmt(f)
            }
            AnyJsExportDefaultDeclaration::TsDeclareFunctionExportDefaultDeclaration(node) => {
                node.format().fmt(f)
            }
            AnyJsExportDefaultDeclaration::TsInterfaceDeclaration(node) => node.format().fmt(f),
        }
    }
}
