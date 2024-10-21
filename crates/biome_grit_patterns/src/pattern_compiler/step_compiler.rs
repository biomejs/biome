use super::{
    auto_wrap::wrap_pattern_in_before_and_after_each_file,
    compilation_context::NodeCompilationContext, PatternCompiler,
};
use crate::{diagnostics::CompilerDiagnostic, grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::AnyGritPattern;
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{Pattern, Step};

const SEQUENTIAL_WARNING: &str = "`sequential` matches at the top of the file. \
    If a pattern matched outside of a sequential, but no longer matches, \
    it is likely because naked patterns are automatically wrapped with \
    `contains bubble <pattern>`";

pub(crate) struct StepCompiler;

impl StepCompiler {
    pub(crate) fn from_node(
        node: &AnyGritPattern,
        context: &mut NodeCompilationContext,
    ) -> Result<Step<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_node(node, context)?;
        match pattern {
            Pattern::File(_)
            | Pattern::Files(_)
            | Pattern::Contains(_)
            | Pattern::Includes(_)
            | Pattern::Maybe(_)
            | Pattern::Call(_)
            | Pattern::Where(_)
            | Pattern::Bubble(_) => {}
            Pattern::And(_)
            | Pattern::Or(_)
            | Pattern::AstNode(_)
            | Pattern::List(_)
            | Pattern::ListIndex(_)
            | Pattern::Map(_)
            | Pattern::Accessor(_)
            | Pattern::Regex(_)
            | Pattern::Limit(_)
            | Pattern::CallBuiltIn(_)
            | Pattern::CallFunction(_)
            | Pattern::CallForeignFunction(_)
            | Pattern::CallbackPattern(_)
            | Pattern::Assignment(_)
            | Pattern::Accumulate(_)
            | Pattern::Any(_)
            | Pattern::Not(_)
            | Pattern::If(_)
            | Pattern::Undefined
            | Pattern::Top
            | Pattern::Bottom
            | Pattern::Underscore
            | Pattern::StringConstant(_)
            | Pattern::AstLeafNode(_)
            | Pattern::IntConstant(_)
            | Pattern::FloatConstant(_)
            | Pattern::BooleanConstant(_)
            | Pattern::Dynamic(_)
            | Pattern::CodeSnippet(_)
            | Pattern::Variable(_)
            | Pattern::Rewrite(_)
            | Pattern::Range(_)
            | Pattern::Within(_)
            | Pattern::After(_)
            | Pattern::Before(_)
            | Pattern::Some(_)
            | Pattern::Every(_)
            | Pattern::Add(_)
            | Pattern::Subtract(_)
            | Pattern::Multiply(_)
            | Pattern::Divide(_)
            | Pattern::Modulo(_)
            | Pattern::Dots
            | Pattern::Like(_) => {
                context.log(CompilerDiagnostic::new_warning(
                    SEQUENTIAL_WARNING,
                    node.syntax().text_trimmed_range(),
                ));
            }
            Pattern::Sequential(ref s) => {
                for step in s.iter() {
                    if !matches!(
                        step.pattern,
                        Pattern::File(_)
                            | Pattern::Files(_)
                            | Pattern::Contains(_)
                            | Pattern::Includes(_)
                            | Pattern::Maybe(_)
                            | Pattern::Call(_)
                            | Pattern::Where(_)
                    ) {
                        context.log(CompilerDiagnostic::new_warning(
                            SEQUENTIAL_WARNING,
                            node.syntax().text_trimmed_range(),
                        ));
                        break;
                    }
                }
            }
        }
        let pattern = wrap_pattern_in_before_and_after_each_file(pattern, context)?;

        Ok(Step::new(pattern))
    }
}
