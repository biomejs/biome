//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttrUnit;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttrUnit;
impl FormatRule<AnyCssAttrUnit> for FormatAnyCssAttrUnit {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttrUnit, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttrUnit::CssRegularAttrUnit(node) => node.format().fmt(f),
            AnyCssAttrUnit::CssUnknownAttrUnit(node) => node.format().fmt(f),
        }
    }
}
