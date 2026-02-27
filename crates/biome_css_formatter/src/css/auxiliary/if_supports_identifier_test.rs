use crate::prelude::*;
use biome_css_syntax::{CssIfSupportsIdentifierTest, CssIfSupportsIdentifierTestFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssIfSupportsIdentifierTest;

impl FormatNodeRule<CssIfSupportsIdentifierTest> for FormatCssIfSupportsIdentifierTest {
    fn fmt_fields(
        &self,
        node: &CssIfSupportsIdentifierTest,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssIfSupportsIdentifierTestFields {
            ident,
            colon_token,
            value,
        } = node.as_fields();

        write!(f, [ident.format(), colon_token.format(), value.format()])
    }
}
