use crate::prelude::*;
use biome_css_syntax::{ScssKeyframesSelector, ScssKeyframesSelectorFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssKeyframesSelector;
impl FormatNodeRule<ScssKeyframesSelector> for FormatScssKeyframesSelector {
    fn fmt_fields(&self, node: &ScssKeyframesSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssKeyframesSelectorFields {
            selector,
            percent_token,
        } = node.as_fields();

        write!(f, [selector.format(), percent_token.format()])
    }
}
