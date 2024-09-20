use super::{
    accumulate_compiler::PrAccumulateCompiler, and_compiler::PrAndCompiler,
    any_compiler::PrAnyCompiler, assignment_compiler::PrAssignmentCompiler,
    compilation_context::NodeCompilationContext, equal_compiler::PrEqualCompiler,
    if_compiler::PrIfCompiler, match_compiler::PrMatchCompiler, maybe_compiler::PrMaybeCompiler,
    not_compiler::PrNotCompiler, or_compiler::PrOrCompiler,
    predicate_call_compiler::PrCallCompiler, predicate_return_compiler::PrReturnCompiler,
    rewrite_compiler::PrRewriteCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritPredicate, GritSyntaxKind};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::Predicate;

pub(crate) struct PredicateCompiler;

impl PredicateCompiler {
    pub(crate) fn from_node(
        node: &AnyGritPredicate,
        context: &mut NodeCompilationContext,
    ) -> Result<Predicate<GritQueryContext>, CompileError> {
        match node {
            AnyGritPredicate::GritBooleanLiteral(node) => Ok(match node.value()?.text_trimmed() {
                "true" => Predicate::True,
                _ => Predicate::False,
            }),
            AnyGritPredicate::GritBracketedPredicate(node) => {
                Self::from_node(&node.predicate()?, context)
            }
            AnyGritPredicate::GritPredicateAccumulate(node) => Ok(Predicate::Accumulate(Box::new(
                PrAccumulateCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateAnd(node) => Ok(Predicate::And(Box::new(
                PrAndCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateAny(node) => Ok(Predicate::Any(Box::new(
                PrAnyCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateAssignment(node) => Ok(Predicate::Assignment(Box::new(
                PrAssignmentCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateCall(node) => PrCallCompiler::from_node(node, context),
            AnyGritPredicate::GritPredicateEqual(node) => Ok(Predicate::Equal(Box::new(
                PrEqualCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateGreater(node) => {
                Err(CompileError::UnsupportedKind(node.syntax().kind().into())) // Not supported by Grit either.
            }
            AnyGritPredicate::GritPredicateGreaterEqual(node) => {
                Err(CompileError::UnsupportedKind(node.syntax().kind().into())) // Not supported by Grit either.
            }
            AnyGritPredicate::GritPredicateIfElse(node) => Ok(Predicate::If(Box::new(
                PrIfCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateLess(node) => {
                Err(CompileError::UnsupportedKind(node.syntax().kind().into())) // Not supported by Grit either.
            }
            AnyGritPredicate::GritPredicateLessEqual(node) => {
                Err(CompileError::UnsupportedKind(node.syntax().kind().into())) // Not supported by Grit either.
            }
            AnyGritPredicate::GritPredicateMatch(node) => Ok(Predicate::Match(Box::new(
                PrMatchCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateMaybe(node) => Ok(Predicate::Maybe(Box::new(
                PrMaybeCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateNot(node) => Ok(Predicate::Not(Box::new(
                PrNotCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateNotEqual(node) => {
                Err(CompileError::UnsupportedKind(node.syntax().kind().into())) // Not supported by Grit either.
            }
            AnyGritPredicate::GritPredicateOr(node) => Ok(Predicate::Or(Box::new(
                PrOrCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateReturn(node) => Ok(Predicate::Return(Box::new(
                PrReturnCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateRewrite(node) => Ok(Predicate::Rewrite(Box::new(
                PrRewriteCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritBogusPredicate(_) => Err(CompileError::UnexpectedKind(
                GritSyntaxKind::GRIT_BOGUS_PREDICATE.into(),
            )),
        }
    }
}
