use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::SvelteLiteral;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteLiteral;
impl FormatNodeRule<SvelteLiteral> for FormatSvelteLiteral {
    fn fmt_fields(&self, node: &SvelteLiteral, f: &mut HtmlFormatter) -> FormatResult<()> {
        write!(f, [node.value_token().format()])
    }
}
