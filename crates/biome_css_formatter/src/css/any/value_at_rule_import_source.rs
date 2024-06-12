//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssValueAtRuleImportSource;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssValueAtRuleImportSource;
impl FormatRule<AnyCssValueAtRuleImportSource> for FormatAnyCssValueAtRuleImportSource {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssValueAtRuleImportSource, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssValueAtRuleImportSource::CssIdentifier(node) => node.format().fmt(f),
            AnyCssValueAtRuleImportSource::CssString(node) => node.format().fmt(f),
        }
    }
}
