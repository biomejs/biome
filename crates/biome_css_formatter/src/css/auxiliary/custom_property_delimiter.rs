use crate::prelude::*;
use biome_css_syntax::CssCustomPropertyDelimiter;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyDelimiter;

impl FormatNodeRule<CssCustomPropertyDelimiter> for FormatCssCustomPropertyDelimiter {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyDelimiter,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        write!(f, [node.value().format()])
    }
}
