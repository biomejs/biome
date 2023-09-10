use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;

use biome_js_syntax::{TsIndexSignatureTypeMember, TsIndexSignatureTypeMemberFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsIndexSignatureTypeMember;

impl FormatNodeRule<TsIndexSignatureTypeMember> for FormatTsIndexSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsIndexSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsIndexSignatureTypeMemberFields {
            readonly_token,
            l_brack_token,
            parameter,
            r_brack_token,
            type_annotation,
            separator_token,
        } = node.as_fields();

        if let Some(readonly_token) = readonly_token {
            write!(f, [readonly_token.format(), space()])?;
        }

        write![
            f,
            [
                l_brack_token.format(),
                parameter.format(),
                r_brack_token.format(),
                type_annotation.format(),
                FormatTypeMemberSeparator::new(separator_token.as_ref()),
            ]
        ]
    }
}
