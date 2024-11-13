use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritNegativeIntLiteral, GritNegativeIntLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritNegativeIntLiteral;
impl FormatNodeRule<GritNegativeIntLiteral> for FormatGritNegativeIntLiteral {
    fn fmt_fields(&self, node: &GritNegativeIntLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        let GritNegativeIntLiteralFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
