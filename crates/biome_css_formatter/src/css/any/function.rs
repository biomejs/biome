//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssFunction;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssFunction;
impl FormatRule<AnyCssFunction> for FormatAnyCssFunction {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssFunction, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssFunction::CssFunction(node) => node.format().fmt(f),
            AnyCssFunction::CssUrlFunction(node) => node.format().fmt(f),
            AnyCssFunction::TwAlphaFunction(node) => node.format().fmt(f),
            AnyCssFunction::TwModifierFunction(node) => node.format().fmt(f),
            AnyCssFunction::TwSpacingFunction(node) => node.format().fmt(f),
            AnyCssFunction::TwValueFunction(node) => node.format().fmt(f),
        }
    }
}
