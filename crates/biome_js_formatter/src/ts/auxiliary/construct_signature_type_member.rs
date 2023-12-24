use crate::prelude::*;
use biome_formatter::{format_args, write};

use crate::utils::FormatTypeMemberSeparator;

use biome_js_syntax::{TsConstructSignatureTypeMember, TsConstructSignatureTypeMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsConstructSignatureTypeMember;

impl FormatNodeRule<TsConstructSignatureTypeMember> for FormatTsConstructSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsConstructSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsConstructSignatureTypeMemberFields {
            new_token,
            type_parameters,
            parameters,
            type_annotation,
            separator_token,
        } = node.as_fields();

        write![
            f,
            [
                new_token.format(),
                space(),
                group(&format_args![
                    type_parameters.format(),
                    parameters.format(),
                    type_annotation.format()
                ]),
                FormatTypeMemberSeparator::new(separator_token.as_ref()),
            ]
        ]
    }
}
