use biome_css_syntax::{CssRegularSyntaxTypeName, CssRegularSyntaxTypeNameFields};
use biome_formatter::write;

use crate::prelude::*;

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
