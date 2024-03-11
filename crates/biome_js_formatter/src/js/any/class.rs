//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsClass;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsClass;
impl FormatRule<AnyJsClass> for FormatAnyJsClass {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsClass, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsClass::JsClassDeclaration(node) => node.format().fmt(f),
            AnyJsClass::JsClassExportDefaultDeclaration(node) => node.format().fmt(f),
            AnyJsClass::JsClassExpression(node) => node.format().fmt(f),
        }
    }
}
