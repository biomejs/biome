use super::call_compiler::*;
use super::compilation_context::NodeCompilationContext;
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritNamedArgList;
use grit_pattern_matcher::pattern::{Log, Pattern, VariableInfo};

pub(crate) struct LogCompiler;

impl LogCompiler {
    pub(crate) fn from_named_args(
        named_args: GritNamedArgList,
        context: &mut NodeCompilationContext,
    ) -> Result<Log<GritQueryContext>, CompileError> {
        let named_args = node_to_args_pairs(
            "log",
            named_args,
            &context.compilation.lang,
            &Some(vec!["message".to_owned(), "variable".to_owned()]),
        )?;
        let mut args = named_args_to_map(named_args, context)?;
        let message = args.remove("$message");
        let variable = args.remove("$variable");
        let variable = variable.and_then(|pattern| match pattern {
            Pattern::Variable(variable) => {
                let source_location = &context.vars_array[variable.scope][variable.index];
                Some(VariableInfo::new(source_location.name.clone(), variable))
            }
            _ => None,
        });

        Ok(Log::new(variable, message))
    }
}
