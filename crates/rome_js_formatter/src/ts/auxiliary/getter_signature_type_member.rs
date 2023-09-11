use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;

use biome_formatter::write;
use biome_js_syntax::{TsGetterSignatureTypeMember, TsGetterSignatureTypeMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsGetterSignatureTypeMember;

impl FormatNodeRule<TsGetterSignatureTypeMember> for FormatTsGetterSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsGetterSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsGetterSignatureTypeMemberFields {
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            type_annotation,
            separator_token,
        } = node.as_fields();

        write![
            f,
            [
                get_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                r_paren_token.format(),
                type_annotation.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref())
            ]
        ]
    }
}
