use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritDivOperation, GritDivOperationFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDivOperation;
impl FormatNodeRule<GritDivOperation> for FormatGritDivOperation {
    fn fmt_fields(&self, node: &GritDivOperation, f: &mut GritFormatter) -> FormatResult<()> {
        let GritDivOperationFields {
            left,
            right,
            slash_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                slash_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
