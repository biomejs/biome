use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsOptionalPropertyAnnotation;
use biome_js_syntax::TsOptionalPropertyAnnotationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsOptionalPropertyAnnotation;

impl FormatNodeRule<TsOptionalPropertyAnnotation> for FormatTsOptionalPropertyAnnotation {
    fn fmt_fields(
        &self,
        node: &TsOptionalPropertyAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsOptionalPropertyAnnotationFields {
            question_mark_token,
            type_annotation,
        } = node.as_fields();

        write![f, [question_mark_token.format(), type_annotation.format()]]
    }
}
