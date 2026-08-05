use crate::prelude::*;
use biome_css_syntax::{CssDeclarationSnippetRoot, CssDeclarationSnippetRootFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationSnippetRoot;

impl FormatNodeRule<CssDeclarationSnippetRoot> for FormatCssDeclarationSnippetRoot {
    fn fmt_fields(
        &self,
        node: &CssDeclarationSnippetRoot,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssDeclarationSnippetRootFields {
            declarations,
            eof_token,
        } = node.as_fields();

        write!(f, [declarations.format(), format_removed(&eof_token?)])
    }
}
