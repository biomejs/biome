use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatOptionalSemicolon;

use biome_js_syntax::{TsGetterSignatureClassMember, TsGetterSignatureClassMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsGetterSignatureClassMember;

impl FormatNodeRule<TsGetterSignatureClassMember> for FormatTsGetterSignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsGetterSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsGetterSignatureClassMemberFields {
            modifiers,
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            return_type,
            semicolon_token,
        } = node.as_fields();

        write!(
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
                FormatOptionalSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
