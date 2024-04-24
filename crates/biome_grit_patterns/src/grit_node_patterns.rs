use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_node::GritNode;
use crate::resolved_pattern::GritResolvedPattern;
use anyhow::Result;
use grit_core_patterns::pattern::ast_node_pattern::{AstLeafNodePattern, AstNodePattern};
use grit_core_patterns::pattern::iter_pattern::PatternOrPredicate;
use grit_core_patterns::pattern::patterns::{Matcher, PatternName};
use grit_core_patterns::pattern::state::State;
use grit_util::AnalysisLogs;

#[derive(Clone, Debug)]
pub(crate) struct GritNodePattern;

impl AstNodePattern<GritQueryContext> for GritNodePattern {
    fn children(&self) -> Vec<PatternOrPredicate<GritQueryContext>> {
        todo!()
    }

    fn matches_kind_of(&self, _node: &GritNode) -> bool {
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
