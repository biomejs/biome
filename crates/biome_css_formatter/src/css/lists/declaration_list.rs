use crate::{prelude::*, separated::FormatAstSeparatedListExtension};
use biome_css_syntax::CssDeclarationList;
use biome_formatter::separated::TrailingSeparator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationList;
impl FormatRule<CssDeclarationList> for FormatCssDeclarationList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssDeclarationList, f: &mut CssFormatter) -> FormatResult<()> {
        f.join_with(&hard_line_break())
            .entries(
                node.format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Mandatory),
            )
            .finish()
    }
}
