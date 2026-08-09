use crate::prelude::*;
use crate::separated::FormatAstSeparatedListWithScopedOptionsExtension;
use biome_css_syntax::CssPseudoValueList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoValueList;
impl FormatRule<CssPseudoValueList> for FormatCssPseudoValueList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssPseudoValueList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated_with_scoped_options(",", CssCase::Preserve) {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
