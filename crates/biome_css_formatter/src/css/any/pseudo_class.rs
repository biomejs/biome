//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssPseudoClass;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssPseudoClass;
impl FormatRule<AnyCssPseudoClass> for FormatAnyCssPseudoClass {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssPseudoClass, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssPseudoClass::CssBogusPseudoClass(node) => node.format().fmt(f),
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelector(node) => node.format().fmt(f),
            AnyCssPseudoClass::CssPseudoClassFunctionCompoundSelectorList(node) => {
                node.format().fmt(f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionCustomIdentifierList(node) => {
                node.format().fmt(f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionIdentifier(node) => node.format().fmt(f),
            AnyCssPseudoClass::CssPseudoClassFunctionNth(node) => node.format().fmt(f),
            AnyCssPseudoClass::CssPseudoClassFunctionRelativeSelectorList(node) => {
                node.format().fmt(f)
            }
            AnyCssPseudoClass::CssPseudoClassFunctionSelector(node) => node.format().fmt(f),
            AnyCssPseudoClass::CssPseudoClassFunctionSelectorList(node) => node.format().fmt(f),
            AnyCssPseudoClass::CssPseudoClassFunctionValueList(node) => node.format().fmt(f),
            AnyCssPseudoClass::CssPseudoClassIdentifier(node) => node.format().fmt(f),
        }
    }
}
