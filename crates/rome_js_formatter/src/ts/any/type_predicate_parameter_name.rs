//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsTypePredicateParameterName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsTypePredicateParameterName;
impl FormatRule<AnyTsTypePredicateParameterName> for FormatAnyTsTypePredicateParameterName {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsTypePredicateParameterName, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsTypePredicateParameterName::JsReferenceIdentifier(node) => node.format().fmt(f),
            AnyTsTypePredicateParameterName::TsThisType(node) => node.format().fmt(f),
        }
    }
}
