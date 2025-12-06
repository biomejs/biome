//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssRoot;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssRoot;
impl FormatRule<AnyCssRoot> for FormatAnyCssRoot {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssRoot, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssRoot::CssRoot(node) => node.format().fmt(f),
            AnyCssRoot::CssSnippetRoot(node) => node.format().fmt(f),
        }
    }
}
