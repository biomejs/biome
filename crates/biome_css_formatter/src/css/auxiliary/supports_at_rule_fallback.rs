use crate::prelude::*;
use crate::verbatim::format_css_verbatim_fields;
use biome_css_syntax::CssSupportsAtRuleFallback;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsAtRuleFallback;
impl FormatNodeRule<CssSupportsAtRuleFallback> for FormatCssSupportsAtRuleFallback {
    fn fmt_fields(
        &self,
        node: &CssSupportsAtRuleFallback,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_css_verbatim_fields(node.syntax()).fmt(f)
    }
}
