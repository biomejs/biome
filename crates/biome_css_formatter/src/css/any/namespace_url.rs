//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssNamespaceUrl;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssNamespaceUrl;
impl FormatRule<AnyCssNamespaceUrl> for FormatAnyCssNamespaceUrl {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssNamespaceUrl, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssNamespaceUrl::CssString(node) => node.format().fmt(f),
            AnyCssNamespaceUrl::CssUrlFunction(node) => node.format().fmt(f),
        }
    }
}
