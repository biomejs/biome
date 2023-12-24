use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;
use biome_formatter::{format_args, write};

use biome_js_syntax::{TsCallSignatureTypeMember, TsCallSignatureTypeMemberFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsCallSignatureTypeMember;

impl FormatNodeRule<TsCallSignatureTypeMember> for FormatTsCallSignatureTypeMember {
    fn fmt_fields(
        &self,
        node: &TsCallSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsCallSignatureTypeMemberFields {
            type_parameters,
            parameters,
            return_type_annotation,
            separator_token,
        } = node.as_fields();

        write!(
            f,
            [
                group(&format_args![
                    type_parameters.format(),
                    parameters.format(),
                    return_type_annotation.format(),
                ]),
                FormatTypeMemberSeparator::new(separator_token.as_ref())
            ]
        )
    }
}
