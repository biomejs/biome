use crate::prelude::*;
use biome_css_syntax::{CssScopeRangeInterval, CssScopeRangeIntervalFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeRangeInterval;
impl FormatNodeRule<CssScopeRangeInterval> for FormatCssScopeRangeInterval {
    fn fmt_fields(&self, node: &CssScopeRangeInterval, f: &mut CssFormatter) -> FormatResult<()> {
        let CssScopeRangeIntervalFields {
            start,
            to_token,
            end,
        } = node.as_fields();

        write!(
            f,
            [
                start.format(),
                space(),
                to_token.format(),
                space(),
                end.format()
            ]
        )
    }
}
