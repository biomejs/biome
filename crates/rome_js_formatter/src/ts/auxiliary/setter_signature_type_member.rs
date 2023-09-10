use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;

use biome_js_syntax::{TsSetterSignatureTypeMember, TsSetterSignatureTypeMemberFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsSetterSignatureTypeMember;

impl FormatNodeRule<TsSetterSignatureTypeMember> for FormatTsSetterSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsSetterSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsSetterSignatureTypeMemberFields {
            set_token,
            name,
            l_paren_token,
            parameter,
            r_paren_token,
            separator_token,
        } = node.as_fields();

        write![
            f,
            [
                set_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                parameter.format(),
                r_paren_token.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref())
            ]
        ]
    }
}
