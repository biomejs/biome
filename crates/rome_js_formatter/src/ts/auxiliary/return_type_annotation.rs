use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsReturnTypeAnnotation;
use biome_js_syntax::TsReturnTypeAnnotationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsReturnTypeAnnotation;

impl FormatNodeRule<TsReturnTypeAnnotation> for FormatTsReturnTypeAnnotation {
    fn fmt_fields(&self, node: &TsReturnTypeAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        let TsReturnTypeAnnotationFields { colon_token, ty } = node.as_fields();
        write![f, [colon_token.format(), space(), ty.format()]]
    }
}
