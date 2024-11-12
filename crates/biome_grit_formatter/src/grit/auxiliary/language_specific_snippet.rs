use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritLanguageSpecificSnippet, GritLanguageSpecificSnippetFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritLanguageSpecificSnippet;
impl FormatNodeRule<GritLanguageSpecificSnippet> for FormatGritLanguageSpecificSnippet {
    fn fmt_fields(
        &self,
        node: &GritLanguageSpecificSnippet,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritLanguageSpecificSnippetFields {
            language,
            snippet_token,
        } = node.as_fields();

        write!(f, [language.format(), snippet_token.format()])
    }
}
