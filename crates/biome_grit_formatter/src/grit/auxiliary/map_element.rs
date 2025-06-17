use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritMapElement, GritMapElementFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMapElement;
impl FormatNodeRule<GritMapElement> for FormatGritMapElement {
    fn fmt_fields(&self, node: &GritMapElement, f: &mut GritFormatter) -> FormatResult<()> {
        let GritMapElementFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                key.format(),
                colon_token.format(),
                soft_space_or_block_indent(&value.format()),
            ]
        )
    }
}
