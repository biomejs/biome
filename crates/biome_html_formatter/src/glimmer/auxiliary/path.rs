use crate::prelude::*;
use crate::verbatim::format_html_verbatim_node;
use biome_html_syntax::GlimmerPath;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerPath;

impl FormatNodeRule<GlimmerPath> for FormatGlimmerPath {
    fn fmt_fields(&self, node: &GlimmerPath, f: &mut HtmlFormatter) -> FormatResult<()> {
        write!(f, [format_html_verbatim_node(node.syntax())])
    }
}
