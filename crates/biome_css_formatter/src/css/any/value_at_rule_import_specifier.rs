//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssValueAtRuleImportSpecifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssValueAtRuleImportSpecifier;
impl FormatRule<AnyCssValueAtRuleImportSpecifier> for FormatAnyCssValueAtRuleImportSpecifier {
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &AnyCssValueAtRuleImportSpecifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyCssValueAtRuleImportSpecifier::CssValueAtRuleImportSpecifier(node) => {
                node.format().fmt(f)
            }
            AnyCssValueAtRuleImportSpecifier::CssValueAtRuleNamedImportSpecifier(node) => {
                node.format().fmt(f)
            }
        }
    }
}
