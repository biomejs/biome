use crate::prelude::*;
use crate::utils::FormatOptionalSemicolon;

use biome_formatter::write;
use biome_js_syntax::{TsSetterSignatureClassMember, TsSetterSignatureClassMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsSetterSignatureClassMember;

impl FormatNodeRule<TsSetterSignatureClassMember> for FormatTsSetterSignatureClassMember {
    fn fmt_fields(
        &self,
        node: &TsSetterSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsSetterSignatureClassMemberFields {
            modifiers,
            set_token,
            name,
            l_paren_token,
            parameter,
            comma_token,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                modifiers.format(),
                space(),
                set_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                parameter.format(),
                comma_token.format(),
                r_paren_token.format(),
                FormatOptionalSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
