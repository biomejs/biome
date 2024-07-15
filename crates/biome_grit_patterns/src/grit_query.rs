use crate::diagnostics::CompilerDiagnostic;
use crate::grit_context::{GritExecContext, GritQueryContext, GritTargetFile};
use crate::grit_resolved_pattern::GritResolvedPattern;
use crate::grit_target_language::GritTargetLanguage;
use crate::grit_tree::GritTargetTree;
use crate::pattern_compiler::{auto_wrap_pattern, PatternCompiler};
use crate::pattern_compiler::{
    compilation_context::CompilationContext, compilation_context::NodeCompilationContext,
};
use crate::variables::{VarRegistry, VariableLocations};
use crate::CompileError;
use anyhow::bail;
use anyhow::Result;
use biome_grit_syntax::{GritRoot, GritRootExt};
use grit_pattern_matcher::constants::{
    ABSOLUTE_PATH_INDEX, FILENAME_INDEX, NEW_FILES_INDEX, PROGRAM_INDEX,
};
use grit_pattern_matcher::file_owners::{FileOwner, FileOwners};
use grit_pattern_matcher::pattern::{
    FilePtr, FileRegistry, Matcher, Pattern, ResolvedPattern, State, VariableSourceLocations,
};
use grit_util::{Ast, ByteRange, InputRanges, Range, VariableMatch};
use im::Vector;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

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
#[derive(Clone, Debug)]
pub struct GritQuery {
    pub pattern: Pattern<GritQueryContext>,

    /// Diagnostics discovered during compilation of the query.
    pub diagnostics: Vec<CompilerDiagnostic>,

    /// The name of the snippet being executed.
    pub name: Option<String>,

    /// Target language for the query.
    language: GritTargetLanguage,

    /// All variables discovered during query compilation.
    variable_locations: VariableLocations,
}

impl GritQuery {
    pub fn execute(&self, file: GritTargetFile) -> Result<Vec<GritQueryResult>> {
        let file_owners = FileOwners::new();
        let files = vec![file];
        let file_ptr = FilePtr::new(0, 0);
        let context = GritExecContext::new(
            self.language.clone(),
            self.name.as_deref(),
            &files,
            &file_owners,
        );

        let var_registry = VarRegistry::from_locations(&self.variable_locations);

        let file_registry =
            FileRegistry::new_from_paths(files.iter().map(|file| &file.path).collect());
        let binding = FilePattern::Single(file_ptr);

        let mut state = State::new(var_registry.into(), file_registry);
        let mut logs = Vec::new().into();

        let mut results: Vec<GritQueryResult> = Vec::new();
        if self
            .pattern
            .execute(&binding.into(), &mut state, &context, &mut logs)?
        {
            for file in state.files.files() {
                if let Some(result) = GritQueryResult::from_file(file)? {
                    results.push(result)
                }
            }
        }

        Ok(results)
    }

    pub fn from_node(
        root: GritRoot,
        path: Option<&Path>,
        lang: GritTargetLanguage,
    ) -> Result<Self, CompileError> {
        let context = CompilationContext::new(path, lang);

        let mut vars_array = vec![GLOBAL_VARS
            .iter()
            .map(|global_var| VariableSourceLocations {
                name: global_var.0.to_string(),
                file: path
                    .map(Path::to_string_lossy)
                    .map_or_else(|| "unnamed".to_owned(), |p| p.to_string()),
                locations: BTreeSet::new(),
            })
            .collect::<Vec<VariableSourceLocations>>()];
        let mut global_vars: BTreeMap<String, usize> = GLOBAL_VARS
            .iter()
            .map(|(global_var, index)| ((*global_var).to_string(), *index))
            .collect();
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

        let mut pattern_definitions = Vec::new();
        let pattern = auto_wrap_pattern(
            pattern,
            &mut pattern_definitions,
            true,
            None,
            &mut node_context,
            None,
        )?;

        let name = path
            .and_then(Path::file_stem)
            .map(OsStr::to_string_lossy)
            .map(|stem| stem.to_string());
        let language = context.lang;
        let variable_locations = VariableLocations::new(vars_array);

        Ok(Self {
            pattern,
            name,
            language,
            diagnostics,
            variable_locations,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GritQueryResult {
    Match(Match),
    Rewrite(Rewrite),
    CreateFile(CreateFile),
}

impl GritQueryResult {
    pub fn from_file(file: &Vector<&FileOwner<GritTargetTree>>) -> anyhow::Result<Option<Self>> {
        if file.is_empty() {
            bail!("cannot have file with no versions")
        }

        let result = if file.len() == 1 {
            let file = file.last().unwrap();
            if file.new {
                Some(GritQueryResult::CreateFile(CreateFile::new(
                    &file.name,
                    &file.tree.source(),
                )))
            } else if let Some(ranges) = &file.matches.borrow().input_matches {
                if ranges.suppressed {
                    None
                } else {
                    Some(GritQueryResult::Match(Match::from_file_ranges(
                        ranges, &file.name,
                    )))
                }
            } else {
                None
            }
        } else {
            Some(GritQueryResult::Rewrite(Rewrite::from_file(
                file.front().unwrap(),
                file.back().unwrap(),
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

impl From<Rewrite> for GritQueryResult {
    fn from(value: Rewrite) -> Self {
        GritQueryResult::Rewrite(value)
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
    ) -> anyhow::Result<Self> {
        let original = if let Some(ranges) = &initial.matches.borrow().input_matches {
            Match::from_file_ranges(ranges, &initial.name)
        } else {
            bail!("cannot have rewrite without matches")
        };
        let rewritten = OutputFile::from_file(rewritten_file);
        Ok(Rewrite::new(original, rewritten))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateFile {
    pub rewritten: OutputFile,
    pub ranges: Option<Vec<Range>>,
}

impl From<CreateFile> for GritQueryResult {
    fn from(value: CreateFile) -> Self {
        GritQueryResult::CreateFile(value)
    }
}

impl CreateFile {
    fn new(path: &Path, body: &str) -> Self {
        CreateFile {
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
