use crate::diagnostics::CompilerDiagnostic;
use crate::grit_built_in_functions::BuiltIns;
use crate::grit_context::{GritExecContext, GritQueryContext, GritTargetFile};
use crate::grit_definitions::{
    Definitions, ScannedDefinitionInfo, compile_definitions, scan_definitions,
};
use crate::grit_file::GritFile;
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::GritTargetLanguage;
use crate::grit_target_node::GritTargetSyntaxKind;
use crate::grit_tree::GritTargetTree;
use crate::pattern_compiler::{PatternCompiler, auto_wrap_pattern};
use crate::pattern_compiler::{
    compilation_context::CompilationContext, compilation_context::NodeCompilationContext,
};
use crate::variables::{VarRegistry, VariableLocations};
use crate::{BuiltInFunction, CompileError};
use biome_analyze::RuleDiagnostic;
use biome_grit_syntax::{GritRoot, GritRootExt};
use camino::Utf8Path;
use grit_pattern_matcher::constants::{
    ABSOLUTE_PATH_INDEX, FILENAME_INDEX, GLOBAL_VARS_SCOPE_INDEX, NEW_FILES_INDEX, PROGRAM_INDEX,
};
use grit_pattern_matcher::context::ExecContext;
use grit_pattern_matcher::file_owners::{FileOwner, FileOwners};
use grit_pattern_matcher::pattern::{
    File as GritFileTrait, FilePtr, FileRegistry, Matcher, Pattern, Predicate, ResolvedPattern,
    State, VariableSource,
};
use grit_util::error::{GritPatternError, GritResult};
use grit_util::{AnalysisLogs, Ast, ByteRange, InputRanges, Range, VariableMatch};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

// These need to remain ordered by index.
const GLOBAL_VARS: [(&str, usize); 4] = [
    ("$new_files", NEW_FILES_INDEX),
    ("$program", PROGRAM_INDEX),
    ("$filename", FILENAME_INDEX),
    ("$absolute_filename", ABSOLUTE_PATH_INDEX),
];

/// Represents a top-level Grit query.
///
/// Grit queries provide the
#[derive(Debug)]
pub struct GritQuery {
    pub pattern: Pattern<GritQueryContext>,

    /// Definitions for named patterns, predicates and functions.
    pub definitions: Definitions,

    /// Diagnostics discovered during compilation of the query.
    pub diagnostics: Vec<CompilerDiagnostic>,

    /// The name of the snippet being executed.
    pub name: Option<String>,

    /// Target language for the query.
    pub language: GritTargetLanguage,

    /// Built-in functions available to the query.
    built_ins: BuiltIns,

    /// All variables discovered during query compilation.
    variable_locations: VariableLocations,
}

impl GritQuery {
    fn make_exec_context<'a>(
        &'a self,
        files: &'a [GritTargetFile],
        file_owners: &'a FileOwners<GritTargetTree>,
    ) -> GritExecContext<'a> {
        GritExecContext {
            lang: self.language.clone(),
            name: self.name.as_deref(),
            loadable_files: files,
            files: file_owners,
            built_ins: &self.built_ins,
            functions: &self.definitions.functions,
            patterns: &self.definitions.patterns,
            predicates: &self.definitions.predicates,
            diagnostics: Mutex::new(Vec::new()),
        }
    }

    fn make_initial_state<'a>(&self, files: &'a [GritTargetFile]) -> State<'a, GritQueryContext> {
        let var_registry = VarRegistry::from_locations(&self.variable_locations);
        let paths: Vec<_> = files.iter().map(|file| file.path.as_std_path()).collect();
        let file_registry = FileRegistry::new_from_paths(paths);
        State::new(var_registry.into(), file_registry)
    }

    pub fn execute(&self, file: GritTargetFile) -> GritResult<GritQueryResult> {
        let file_owners = FileOwners::new();
        let files = vec![file];
        let file_ptr = FilePtr::new(0, 0);
        let context = self.make_exec_context(&files, &file_owners);
        let mut state = self.make_initial_state(&files);
        let binding = FilePattern::Single(file_ptr);

        let mut logs = Vec::new().into();

        let mut effects: Vec<GritQueryEffect> = Vec::new();
        if self
            .pattern
            .execute(&binding.into(), &mut state, &context, &mut logs)?
        {
            for file in state.files.files() {
                if let Some(effect) = GritQueryEffect::from_file(file)? {
                    effects.push(effect)
                }
            }
        }

        Ok(GritQueryResult {
            effects,
            diagnostics: context.into_diagnostics(),
            logs,
        })
    }

    /// Returns the syntax kinds that this query's pattern targets.
    ///
    /// Extracts kinds from the inner CodeSnippet or AstNode patterns
    /// by navigating the compiled pattern tree. Returns an empty vec
    /// if the pattern structure can't be analyzed.
    pub fn anchor_kinds(&self) -> Vec<GritTargetSyntaxKind> {
        extract_anchor_kinds(&self.pattern)
    }

    /// Optimized execution that replaces the Contains full-tree walk
    /// with an anchor-kind-filtered walk.
    ///
    /// Instead of walking every node in the tree (what Contains does),
    /// this only executes the inner Bubble pattern at nodes matching
    /// the extracted anchor kinds.
    ///
    /// Falls back to `execute()` if anchor extraction fails.
    pub fn execute_optimized(&self, file: GritTargetFile) -> GritResult<GritQueryResult> {
        let anchor_kinds = self.anchor_kinds();
        let inner = extract_contains_inner(&self.pattern);

        let Some(inner) = inner else {
            return self.execute(file);
        };
        if anchor_kinds.is_empty() {
            return self.execute(file);
        }
        // Create tree independently of state to avoid borrow conflicts.
        // from_cached_parse_result wraps the existing parsed tree — O(1).
        let mut logs: AnalysisLogs = Vec::new().into();
        let tree = self.language.get_parser().from_cached_parse_result(
            &file.parse,
            Some(file.path.as_std_path()),
            &mut logs,
        );
        let Some(tree) = tree else {
            return self.execute(file);
        };

        // Collect anchor-kind nodes from the independent tree.
        // Use Vec::contains — anchor_kinds is tiny (1-3 items), faster than hashing.
        let root = tree.root_node();
        let anchor_nodes: Vec<_> = root
            .descendants()
            .filter(|node| anchor_kinds.contains(&node.kind()))
            .collect();

        // Set up context and state (same as execute).
        let file_owners = FileOwners::new();
        let files = vec![file];
        let file_ptr = FilePtr::new(0, 0);
        let context = self.make_exec_context(&files, &file_owners);
        let mut state = self.make_initial_state(&files);

        // Load file (creates FileOwner in file_owners, loads into state.files).
        let grit_file = GritFile::Ptr(file_ptr);
        context.load_file(&grit_file, &mut state, &mut logs)?;

        // Replicate the global-variable binding from `FilePattern::execute` in
        // `grit-pattern-matcher` (crate `grit-pattern-matcher`, module `pattern/file_pattern.rs`).
        // If the upstream binding logic changes, this block must be updated to match.
        let name_val = grit_file.name(&state.files);
        let program_val = grit_file.binding(&state.files);
        let abs_path_val = grit_file.absolute_path(&state.files, &context.lang)?;

        state.bindings[GLOBAL_VARS_SCOPE_INDEX as usize]
            .last_mut()
            .unwrap()[FILENAME_INDEX]
            .value = Some(name_val);
        state.bindings[GLOBAL_VARS_SCOPE_INDEX as usize]
            .last_mut()
            .unwrap()[PROGRAM_INDEX]
            .value = Some(program_val);
        state.bindings[GLOBAL_VARS_SCOPE_INDEX as usize]
            .last_mut()
            .unwrap()[ABSOLUTE_PATH_INDEX]
            .value = Some(abs_path_val);
        state.bindings[GLOBAL_VARS_SCOPE_INDEX as usize]
            .last_mut()
            .unwrap()[NEW_FILES_INDEX]
            .value = Some(GritResolvedPattern::from_list_parts([].into_iter()));

        // Execute inner pattern (Bubble) at each anchor-kind node.
        let mut matched = false;
        for node in anchor_nodes {
            let binding = GritResolvedPattern::from_node_binding(node);
            let saved = state.clone();
            if inner.execute(&binding, &mut state, &context, &mut logs)? {
                matched = true;
            } else {
                state = saved;
            }
        }

        // Collect match ranges and set on file (replicate exec_step behavior).
        if matched {
            let (variables, ranges, suppressed) =
                state.bindings_history_to_ranges(&context.lang, context.name);
            let unique_ranges: Vec<_> = ranges
                .into_iter()
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect();
            let input_ranges = InputRanges {
                ranges: unique_ranges,
                variables,
                suppressed,
            };
            let file_owner = state.files.get_file_owner(file_ptr);
            file_owner.matches.borrow_mut().input_matches = Some(input_ranges);
        }

        // Collect effects.
        let mut effects = Vec::new();
        for file in state.files.files() {
            if let Some(effect) = GritQueryEffect::from_file(file)? {
                effects.push(effect);
            }
        }

        Ok(GritQueryResult {
            effects,
            diagnostics: context.into_diagnostics(),
            logs,
        })
    }

    pub fn from_node(
        root: GritRoot,
        source_path: Option<&Utf8Path>,
        lang: GritTargetLanguage,
        extra_built_ins: Vec<BuiltInFunction>,
    ) -> Result<Self, CompileError> {
        let ScannedDefinitionInfo {
            pattern_definition_info,
            predicate_definition_info,
            function_definition_info,
        } = scan_definitions(root.definitions())?;

        let mut built_ins = BuiltIns::default();
        for built_in in extra_built_ins {
            built_ins.add(built_in);
        }

        let context = CompilationContext {
            source_path,
            lang,
            built_ins: &built_ins,
            pattern_definition_info,
            predicate_definition_info,
            function_definition_info,
        };

        // Global variables are in scope 0, local pattern variables will be in scope 1.
        let mut vars_array = vec![
            GLOBAL_VARS
                .iter()
                .map(|global_var| VariableSource::Compiled {
                    name: global_var.0.to_string(),
                    file: source_path
                        .map(Utf8Path::to_path_buf)
                        .map_or_else(|| "unnamed".to_owned(), |p| p.to_string()),
                    locations: BTreeSet::new(),
                })
                .collect::<Vec<VariableSource>>(),
        ];
        let mut global_vars: BTreeMap<String, usize> = GLOBAL_VARS
            .iter()
            .map(|(global_var, index)| ((*global_var).to_string(), *index))
            .collect();
        let mut diagnostics = Vec::new();
        let mut vars = BTreeMap::new();

        let mut node_context = NodeCompilationContext::new(
            &context,
            &mut vars,
            &mut vars_array,
            &mut global_vars,
            &mut diagnostics,
        );

        let mut definitions = compile_definitions(root.definitions(), &mut node_context)?;

        let pattern = PatternCompiler::from_node(
            &root.pattern().ok_or(CompileError::MissingPattern)?,
            &mut node_context,
        )?;

        let pattern = auto_wrap_pattern(
            pattern,
            &mut definitions.patterns,
            true,
            None,
            &mut node_context,
            None,
        )?;

        let name = source_path
            .and_then(Utf8Path::file_stem)
            .map(|stem| stem.to_string());
        let language = context.lang;
        let variable_locations = VariableLocations::new(vars_array);

        Ok(Self {
            pattern,
            definitions,
            name,
            built_ins,
            language,
            diagnostics,
            variable_locations,
        })
    }
}

#[derive(Debug)]
pub struct GritQueryResult {
    pub effects: Vec<GritQueryEffect>,
    pub diagnostics: Vec<RuleDiagnostic>,
    pub logs: AnalysisLogs,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GritQueryEffect {
    Match(Match),
    Rewrite(Rewrite),
    CreateFile(CreateFile),
}

impl GritQueryEffect {
    pub fn from_file(file: &[&FileOwner<GritTargetTree>]) -> GritResult<Option<Self>> {
        if file.is_empty() {
            return Err(GritPatternError::new("cannot have file with no versions"));
        }

        let result = if file.len() == 1 {
            let file = file.last().unwrap();
            if file.new {
                Some(Self::CreateFile(CreateFile::new(
                    &file.name,
                    &file.tree.source(),
                )))
            } else if let Some(ranges) = &file.matches.borrow().input_matches {
                if ranges.suppressed {
                    None
                } else {
                    Some(Self::Match(Match::from_file_ranges(ranges, &file.name)))
                }
            } else {
                None
            }
        } else {
            Some(Self::Rewrite(Rewrite::from_file(
                file.first().unwrap(),
                file.last().unwrap(),
            )?))
        };

        Ok(result)
    }
}
enum FilePattern {
    Single(FilePtr),
    Many(Vec<FilePtr>),
}

impl From<FilePtr> for FilePattern {
    fn from(file: FilePtr) -> Self {
        Self::Single(file)
    }
}

impl From<Vec<FilePtr>> for FilePattern {
    fn from(files: Vec<FilePtr>) -> Self {
        Self::Many(files)
    }
}

impl From<FilePattern> for GritResolvedPattern<'_> {
    fn from(val: FilePattern) -> Self {
        match val {
            FilePattern::Single(file) => Self::from_file_pointer(file),
            FilePattern::Many(files) => Self::from_files(Self::from_list_parts(
                files.into_iter().map(Self::from_file_pointer),
            )),
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Match {
    pub messages: Vec<Message>,
    pub variables: Vec<VariableMatch>,
    pub source_file: PathBuf,
    pub ranges: Vec<Range>,
}

impl Match {
    fn from_file_ranges(match_ranges: &InputRanges, path: &Path) -> Self {
        Self {
            source_file: path.to_owned(),
            ranges: match_ranges.ranges.clone(),
            variables: match_ranges.variables.clone(),
            messages: vec![],
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rewrite {
    pub original: Match,
    pub rewritten: OutputFile,
}

impl From<Rewrite> for GritQueryEffect {
    fn from(value: Rewrite) -> Self {
        Self::Rewrite(value)
    }
}

impl Rewrite {
    fn new(original: Match, rewritten: OutputFile) -> Self {
        Self {
            original,
            rewritten,
        }
    }

    fn from_file(
        initial: &FileOwner<GritTargetTree>,
        rewritten_file: &FileOwner<GritTargetTree>,
    ) -> GritResult<Self> {
        let original = if let Some(ranges) = &initial.matches.borrow().input_matches {
            Match::from_file_ranges(ranges, &initial.name)
        } else {
            return Err(GritPatternError::new("cannot have rewrite without matches"));
        };
        let rewritten = OutputFile::from_file(rewritten_file);
        Ok(Self::new(original, rewritten))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateFile {
    pub rewritten: OutputFile,
    pub ranges: Option<Vec<Range>>,
}

impl From<CreateFile> for GritQueryEffect {
    fn from(value: CreateFile) -> Self {
        Self::CreateFile(value)
    }
}

impl CreateFile {
    fn new(path: &Path, body: &str) -> Self {
        Self {
            rewritten: OutputFile::new(path, body, None),
            ranges: None,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OutputFile {
    pub messages: Vec<Message>,
    pub variables: Vec<VariableMatch>,
    pub source_file: PathBuf,
    pub content: String,
    pub byte_ranges: Option<Vec<ByteRange>>,
}

impl OutputFile {
    fn new(name: &Path, body: &str, byte_range: Option<&[ByteRange]>) -> Self {
        Self {
            source_file: name.to_owned(),
            content: body.to_owned(),
            variables: Vec::new(),
            messages: Vec::new(),
            byte_ranges: byte_range.map(|range| range.to_vec()),
        }
    }

    fn from_file(file: &FileOwner<GritTargetTree>) -> Self {
        Self::new(
            &file.name,
            &file.tree.source(),
            file.matches.borrow().byte_ranges.as_deref(),
        )
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Message {
    pub message: String,
    pub range: Vec<Range>,
    pub variable_runtime_id: String,
}

/// Extracts the syntax kinds that a pattern targets by navigating
/// the auto-wrapped pattern tree.
///
/// The auto-wrap chain is:
///   Sequential → Step → [And →] File → Contains → Bubble → Where → Predicate::Match → inner
///
/// Returns an empty vec (triggering fallback to full execute) when
/// the pattern structure can't be statically analyzed.
fn extract_anchor_kinds(pattern: &Pattern<GritQueryContext>) -> Vec<GritTargetSyntaxKind> {
    match pattern {
        Pattern::Sequential(seq) => seq
            .iter()
            .flat_map(|step| extract_anchor_kinds(&step.pattern))
            .collect(),
        Pattern::File(file) => extract_anchor_kinds(&file.body),
        Pattern::Contains(contains) => {
            if contains.until.is_some() {
                return vec![];
            }
            extract_anchor_kinds(&contains.contains)
        }
        Pattern::Bubble(bubble) => extract_anchor_kinds(bubble.pattern_def.pattern()),
        Pattern::Where(where_pat) => {
            let mut kinds = extract_anchor_kinds(&where_pat.pattern);
            if kinds.is_empty() {
                kinds = extract_anchor_kinds_from_predicate(&where_pat.side_condition);
            }
            kinds
        }
        // NOTE: collects kinds from ALL And branches. extract_contains_inner
        // uses find_map (returns the first Contains only), so when an And has
        // multiple Contains branches with different kind sets, we may run the
        // inner pattern on extra nodes. This is harmless — the inner pattern
        // simply won't match — but causes unnecessary evaluations.
        Pattern::And(and) => and.patterns.iter().flat_map(extract_anchor_kinds).collect(),
        // For Or/Any: if ANY branch is universal (returns []), the whole
        // pattern is universal — we can't restrict to specific kinds.
        Pattern::Or(or) => {
            let all: Vec<_> = or.patterns.iter().map(extract_anchor_kinds).collect();
            if all.iter().any(|kinds| kinds.is_empty()) {
                return vec![];
            }
            all.into_iter().flatten().collect()
        }
        Pattern::Any(any) => {
            let all: Vec<_> = any.patterns.iter().map(extract_anchor_kinds).collect();
            if all.iter().any(|kinds| kinds.is_empty()) {
                return vec![];
            }
            all.into_iter().flatten().collect()
        }
        // Not: extracting anchors from the negated pattern is wrong.
        // `not { X }` succeeds on nodes where X does NOT match, so
        // restricting to X's kinds would miss valid matches.
        Pattern::Not(_) => vec![],
        Pattern::Rewrite(rw) => extract_anchor_kinds(&rw.left),
        Pattern::Maybe(maybe) => extract_anchor_kinds(&maybe.pattern),
        Pattern::Limit(limit) => extract_anchor_kinds(&limit.pattern),
        Pattern::CodeSnippet(snippet) => snippet.patterns.iter().map(|(kind, _)| *kind).collect(),
        Pattern::AstNode(node) => vec![node.kind],
        _ => vec![],
    }
}

/// Extracts anchor kinds from a predicate expression.
fn extract_anchor_kinds_from_predicate(
    predicate: &Predicate<GritQueryContext>,
) -> Vec<GritTargetSyntaxKind> {
    match predicate {
        Predicate::Match(m) => {
            if let Some(pattern) = &m.pattern {
                extract_anchor_kinds(pattern)
            } else {
                vec![]
            }
        }
        Predicate::And(a) => a
            .predicates
            .iter()
            .flat_map(extract_anchor_kinds_from_predicate)
            .collect(),
        Predicate::Or(o) => {
            let all: Vec<_> = o
                .predicates
                .iter()
                .map(extract_anchor_kinds_from_predicate)
                .collect();
            if all.iter().any(|kinds| kinds.is_empty()) {
                return vec![];
            }
            all.into_iter().flatten().collect()
        }
        _ => vec![],
    }
}

/// Navigates the auto-wrapped pattern tree to find the inner pattern
/// of the Contains node (the Bubble pattern).
///
/// Returns None if the pattern structure doesn't match the expected
/// auto-wrap chain.
///
/// Note: only inspects the first step of Sequential, matching the
/// auto-wrap structure where Contains is always in the first step.
fn extract_contains_inner(
    pattern: &Pattern<GritQueryContext>,
) -> Option<&Pattern<GritQueryContext>> {
    match pattern {
        Pattern::Sequential(seq) => seq
            .first()
            .and_then(|step| extract_contains_inner(&step.pattern)),
        Pattern::File(file) => extract_contains_inner(&file.body),
        Pattern::Contains(contains) => {
            if contains.until.is_some() {
                return None;
            }
            Some(&contains.contains)
        }
        // NOTE: returns only the first Contains found. extract_anchor_kinds
        // collects from ALL And branches, so there may be an asymmetry when
        // multiple Contains exist. See the matching NOTE there.
        Pattern::And(and) => and.patterns.iter().find_map(extract_contains_inner),
        Pattern::Limit(limit) => extract_contains_inner(&limit.pattern),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{compile_js_query, make_js_file};

    // -- extract_anchor_kinds tests --

    #[test]
    fn anchor_kinds_extracts_from_code_snippet() {
        let query = compile_js_query("`console.log($msg)`");
        let kinds = query.anchor_kinds();
        assert!(
            !kinds.is_empty(),
            "code snippet should produce anchor kinds"
        );
    }

    #[test]
    fn anchor_kinds_returns_empty_for_metavariable() {
        // A bare metavariable like `$x` matches any node — universal.
        let query = compile_js_query("$x");
        let kinds = query.anchor_kinds();
        assert!(
            kinds.is_empty(),
            "bare metavariable is universal, expected empty anchor kinds"
        );
    }

    #[test]
    fn anchor_kinds_returns_empty_for_not() {
        // `not` negation: can't restrict to specific kinds.
        let query = compile_js_query("not `console.log($msg)`");
        let kinds = query.anchor_kinds();
        assert!(
            kinds.is_empty(),
            "Not patterns should return empty anchor kinds"
        );
    }

    #[test]
    fn anchor_kinds_or_with_universal_branch_returns_empty() {
        // If any Or branch is universal (metavariable), the whole Or is universal.
        let query = compile_js_query("or { `console.log($x)`, $y }");
        let kinds = query.anchor_kinds();
        assert!(
            kinds.is_empty(),
            "Or with universal branch should return empty anchor kinds"
        );
    }

    #[test]
    fn anchor_kinds_or_with_all_specific_branches() {
        let query = compile_js_query("or { `console.log($x)`, `console.warn($x)` }");
        let kinds = query.anchor_kinds();
        assert!(
            !kinds.is_empty(),
            "Or with all specific branches should return anchor kinds"
        );
    }

    #[test]
    fn anchor_kinds_where_clause_extracts_from_match() {
        let query = compile_js_query("`console.log($msg)` where { $msg <: `\"hello\"` }");
        let kinds = query.anchor_kinds();
        assert!(
            !kinds.is_empty(),
            "where clause with code snippet should produce anchor kinds"
        );
    }

    // -- extract_contains_inner tests --

    #[test]
    fn contains_inner_found_for_code_snippet() {
        let query = compile_js_query("`console.log($msg)`");
        let inner = extract_contains_inner(&query.pattern);
        assert!(
            inner.is_some(),
            "should find inner pattern in auto-wrapped Contains"
        );
    }

    #[test]
    fn contains_inner_found_for_where_pattern() {
        let query = compile_js_query("`console.log($msg)` where { $msg <: `\"test\"` }");
        let inner = extract_contains_inner(&query.pattern);
        assert!(
            inner.is_some(),
            "should find inner pattern for where-clause patterns"
        );
    }

    // -- execute_optimized equivalence tests --

    #[test]
    fn execute_optimized_matches_execute_for_simple_pattern() {
        let query = compile_js_query("`console.log($msg)`");
        let code = r#"
            console.log("hello");
            const x = 42;
            console.log("world");
        "#;

        let opt_result = query
            .execute_optimized(make_js_file(code))
            .expect("optimized failed");
        let full_result = query.execute(make_js_file(code)).expect("execute failed");

        assert_eq!(
            opt_result.effects, full_result.effects,
            "optimized and full execute should produce identical effects"
        );
    }

    #[test]
    fn execute_optimized_matches_execute_for_where_clause() {
        let query = compile_js_query(r#"`console.log($msg)` where { $msg <: `"hello"` }"#);
        let code = r#"
            console.log("hello");
            console.log("world");
        "#;

        let opt_result = query
            .execute_optimized(make_js_file(code))
            .expect("optimized failed");
        let full_result = query.execute(make_js_file(code)).expect("execute failed");

        assert_eq!(
            opt_result.effects, full_result.effects,
            "where-clause: optimized and full should produce identical effects"
        );
    }

    #[test]
    fn execute_optimized_falls_back_for_non_optimizable() {
        // Bare metavariable — anchor_kinds() returns empty, should fallback.
        let query = compile_js_query("$x");
        let code = "const x = 1;";

        let result = query
            .execute_optimized(make_js_file(code))
            .expect("fallback execution failed");

        // Should still produce results via fallback.
        assert!(
            result.diagnostics.is_empty(),
            "fallback should not produce errors"
        );
    }

    #[test]
    fn execute_optimized_no_matches_when_pattern_absent() {
        let query = compile_js_query("`console.log($msg)`");
        let code = "const x = 42;";

        let opt_result = query
            .execute_optimized(make_js_file(code))
            .expect("optimized failed");
        let full_result = query.execute(make_js_file(code)).expect("execute failed");

        assert_eq!(
            opt_result.effects, full_result.effects,
            "no-match: optimized and full should produce identical effects"
        );
    }
}
