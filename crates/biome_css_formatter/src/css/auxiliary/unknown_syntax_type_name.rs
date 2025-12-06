use crate::prelude::*;
use biome_css_syntax::{CssUnknownSyntaxTypeName, CssUnknownSyntaxTypeNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnknownSyntaxTypeName;

impl FormatNodeRule<CssUnknownSyntaxTypeName> for FormatCssUnknownSyntaxTypeName {
    fn fmt_fields(
        &self,
        node: &CssUnknownSyntaxTypeName,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssUnknownSyntaxTypeNameFields { name_token } = node.as_fields();
        write!(f, [name_token.format()])
    }
}
