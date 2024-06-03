use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_target_node::GritTargetNode;
use crate::resolved_pattern::GritResolvedPattern;
use anyhow::Result;
use grit_pattern_matcher::pattern::{
    AstLeafNodePattern, AstNodePattern, Matcher, PatternName, PatternOrPredicate, State,
};
use grit_util::AnalysisLogs;

#[derive(Clone, Debug)]
pub(crate) struct GritNodePattern;

impl AstNodePattern<GritQueryContext> for GritNodePattern {
    const INCLUDES_TRIVIA: bool = true;

    fn children(&self) -> Vec<PatternOrPredicate<GritQueryContext>> {
        todo!()
    }

    fn matches_kind_of(&self, _node: &GritTargetNode) -> bool {
        todo!()
    }
}

impl Matcher<GritQueryContext> for GritNodePattern {
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

impl PatternName for GritNodePattern {
    fn name(&self) -> &'static str {
        "GritNode"
    }
}

#[derive(Clone, Debug)]
pub(crate) struct GritLeafNodePattern;

impl AstLeafNodePattern<GritQueryContext> for GritLeafNodePattern {}

impl Matcher<GritQueryContext> for GritLeafNodePattern {
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

impl PatternName for GritLeafNodePattern {
    fn name(&self) -> &'static str {
        "GritLeafNode"
    }
}
