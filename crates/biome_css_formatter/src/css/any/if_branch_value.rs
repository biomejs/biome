//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssIfBranchValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssIfBranchValue;
impl FormatRule<AnyCssIfBranchValue> for FormatAnyCssIfBranchValue {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssIfBranchValue, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssIfBranchValue::CssGenericComponentValueList(node) => node.format().fmt(f),
            AnyCssIfBranchValue::ScssExpression(node) => node.format().fmt(f),
        }
    }
}
