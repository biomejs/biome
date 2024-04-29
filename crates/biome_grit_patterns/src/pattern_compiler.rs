pub(crate) mod compilation_context;

mod add_compiler;
mod after_compiler;
mod and_compiler;
mod any_compiler;
mod before_compiler;
mod divide_compiler;
mod literal_compiler;
mod map_accessor_compiler;
mod modulo_compiler;
mod multiply_compiler;
mod not_compiler;
mod or_compiler;
mod subtract_compiler;

use self::{
    add_compiler::AddCompiler, after_compiler::AfterCompiler, and_compiler::AndCompiler,
    any_compiler::AnyCompiler, before_compiler::BeforeCompiler,
    compilation_context::CompilationContext, divide_compiler::DivideCompiler,
    literal_compiler::LiteralCompiler, modulo_compiler::ModuloCompiler,
    multiply_compiler::MultiplyCompiler, not_compiler::NotCompiler, or_compiler::OrCompiler,
    subtract_compiler::SubtractCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritPattern, GritSyntaxKind};
use grit_pattern_matcher::pattern::{DynamicPattern, DynamicSnippet, DynamicSnippetPart, Pattern};

pub(crate) struct PatternCompiler;

impl PatternCompiler {
    pub(crate) fn from_node(
        node: &AnyGritPattern,
        context: &mut CompilationContext,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        Self::from_node_with_rhs(node, context, false)
    }

    fn from_node_with_rhs(
        node: &AnyGritPattern,
        context: &mut CompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        match node {
            AnyGritPattern::AnyGritLiteral(literal) => {
                LiteralCompiler::from_node_with_rhs(literal, context, is_rhs)
            }
            AnyGritPattern::GritAddOperation(add) => Ok(Pattern::Add(Box::new(
                AddCompiler::from_node(add, context)?,
            ))),
            AnyGritPattern::GritAssignmentAsPattern(_) => todo!(),
            AnyGritPattern::GritBracketedPattern(_) => todo!(),
            AnyGritPattern::GritBubble(_) => todo!(),
            AnyGritPattern::GritDivOperation(divide) => Ok(Pattern::Divide(Box::new(
                DivideCompiler::from_node(divide, context)?,
            ))),
            AnyGritPattern::GritDot(_) => {
                Ok(Pattern::Dynamic(DynamicPattern::Snippet(DynamicSnippet {
                    parts: vec![DynamicSnippetPart::String(String::new())],
                })))
            }
            AnyGritPattern::GritEvery(_) => todo!(),
            AnyGritPattern::GritFiles(_) => todo!(),
            AnyGritPattern::GritLike(_) => todo!(),
            AnyGritPattern::GritListAccessor(_) => todo!(),
            AnyGritPattern::GritMapAccessor(_) => todo!(),
            AnyGritPattern::GritModOperation(modulo) => Ok(Pattern::Modulo(Box::new(
                ModuloCompiler::from_node(modulo, context)?,
            ))),
            AnyGritPattern::GritMulOperation(multiply) => Ok(Pattern::Multiply(Box::new(
                MultiplyCompiler::from_node(multiply, context)?,
            ))),
            AnyGritPattern::GritNodeLike(_) => todo!(),
            AnyGritPattern::GritPatternAccumulate(_) => todo!(),
            AnyGritPattern::GritPatternAfter(after) => Ok(Pattern::After(Box::new(
                AfterCompiler::from_node(after, context)?,
            ))),
            AnyGritPattern::GritPatternAnd(and) => Ok(Pattern::And(Box::new(
                AndCompiler::from_node(and, context)?,
            ))),
            AnyGritPattern::GritPatternAny(any) => Ok(Pattern::Any(Box::new(
                AnyCompiler::from_node(any, context)?,
            ))),
            AnyGritPattern::GritPatternAs(_) => todo!(),
            AnyGritPattern::GritPatternBefore(before) => Ok(Pattern::Before(Box::new(
                BeforeCompiler::from_node(before, context)?,
            ))),
            AnyGritPattern::GritPatternContains(_) => todo!(),
            AnyGritPattern::GritPatternIfElse(_) => todo!(),
            AnyGritPattern::GritPatternIncludes(_) => todo!(),
            AnyGritPattern::GritPatternLimit(_) => todo!(),
            AnyGritPattern::GritPatternMaybe(_) => todo!(),
            AnyGritPattern::GritPatternNot(not) => Ok(Pattern::Not(Box::new(
                NotCompiler::from_node(not, context)?,
            ))),
            AnyGritPattern::GritPatternOr(or) => {
                Ok(Pattern::Or(Box::new(OrCompiler::from_node(or, context)?)))
            }
            AnyGritPattern::GritPatternOrElse(_) => todo!(),
            AnyGritPattern::GritPatternWhere(_) => todo!(),
            AnyGritPattern::GritRegexPattern(_) => todo!(),
            AnyGritPattern::GritRewrite(_) => todo!(),
            AnyGritPattern::GritSequential(_) => todo!(),
            AnyGritPattern::GritSome(_) => todo!(),
            AnyGritPattern::GritSubOperation(subtract) => Ok(Pattern::Subtract(Box::new(
                SubtractCompiler::from_node(subtract, context)?,
            ))),
            AnyGritPattern::GritUnderscore(_) => Ok(Pattern::Underscore),
            AnyGritPattern::GritVariable(_) => todo!(),
            AnyGritPattern::GritWithin(_) => todo!(),
            AnyGritPattern::GritBogusPattern(_) => Err(CompileError::UnexpectedKind(
                GritSyntaxKind::GRIT_BOGUS_PATTERN.into(),
            )),
        }
    }
}
