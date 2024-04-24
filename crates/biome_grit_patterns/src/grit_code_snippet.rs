use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::resolved_pattern::GritResolvedPattern;
use anyhow::Result;
use grit_core_patterns::pattern::dynamic_snippet::DynamicPattern;
use grit_core_patterns::pattern::patterns::{CodeSnippet, Matcher, Pattern, PatternName};
use grit_core_patterns::pattern::state::State;
use grit_util::AnalysisLogs;

#[derive(Clone, Debug)]
pub(crate) struct GritCodeSnippet;

impl CodeSnippet<GritQueryContext> for GritCodeSnippet {
    fn patterns(&self) -> impl Iterator<Item = &Pattern<GritQueryContext>> {
        TodoIterator { _snippet: self }
    }

    fn dynamic_snippet(&self) -> Option<&DynamicPattern<GritQueryContext>> {
        todo!()
    }
}

impl Matcher<GritQueryContext> for GritCodeSnippet {
    fn execute<'a>(
        &'a self,
        _binding: &GritResolvedPattern,
        _state: &mut State<'a, GritQueryContext>,
        _context: &'a GritExecContext,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        todo!()
    }
}

impl PatternName for GritCodeSnippet {
    fn name(&self) -> &'static str {
        "CodeSnippet"
    }
}

#[derive(Clone)]
struct TodoIterator<'a> {
    _snippet: &'a GritCodeSnippet,
}

impl<'a> Iterator for TodoIterator<'a> {
    type Item = &'a Pattern<GritQueryContext>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
