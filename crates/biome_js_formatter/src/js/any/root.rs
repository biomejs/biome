//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsRoot;
impl FormatRule<AnyJsRoot> for FormatAnyJsRoot {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsRoot, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsRoot::JsExpressionSnipped(node) => node.format().fmt(f),
            AnyJsRoot::JsModule(node) => node.format().fmt(f),
            AnyJsRoot::JsScript(node) => node.format().fmt(f),
            AnyJsRoot::TsDeclarationModule(node) => node.format().fmt(f),
        }
    }
}
