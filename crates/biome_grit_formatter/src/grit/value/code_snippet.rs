use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritCodeSnippet, GritCodeSnippetFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritCodeSnippet;
impl FormatNodeRule<GritCodeSnippet> for FormatGritCodeSnippet {
    fn fmt_fields(&self, node: &GritCodeSnippet, f: &mut GritFormatter) -> FormatResult<()> {
        let GritCodeSnippetFields { source } = node.as_fields();
        write!(f, [source.format()])
    }
}
