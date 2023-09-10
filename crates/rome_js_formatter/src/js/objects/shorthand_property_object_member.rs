use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsShorthandPropertyObjectMember;
use biome_js_syntax::JsShorthandPropertyObjectMemberFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsShorthandPropertyObjectMember;

impl FormatNodeRule<JsShorthandPropertyObjectMember> for FormatJsShorthandPropertyObjectMember {
    fn fmt_fields(
        &self,
        node: &JsShorthandPropertyObjectMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsShorthandPropertyObjectMemberFields { name } = node.as_fields();

        write![f, [name.format()]]
    }
}
