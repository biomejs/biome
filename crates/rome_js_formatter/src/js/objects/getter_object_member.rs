use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsGetterObjectMember;
use biome_js_syntax::JsGetterObjectMemberFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsGetterObjectMember;

impl FormatNodeRule<JsGetterObjectMember> for FormatJsGetterObjectMember {
    fn fmt_fields(&self, node: &JsGetterObjectMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsGetterObjectMemberFields {
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            body,
        } = node.as_fields();

        write![
            f,
            [
                get_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                r_paren_token.format(),
                return_type.format(),
                space(),
                body.format()
            ]
        ]
    }
}
