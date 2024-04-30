pub(crate) mod compilation_context;

mod add_compiler;
mod after_compiler;
mod and_compiler;
mod any_compiler;
mod auto_wrap;
mod before_compiler;
mod container_compiler;
mod divide_compiler;
mod every_compiler;
mod list_compiler;
mod list_index_compiler;
mod literal_compiler;
mod map_accessor_compiler;
mod map_compiler;
mod maybe_compiler;
mod modulo_compiler;
mod multiply_compiler;
mod not_compiler;
mod or_compiler;
mod sequential_compiler;
mod some_compiler;
mod step_compiler;
mod subtract_compiler;
mod variable_compiler;
mod within_compiler;

use self::{
    add_compiler::AddCompiler, after_compiler::AfterCompiler, and_compiler::AndCompiler,
    any_compiler::AnyCompiler, before_compiler::BeforeCompiler,
    compilation_context::NodeCompilationContext, divide_compiler::DivideCompiler,
    every_compiler::EveryCompiler, list_index_compiler::ListIndexCompiler,
    literal_compiler::LiteralCompiler, map_accessor_compiler::MapAccessorCompiler,
    maybe_compiler::MaybeCompiler, modulo_compiler::ModuloCompiler,
    multiply_compiler::MultiplyCompiler, not_compiler::NotCompiler, or_compiler::OrCompiler,
    sequential_compiler::SequentialCompiler, some_compiler::SomeCompiler,
    subtract_compiler::SubtractCompiler, variable_compiler::VariableCompiler,
    within_compiler::WithinCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritMaybeCurlyPattern, AnyGritPattern, GritSyntaxKind};
use grit_pattern_matcher::pattern::{DynamicPattern, DynamicSnippet, DynamicSnippetPart, Pattern};

pub(crate) struct PatternCompiler;

impl PatternCompiler {
    pub(crate) fn from_node(
        node: &AnyGritPattern,
        context: &mut NodeCompilationContext,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        Self::from_node_with_rhs(node, context, false)
    }

    pub(crate) fn from_maybe_curly_node(
        node: &AnyGritMaybeCurlyPattern,
        context: &mut NodeCompilationContext,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        match node {
            AnyGritMaybeCurlyPattern::AnyGritPattern(pattern) => Self::from_node(pattern, context),
            AnyGritMaybeCurlyPattern::GritCurlyPattern(pattern) => {
                Self::from_node(&pattern.pattern()?, context)
            }
        }
    }

    fn from_node_with_rhs(
        node: &AnyGritPattern,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        match node {
            AnyGritPattern::AnyGritLiteral(node) => {
                LiteralCompiler::from_node_with_rhs(node, context, is_rhs)
            }
            AnyGritPattern::GritAddOperation(node) => Ok(Pattern::Add(Box::new(
                AddCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritAssignmentAsPattern(_) => todo!(),
            AnyGritPattern::GritBracketedPattern(_) => todo!(),
            AnyGritPattern::GritBubble(_) => todo!(),
            AnyGritPattern::GritDivOperation(node) => Ok(Pattern::Divide(Box::new(
                DivideCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritDot(_) => {
                Ok(Pattern::Dynamic(DynamicPattern::Snippet(DynamicSnippet {
                    parts: vec![DynamicSnippetPart::String(String::new())],
                })))
            }
            AnyGritPattern::GritEvery(node) => Ok(Pattern::Every(Box::new(
                EveryCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritFiles(node) => Ok(Pattern::Sequential(
                SequentialCompiler::from_files_node(node, context)?,
            )),
            AnyGritPattern::GritLike(_) => todo!(),
            AnyGritPattern::GritListAccessor(node) => Ok(Pattern::ListIndex(Box::new(
                ListIndexCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritMapAccessor(node) => Ok(Pattern::Accessor(Box::new(
                MapAccessorCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritModOperation(node) => Ok(Pattern::Modulo(Box::new(
                ModuloCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritMulOperation(node) => Ok(Pattern::Multiply(Box::new(
                MultiplyCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritNodeLike(_) => todo!(),
            AnyGritPattern::GritPatternAccumulate(_) => todo!(),
            AnyGritPattern::GritPatternAfter(node) => Ok(Pattern::After(Box::new(
                AfterCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAnd(node) => Ok(Pattern::And(Box::new(
                AndCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAny(node) => Ok(Pattern::Any(Box::new(
                AnyCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAs(_) => todo!(),
            AnyGritPattern::GritPatternBefore(node) => Ok(Pattern::Before(Box::new(
                BeforeCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternContains(_) => todo!(),
            AnyGritPattern::GritPatternIfElse(_) => todo!(),
            AnyGritPattern::GritPatternIncludes(_) => todo!(),
            AnyGritPattern::GritPatternLimit(_) => todo!(),
            AnyGritPattern::GritPatternMaybe(node) => Ok(Pattern::Maybe(Box::new(
                MaybeCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternNot(node) => Ok(Pattern::Not(Box::new(
                NotCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternOr(node) => {
                Ok(Pattern::Or(Box::new(OrCompiler::from_node(node, context)?)))
            }
            AnyGritPattern::GritPatternOrElse(_) => todo!(),
            AnyGritPattern::GritPatternWhere(_) => todo!(),
            AnyGritPattern::GritRegexPattern(_) => todo!(),
            AnyGritPattern::GritRewrite(_) => todo!(),
            AnyGritPattern::GritSequential(node) => Ok(Pattern::Sequential(
                SequentialCompiler::from_node(node, context)?,
            )),
            AnyGritPattern::GritSome(node) => Ok(Pattern::Some(Box::new(SomeCompiler::from_node(
                node, context,
            )?))),
            AnyGritPattern::GritSubOperation(node) => Ok(Pattern::Subtract(Box::new(
                SubtractCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritUnderscore(_) => Ok(Pattern::Underscore),
            AnyGritPattern::GritVariable(node) => Ok(Pattern::Variable(
                VariableCompiler::from_node(node, context)?,
            )),
            AnyGritPattern::GritWithin(node) => Ok(Pattern::Within(Box::new(
                WithinCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritBogusPattern(_) => Err(CompileError::UnexpectedKind(
                GritSyntaxKind::GRIT_BOGUS_PATTERN.into(),
            )),
        }
    }
}
