use crate::diagnostics::CompilerDiagnostic;
use crate::grit_context::{GritExecContext, GritQueryContext};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::GritTargetLanguage;
use crate::grit_tree::GritTargetTree;
use crate::pattern_compiler::PatternCompiler;
use crate::pattern_compiler::{
    compilation_context::CompilationContext, compilation_context::NodeCompilationContext,
};
use crate::variables::{VarRegistry, VariableLocations};
use crate::CompileError;
use anyhow::Result;
use biome_grit_syntax::{GritRoot, GritRootExt};
use grit_pattern_matcher::effects::Effect;
use grit_pattern_matcher::pattern::{FileRegistry, Matcher, Pattern, State};
use im::Vector;
use std::collections::BTreeMap;

/// Represents a top-level Grit query.
///
/// Grit queries provide the
pub struct GritQuery {
    pub pattern: Pattern<GritQueryContext>,

    /// Diagnostics discovered during compilation of the query.
    pub diagnostics: Vec<CompilerDiagnostic>,

    /// Context for executing the query.
    context: GritExecContext,

    /// All variables discovered during query compilation.
    variables: VariableLocations,
}

impl GritQuery {
    pub fn execute<'a>(
        &'a self,
        tree: &'a GritTargetTree,
    ) -> Result<Vector<Effect<'a, GritQueryContext>>> {
        let var_registry = VarRegistry::from_locations(&self.variables);

        let binding = GritResolvedPattern::from_tree(tree);
        let mut state = State::new(
            var_registry.into(),
            FileRegistry::new_from_paths(Vec::new()),
        );
        let mut logs = Vec::new().into();

        self.pattern
            .execute(&binding, &mut state, &self.context, &mut logs)?;

        Ok(state.effects)
    }

    pub fn from_node(root: GritRoot, lang: GritTargetLanguage) -> Result<Self, CompileError> {
        let context = CompilationContext::new_anonymous(lang);

        let mut vars_array = Vec::new();
        let mut global_vars = BTreeMap::new();
        let mut diagnostics = Vec::new();

        // We're not in a local scope yet, so this map is kinda useless.
        // It's just there because all node compilers expect one.
        let mut vars = BTreeMap::new();

        let mut node_context = NodeCompilationContext::new(
            &context,
            &mut vars,
            &mut vars_array,
            &mut global_vars,
            &mut diagnostics,
        );

        let pattern = PatternCompiler::from_node(
            &root.pattern().ok_or(CompileError::MissingPattern)?,
            &mut node_context,
        )?;

        let context = GritExecContext::new(context.lang);
        let locations = VariableLocations::new(vars_array);

        Ok(Self {
            pattern,
            context,
            diagnostics,
            variables: locations,
        })
    }
}
