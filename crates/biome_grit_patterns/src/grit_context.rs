use crate::grit_binding::GritBinding;
use crate::grit_built_in_functions::BuiltIns;
use crate::grit_code_snippet::GritCodeSnippet;
use crate::grit_file::GritFile;
use crate::grit_node_patterns::{GritLeafNodePattern, GritNodePattern};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::GritTargetLanguage;
use crate::grit_target_node::GritTargetNode;
use crate::grit_tree::GritTargetTree;
use biome_analyze::RuleDiagnostic;
use biome_parser::AnyParse;
use camino::Utf8PathBuf;
use grit_pattern_matcher::constants::{GLOBAL_VARS_SCOPE_INDEX, NEW_FILES_INDEX};
use grit_pattern_matcher::context::{ExecContext, QueryContext};
use grit_pattern_matcher::file_owners::{FileOwner, FileOwners};
use grit_pattern_matcher::pattern::{
    CallBuiltIn, File, FilePtr, GritFunctionDefinition, Matcher, Pattern, PatternDefinition,
    PredicateDefinition, ResolvedPattern, State,
};
use grit_util::error::GritPatternError;
use grit_util::{error::GritResult, AnalysisLogs, FileOrigin, InputRanges, MatchRanges};
use path_absolutize::Absolutize;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Clone, Debug, PartialEq)]
pub struct GritQueryContext;

impl QueryContext for GritQueryContext {
    type Node<'a> = GritTargetNode<'a>;
    type NodePattern = GritNodePattern;
    type LeafNodePattern = GritLeafNodePattern;
    type ExecContext<'a> = GritExecContext<'a>;
    type Binding<'a> = GritBinding<'a>;
    type CodeSnippet = GritCodeSnippet;
    type ResolvedPattern<'a> = GritResolvedPattern<'a>;
    type Language<'a> = GritTargetLanguage;
    type File<'a> = GritFile<'a>;
    type Tree<'a> = GritTargetTree;
}

pub struct GritExecContext<'a> {
    /// The language to which the snippet should apply.
    pub lang: GritTargetLanguage,

    /// The name of the snippet being executed.
    pub name: Option<&'a str>,

    pub loadable_files: &'a [GritTargetFile],
    pub files: &'a FileOwners<GritTargetTree>,
    pub built_ins: &'a BuiltIns,
    pub functions: &'a [GritFunctionDefinition<GritQueryContext>],
    pub patterns: &'a [PatternDefinition<GritQueryContext>],
    pub predicates: &'a [PredicateDefinition<GritQueryContext>],

    pub diagnostics: Mutex<Vec<RuleDiagnostic>>,
}

impl GritExecContext<'_> {
    pub fn add_diagnostic(&self, diagnostic: RuleDiagnostic) {
        self.diagnostics.lock().unwrap().push(diagnostic);
    }

    pub fn into_diagnostics(self) -> Vec<RuleDiagnostic> {
        self.diagnostics.into_inner().unwrap()
    }
}

impl<'a> ExecContext<'a, GritQueryContext> for GritExecContext<'a> {
    fn pattern_definitions(&self) -> &[PatternDefinition<GritQueryContext>] {
        self.patterns
    }

    fn predicate_definitions(&self) -> &[PredicateDefinition<GritQueryContext>] {
        self.predicates
    }

    fn function_definitions(&self) -> &[GritFunctionDefinition<GritQueryContext>] {
        self.functions
    }

    fn ignore_limit_pattern(&self) -> bool {
        false
    }

    fn call_built_in(
        &self,
        call: &'a CallBuiltIn<GritQueryContext>,
        context: &'a Self,
        state: &mut State<'a, GritQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<GritResolvedPattern<'a>> {
        self.built_ins
            .call(call, context, state, logs)
            .map_err(|e| GritPatternError::new(e.to_string()))
    }

    fn call_callback<'b>(
        &self,
        call: &'a grit_pattern_matcher::pattern::CallbackPattern,
        context: &'a Self,
        binding: &'b <GritQueryContext as QueryContext>::ResolvedPattern<'a>,
        state: &mut State<'a, GritQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        self.built_ins
            .call_callback(call, context, binding, state, logs)
            .map_err(|e| GritPatternError::new(e.to_string()))
    }

    fn files(&self) -> &FileOwners<GritTargetTree> {
        self.files
    }

    fn language(&self) -> &GritTargetLanguage {
        &self.lang
    }

    fn exec_step(
        &'a self,
        step: &'a Pattern<GritQueryContext>,
        binding: &GritResolvedPattern,
        state: &mut State<'a, GritQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let mut files = if let Some(files) = binding.get_file_pointers() {
            files
                .iter()
                .map(|f| state.files.latest_revision(f))
                .collect::<Vec<FilePtr>>()
        } else {
            return Ok(false);
        };

        let binding = if files.len() == 1 {
            ResolvedPattern::from_file_pointer(*files.last().unwrap())
        } else {
            // Load all files into memory and collect successful file pointers
            files.retain(|file_ptr| {
                self.load_file(&GritFile::Ptr(*file_ptr), state, logs)
                    .unwrap_or(false)
            });
            ResolvedPattern::from_files(ResolvedPattern::from_list_parts(
                files.iter().map(|f| ResolvedPattern::from_file_pointer(*f)),
            ))
        };
        if !step.execute(&binding, state, self, logs)? {
            return Ok(false);
        }

        // todo, for multifile we need to split up the matches by file.
        let (variables, ranges, suppressed) =
            state.bindings_history_to_ranges(&self.lang, self.name());

        let input_ranges = InputRanges {
            ranges,
            variables,
            suppressed,
        };
        for file_ptr in files {
            let file = state.files.get_file_owner(file_ptr);
            let mut match_log = file.matches.borrow_mut();

            if match_log.input_matches.is_none() {
                match_log.input_matches = Some(input_ranges.clone());
            }

            // TODO: Implement effect application
        }

        let new_files_binding = &mut state.bindings[GLOBAL_VARS_SCOPE_INDEX as usize]
            .last_mut()
            .unwrap()[NEW_FILES_INDEX];
        if new_files_binding.value.is_none() {
            new_files_binding.value = Some(GritResolvedPattern::from_list_parts([].into_iter()));
        }

        let Some(new_files) = new_files_binding
            .value
            .as_ref()
            .and_then(ResolvedPattern::get_list_items)
        else {
            return Err(GritPatternError::new("Expected a list of files"));
        };

        for f in new_files {
            let Some(file) = f.get_file() else {
                return Err(GritPatternError::new("Expected a list of files"));
            };

            let name: PathBuf = file
                .name(&state.files)
                .text(&state.files, &self.lang)?
                .as_ref()
                .into();
            let body = file.body(&state.files).text(&state.files, &self.lang)?;
            let owned_file =
                new_file_owner(name.clone(), &body, &self.lang, logs)?.ok_or_else(|| {
                    GritPatternError::Builder(format!(
                        "failed to construct new file for file {}",
                        name.to_string_lossy()
                    ))
                })?;
            self.files().push(owned_file);
            // SAFETY: We just pushed to the list of files, so there must be one.
            let _ = state.files.push_new_file(self.files().last().unwrap());
        }

        state.effects = vec![];
        new_files_binding.value = Some(ResolvedPattern::from_list_parts([].into_iter()));
        Ok(true)
    }

    fn name(&self) -> Option<&str> {
        self.name
    }

    fn load_file(
        &self,
        file: &GritFile<'a>,
        state: &mut State<'a, GritQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        match file {
            GritFile::Resolved(_) => {
                // Assume the file is already loaded
            }
            GritFile::Ptr(ptr) => {
                if state.files.is_loaded(ptr) {
                    return Ok(true);
                }

                let index = ptr.file;
                let file = &self.loadable_files[index as usize];

                // TODO: Verify the workspace's maximum file size.

                let file = file_owner_from_matches(
                    &file.path,
                    &file.parse,
                    None,
                    FileOrigin::Fresh,
                    &self.lang,
                    logs,
                )?;
                if let Some(file) = file {
                    self.files.push(file);
                    state.files.load_file(ptr, self.files.last().unwrap());
                }
            }
        }
        Ok(true)
    }
}

fn file_owner_from_matches(
    name: impl Into<PathBuf>,
    parse: &AnyParse,
    matches: Option<MatchRanges>,
    old_tree: FileOrigin<'_, GritTargetTree>,
    language: &GritTargetLanguage,
    logs: &mut AnalysisLogs,
) -> GritResult<Option<FileOwner<GritTargetTree>>> {
    let name = name.into();
    let new = !old_tree.is_fresh();

    let Some(tree) = language
        .get_parser()
        .from_cached_parse_result(parse, Some(&name), logs)
    else {
        return Ok(None);
    };

    let absolute_path = name.absolutize()?.to_path_buf();
    Ok(Some(FileOwner {
        name,
        absolute_path,
        tree,
        matches: matches.unwrap_or_default().into(),
        new,
    }))
}

fn new_file_owner(
    name: impl Into<PathBuf>,
    source: &str,
    language: &GritTargetLanguage,
    logs: &mut AnalysisLogs,
) -> GritResult<Option<FileOwner<GritTargetTree>>> {
    let name = name.into();

    let Some(tree) = language
        .get_parser()
        .parse_file(source, Some(&name), logs, FileOrigin::New)
    else {
        return Ok(None);
    };

    let absolute_path = name.absolutize()?.to_path_buf();
    Ok(Some(FileOwner {
        name,
        absolute_path,
        tree,
        matches: Default::default(),
        new: true,
    }))
}

/// Simple wrapper for target files so that we can avoid doing file I/O inside
/// the Grit engine.
///
/// This should suffice as long as we only do single-file queries, but when we
/// want to support multifile queries, we probably need to implement a solution
/// that can use the Biome workspace.
#[derive(Clone, Debug)]
pub struct GritTargetFile {
    pub path: Utf8PathBuf,
    pub parse: AnyParse,
}

impl GritTargetFile {
    pub fn parse(source: &str, path: Utf8PathBuf, target_language: GritTargetLanguage) -> Self {
        let parser = target_language.get_parser();
        let parse = parser.parse_with_path(source, &path);

        Self { parse, path }
    }
}
