use crate::prelude::*;
use biome_css_syntax::{CssRegularSyntaxTypeName, CssRegularSyntaxTypeNameFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularSyntaxTypeName;

impl FormatNodeRule<CssRegularSyntaxTypeName> for FormatCssRegularSyntaxTypeName {
    fn fmt_fields(
        &self,
        node: &CssRegularSyntaxTypeName,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssRegularSyntaxTypeNameFields { name_token } = node.as_fields();
        write!(f, [name_token.format()])
    }
}
