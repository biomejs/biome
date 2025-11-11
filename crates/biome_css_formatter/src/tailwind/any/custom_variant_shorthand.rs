//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyTwCustomVariantShorthand;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTwCustomVariantShorthand;
impl FormatRule<AnyTwCustomVariantShorthand> for FormatAnyTwCustomVariantShorthand {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyTwCustomVariantShorthand, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyTwCustomVariantShorthand::CssAtRuleDeclarator(node) => node.format().fmt(f),
            AnyTwCustomVariantShorthand::CssSelectorList(node) => node.format().fmt(f),
        }
    }
}
