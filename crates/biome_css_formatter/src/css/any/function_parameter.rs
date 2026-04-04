//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssFunctionParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssFunctionParameter;
impl FormatRule<AnyCssFunctionParameter> for FormatAnyCssFunctionParameter {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssFunctionParameter, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssFunctionParameter::CssBogusFunctionParameter(node) => node.format().fmt(f),
            AnyCssFunctionParameter::CssFunctionParameter(node) => node.format().fmt(f),
        }
    }
}
