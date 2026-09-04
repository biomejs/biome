use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::{GfmTableDelimiterDash, GfmTableDelimiterDashFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTableDelimiterDash;
impl FormatNodeRule<GfmTableDelimiterDash> for FormatGfmTableDelimiterDash {
    fn fmt_fields(
        &self,
        node: &GfmTableDelimiterDash,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        let GfmTableDelimiterDashFields { minus_token } = node.as_fields();
        write!(f, [minus_token.format()])
    }
}
