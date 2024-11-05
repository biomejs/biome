use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritRegexLiteral, GritRegexLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRegexLiteral;
impl FormatNodeRule<GritRegexLiteral> for FormatGritRegexLiteral {
    fn fmt_fields(&self, node: &GritRegexLiteral, f: &mut GritFormatter) -> FormatResult<()> {
        let GritRegexLiteralFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
