use crate::prelude::*;
use biome_css_syntax::{CssUniversalSyntax, CssUniversalSyntaxFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUniversalSyntax;

impl FormatNodeRule<CssUniversalSyntax> for FormatCssUniversalSyntax {
    fn fmt_fields(&self, node: &CssUniversalSyntax, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUniversalSyntaxFields { star_token } = node.as_fields();
        write!(f, [star_token.format()])
    }
}
