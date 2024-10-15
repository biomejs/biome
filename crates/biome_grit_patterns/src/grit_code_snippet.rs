use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_node::GritTargetSyntaxKind;
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::context::ExecContext;
use grit_pattern_matcher::pattern::{
    CodeSnippet, DynamicPattern, Matcher, Pattern, PatternName, ResolvedPattern, State,
};
use grit_util::error::GritResult;
use grit_util::AnalysisLogs;

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

        if let Some((_, pattern)) = self.patterns.iter().find(|(kind, _)| *kind == node.kind()) {
            pattern.execute(resolved, state, context, logs)
        } else {
            Ok(false)
        }
    }
}

impl PatternName for GritCodeSnippet {
    fn name(&self) -> &'static str {
        "CodeSnippet"
    }
}
