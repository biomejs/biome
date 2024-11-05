use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritStringLiteral, GritStringLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritStringLiteral;
impl FormatNodeRule<GritStringLiteral> for FormatGritStringLiteral {
    fn fmt_fields(&self, node: &GritStringLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        let GritStringLiteralFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
