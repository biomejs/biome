//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::GritPredicateMatchSubject;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritPredicateMatchSubject;
impl FormatRule<GritPredicateMatchSubject> for FormatGritPredicateMatchSubject {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritPredicateMatchSubject, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            GritPredicateMatchSubject::AnyGritContainer(node) => node.format().fmt(f),
            GritPredicateMatchSubject::AnyGritLiteral(node) => node.format().fmt(f),
        }
    }
}
