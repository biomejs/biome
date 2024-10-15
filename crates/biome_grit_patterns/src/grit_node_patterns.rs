use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::LeafEquivalenceClass;
use crate::grit_target_node::{GritTargetNode, GritTargetSyntaxKind};
use crate::{CompileError, GritTargetLanguage};
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::context::{ExecContext, StaticDefinitions};
use grit_pattern_matcher::pattern::{
    AstLeafNodePattern, AstNodePattern, Matcher, Pattern, PatternName, PatternOrPredicate,
    ResolvedPattern, State,
};
use grit_util::error::GritResult;
use grit_util::{AnalysisLogs, Language};

#[derive(Clone, Debug)]
pub struct GritNodePattern {
    pub kind: GritTargetSyntaxKind,
    pub args: Vec<GritNodePatternArg>,
}

impl AstNodePattern<GritQueryContext> for GritNodePattern {
    const INCLUDES_TRIVIA: bool = true;

    fn children(
        &self,
        _definitions: &StaticDefinitions<GritQueryContext>,
    ) -> Vec<PatternOrPredicate<GritQueryContext>> {
        self.args
            .iter()
            .map(|arg| PatternOrPredicate::Pattern(&arg.pattern))
            .collect()
    }

    fn matches_kind_of(&self, node: &GritTargetNode) -> bool {
        self.kind == node.kind()
    }
}

impl Matcher<GritQueryContext> for GritNodePattern {
    fn execute<'a>(
        &'a self,
        binding: &GritResolvedPattern<'a>,
        init_state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let Some(binding) = binding.get_last_binding() else {
            return Ok(false);
        };
        let Some(node) = binding.singleton() else {
            return Ok(false);
        };
        if binding.is_list() {
            return self.execute(
                &ResolvedPattern::from_node_binding(node),
                init_state,
                context,
                logs,
            );
        }

        if node.kind() != self.kind {
            return Ok(false);
        }
        if self.args.is_empty() {
            return Ok(true);
        }

        if context.language().is_comment_kind(self.kind) {
            let Some(range) = context.language().comment_text_range(&node) else {
                return Ok(false);
            };

            return self.args[0].pattern.execute(
                &ResolvedPattern::from_range_binding(range, node.text()),
                init_state,
                context,
                logs,
            );
        }

        let mut running_state = init_state.clone();
        for GritNodePatternArg {
            pattern,
            slot_index,
        } in &self.args
        {
            let mut cur_state = running_state.clone();

            let res = pattern.execute(
                &match node.child_by_slot_index(*slot_index) {
                    Some(child) => GritResolvedPattern::from_node_binding(child),
                    None => GritResolvedPattern::from_empty_binding(node.clone(), *slot_index),
                },
                &mut cur_state,
                context,
                logs,
            );
            if res? {
                running_state = cur_state;
            } else {
                return Ok(false);
            }
        }
        *init_state = running_state;
        Ok(true)
    }
}

impl PatternName for GritNodePattern {
    fn name(&self) -> &'static str {
        "GritNode"
    }
}

#[derive(Clone, Debug)]
pub struct GritNodePatternArg {
    pub slot_index: u32,
    pub pattern: Pattern<GritQueryContext>,
}

impl GritNodePatternArg {
    pub fn new(slot_index: u32, pattern: Pattern<GritQueryContext>) -> Self {
        Self {
            slot_index,
            pattern,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GritLeafNodePattern {
    kind: GritTargetSyntaxKind,
    equivalence_class: Option<LeafEquivalenceClass>,
    text: String,
}

impl GritLeafNodePattern {
    pub fn new(
        kind: GritTargetSyntaxKind,
        text: impl Into<String>,
        lang: &GritTargetLanguage,
    ) -> Result<Self, CompileError> {
        let text = text.into();
        let equivalence_class = lang.get_equivalence_class(kind, &text)?;
        Ok(Self {
            kind,
            equivalence_class,
            text,
        })
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
        binding: &GritResolvedPattern,
        _state: &mut State<'a, GritQueryContext>,
        _context: &'a GritExecContext,
        _logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let Some(node) = binding.get_last_binding().and_then(Binding::singleton) else {
            return Ok(false);
        };

        Ok(match &self.equivalence_class {
            Some(class) => class.are_equivalent(node.kind(), node.text()),
            None => node.kind() == self.kind && node.text() == self.text,
        })
    }
}

impl PatternName for GritLeafNodePattern {
    fn name(&self) -> &'static str {
        "GritLeafNode"
    }
}
