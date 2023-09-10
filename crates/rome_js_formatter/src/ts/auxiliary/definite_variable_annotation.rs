use crate::prelude::*;

use biome_js_syntax::TsDefiniteVariableAnnotation;
use biome_js_syntax::TsDefiniteVariableAnnotationFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDefiniteVariableAnnotation;

impl FormatNodeRule<TsDefiniteVariableAnnotation> for FormatTsDefiniteVariableAnnotation {
    fn fmt_fields(
        &self,
        node: &TsDefiniteVariableAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsDefiniteVariableAnnotationFields {
            excl_token,
            type_annotation,
        } = node.as_fields();

        write![f, [excl_token.format(), type_annotation.format()]]
    }
}
