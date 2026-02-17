//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssFunctionName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssFunctionName;
impl FormatRule<AnyCssFunctionName> for FormatAnyCssFunctionName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssFunctionName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssFunctionName::CssIdentifier(node) => node.format().fmt(f),
            AnyCssFunctionName::ScssQualifiedName(node) => node.format().fmt(f),
        }
    }
}
