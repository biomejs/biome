use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsObjectAssignmentPatternRest;
use biome_js_syntax::JsObjectAssignmentPatternRestFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectAssignmentPatternRest;

impl FormatNodeRule<JsObjectAssignmentPatternRest> for FormatJsObjectAssignmentPatternRest {
    fn fmt_fields(
        &self,
        node: &JsObjectAssignmentPatternRest,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectAssignmentPatternRestFields {
            dotdotdot_token,
            target,
        } = node.as_fields();

        write!(f, [dotdotdot_token.format(), target.format()])
    }
}
