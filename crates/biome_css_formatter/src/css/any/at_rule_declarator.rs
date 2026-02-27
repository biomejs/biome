//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAtRuleDeclarator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAtRuleDeclarator;
impl FormatRule<AnyCssAtRuleDeclarator> for FormatAnyCssAtRuleDeclarator {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAtRuleDeclarator, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAtRuleDeclarator::CssColorProfileAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssContainerAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssCounterStyleAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssFontFaceAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssFontPaletteValuesAtRuleDeclarator(node) => {
                node.format().fmt(f)
            }
            AnyCssAtRuleDeclarator::CssFunctionAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssMediaAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssPositionTryAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssPropertyAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssScopeAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssStartingStyleAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssSupportsAtRuleDeclarator(node) => node.format().fmt(f),
            AnyCssAtRuleDeclarator::CssViewTransitionAtRuleDeclarator(node) => node.format().fmt(f),
        }
    }
}
