//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyScssKeyframesName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyScssKeyframesName;
impl FormatRule<AnyScssKeyframesName> for FormatAnyScssKeyframesName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyScssKeyframesName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyScssKeyframesName::ScssInterpolatedIdentifier(node) => node.format().fmt(f),
            AnyScssKeyframesName::ScssVariable(node) => node.format().fmt(f),
        }
    }
}
