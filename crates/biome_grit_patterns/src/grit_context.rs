use crate::grit_binding::GritBinding;
use crate::grit_code_snippet::GritCodeSnippet;
use crate::grit_file::GritFile;
use crate::grit_node_patterns::{GritLeafNodePattern, GritNodePattern};
use crate::grit_target_language::GritTargetLanguage;
use crate::grit_target_node::GritTargetNode;
use crate::grit_tree::GritTree;
use crate::resolved_pattern::GritResolvedPattern;
use anyhow::Result;
use grit_pattern_matcher::context::{ExecContext, QueryContext};
use grit_pattern_matcher::file_owners::FileOwners;
use grit_pattern_matcher::pattern::{
    CallBuiltIn, GritFunctionDefinition, Pattern, PatternDefinition, PredicateDefinition, State,
};
use grit_util::AnalysisLogs;

#[derive(Clone, Debug)]
pub(crate) struct GritQueryContext;

impl QueryContext for GritQueryContext {
    type Node<'a> = GritTargetNode;
    type NodePattern = GritNodePattern;
    type LeafNodePattern = GritLeafNodePattern;
    type ExecContext<'a> = GritExecContext;
    type Binding<'a> = GritBinding;
    type CodeSnippet = GritCodeSnippet;
    type ResolvedPattern<'a> = GritResolvedPattern;
    type Language<'a> = GritTargetLanguage;
    type File<'a> = GritFile;
    type Tree<'a> = GritTree;
}

#[derive(Debug)]
pub(crate) struct GritExecContext;

impl<'a> ExecContext<'a, GritQueryContext> for GritExecContext {
    fn pattern_definitions(&self) -> &[PatternDefinition<GritQueryContext>] {
        todo!()
    }

    fn predicate_definitions(&self) -> &[PredicateDefinition<GritQueryContext>] {
        todo!()
    }

    fn function_definitions(&self) -> &[GritFunctionDefinition<GritQueryContext>] {
        todo!()
    }

    fn ignore_limit_pattern(&self) -> bool {
        todo!()
    }

    fn call_built_in(
        &self,
        _call: &'a CallBuiltIn<GritQueryContext>,
        _context: &'a Self,
        _state: &mut State<'a, GritQueryContext>,
        _logs: &mut AnalysisLogs,
    ) -> Result<GritResolvedPattern> {
        todo!()
    }

    fn files(&self) -> &FileOwners<GritTree> {
        todo!()
    }

    fn language(&self) -> &GritTargetLanguage {
        todo!()
    }

    fn exec_step(
        &'a self,
        _step: &'a Pattern<GritQueryContext>,
        _binding: &GritResolvedPattern,
        _state: &mut State<'a, GritQueryContext>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        todo!()
    }

    fn name(&self) -> Option<&str> {
        todo!()
    }

    fn load_file(
        &self,
        _file: &<GritQueryContext as QueryContext>::File<'a>,
        _state: &mut State<'a, GritQueryContext>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        todo!()
    }
}
