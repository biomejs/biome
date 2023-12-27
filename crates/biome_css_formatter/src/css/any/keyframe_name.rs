//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssKeyframeName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssKeyframeName;
impl FormatRule<AnyCssKeyframeName> for FormatAnyCssKeyframeName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssKeyframeName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssKeyframeName::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssKeyframeName::CssString(node) => node.format().fmt(f),
        }
    }
}
