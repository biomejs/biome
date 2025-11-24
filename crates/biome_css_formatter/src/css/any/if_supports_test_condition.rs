//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfSupportsTestCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfSupportsTestCondition;
impl FormatRule<AnyCssIfSupportsTestCondition> for FormatAnyCssIfSupportsTestCondition {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfSupportsTestCondition, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfSupportsTestCondition::AnyCssImportSupportsCondition(node) => {
                node.format().fmt(f)
            }
            AnyCssIfSupportsTestCondition::CssIfSupportsIdentifierTest(node) => {
                node.format().fmt(f)
            }
        }
    }
}
