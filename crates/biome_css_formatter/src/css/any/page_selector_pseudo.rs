//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPageSelectorPseudo;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPageSelectorPseudo;
impl FormatRule<AnyCssPageSelectorPseudo> for FormatAnyCssPageSelectorPseudo {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPageSelectorPseudo, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPageSelectorPseudo::CssBogusPageSelectorPseudo(node) => node.format().fmt(f),
            AnyCssPageSelectorPseudo::CssPageSelectorPseudo(node) => node.format().fmt(f),
        }
    }
}
