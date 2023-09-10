//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsName;
impl FormatRule<AnyTsName> for FormatAnyTsName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsName::JsReferenceIdentifier(node) => node.format().fmt(f),
            AnyTsName::TsQualifiedName(node) => node.format().fmt(f),
        }
    }
}
