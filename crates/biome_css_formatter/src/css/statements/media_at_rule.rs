use crate::prelude::*;
use biome_css_syntax::{CssMediaAtRule, CssMediaAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMediaAtRule;
impl FormatNodeRule<CssMediaAtRule> for FormatCssMediaAtRule {
    fn fmt_fields(&self, node: &CssMediaAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssMediaAtRuleFields {
            media_token,
            queries,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                media_token.format(),
                space(),
                // A regular indent here keeps the start of the query on the
                // same line, even if it ends up breaking over multiple lines
                // afterward, then lets the block start on the same line as
                // well. Example:
                //   @media all and (-webkit-min-device-pixel-ratio: 1.5),
                // 	     all and (-o-min-device-pixel-ratio: 3 / 2),
                // 	     all and (min--moz-device-pixel-ratio: 1.5),
                // 	     all and (min-device-pixel-ratio: 1.5) {
                //   }
                // Most other instances use a `soft_block_indent`, but that
                // would put the query on its own set of lines, which doesn't
                // flow as neatly:
                //   @media
                //       all and (-webkit-min-device-pixel-ratio: 1.5),
                // 	     all and (-o-min-device-pixel-ratio: 3 / 2),
                // 	     all and (min--moz-device-pixel-ratio: 1.5),
                // 	     all and (min-device-pixel-ratio: 1.5)
                //   {
                //   }
                group(&indent(&queries.format())),
                space(),
                block.format()
            ]
        )
    }
}
