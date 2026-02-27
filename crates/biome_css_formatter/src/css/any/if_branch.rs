//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfBranch;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfBranch;
impl FormatRule<AnyCssIfBranch> for FormatAnyCssIfBranch {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfBranch, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfBranch::CssBogusIfBranch(node) => node.format().fmt(f),
            AnyCssIfBranch::CssIfBranch(node) => node.format().fmt(f),
        }
    }
}
