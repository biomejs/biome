//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_css_syntax::AnyCssScopeRange;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyCssScopeRange;
impl FormatRule<AnyCssScopeRange> for FormatAnyCssScopeRange {
    type Context = CssFormatContext;
    fn fmt(&self, node: &AnyCssScopeRange, f: &mut CssFormatter) -> FormatResult<()> {
        match node {
            AnyCssScopeRange::CssBogusScopeRange(node) => node.format().fmt(f),
            AnyCssScopeRange::CssScopeRangeEnd(node) => node.format().fmt(f),
            AnyCssScopeRange::CssScopeRangeInterval(node) => node.format().fmt(f),
            AnyCssScopeRange::CssScopeRangeStart(node) => node.format().fmt(f),
        }
    }
}
