//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssKeyframesIdentifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssKeyframesIdentifier;
impl FormatRule<AnyCssKeyframesIdentifier> for FormatAnyCssKeyframesIdentifier {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssKeyframesIdentifier, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssKeyframesIdentifier::CssCustomIdentifier(node) => node.format().fmt(f),
            AnyCssKeyframesIdentifier::CssString(node) => node.format().fmt(f),
        }
    }
}
