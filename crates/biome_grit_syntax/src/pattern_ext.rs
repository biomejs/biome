use crate::{AnyGritPattern, GritSyntaxNode};
use biome_rowan::AstNode;

pub trait PatternExt {
    fn syntax(&self) -> &GritSyntaxNode;
}

impl PatternExt for AnyGritPattern {
    fn syntax(&self) -> &GritSyntaxNode {
        match self {
            AnyGritPattern::AnyGritLiteral(pattern) => pattern.syntax(),
            AnyGritPattern::GritAddOperation(pattern) => pattern.syntax(),
            AnyGritPattern::GritAssignmentAsPattern(pattern) => pattern.syntax(),
            AnyGritPattern::GritBogusPattern(pattern) => pattern.syntax(),
            AnyGritPattern::GritBracketedPattern(pattern) => pattern.syntax(),
            AnyGritPattern::GritBubble(pattern) => pattern.syntax(),
            AnyGritPattern::GritDivOperation(pattern) => pattern.syntax(),
            AnyGritPattern::GritDot(pattern) => pattern.syntax(),
            AnyGritPattern::GritEvery(pattern) => pattern.syntax(),
            AnyGritPattern::GritFiles(pattern) => pattern.syntax(),
            AnyGritPattern::GritLike(pattern) => pattern.syntax(),
            AnyGritPattern::GritListAccessor(pattern) => pattern.syntax(),
            AnyGritPattern::GritMapAccessor(pattern) => pattern.syntax(),
            AnyGritPattern::GritModOperation(pattern) => pattern.syntax(),
            AnyGritPattern::GritMulOperation(pattern) => pattern.syntax(),
            AnyGritPattern::GritNodeLike(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternAccumulate(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternAfter(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternAnd(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternAny(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternAs(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternBefore(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternContains(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternIfElse(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternIncludes(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternLimit(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternMaybe(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternNot(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternOr(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternOrElse(pattern) => pattern.syntax(),
            AnyGritPattern::GritPatternWhere(pattern) => pattern.syntax(),
            AnyGritPattern::GritRegexPattern(pattern) => pattern.syntax(),
            AnyGritPattern::GritRewrite(pattern) => pattern.syntax(),
            AnyGritPattern::GritSequential(pattern) => pattern.syntax(),
            AnyGritPattern::GritSome(pattern) => pattern.syntax(),
            AnyGritPattern::GritSubOperation(pattern) => pattern.syntax(),
            AnyGritPattern::GritUnderscore(pattern) => pattern.syntax(),
            AnyGritPattern::GritVariable(pattern) => pattern.syntax(),
            AnyGritPattern::GritWithin(pattern) => pattern.syntax(),
        }
    }
}
