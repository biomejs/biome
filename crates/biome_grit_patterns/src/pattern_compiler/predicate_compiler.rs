use super::{
    and_compiler::PrAndCompiler, any_compiler::PrAnyCompiler,
    compilation_context::NodeCompilationContext, maybe_compiler::PrMaybeCompiler,
    not_compiler::PrNotCompiler, or_compiler::PrOrCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritPredicate, GritSyntaxKind};
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
            AnyGritPredicate::GritPredicateAccumulate(_) => todo!(),
            AnyGritPredicate::GritPredicateAnd(node) => Ok(Predicate::And(Box::new(
                PrAndCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateAny(node) => Ok(Predicate::Any(Box::new(
                PrAnyCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateAssignment(_) => todo!(),
            AnyGritPredicate::GritPredicateCall(_) => todo!(),
            AnyGritPredicate::GritPredicateEqual(_) => todo!(),
            AnyGritPredicate::GritPredicateGreater(_) => todo!(),
            AnyGritPredicate::GritPredicateGreaterEqual(_) => todo!(),
            AnyGritPredicate::GritPredicateIfElse(_) => todo!(),
            AnyGritPredicate::GritPredicateLess(_) => todo!(),
            AnyGritPredicate::GritPredicateLessEqual(_) => todo!(),
            AnyGritPredicate::GritPredicateMatch(_) => todo!(),
            AnyGritPredicate::GritPredicateMaybe(node) => Ok(Predicate::Maybe(Box::new(
                PrMaybeCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateNot(node) => Ok(Predicate::Not(Box::new(
                PrNotCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateNotEqual(_) => todo!(),
            AnyGritPredicate::GritPredicateOr(node) => Ok(Predicate::Or(Box::new(
                PrOrCompiler::from_node(node, context)?,
            ))),
            AnyGritPredicate::GritPredicateReturn(_) => todo!(),
            AnyGritPredicate::GritPredicateRewrite(_) => todo!(),
            AnyGritPredicate::GritBogusPredicate(_) => Err(CompileError::UnexpectedKind(
                GritSyntaxKind::GRIT_BOGUS_PREDICATE.into(),
            )),
        }
    }
}
