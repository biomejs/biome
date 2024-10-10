use super::{
    compilation_context::NodeCompilationContext, variable_compiler::VariableCompiler,
    PatternCompiler,
};
use crate::{grit_code_snippet::GritCodeSnippet, grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritPatternAccumulate, GritPredicateAccumulate};
use grit_pattern_matcher::pattern::{Accumulate, DynamicPattern, Pattern};

pub(crate) struct AccumulateCompiler;

impl AccumulateCompiler {
    pub(crate) fn from_node(
        node: &GritPatternAccumulate,
        context: &mut NodeCompilationContext,
    ) -> Result<Accumulate<GritQueryContext>, CompileError> {
        let left = PatternCompiler::from_node(&node.left()?, context)?;
        let right = PatternCompiler::from_node_with_rhs(&node.right()?, context, true)?;
        let dynamic_right = match right.clone() {
            Pattern::Dynamic(pattern) => Some(pattern),
            Pattern::CodeSnippet(GritCodeSnippet {
                dynamic_snippet: Some(snippet),
                ..
            }) => Some(snippet),
            Pattern::Variable(variable) => Some(DynamicPattern::Variable(variable)),
            _ => None,
        };

        Ok(Accumulate::new(left, right, dynamic_right))
    }
}

pub(crate) struct PrAccumulateCompiler;

impl PrAccumulateCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateAccumulate,
        context: &mut NodeCompilationContext,
    ) -> Result<Accumulate<GritQueryContext>, CompileError> {
        let left = Pattern::Variable(VariableCompiler::from_node(&node.left()?, context));
        let right = PatternCompiler::from_node_with_rhs(&node.right()?, context, true)?;
        let dynamic_right = match right.clone() {
            Pattern::Dynamic(pattern) => Some(pattern),
            Pattern::CodeSnippet(GritCodeSnippet {
                dynamic_snippet: Some(snippet),
                ..
            }) => Some(snippet),
            Pattern::Variable(variable) => Some(DynamicPattern::Variable(variable)),
            _ => None,
        };

        Ok(Accumulate::new(left, right, dynamic_right))
    }
}
