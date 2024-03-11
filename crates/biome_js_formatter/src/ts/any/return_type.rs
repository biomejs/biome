//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsReturnType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsReturnType;
impl FormatRule<AnyTsReturnType> for FormatAnyTsReturnType {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsReturnType, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsReturnType::AnyTsType(node) => node.format().fmt(f),
            AnyTsReturnType::TsAssertsReturnType(node) => node.format().fmt(f),
            AnyTsReturnType::TsPredicateReturnType(node) => node.format().fmt(f),
        }
    }
}
