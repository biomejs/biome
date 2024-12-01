use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, util::TextRangeGritExt, CompileError};
use biome_grit_syntax::GritBubble;
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{Bubble, Pattern, PatternDefinition};
use std::collections::BTreeMap;

pub(crate) struct BubbleCompiler;

impl BubbleCompiler {
    pub(crate) fn from_node(
        node: &GritBubble,
        context: &mut NodeCompilationContext,
    ) -> Result<Bubble<GritQueryContext>, CompileError> {
        let mut local_vars = BTreeMap::new();
        let (local_scope_index, mut local_context) = create_scope!(context, local_vars);

        // important that this occurs first, as calls assume
        // that parameters are registered first

        let parameters: Vec<_> = node
            .scope()
            .into_iter()
            .map(|node| {
                let syntax = node.syntax();
                (
                    syntax.text_trimmed().to_string(),
                    syntax.text_trimmed_range().to_byte_range(),
                )
            })
            .collect();
        if parameters
            .iter()
            .map(|n| &n.0)
            .collect::<rustc_hash::FxHashSet<_>>()
            .len()
            != parameters.len()
        {
            return Err(CompileError::DuplicateParameters);
        }

        let params = local_context.get_variables(&parameters);

        let body = PatternCompiler::from_maybe_curly_node(&node.pattern()?, &mut local_context)?;

        let args = parameters
            .into_iter()
            .map(|(name, range)| Ok(Pattern::Variable(context.register_variable(name, range))))
            .collect::<Result<Vec<_>, CompileError>>()?;

        let pattern_def =
            PatternDefinition::new("<bubble>".to_owned(), local_scope_index, params, body);

        Ok(Bubble::new(pattern_def, args))
    }
}
