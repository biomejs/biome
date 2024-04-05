//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssFontFamilyName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssFontFamilyName;
impl FormatRule<AnyCssFontFamilyName> for FormatAnyCssFontFamilyName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssFontFamilyName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssFontFamilyName::CssBogusFontFamilyName(node) => node.format().fmt(f),
            AnyCssFontFamilyName::CssFontFamilyName(node) => node.format().fmt(f),
            AnyCssFontFamilyName::CssString(node) => node.format().fmt(f),
        }
    }
}
