use crate::diagnostics::CompilerDiagnostic;
use crate::grit_built_in_functions::BuiltIns;
use crate::grit_context::{GritExecContext, GritQueryContext, GritTargetFile, new_file_owner};
use crate::grit_definitions::{
    Definitions, ScannedDefinitionInfo, compile_definitions, scan_definitions,
};
use crate::grit_file::GritFile;
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::GritTargetLanguage;
use crate::grit_target_node::{
    GritSyntaxSlot, GritTargetLanguageNode, GritTargetNode, GritTargetSyntaxKind,
};
use crate::grit_tree::GritTargetTree;
use crate::linearization::apply_effects;
use crate::pattern_compiler::{PatternCompiler, auto_wrap_pattern};
use crate::pattern_compiler::{
    compilation_context::CompilationContext, compilation_context::NodeCompilationContext,
};
use crate::variables::{VarRegistry, VariableLocations};
use crate::{BuiltInFunction, CompileError};
use biome_analyze::RuleDiagnostic;
use biome_diagnostics::Applicability;
use biome_grit_syntax::{GritRoot, GritRootExt, GritSyntaxKind};
use biome_rowan::AstNode;
use biome_rowan::syntax::SyntaxElementKey;
use camino::Utf8Path;
use grit_pattern_matcher::constants::{
    ABSOLUTE_PATH_INDEX, FILENAME_INDEX, GLOBAL_VARS_SCOPE_INDEX, NEW_FILES_INDEX, PROGRAM_INDEX,
};
use grit_pattern_matcher::context::ExecContext;
use grit_pattern_matcher::file_owners::{FileOwner, FileOwners};
use grit_pattern_matcher::pattern::{
    Container, File as GritFileTrait, FilePtr, FileRegistry, Matcher, Pattern, Predicate,
    ResolvedPattern, State, Variable, VariableSource,
};
use grit_util::error::{GritPatternError, GritResult};
use grit_util::{AnalysisLogs, Ast, ByteRange, InputRanges, Range, VariableMatch};
use rustc_hash::FxHashSet;
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

    /// The pattern as compiled, before its automatic `contains` wrapping.
    ///
    /// Only present when the query was compiled as a node selector.
    selector_pattern: Option<Pattern<GritQueryContext>>,

    /// Cached syntax kinds targeted by the compiled pattern.
    anchor_kinds: Vec<GritTargetSyntaxKind>,

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
            node_selector: self.selector_pattern.is_some(),
        }
    }

    fn make_initial_state<'a>(&self, files: &'a [GritTargetFile]) -> State<'a, GritQueryContext> {
        let var_registry = VarRegistry::from_locations(&self.variable_locations);
        let paths: Vec<_> = files.iter().map(|file| file.path.as_std_path()).collect();
        let file_registry = FileRegistry::new_from_paths(paths);
        State::new(var_registry.into(), file_registry)
    }

    fn bind_global_variables<'a>(
        grit_file: &GritFile<'a>,
        state: &mut State<'a, GritQueryContext>,
        language: &GritTargetLanguage,
    ) -> GritResult<()> {
        let name_val = grit_file.name(&state.files);
        let program_val = grit_file.binding(&state.files);
        let abs_path_val = grit_file.absolute_path(&state.files, language)?;

        let globals = state.bindings[GLOBAL_VARS_SCOPE_INDEX as usize]
            .last_mut()
            .expect("the global variable scope must be initialized");
        globals[FILENAME_INDEX].value = Some(name_val);
        globals[PROGRAM_INDEX].value = Some(program_val);
        globals[ABSOLUTE_PATH_INDEX].value = Some(abs_path_val);
        globals[NEW_FILES_INDEX].value = Some(GritResolvedPattern::from_list_parts([].into_iter()));

        Ok(())
    }

    /// Returns the nodes selected by a selector query.
    ///
    /// The pattern is executed against every node and token in `file`, as
    /// written by the user. A pattern that matches a list selects each of the
    /// list's named children. Neither rewrites nor diagnostics are applied.
    ///
    /// # Errors
    ///
    /// Returns an error if the query was not compiled with
    /// [`CompilePatternOptions::as_node_selector()`](crate::CompilePatternOptions::as_node_selector).
    pub fn find_matches(&self, file: GritTargetFile) -> GritResult<Vec<GritTargetLanguageNode>> {
        let Some(selector_pattern) = &self.selector_pattern else {
            return Err(GritPatternError::new(
                "This Grit query was not compiled as a node selector.",
            ));
        };

        let file_owners = FileOwners::new();
        let files = vec![file];
        let file_ptr = FilePtr::new(0, 0);
        let context = self.make_exec_context(&files, &file_owners);
        let mut state = self.make_initial_state(&files);

        let mut logs: AnalysisLogs = Vec::new().into();
        let grit_file = GritFile::Ptr(file_ptr);
        context.load_file(&grit_file, &mut state, &mut logs)?;
        Self::bind_global_variables(&grit_file, &mut state, &self.language)?;

        let file_owner = state.files.get_file_owner(file_ptr);
        let candidates = selection_candidates(&file_owner.tree.root_node());
        let patterns = selector_patterns(selector_pattern);

        let mut matches = Vec::new();
        let mut match_keys = FxHashSet::default();
        for candidate in candidates {
            if !self.anchor_kinds.is_empty() && !self.anchor_kinds.contains(&candidate.kind()) {
                continue;
            }

            for pattern in &patterns {
                // Each attempt starts from the same state, so variables bound by
                // a failed attempt cannot affect later candidates.
                let mut candidate_state = state.clone();
                let binding = GritResolvedPattern::from_node_binding(candidate.clone());
                if pattern.execute(&binding, &mut candidate_state, &context, &mut logs)? {
                    if candidate.is_list() {
                        for child in candidate.named_children() {
                            push_unique(&mut matches, &mut match_keys, child.language_node());
                        }
                    } else {
                        push_unique(&mut matches, &mut match_keys, candidate.language_node());
                    }
                    break;
                }
            }
        }

        Ok(matches)
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
    /// Returns an empty slice if the pattern structure can't be statically
    /// analyzed.
    pub fn anchor_kinds(&self) -> &[GritTargetSyntaxKind] {
        &self.anchor_kinds
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

        let mut logs: AnalysisLogs = Vec::new().into();

        // Set up context and state (same as execute).
        let file_owners = FileOwners::new();
        let files = vec![file];
        let file_ptr = FilePtr::new(0, 0);
        let context = self.make_exec_context(&files, &file_owners);
        let mut state = self.make_initial_state(&files);

        // Load file (creates FileOwner in file_owners, loads into state.files).
        let grit_file = GritFile::Ptr(file_ptr);
        context.load_file(&grit_file, &mut state, &mut logs)?;

        // Collect anchor-kind nodes from the LOADED tree so that effect
        // bindings reference the same source allocation as the file owner.
        // `get_file_owner` returns `&'a FileOwner` (lifetime from file_owners,
        // not from &self on state), so anchor_nodes don't borrow state.
        let file_owner = state.files.get_file_owner(file_ptr);
        let root = file_owner.tree.root_node();
        let anchor_nodes: Vec<_> = root
            .descendants()
            .filter(|node| anchor_kinds.contains(&node.kind()))
            .collect();

        // Replicate the global-variable binding from `FilePattern::execute` in
        // `grit-pattern-matcher` (crate `grit-pattern-matcher`, module `pattern/file_pattern.rs`).
        // If the upstream binding logic changes, this block must be updated to match.
        Self::bind_global_variables(&grit_file, &mut state, &self.language)?;

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
            let mut match_log = file_owner.matches.borrow_mut();
            if match_log.input_matches.is_none() {
                match_log.input_matches = Some(input_ranges);
            }
        }

        // Apply effects: if there are accumulated effects, linearize
        // them to produce rewritten source and push a new file version.
        // This replicates the logic from exec_step in grit_context.rs.
        if !state.effects.is_empty() {
            let file_owner = state.files.get_file_owner(file_ptr);
            let source = file_owner.tree.text();

            let new_src = apply_effects(
                source,
                &state.effects,
                &state.files,
                &context.lang,
                &mut logs,
            )?;

            if new_src != source {
                let file_name = file_owner.name.clone();
                if let Some(new_owner) =
                    new_file_owner(file_name, &new_src, &self.language, &mut logs)?
                {
                    file_owners.push(new_owner);
                    state
                        .files
                        .push_revision(&file_ptr, file_owners.last().unwrap());
                }
            }
            state.effects = vec![];
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
        Self::from_node_impl(root, source_path, lang, extra_built_ins, false)
    }

    /// Compiles a node selector query.
    ///
    /// Unlike [`Self::from_node()`], the query keeps the pattern before its
    /// automatic `contains` wrapping, so [`Self::find_matches()`] can execute it
    /// against individual nodes. Rewrites are rejected because a selector does
    /// not edit source text.
    pub fn from_node_selector(
        root: GritRoot,
        source_path: Option<&Utf8Path>,
        lang: GritTargetLanguage,
        extra_built_ins: Vec<BuiltInFunction>,
    ) -> Result<Self, CompileError> {
        Self::from_node_impl(root, source_path, lang, extra_built_ins, true)
    }

    fn from_node_impl(
        root: GritRoot,
        source_path: Option<&Utf8Path>,
        lang: GritTargetLanguage,
        extra_built_ins: Vec<BuiltInFunction>,
        node_selector: bool,
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

        if node_selector && has_rewrite(&root) {
            return Err(CompileError::RewritesNotAllowed);
        }

        let selector_pattern = node_selector.then(|| pattern.clone());

        let pattern = auto_wrap_pattern(
            pattern,
            &mut definitions.patterns,
            true,
            None,
            &mut node_context,
            None,
        )?;

        let anchor_kinds = extract_anchor_kinds(&pattern);

        let name = source_path
            .and_then(Utf8Path::file_stem)
            .map(|stem| stem.to_string());
        let language = context.lang;
        let variable_locations = VariableLocations::new(vars_array);

        Ok(Self {
            pattern,
            selector_pattern,
            anchor_kinds,
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
    pub diagnostics: Vec<(RuleDiagnostic, Applicability)>,
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

/// Returns whether the Grit source contains a rewrite.
fn has_rewrite(root: &GritRoot) -> bool {
    root.syntax().descendants().any(|node| {
        matches!(
            node.kind(),
            GritSyntaxKind::GRIT_REWRITE | GritSyntaxKind::GRIT_PREDICATE_REWRITE
        )
    })
}

/// Returns the patterns that a selector executes at each candidate.
///
/// An explicit `sequential { ... }` holds several independent patterns, so each
/// step becomes a selector. Other patterns are executed as written.
fn selector_patterns(pattern: &Pattern<GritQueryContext>) -> Vec<&Pattern<GritQueryContext>> {
    match pattern {
        Pattern::Sequential(steps) => steps.iter().map(|step| &step.pattern).collect(),
        _ => vec![pattern],
    }
}

/// Collects a node and all of its descendants, including tokens.
///
/// Grit matches can be anchored on leaf tokens such as operators, so selectors
/// must visit tokens as well as nodes.
fn selection_candidates<'a>(node: &GritTargetNode<'a>) -> Vec<GritTargetNode<'a>> {
    fn collect<'a>(node: &GritTargetNode<'a>, candidates: &mut Vec<GritTargetNode<'a>>) {
        candidates.push(node.clone());
        if let Some(slots) = node.slots() {
            for slot in slots {
                if let GritSyntaxSlot::Node(child) = slot {
                    collect(&child, candidates);
                }
            }
        }
    }

    let mut candidates = Vec::new();
    collect(node, &mut candidates);
    candidates
}

fn push_unique(
    matches: &mut Vec<GritTargetLanguageNode>,
    keys: &mut FxHashSet<SyntaxElementKey>,
    node: &GritTargetLanguageNode,
) {
    if keys.insert(node.key()) {
        matches.push(node.clone());
    }
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
            if kinds.is_empty()
                && let Pattern::Variable(variable) = &where_pat.pattern
            {
                kinds = extract_anchor_kinds_from_predicate(&where_pat.side_condition, variable);
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
        // Maybe matches every node because its inner pattern is optional.
        Pattern::Maybe(_) => vec![],
        Pattern::Limit(limit) => extract_anchor_kinds(&limit.pattern),
        Pattern::CodeSnippet(snippet) => snippet.patterns.iter().map(|(kind, _)| *kind).collect(),
        Pattern::AstNode(node) => vec![node.kind],
        _ => vec![],
    }
}

/// Extracts anchor kinds for `variable` from a predicate.
///
/// Only constraints on the same variable determine the kind of the node that
/// matched. Predicates on other variables, such as `$program`, cannot narrow
/// the candidate kinds.
fn extract_anchor_kinds_from_predicate(
    predicate: &Predicate<GritQueryContext>,
    variable: &Variable,
) -> Vec<GritTargetSyntaxKind> {
    match predicate {
        Predicate::Match(m) => {
            let Container::Variable(predicate_variable) = &m.val else {
                return vec![];
            };
            let (Ok(predicate_scope), Ok(predicate_index)) = (
                predicate_variable.try_scope(),
                predicate_variable.try_index(),
            ) else {
                return vec![];
            };
            let (Ok(variable_scope), Ok(variable_index)) =
                (variable.try_scope(), variable.try_index())
            else {
                return vec![];
            };
            if predicate_scope != variable_scope || predicate_index != variable_index {
                return vec![];
            }
            m.pattern
                .as_ref()
                .map(extract_anchor_kinds)
                .unwrap_or_default()
        }
        Predicate::And(and) => and
            .predicates
            .iter()
            .flat_map(|predicate| extract_anchor_kinds_from_predicate(predicate, variable))
            .collect(),
        Predicate::Or(or) => {
            let all: Vec<_> = or
                .predicates
                .iter()
                .map(|predicate| extract_anchor_kinds_from_predicate(predicate, variable))
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
/// Note: only extracts from single-step Sequential patterns, matching
/// the auto-wrap structure. Multi-step Sequential falls back to `execute()`.
fn extract_contains_inner(
    pattern: &Pattern<GritQueryContext>,
) -> Option<&Pattern<GritQueryContext>> {
    match pattern {
        Pattern::Sequential(seq) if seq.len() == 1 => seq
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
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{compile_js_query, compile_js_selector, make_js_file};
    use biome_grit_parser::parse_grit;
    use biome_js_syntax::JsSyntaxKind;
    use biome_rowan::NodeOrToken;

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
        let query = compile_js_query("not `console.log($msg)`");
        let kinds = query.anchor_kinds();
        assert!(
            kinds.is_empty(),
            "Not patterns should return empty anchor kinds"
        );
    }

    #[test]
    fn anchor_kinds_returns_empty_for_maybe() {
        let query = compile_js_query("maybe `console.log($msg)`");
        let kinds = query.anchor_kinds();
        assert!(
            kinds.is_empty(),
            "Maybe patterns match every node and cannot be narrowed by kind"
        );
    }

    #[test]
    fn anchor_kinds_ignores_predicates_on_other_variables() {
        let query = compile_js_query("$x where { $program <: contains `console.log($msg)` }");
        let kinds = query.anchor_kinds();
        assert!(
            kinds.is_empty(),
            "predicates on another variable cannot narrow the matched kind"
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
    fn execute_optimized_matches_execute_for_rewrite() {
        let query = compile_js_query("`console.log($msg)` => `console.info($msg)`");
        let code = r#"console.log("hello");"#;

        let opt_result = query
            .execute_optimized(make_js_file(code))
            .expect("optimized failed");
        let full_result = query.execute(make_js_file(code)).expect("execute failed");

        assert!(
            !full_result.effects.is_empty(),
            "full execute should produce rewrite effects"
        );
        assert_eq!(
            opt_result.effects.len(),
            full_result.effects.len(),
            "rewrite: optimized and full should produce same number of effects"
        );
    }

    #[test]
    fn rewrites_metavariables_inside_quoted_strings() {
        let query = compile_js_query(
            r#"`"$value"` where {
                $token = "my-old-css-variable",
                $replacement = "my-new-css-variable",
                $value <: r`(.*)(.*)-\\[--$token\\](.*)`($prefix, $class, $suffix) => `$prefix$class-[--$replacement]$suffix`
            }"#,
        );
        let code = r#"const value = "bg-[--my-old-css-variable] text-sm";"#;
        let expected = r#"const value = "bg-[--my-new-css-variable] text-sm";"#;

        let results = [
            query.execute(make_js_file(code)).expect("execute failed"),
            query
                .execute_optimized(make_js_file(code))
                .expect("optimized failed"),
        ];

        for result in results {
            let [GritQueryEffect::Rewrite(rewrite)] = result.effects.as_slice() else {
                panic!("expected one rewrite effect, got {:?}", result.effects);
            };
            assert_eq!(rewrite.rewritten.content, expected);
        }
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

    // -- find_matches tests --

    #[test]
    fn find_matches_returns_each_node_of_a_sequence() {
        let query = compile_js_selector("`foo(); bar()`");
        let matches = query
            .find_matches(make_js_file("foo(); bar();"))
            .expect("selector failed");

        assert_eq!(
            matches
                .iter()
                .map(|node| node.kind().as_js_kind().expect("JS node"))
                .collect::<Vec<_>>(),
            [
                JsSyntaxKind::JS_EXPRESSION_STATEMENT,
                JsSyntaxKind::JS_EXPRESSION_STATEMENT
            ]
        );
    }

    #[test]
    fn find_matches_does_not_duplicate_wrapper_and_child_matches() {
        let query = compile_js_selector("`undefined`");
        let matches = query
            .find_matches(make_js_file("undefined; undefined;"))
            .expect("selector failed");

        assert_eq!(matches.len(), 2, "got {matches:?}");
    }

    #[test]
    fn find_matches_requires_the_whole_sequence() {
        let query = compile_js_selector("`foo(); bar()`");
        let matches = query
            .find_matches(make_js_file("foo(); bar(); baz();"))
            .expect("selector failed");

        assert!(
            matches.is_empty(),
            "expected no match when the list has extra children, got {matches:?}"
        );
    }

    #[test]
    fn regular_queries_do_not_match_multi_child_lists() {
        let query = compile_js_query("`foo(); bar()`");
        let result = query
            .execute(make_js_file("foo(); bar();"))
            .expect("execute failed");

        assert!(
            result.effects.is_empty(),
            "regular Grit queries should keep their list-matching behavior, got {:?}",
            result.effects
        );
    }

    #[test]
    fn find_matches_does_not_prefilter_candidates_by_kind() {
        let query = compile_js_selector("maybe `foo()`");
        let matches = query
            .find_matches(make_js_file("const value = 1;"))
            .expect("selector failed");

        assert!(
            !matches.is_empty(),
            "kind-independent patterns must see every candidate"
        );
    }

    #[test]
    fn find_matches_supports_sequential_steps() {
        let query = compile_js_selector("sequential { `foo()`, `bar()` }");
        let matches = query
            .find_matches(make_js_file("foo(); bar(); baz();"))
            .expect("selector failed");

        assert_eq!(
            matches
                .iter()
                .map(|node| node.kind().as_js_kind().expect("JS node"))
                .collect::<Vec<_>>(),
            [
                JsSyntaxKind::JS_CALL_EXPRESSION,
                JsSyntaxKind::JS_CALL_EXPRESSION
            ]
        );
    }

    #[test]
    fn find_matches_returns_leaf_tokens() {
        let query = compile_js_selector("`+`");
        let matches = query
            .find_matches(make_js_file("const value = 1 + 2;"))
            .expect("selector failed");

        let [matched] = matches.as_slice() else {
            panic!("expected the plus token, got {matches:?}");
        };
        assert_eq!(matched.kind().as_js_kind(), Some(JsSyntaxKind::PLUS));
        assert!(matches!(
            matched,
            GritTargetLanguageNode::JsLanguage(NodeOrToken::Token(_))
        ));
    }

    #[test]
    fn selector_rejects_rewrites() {
        let parsed = parse_grit("`foo()` => `bar()`");
        assert!(parsed.diagnostics().is_empty());

        let error = GritQuery::from_node_selector(
            parsed.tree(),
            None,
            GritTargetLanguage::JsTargetLanguage(crate::JsTargetLanguage),
            Vec::new(),
        )
        .expect_err("rewrites are not selectors");

        assert!(matches!(error, CompileError::RewritesNotAllowed));
    }
}
