//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssModuleMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssModuleMember;
impl FormatRule<AnyScssModuleMember> for FormatAnyScssModuleMember {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssModuleMember, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssModuleMember::CssIdentifier(node) => node.format().fmt(f),
            AnyScssModuleMember::ScssIdentifier(node) => node.format().fmt(f),
        }
    }
}
