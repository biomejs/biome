use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritJavascriptBodyWrapper, GritJavascriptBodyWrapperFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritJavascriptBodyWrapper;
impl FormatNodeRule<GritJavascriptBodyWrapper> for FormatGritJavascriptBodyWrapper {
    fn fmt_fields(
        &self,
        node: &GritJavascriptBodyWrapper,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritJavascriptBodyWrapperFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
