//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssParameter;
impl FormatRule<AnyCssParameter> for FormatAnyCssParameter {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssParameter, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssParameter::AnyCssExpression(node) => node.format().fmt(f),
            AnyCssParameter::CssListOfComponentValuesExpress(node) => node.format().fmt(f),
        }
    }
}
