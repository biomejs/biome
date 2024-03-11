//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssImportUrl;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssImportUrl;
impl FormatRule<AnyCssImportUrl> for FormatAnyCssImportUrl {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssImportUrl, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssImportUrl::CssString(node) => node.format().fmt(f),
            AnyCssImportUrl::CssUrlFunction(node) => node.format().fmt(f),
        }
    }
}
