use crate::prelude::*;
use biome_css_syntax::{ScssParentSelectorSuffixHyphen, ScssParentSelectorSuffixHyphenFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssParentSelectorSuffixHyphen;
impl FormatNodeRule<ScssParentSelectorSuffixHyphen> for FormatScssParentSelectorSuffixHyphen {
    fn fmt_fields(
        &self,
        node: &ScssParentSelectorSuffixHyphen,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssParentSelectorSuffixHyphenFields { minus_token } = node.as_fields();

        write!(f, [minus_token.format()])
    }
}
