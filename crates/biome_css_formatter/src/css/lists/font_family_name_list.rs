use crate::prelude::*;
use biome_css_syntax::CssFontFamilyNameList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFamilyNameList;
impl FormatRule<CssFontFamilyNameList> for FormatCssFontFamilyNameList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssFontFamilyNameList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
