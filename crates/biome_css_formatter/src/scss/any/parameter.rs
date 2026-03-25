//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssParameter;
impl FormatRule<AnyScssParameter> for FormatAnyScssParameter {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssParameter, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssParameter::CssBogusParameter(node) => node.format().fmt(f),
            AnyScssParameter::ScssParameter(node) => node.format().fmt(f),
        }
    }
}
