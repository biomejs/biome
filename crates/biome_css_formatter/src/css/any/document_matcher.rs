//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssDocumentMatcher;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssDocumentMatcher;
impl FormatRule<AnyCssDocumentMatcher> for FormatAnyCssDocumentMatcher {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssDocumentMatcher, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssDocumentMatcher::CssBogusDocumentMatcher(node) => node.format().fmt(f),
            AnyCssDocumentMatcher::CssDocumentCustomMatcher(node) => node.format().fmt(f),
            AnyCssDocumentMatcher::CssUrlFunction(node) => node.format().fmt(f),
        }
    }
}
