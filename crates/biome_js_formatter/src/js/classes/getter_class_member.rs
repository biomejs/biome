use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsGetterClassMember;
use biome_js_syntax::JsGetterClassMemberFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsGetterClassMember;

impl FormatNodeRule<JsGetterClassMember> for FormatJsGetterClassMember {
    fn fmt_fields(&self, node: &JsGetterClassMember, f: &mut JsFormatter) -> FormatResult<()> {
        let JsGetterClassMemberFields {
            modifiers,
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
                modifiers.format(),
                space(),
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
