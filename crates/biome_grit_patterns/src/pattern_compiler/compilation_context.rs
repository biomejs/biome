use biome_rowan::TextRange;
use grit_pattern_matcher::pattern::VariableSourceLocations;

use crate::{diagnostics::CompilerDiagnostic, grit_target_language::GritTargetLanguage};
use std::{collections::BTreeMap, path::Path};

pub(crate) struct CompilationContext<'a> {
    /// Path of the source file being compiled.
    pub source_path: Option<&'a Path>,

    /// The target language being matched on.
    pub lang: GritTargetLanguage,

    pub pattern_definition_info: BTreeMap<String, DefinitionInfo>,
    pub predicate_definition_info: BTreeMap<String, DefinitionInfo>,
    pub function_definition_info: BTreeMap<String, DefinitionInfo>,
}

impl<'a> CompilationContext<'a> {
    pub(crate) fn new(source_path: Option<&'a Path>, lang: GritTargetLanguage) -> Self {
        Self {
            source_path,
            lang,
            pattern_definition_info: Default::default(),
            predicate_definition_info: Default::default(),
            function_definition_info: Default::default(),
        }
    }
}

pub(crate) struct NodeCompilationContext<'a> {
    pub compilation: &'a CompilationContext<'a>,

    /// Used to lookup local variables in the `vars_array`.
    pub vars: &'a mut BTreeMap<String, usize>,

    /// Storage for variable information.
    ///
    /// The outer vector can be index using `scope_index`, while the individual
    /// variables in a scope can be indexed using the indices stored in `vars`
    /// and `global_vars`.
    pub vars_array: &'a mut Vec<Vec<VariableSourceLocations>>,

    /// Index of the local scope.
    ///
    /// Corresponds to the index in the outer vector of `vars_array`.
    pub scope_index: usize,

    /// Used to lookup global variables in the `vars_array`.
    ///
    /// Global variables are always at scope 0.
    pub global_vars: &'a mut BTreeMap<String, usize>,

    /// Diagnostics discovered during compilation.
    pub diagnostics: &'a mut Vec<CompilerDiagnostic>,
}

impl<'a> NodeCompilationContext<'a> {
    pub(crate) fn new(
        compilation_context: &'a CompilationContext,
        vars: &'a mut BTreeMap<String, usize>,
        vars_array: &'a mut Vec<Vec<VariableSourceLocations>>,
        global_vars: &'a mut BTreeMap<String, usize>,
        diagnostics: &'a mut Vec<CompilerDiagnostic>,
    ) -> Self {
        Self {
            compilation: compilation_context,
            vars,
            vars_array,
            scope_index: 0,
            global_vars,
            diagnostics,
        }
    }

    pub(crate) fn log(&mut self, diagnostic: CompilerDiagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

pub(crate) struct DefinitionInfo {
    pub(crate) index: usize,
    pub(crate) parameters: Vec<(String, TextRange)>,
}
