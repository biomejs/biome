use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_node::{GritTargetNode, GritTargetSyntaxKind};
use anyhow::Result;
use grit_pattern_matcher::pattern::{
    AstLeafNodePattern, AstNodePattern, Matcher, Pattern, PatternName, PatternOrPredicate, State,
};
use grit_util::AnalysisLogs;

#[derive(Clone, Debug)]
pub(crate) struct GritNodePattern {
    pub kind: GritTargetSyntaxKind,
    pub args: Vec<GritNodeArg>,
}

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
pub(crate) struct GritNodeArg {
    slot_index: usize,
    pattern: Pattern<GritQueryContext>,
}

impl GritNodeArg {
    pub fn new(slot_index: usize, pattern: Pattern<GritQueryContext>) -> Self {
        Self {
            slot_index,
            pattern,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct GritLeafNodePattern {
    kind: GritTargetSyntaxKind,
    text: String,
}

impl GritLeafNodePattern {
    pub fn new(kind: GritTargetSyntaxKind, text: impl Into<String>) -> Self {
        Self {
            kind,
            text: text.into(),
        }
    }
}

impl AstLeafNodePattern<GritQueryContext> for GritLeafNodePattern {
    fn text(&self) -> Option<&str> {
        Some(&self.text)
    }
}

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
