use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsAssignmentWithDefault;
use biome_js_syntax::JsAssignmentWithDefaultFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAssignmentWithDefault;

impl FormatNodeRule<JsAssignmentWithDefault> for FormatJsAssignmentWithDefault {
    fn fmt_fields(&self, node: &JsAssignmentWithDefault, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAssignmentWithDefaultFields {
            pattern,
            eq_token,
            default,
        } = node.as_fields();

        write!(
            f,
            [
                pattern.format(),
                space(),
                eq_token.format(),
                space(),
                default.format(),
            ]
        )
    }
}
