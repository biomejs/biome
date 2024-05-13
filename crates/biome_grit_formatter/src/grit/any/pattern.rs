//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritPattern;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritPattern;
impl FormatRule<AnyGritPattern> for FormatAnyGritPattern {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritPattern, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritPattern::AnyGritLiteral(node) => node.format().fmt(f),
            AnyGritPattern::GritAddOperation(node) => node.format().fmt(f),
            AnyGritPattern::GritAssignmentAsPattern(node) => node.format().fmt(f),
            AnyGritPattern::GritBogusPattern(node) => node.format().fmt(f),
            AnyGritPattern::GritBracketedPattern(node) => node.format().fmt(f),
            AnyGritPattern::GritBubble(node) => node.format().fmt(f),
            AnyGritPattern::GritDivOperation(node) => node.format().fmt(f),
            AnyGritPattern::GritDot(node) => node.format().fmt(f),
            AnyGritPattern::GritEvery(node) => node.format().fmt(f),
            AnyGritPattern::GritFiles(node) => node.format().fmt(f),
            AnyGritPattern::GritLike(node) => node.format().fmt(f),
            AnyGritPattern::GritListAccessor(node) => node.format().fmt(f),
            AnyGritPattern::GritMapAccessor(node) => node.format().fmt(f),
            AnyGritPattern::GritModOperation(node) => node.format().fmt(f),
            AnyGritPattern::GritMulOperation(node) => node.format().fmt(f),
            AnyGritPattern::GritNodeLike(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternAccumulate(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternAfter(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternAnd(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternAny(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternAs(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternBefore(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternContains(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternIfElse(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternIncludes(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternLimit(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternMaybe(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternNot(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternOr(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternOrElse(node) => node.format().fmt(f),
            AnyGritPattern::GritPatternWhere(node) => node.format().fmt(f),
            AnyGritPattern::GritRegexPattern(node) => node.format().fmt(f),
            AnyGritPattern::GritRewrite(node) => node.format().fmt(f),
            AnyGritPattern::GritSequential(node) => node.format().fmt(f),
            AnyGritPattern::GritSome(node) => node.format().fmt(f),
            AnyGritPattern::GritSubOperation(node) => node.format().fmt(f),
            AnyGritPattern::GritUnderscore(node) => node.format().fmt(f),
            AnyGritPattern::GritVariable(node) => node.format().fmt(f),
            AnyGritPattern::GritWithin(node) => node.format().fmt(f),
        }
    }
}
