//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssKeyframesName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssKeyframesName;
impl FormatRule<AnyCssKeyframesName> for FormatAnyCssKeyframesName {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssKeyframesName, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssKeyframesName::AnyCssKeyframesIdentifier(node) => node.format().fmt(f),
            AnyCssKeyframesName::CssBogusKeyframesName(node) => node.format().fmt(f),
            AnyCssKeyframesName::CssKeyframesScopedName(node) => node.format().fmt(f),
        }
    }
}
