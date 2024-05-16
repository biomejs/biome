//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritMaybeCurlyPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritMaybeCurlyPattern;
impl FormatRule<AnyGritMaybeCurlyPattern> for FormatAnyGritMaybeCurlyPattern {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritMaybeCurlyPattern, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritMaybeCurlyPattern::AnyGritPattern(node) => node.format().fmt(f),
            AnyGritMaybeCurlyPattern::GritCurlyPattern(node) => node.format().fmt(f),
        }
    }
}
