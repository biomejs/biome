//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsPropertySignatureAnnotation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsPropertySignatureAnnotation;
impl FormatRule<AnyTsPropertySignatureAnnotation> for FormatAnyTsPropertySignatureAnnotation {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &AnyTsPropertySignatureAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyTsPropertySignatureAnnotation::TsOptionalPropertyAnnotation(node) => {
                node.format().fmt(f)
            }
            AnyTsPropertySignatureAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
        }
    }
}
