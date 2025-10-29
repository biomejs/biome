use crate::prelude::*;
use biome_html_syntax::GlimmerPathSegment;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerPathSegment;

impl FormatNodeRule<GlimmerPathSegment> for FormatGlimmerPathSegment {
    fn fmt_fields(&self, node: &GlimmerPathSegment, f: &mut HtmlFormatter) -> FormatResult<()> {
        write!(f, [format_verbatim(node.syntax())])
    }
}
