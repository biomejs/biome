//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritPredicate;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritPredicate;
impl FormatRule<AnyGritPredicate> for FormatAnyGritPredicate {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritPredicate, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritPredicate::GritBogusPredicate(node) => node.format().fmt(f),
            AnyGritPredicate::GritBooleanLiteral(node) => node.format().fmt(f),
            AnyGritPredicate::GritBracketedPredicate(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateAccumulate(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateAnd(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateAny(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateAssignment(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateCall(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateEqual(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateGreater(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateGreaterEqual(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateIfElse(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateLess(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateLessEqual(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateMatch(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateMaybe(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateNot(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateNotEqual(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateOr(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateReturn(node) => node.format().fmt(f),
            AnyGritPredicate::GritPredicateRewrite(node) => node.format().fmt(f),
        }
    }
}
