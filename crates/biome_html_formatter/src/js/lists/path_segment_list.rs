use crate::prelude::*;
use biome_html_syntax::GlimmerPathSegmentList;
use biome_rowan::AstSeparatedList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGlimmerPathSegmentList;
impl FormatRule<GlimmerPathSegmentList> for FormatGlimmerPathSegmentList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &GlimmerPathSegmentList, f: &mut HtmlFormatter) -> FormatResult<()> {
        // For separated lists, format each element and its separator
        for element in node.elements() {
            crate::prelude::write!(f, [element.node.format()])?;
            if let Ok(Some(sep)) = element.trailing_separator() {
                crate::prelude::write!(f, [sep.format()])?;
            }
        }
        Ok(())
    }
}
