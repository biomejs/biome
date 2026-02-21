//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAttrType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAttrType;
impl FormatRule<AnyCssAttrType> for FormatAnyCssAttrType {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAttrType, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAttrType::AnyCssAttrUnit(node) => node.format().fmt(f),
            AnyCssAttrType::CssNumberDeclarator(node) => node.format().fmt(f),
            AnyCssAttrType::CssRawStringDeclarator(node) => node.format().fmt(f),
            AnyCssAttrType::CssTypeFunction(node) => node.format().fmt(f),
        }
    }
}
