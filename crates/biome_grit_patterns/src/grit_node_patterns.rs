use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::LeafEquivalenceClass;
use crate::grit_target_node::{GritTargetNode, GritTargetSyntaxKind};
use crate::{CompileError, GritTargetLanguage};
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::context::{ExecContext, StaticDefinitions};
use grit_pattern_matcher::pattern::{
    AstLeafNodePattern, AstNodePattern, Matcher, Pattern, PatternName, PatternOrPredicate,
    ResolvedPattern, State, Variable,
};
use grit_util::error::GritResult;
use grit_util::{AnalysisLogs, ByteRange, Language};
use regex::Regex;

/// Check if two syntax kinds are compatible for import pattern matching
fn are_import_kinds_compatible(
    pattern_kind: GritTargetSyntaxKind,
    node_kind: GritTargetSyntaxKind,
) -> bool {
    let Some(pattern_js_kind) = pattern_kind.as_js_kind() else {
        return false;
    };
    let Some(node_js_kind) = node_kind.as_js_kind() else {
        return false;
    };

    use biome_js_syntax::JsSyntaxKind::*;

    // Only allow specific import transformations to avoid over-matching
    match (pattern_js_kind, node_js_kind) {
        // Default import pattern can match named import
        (JS_IMPORT_DEFAULT_CLAUSE, JS_IMPORT_NAMED_CLAUSE) => true,
        // Named import pattern can match default import
        (JS_IMPORT_NAMED_CLAUSE, JS_IMPORT_DEFAULT_CLAUSE) => true,
        // Default import specifier can match individual named specifiers
        (JS_DEFAULT_IMPORT_SPECIFIER, JS_SHORTHAND_NAMED_IMPORT_SPECIFIER) => true,
        (JS_DEFAULT_IMPORT_SPECIFIER, JS_NAMED_IMPORT_SPECIFIER) => true,
        (JS_DEFAULT_IMPORT_SPECIFIER, JS_NAMESPACE_IMPORT_SPECIFIER) => true,
        // Named specifiers can match default specifier
        (JS_SHORTHAND_NAMED_IMPORT_SPECIFIER, JS_DEFAULT_IMPORT_SPECIFIER) => true,
        (JS_NAMED_IMPORT_SPECIFIER, JS_DEFAULT_IMPORT_SPECIFIER) => true,
        (JS_NAMESPACE_IMPORT_SPECIFIER, JS_DEFAULT_IMPORT_SPECIFIER) => true,
        // Import clauses can match each other
        (JS_IMPORT_DEFAULT_CLAUSE, JS_IMPORT_DEFAULT_CLAUSE) => true,
        (JS_IMPORT_NAMED_CLAUSE, JS_IMPORT_NAMED_CLAUSE) => true,
        (JS_IMPORT_NAMESPACE_CLAUSE, JS_IMPORT_NAMESPACE_CLAUSE) => true,
        (JS_IMPORT_COMBINED_CLAUSE, JS_IMPORT_COMBINED_CLAUSE) => true,
        (JS_IMPORT_BARE_CLAUSE, JS_IMPORT_BARE_CLAUSE) => true,
        _ => false,
    }
}

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
    ) -> Vec<PatternOrPredicate<'_, GritQueryContext>> {
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

        // Check if we need to handle import structure transformation
        let needs_import_transformation = self.is_import_transformation_needed(node.kind());

        if node.kind() != self.kind
            && !are_import_kinds_compatible(self.kind, node.kind())
            && !needs_import_transformation
        {
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

            // Get child binding with import transformation if needed
            let child_binding = if needs_import_transformation {
                use biome_js_syntax::JsSyntaxKind::*;
                let Some(pattern_js_kind) = self.kind.as_js_kind() else {
                    return Ok(false);
                };
                let Some(node_js_kind) = node.kind().as_js_kind() else {
                    return Ok(false);
                };

                // Helper function
                let get_child = |source: u32| {
                    node.child_by_slot_index(source).map_or(
                        GritResolvedPattern::from_empty_binding(node.clone(), *slot_index),
                        GritResolvedPattern::from_node_binding,
                    )
                };

                // Import slot mapping table
                let source_slot = match (pattern_js_kind, node_js_kind, slot_index) {
                    // type_token, phase_token
                    (JS_IMPORT_DEFAULT_CLAUSE, JS_IMPORT_NAMED_CLAUSE, 0 | 1) => None,
                    // default_specifier -> named_specifiers
                    (JS_IMPORT_DEFAULT_CLAUSE, JS_IMPORT_NAMED_CLAUSE, 2) => Some(1),
                    // from_token, source, assertion
                    (JS_IMPORT_DEFAULT_CLAUSE, JS_IMPORT_NAMED_CLAUSE, 3..=5) => {
                        Some(slot_index - 1)
                    }
                    // type_token
                    (JS_IMPORT_NAMED_CLAUSE, JS_IMPORT_DEFAULT_CLAUSE, 0) => Some(0),
                    // named_specifiers -> default specifier
                    (JS_IMPORT_NAMED_CLAUSE, JS_IMPORT_DEFAULT_CLAUSE, 1) => Some(2),
                    // from_token, source, assertion
                    (JS_IMPORT_NAMED_CLAUSE, JS_IMPORT_DEFAULT_CLAUSE, 2..=4) => {
                        Some(slot_index + 1)
                    }
                    // normal case
                    _ => Some(*slot_index),
                };

                match source_slot {
                    None => GritResolvedPattern::from_empty_binding(node.clone(), *slot_index),
                    Some(source) => get_child(source),
                }
            } else {
                match node.child_by_slot_index(*slot_index) {
                    Some(child) => GritResolvedPattern::from_node_binding(child),
                    None => GritResolvedPattern::from_empty_binding(node.clone(), *slot_index),
                }
            };

            let res = pattern.execute(&child_binding, &mut cur_state, context, logs);
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

impl GritNodePattern {
    /// Check if this pattern needs import structure transformation
    fn is_import_transformation_needed(&self, node_kind: GritTargetSyntaxKind) -> bool {
        use biome_js_syntax::JsSyntaxKind::*;

        let Some(pattern_js_kind) = self.kind.as_js_kind() else {
            return false;
        };
        let Some(node_js_kind) = node_kind.as_js_kind() else {
            return false;
        };

        matches!(
            (pattern_js_kind, node_js_kind),
            (JS_IMPORT_DEFAULT_CLAUSE, JS_IMPORT_NAMED_CLAUSE)
                | (JS_IMPORT_NAMED_CLAUSE, JS_IMPORT_DEFAULT_CLAUSE)
        )
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

/// Matches a leaf node whose snippet text embeds metavariables, such as the
/// string literal in `` `console.log("Hello, $name")` ``.
///
/// The whole node text must match `regex`. Each capture group binds the
/// variable at the same position in `variables`, and a variable that is
/// already bound must have the same text as its capture.
#[derive(Clone, Debug)]
pub struct GritRegexLeafNodePattern {
    regex: Regex,
    variables: Vec<Variable>,
}

impl GritRegexLeafNodePattern {
    /// Returns `None` if `regex` is not a valid regular expression.
    pub(crate) fn new(regex: &str, variables: Vec<Variable>) -> Option<Self> {
        // Compiled once here because the pattern runs for every candidate node.
        let regex = Regex::new(&format!("^{regex}$")).ok()?;
        Some(Self { regex, variables })
    }
}

impl Matcher<GritQueryContext> for GritRegexLeafNodePattern {
    fn execute<'a>(
        &'a self,
        binding: &GritResolvedPattern<'a>,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let language = context.language();
        let text = binding.text(&state.files, language)?;
        let Some(captures) = self.regex.captures(&text) else {
            return Ok(false);
        };

        let last_binding = binding.get_last_binding();
        for (variable, capture) in self.variables.iter().zip(captures.iter().skip(1)) {
            let Some(capture) = capture else {
                return Ok(false);
            };
            let scope = variable.try_scope()? as usize;
            let index = variable.try_index()? as usize;

            let content = &state.bindings[scope].last().unwrap()[index];
            if let Some(value) = &content.value {
                if value.text(&state.files, language)? != capture.as_str() {
                    return Ok(false);
                }
                continue;
            }
            let pattern = content.pattern;

            let value = match last_binding
                .and_then(|binding| Some((binding.range(language)?, binding.source()?)))
            {
                Some((range, source)) => GritResolvedPattern::from_range_binding(
                    ByteRange::new(range.start + capture.start(), range.start + capture.end()),
                    source,
                ),
                None => GritResolvedPattern::from_string(capture.as_str().to_owned()),
            };
            if let Some(pattern) = pattern
                && !pattern.execute(&value, state, context, logs)?
            {
                return Ok(false);
            }
            state.bindings[scope].last_mut().unwrap()[index].value = Some(value);
        }

        Ok(true)
    }
}

impl PatternName for GritRegexLeafNodePattern {
    fn name(&self) -> &'static str {
        "GritRegexLeafNode"
    }
}

/// Pattern for a leaf node inside a code snippet.
#[derive(Clone, Debug)]
pub enum GritLeafPattern {
    Text(GritLeafNodePattern),
    Regex(GritRegexLeafNodePattern),
}

impl AstLeafNodePattern<GritQueryContext> for GritLeafPattern {
    fn text(&self) -> Option<&str> {
        match self {
            Self::Text(pattern) => pattern.text(),
            Self::Regex(_) => None,
        }
    }
}

impl Matcher<GritQueryContext> for GritLeafPattern {
    fn execute<'a>(
        &'a self,
        binding: &GritResolvedPattern<'a>,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        match self {
            Self::Text(pattern) => pattern.execute(binding, state, context, logs),
            Self::Regex(pattern) => pattern.execute(binding, state, context, logs),
        }
    }
}

impl PatternName for GritLeafPattern {
    fn name(&self) -> &'static str {
        match self {
            Self::Text(pattern) => pattern.name(),
            Self::Regex(pattern) => pattern.name(),
        }
    }
}
