use crate::prelude::*;

use biome_js_syntax::JsRestParameter;
use biome_js_syntax::JsRestParameterFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsRestParameter;

impl FormatNodeRule<JsRestParameter> for FormatJsRestParameter {
    fn fmt_fields(&self, node: &JsRestParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let JsRestParameterFields {
            decorators,
            dotdotdot_token,
            binding,
            type_annotation,
        } = node.as_fields();

        write![
            f,
            [
                decorators.format(),
                dotdotdot_token.format(),
                binding.format(),
                type_annotation.format(),
            ]
        ]
    }
}
