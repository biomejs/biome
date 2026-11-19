use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::TsIndexedAccessType;
use biome_js_syntax::TsIndexedAccessTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsIndexedAccessType;

impl FormatNodeRule<TsIndexedAccessType> for FormatTsIndexedAccessType {
    fn fmt_fields(&self, node: &TsIndexedAccessType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIndexedAccessTypeFields {
            object_type,
            l_brack_token,
            index_type,
            r_brack_token,
        } = node.as_fields();

        let delimiter_spacing = f.options().delimiter_spacing().value();

        write!(f, [object_type.format()])?;

        if delimiter_spacing {
            // Use if_group_fits_on_line for spaces - spaces when it fits, no spaces when it breaks
            write!(
                f,
                [group(&format_args![
                    l_brack_token.format(),
                    if_group_fits_on_line(&space()),
                    soft_block_indent(&index_type.format()),
                    if_group_fits_on_line(&space()),
                    r_brack_token.format()
                ])]
            )
        } else {
            write![
                f,
                [
                    l_brack_token.format(),
                    index_type.format(),
                    r_brack_token.format()
                ]
            ]
        }
    }
}
