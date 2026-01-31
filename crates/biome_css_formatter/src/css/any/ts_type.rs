//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssType;
impl FormatRule<AnyCssType> for FormatAnyCssType {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssType, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssType::AnyCssSyntaxComponent(node) => node.format().fmt(f),
            AnyCssType::CssBogusType(node) => node.format().fmt(f),
            AnyCssType::CssTypeFunction(node) => node.format().fmt(f),
        }
    }
}
