use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteInterpolatedString, SvelteInterpolatedStringFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteInterpolatedString;
impl FormatNodeRule<SvelteInterpolatedString> for FormatSvelteInterpolatedString {
    fn fmt_fields(
        &self,
        node: &SvelteInterpolatedString,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteInterpolatedStringFields { parts } = node.as_fields();
        write!(f, [parts.format()])
    }
}
