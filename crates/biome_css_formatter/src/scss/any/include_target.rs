//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssIncludeTarget;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssIncludeTarget;
impl FormatRule<AnyScssIncludeTarget> for FormatAnyScssIncludeTarget {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssIncludeTarget, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssIncludeTarget::CssIdentifier(node) => node.format().fmt(f),
            AnyScssIncludeTarget::ScssQualifiedName(node) => node.format().fmt(f),
        }
    }
}
