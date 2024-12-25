//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssAtRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssAtRule;
impl FormatRule<AnyCssAtRule> for FormatAnyCssAtRule {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssAtRule::CssBogusAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssCharsetAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssColorProfileAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssContainerAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssCounterStyleAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssDocumentAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssFontFaceAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssFontFeatureValuesAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssFontPaletteValuesAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssImportAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssKeyframesAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssLayerAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssMediaAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssNamespaceAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssPageAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssPositionTryAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssPropertyAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssScopeAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssStartingStyleAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssSupportsAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssUnknownBlockAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssUnknownValueAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssValueAtRule(node) => node.format().fmt(f),
            AnyCssAtRule::CssViewTransitionAtRule(node) => node.format().fmt(f),
        }
    }
}
