use crate::prelude::*;
use biome_css_syntax::{CssRoot, CssRootFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRoot;

impl FormatNodeRule<CssRoot> for FormatCssRoot {
    fn fmt_fields(&self, node: &CssRoot, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRootFields {
            bom_token,
            items,
            eof_token,
        } = node.as_fields();

        write!(f, [bom_token.format(), items.format()])?;

        if f.options().trailing_newline().value() {
            write!(f, [hard_line_break()])?;
        }

        write!(f, [format_removed(&eof_token?)])
    }
}
