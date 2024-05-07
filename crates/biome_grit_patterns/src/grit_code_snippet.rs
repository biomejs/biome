use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::resolved_pattern::GritResolvedPattern;
use anyhow::Result;
use grit_pattern_matcher::pattern::{
    CodeSnippet, DynamicPattern, Matcher, Pattern, PatternName, State,
};
use grit_util::AnalysisLogs;

#[derive(Clone, Debug)]
pub(crate) struct GritCodeSnippet {
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern<GritQueryContext>>,
}

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
