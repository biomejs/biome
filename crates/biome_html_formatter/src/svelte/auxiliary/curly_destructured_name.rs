use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteCurlyDestructuredName, SvelteCurlyDestructuredNameFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteCurlyDestructuredName;
impl FormatNodeRule<SvelteCurlyDestructuredName> for FormatSvelteCurlyDestructuredName {
    fn fmt_fields(
        &self,
        node: &SvelteCurlyDestructuredName,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteCurlyDestructuredNameFields {
            r_curly_token,
            names,
            l_curly_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_curly_token.format(),
                space(),
                names.format(),
                space(),
                r_curly_token.format()
            ]
        )
    }
}
