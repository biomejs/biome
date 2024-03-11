//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsPropertyAnnotation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsPropertyAnnotation;
impl FormatRule<AnyTsPropertyAnnotation> for FormatAnyTsPropertyAnnotation {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsPropertyAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsPropertyAnnotation::TsDefinitePropertyAnnotation(node) => node.format().fmt(f),
            AnyTsPropertyAnnotation::TsOptionalPropertyAnnotation(node) => node.format().fmt(f),
            AnyTsPropertyAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
        }
    }
}
