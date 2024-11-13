use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritRawBacktickSnippetLiteral, GritRawBacktickSnippetLiteralFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritRawBacktickSnippetLiteral;
impl FormatNodeRule<GritRawBacktickSnippetLiteral> for FormatGritRawBacktickSnippetLiteral {
    fn fmt_fields(
        &self,
        node: &GritRawBacktickSnippetLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritRawBacktickSnippetLiteralFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
