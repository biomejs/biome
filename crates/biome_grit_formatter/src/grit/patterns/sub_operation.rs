use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritSubOperation, GritSubOperationFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritSubOperation;
impl FormatNodeRule<GritSubOperation> for FormatGritSubOperation {
    fn fmt_fields(&self, node: &GritSubOperation, f: &mut GritFormatter) -> FormatResult<()> {
        let GritSubOperationFields {
            left,
            right,
            minus_token,
        } = node.as_fields();

        write!(
            f,
            [
                left.format(),
                space(),
                minus_token.format(),
                space(),
                right.format()
            ]
        )
    }
}
