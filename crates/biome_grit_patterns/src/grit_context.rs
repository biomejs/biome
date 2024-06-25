use crate::grit_binding::GritBinding;
use crate::grit_code_snippet::GritCodeSnippet;
use crate::grit_file::GritFile;
use crate::grit_node_patterns::{GritLeafNodePattern, GritNodePattern};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::GritTargetLanguage;
use crate::grit_target_node::GritTargetNode;
use crate::grit_tree::GritTargetTree;
use anyhow::Result;
use grit_pattern_matcher::context::{ExecContext, QueryContext};
use grit_pattern_matcher::file_owners::{FileOwner, FileOwners};
use grit_pattern_matcher::pattern::{
    CallBuiltIn, GritFunctionDefinition, Pattern, PatternDefinition, PredicateDefinition, State,
};
use grit_util::{AnalysisLogs, FileOrigin};
use path_absolutize::Absolutize;
use std::path::PathBuf;

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
    lang: GritTargetLanguage,
    loadable_files: &'a [GritTargetFile],
    files: &'a FileOwners<GritTargetTree>,
    functions: Vec<GritFunctionDefinition<GritQueryContext>>,
    patterns: Vec<PatternDefinition<GritQueryContext>>,
    predicates: Vec<PredicateDefinition<GritQueryContext>>,
}

impl<'a> GritExecContext<'a> {
    pub fn new(
        lang: GritTargetLanguage,
        loadable_files: &'a [GritTargetFile],
        files: &'a FileOwners<GritTargetTree>,
    ) -> Self {
        Self {
            lang,
            loadable_files,
            files,
            functions: Vec::new(),
            patterns: Vec::new(),
            predicates: Vec::new(),
        }
    }
}

impl<'a> ExecContext<'a, GritQueryContext> for GritExecContext<'a> {
    fn pattern_definitions(&self) -> &[PatternDefinition<GritQueryContext>] {
        &self.patterns
    }

    fn predicate_definitions(&self) -> &[PredicateDefinition<GritQueryContext>] {
        &self.predicates
    }

    fn function_definitions(&self) -> &[GritFunctionDefinition<GritQueryContext>] {
        &self.functions
    }

    fn ignore_limit_pattern(&self) -> bool {
        false
    }

    fn call_built_in(
        &self,
        _call: &'a CallBuiltIn<GritQueryContext>,
        _context: &'a Self,
        _state: &mut State<'a, GritQueryContext>,
        _logs: &mut AnalysisLogs,
    ) -> Result<GritResolvedPattern<'a>> {
        unimplemented!("built-in functions are still TODO")
    }

    fn files(&self) -> &FileOwners<GritTargetTree> {
        self.files
    }

    fn language(&self) -> &GritTargetLanguage {
        &self.lang
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
        file: &GritFile<'a>,
        state: &mut State<'a, GritQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
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

                let file = file_owner_from_matches(&file.path, &file.content, &self.lang, logs)?;
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
    source: &str,
    language: &GritTargetLanguage,
    logs: &mut AnalysisLogs,
) -> Result<Option<FileOwner<GritTargetTree>>> {
    let name = name.into();

    let Some(tree) = language
        .get_parser()
        .parse_file(source, Some(&name), logs, FileOrigin::Fresh)
    else {
        return Ok(None);
    };

    let absolute_path = name.absolutize()?.to_path_buf();
    Ok(Some(FileOwner {
        name,
        absolute_path,
        tree,
        matches: Default::default(),
        new: false,
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
    pub path: PathBuf,
    pub content: String,
}
