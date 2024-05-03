use super::{
    compilation_context::NodeCompilationContext, container_compiler::ContainerCompiler,
    PatternCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritPredicateMatchSubject, GritPredicateMatch};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{Container, Match};

pub(crate) struct PrMatchCompiler;

impl PrMatchCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateMatch,
        context: &mut NodeCompilationContext,
    ) -> Result<Match<GritQueryContext>, CompileError> {
        let value = compile_match_subject(&node.left()?, context)?;
        let pattern = Some(PatternCompiler::from_node(&node.right()?, context)?);

        Ok(Match::new(value, pattern))
    }
}

fn compile_match_subject(
    node: &AnyGritPredicateMatchSubject,
    context: &mut NodeCompilationContext,
) -> Result<Container<GritQueryContext>, CompileError> {
    match node {
        AnyGritPredicateMatchSubject::AnyGritContainer(node) => {
            ContainerCompiler::from_node(node, context)
        }
        AnyGritPredicateMatchSubject::AnyGritLiteral(literal) => {
            // FIXME: The grammar says literals are supported here, but the
            //        Grit reference compiler doesn't accept them either.
            Err(CompileError::UnsupportedKind(
                literal.syntax().kind().into(),
            ))
        }
    }
}
