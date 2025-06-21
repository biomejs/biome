use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_node::GritTargetSyntaxKind;
use biome_js_syntax::JsSyntaxKind;
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::context::ExecContext;
use grit_pattern_matcher::pattern::{
    CodeSnippet, DynamicPattern, Matcher, Pattern, PatternName, ResolvedPattern, State,
};
use grit_util::AnalysisLogs;
use grit_util::error::GritResult;

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

/// Check if two syntax kinds are compatible for import pattern matching
fn are_import_kinds_compatible(pattern_kind: GritTargetSyntaxKind, node_kind: GritTargetSyntaxKind) -> bool {
    let Some(pattern_js_kind) = pattern_kind.as_js_kind() else {
        return false;
    };
    let Some(node_js_kind) = node_kind.as_js_kind() else {
        return false;
    };

    use JsSyntaxKind::*;
    
    // Define compatible import kinds - these should be able to match each other
    let import_kinds = [
        JS_IMPORT,
        JS_IMPORT_DEFAULT_CLAUSE,
        JS_IMPORT_NAMED_CLAUSE,
        JS_IMPORT_NAMESPACE_CLAUSE,
        JS_IMPORT_COMBINED_CLAUSE,
        JS_IMPORT_BARE_CLAUSE,
    ];
    
    // If both kinds are import-related, they are compatible
    import_kinds.contains(&pattern_js_kind) && import_kinds.contains(&node_js_kind)
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

        println!("DEBUG: GritCodeSnippet::execute");
        println!("  Node kind: {:?}", node.kind());
        println!("  Pattern kinds: {:?}", self.patterns.iter().map(|(k, _)| k).collect::<Vec<_>>());
        println!("  Source: {}", self.source);

        // First try exact kind match
        if let Some((_, pattern)) = self.patterns.iter().find(|(kind, _)| *kind == node.kind()) {
            println!("  Found exact kind match");
            return pattern.execute(resolved, state, context, logs);
        }
        
        // Then try import-compatible kind match
        if let Some((pattern_kind, pattern)) = self.patterns.iter().find(|(kind, _)| are_import_kinds_compatible(*kind, node.kind())) {
            println!("  Found import-compatible kind match: pattern={:?}, node={:?}", pattern_kind, node.kind());
            return pattern.execute(resolved, state, context, logs);
        }
        
        println!("  No match found");
        Ok(false)
    }
}

impl PatternName for GritCodeSnippet {
    fn name(&self) -> &'static str {
        "CodeSnippet"
    }
}
