use crate::prelude::*;
use biome_css_syntax::{CssEmptyDeclaration, CssEmptyDeclarationFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssEmptyDeclaration;
impl FormatNodeRule<CssEmptyDeclaration> for FormatCssEmptyDeclaration {
    fn fmt_fields(&self, node: &CssEmptyDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let CssEmptyDeclarationFields { semicolon_token } = node.as_fields();
        write!(f, [format_removed(&semicolon_token?)])
    }
}
