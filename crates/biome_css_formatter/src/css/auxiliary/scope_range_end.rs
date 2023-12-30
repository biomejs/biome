use crate::prelude::*;
use biome_css_syntax::{CssScopeRangeEnd, CssScopeRangeEndFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssScopeRangeEnd;
impl FormatNodeRule<CssScopeRangeEnd> for FormatCssScopeRangeEnd {
    fn fmt_fields(&self, node: &CssScopeRangeEnd, f: &mut CssFormatter) -> FormatResult<()> {
        let CssScopeRangeEndFields { to_token, end } = node.as_fields();

        write!(f, [to_token.format(), space(), end.format()])
    }
}
