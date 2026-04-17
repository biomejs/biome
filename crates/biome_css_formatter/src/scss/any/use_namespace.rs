//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssUseNamespace;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssUseNamespace;
impl FormatRule<AnyScssUseNamespace> for FormatAnyScssUseNamespace {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssUseNamespace, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssUseNamespace::CssIdentifier(node) => node.format().fmt(f),
            AnyScssUseNamespace::ScssUseAllNamespace(node) => node.format().fmt(f),
        }
    }
}
