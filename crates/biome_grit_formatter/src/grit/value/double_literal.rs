use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritDoubleLiteral, GritDoubleLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritDoubleLiteral;
impl FormatNodeRule<GritDoubleLiteral> for FormatGritDoubleLiteral {
    fn fmt_fields(&self, node: &GritDoubleLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        let GritDoubleLiteralFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
