//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritListPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritListPattern;
impl FormatRule<AnyGritListPattern> for FormatAnyGritListPattern {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritListPattern, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritListPattern::AnyGritPattern(node) => node.format().fmt(f),
            AnyGritListPattern::GritDotdotdot(node) => node.format().fmt(f),
        }
    }
}
