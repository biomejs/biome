//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsModuleSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsModuleSource;
impl FormatRule<AnyJsModuleSource> for FormatAnyJsModuleSource {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsModuleSource, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsModuleSource::JsMetavariable(node) => node.format().fmt(f),
            AnyJsModuleSource::JsModuleSource(node) => node.format().fmt(f),
        }
    }
}
