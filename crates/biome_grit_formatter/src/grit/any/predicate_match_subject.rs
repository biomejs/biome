//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritPredicateMatchSubject;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritPredicateMatchSubject;
impl FormatRule<AnyGritPredicateMatchSubject> for FormatAnyGritPredicateMatchSubject {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritPredicateMatchSubject, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritPredicateMatchSubject::AnyGritContainer(node) => node.format().fmt(f),
            AnyGritPredicateMatchSubject::AnyGritLiteral(node) => node.format().fmt(f),
        }
    }
}
