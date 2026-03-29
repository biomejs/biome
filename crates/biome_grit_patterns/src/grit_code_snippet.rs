use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_node::GritTargetSyntaxKind;
use biome_js_syntax::JsSyntaxKind;
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::context::ExecContext;
use grit_pattern_matcher::pattern::{
    CodeSnippet, DynamicPattern, Matcher, Pattern, PatternName, ResolvedPattern, State,
};
use grit_util::error::GritResult;
use grit_util::{AnalysisLogs, AstNode};

#[derive(Clone, Debug)]
pub struct GritCodeSnippet {
    pub(crate) patterns: Vec<(GritTargetSyntaxKind, Pattern<GritQueryContext>)>,
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern<GritQueryContext>>,
}

impl CodeSnippet<GritQueryContext> for GritCodeSnippet {
    fn patterns(&self) -> impl Iterator<Item = &Pattern<GritQueryContext>> {
        self.patterns.iter().map(|p| &p.1)
    }

    fn dynamic_snippet(&self) -> Option<&DynamicPattern<GritQueryContext>> {
        self.dynamic_snippet.as_ref()
    }
}

impl Matcher<GritQueryContext> for GritCodeSnippet {
    fn execute<'a>(
        &'a self,
        resolved: &GritResolvedPattern<'a>,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let Some(binding) = resolved.get_last_binding() else {
            return Ok(resolved.text(&state.files, context.language())?.trim() == self.source);
        };

        let Some(node) = binding.singleton() else {
            return Ok(false);
        };

        // Try every parse of the snippet with the same outer kind.
        for (_, pattern) in self
            .patterns
            .iter()
            .filter(|(kind, _)| *kind == node.kind())
        {
            let mut candidate_state = state.clone();
            if pattern.execute(resolved, &mut candidate_state, context, logs)? {
                *state = candidate_state;
                return Ok(true);
            }
        }

        // If node has a single child, try matching against the child
        // This handles wrapper nodes like JS_IDENTIFIER_EXPRESSION wrapping JS_REFERENCE_IDENTIFIER
        if node.kind() == GritTargetSyntaxKind::JsSyntaxKind(JsSyntaxKind::JS_IDENTIFIER_EXPRESSION)
            && let Some(child) = node.first_child()
            && child.next_sibling().is_none()
        {
            let child_binding = GritResolvedPattern::from_node_binding(child);
            for (_, pattern) in &self.patterns {
                let mut candidate_state = state.clone();
                if pattern.execute(&child_binding, &mut candidate_state, context, logs)? {
                    *state = candidate_state;
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }
}

impl PatternName for GritCodeSnippet {
    fn name(&self) -> &'static str {
        "CodeSnippet"
    }
}
