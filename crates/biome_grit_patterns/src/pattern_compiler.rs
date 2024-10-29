pub(crate) mod compilation_context;

/// Creates a new scope within the given `context`.
///
/// This is implemented as a macro instead of method to avoid capturing the
/// entire `context` instance, which would run afoul of the borrow-checking due
/// to its mutable references.
macro_rules! create_scope {
    ($context: expr, $local_vars: expr) => {{
        let scope_index = $context.vars_array.len();
        $context.vars_array.push(Vec::new());
        let context = crate::pattern_compiler::NodeCompilationContext {
            compilation: $context.compilation,
            vars: &mut $local_vars,
            vars_array: $context.vars_array,
            scope_index,
            global_vars: $context.global_vars,
            diagnostics: $context.diagnostics,
        };
        (scope_index, context)
    }};
}

mod accumulate_compiler;
mod add_compiler;
mod after_compiler;
mod and_compiler;
mod any_compiler;
mod as_compiler;
mod assignment_compiler;
mod auto_wrap;
mod before_compiler;
mod bubble_compiler;
mod call_compiler;
mod container_compiler;
mod contains_compiler;
mod divide_compiler;
mod equal_compiler;
mod every_compiler;
mod function_definition_compiler;
mod if_compiler;
mod includes_compiler;
mod like_compiler;
mod limit_compiler;
mod list_compiler;
mod list_index_compiler;
mod literal_compiler;
mod map_accessor_compiler;
mod map_compiler;
mod match_compiler;
mod maybe_compiler;
mod modulo_compiler;
mod multiply_compiler;
mod node_like_compiler;
mod not_compiler;
mod or_compiler;
mod pattern_definition_compiler;
mod predicate_call_compiler;
mod predicate_compiler;
mod predicate_definition_compiler;
mod predicate_return_compiler;
mod regex_compiler;
mod rewrite_compiler;
mod sequential_compiler;
mod snippet_compiler;
mod some_compiler;
mod step_compiler;
mod subtract_compiler;
mod variable_compiler;
mod where_compiler;
mod within_compiler;

pub use function_definition_compiler::FunctionDefinitionCompiler;
pub use pattern_definition_compiler::PatternDefinitionCompiler;
pub use predicate_definition_compiler::PredicateDefinitionCompiler;

use self::{
    accumulate_compiler::AccumulateCompiler, add_compiler::AddCompiler,
    after_compiler::AfterCompiler, and_compiler::AndCompiler, any_compiler::AnyCompiler,
    assignment_compiler::AssignmentCompiler, before_compiler::BeforeCompiler,
    bubble_compiler::BubbleCompiler, compilation_context::NodeCompilationContext,
    contains_compiler::ContainsCompiler, divide_compiler::DivideCompiler,
    every_compiler::EveryCompiler, if_compiler::IfCompiler, includes_compiler::IncludesCompiler,
    like_compiler::LikeCompiler, limit_compiler::LimitCompiler,
    list_index_compiler::ListIndexCompiler, literal_compiler::LiteralCompiler,
    map_accessor_compiler::MapAccessorCompiler, maybe_compiler::MaybeCompiler,
    modulo_compiler::ModuloCompiler, multiply_compiler::MultiplyCompiler,
    not_compiler::NotCompiler, or_compiler::OrCompiler, rewrite_compiler::RewriteCompiler,
    sequential_compiler::SequentialCompiler, some_compiler::SomeCompiler,
    subtract_compiler::SubtractCompiler, variable_compiler::VariableCompiler,
    where_compiler::WhereCompiler, within_compiler::WithinCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use as_compiler::AsCompiler;
use biome_grit_syntax::{AnyGritMaybeCurlyPattern, AnyGritPattern, GritSyntaxKind};
use biome_rowan::AstNode as _;
use grit_pattern_matcher::pattern::{DynamicPattern, DynamicSnippet, DynamicSnippetPart, Pattern};
use node_like_compiler::NodeLikeCompiler;
use regex_compiler::RegexCompiler;

pub(crate) use self::auto_wrap::auto_wrap_pattern;

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
            AnyGritPattern::GritAssignmentAsPattern(node) => Ok(Pattern::Assignment(Box::new(
                AssignmentCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritBracketedPattern(node) => {
                Self::from_node_with_rhs(&node.pattern()?, context, is_rhs)
            }
            AnyGritPattern::GritBubble(node) => Ok(Pattern::Bubble(Box::new(
                BubbleCompiler::from_node(node, context)?,
            ))),
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
            AnyGritPattern::GritLike(node) => Ok(Pattern::Like(Box::new(LikeCompiler::from_node(
                node, context,
            )?))),
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
            AnyGritPattern::GritNodeLike(node) => {
                NodeLikeCompiler::from_node_with_rhs(node, context, is_rhs)
            }
            AnyGritPattern::GritPatternAccumulate(node) => Ok(Pattern::Accumulate(Box::new(
                AccumulateCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAfter(node) => Ok(Pattern::After(Box::new(
                AfterCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAnd(node) => Ok(Pattern::And(Box::new(
                AndCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAny(node) => Ok(Pattern::Any(Box::new(
                AnyCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAs(node) => Ok(Pattern::Where(Box::new(
                AsCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternBefore(node) => Ok(Pattern::Before(Box::new(
                BeforeCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternContains(node) => Ok(Pattern::Contains(Box::new(
                ContainsCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternIfElse(node) => {
                Ok(Pattern::If(Box::new(IfCompiler::from_node(node, context)?)))
            }
            AnyGritPattern::GritPatternIncludes(node) => Ok(Pattern::Includes(Box::new(
                IncludesCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternLimit(node) => Ok(Pattern::Limit(Box::new(
                LimitCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternMaybe(node) => Ok(Pattern::Maybe(Box::new(
                MaybeCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternNot(node) => Ok(Pattern::Not(Box::new(
                NotCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternOr(node) => {
                Ok(Pattern::Or(Box::new(OrCompiler::from_node(node, context)?)))
            }
            AnyGritPattern::GritPatternOrElse(node) => {
                Err(CompileError::UnsupportedKind(node.syntax().kind().into()))
            }
            AnyGritPattern::GritPatternWhere(node) => Ok(Pattern::Where(Box::new(
                WhereCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritRegexPattern(node) => Ok(Pattern::Regex(Box::new(
                RegexCompiler::from_node(node, context, is_rhs)?,
            ))),
            AnyGritPattern::GritRewrite(node) => Ok(Pattern::Rewrite(Box::new(
                RewriteCompiler::from_node(node, context)?,
            ))),
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
                VariableCompiler::from_node(node, context),
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
