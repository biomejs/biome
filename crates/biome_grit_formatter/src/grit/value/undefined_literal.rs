use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritUndefinedLiteral, GritUndefinedLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritUndefinedLiteral;
impl FormatNodeRule<GritUndefinedLiteral> for FormatGritUndefinedLiteral {
    fn fmt_fields(&self, node: &GritUndefinedLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        let GritUndefinedLiteralFields { token_token } = node.as_fields();
        write!(f, [token_token.format()])
    }
}
