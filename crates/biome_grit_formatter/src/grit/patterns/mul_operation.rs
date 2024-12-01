use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritMulOperation, GritMulOperationFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMulOperation;
impl FormatNodeRule<GritMulOperation> for FormatGritMulOperation {
    fn fmt_fields(&self, node: &GritMulOperation, f: &mut GritFormatter) -> FormatResult<()> {
        let GritMulOperationFields {
            left,
            right,
            star_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                star_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
