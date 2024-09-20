use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::HtmlRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlRoot;
impl FormatNodeRule<HtmlRoot> for FormatHtmlRoot {
    fn fmt_fields(&self, node: &HtmlRoot, f: &mut HtmlFormatter) -> FormatResult<()> {
        if let Some(bom) = node.bom_token() {
            bom.format().fmt(f)?;
        }
        if let Some(directive) = node.directive() {
            directive.format().fmt(f)?;
        }

        node.html().format().fmt(f)?;

        if let Ok(eof) = node.eof_token() {
            eof.format().fmt(f)?;
        }
        write!(f, [hard_line_break()])?;

        Ok(())
    }
}
