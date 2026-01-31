use crate::CliDiagnostic;
use crate::cli_options::CliOptions;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{ResultExt, SearchDiagnostic};
use crate::runner::execution::{AnalyzerSelectors, Execution};
use crate::runner::impls::commands::traversal::TraversalCommand;
use crate::runner::impls::executions::summary_verb::SummaryVerbExecution;
use crate::runner::process_file::{
    FileStatus, Message, ProcessFile, ProcessStdinFilePayload, WorkspaceFile,
};
use biome_configuration::vcs::VcsConfiguration;
use biome_configuration::{Configuration, FilesConfiguration};
use biome_console::{Console, MarkupBuf};
use biome_deserialize::Merge;
use biome_diagnostics::{Category, DiagnosticExt, category};
use biome_fs::FileSystem;
use biome_grit_patterns::{GritTargetLanguage, JsTargetLanguage};
use biome_service::workspace::{
    DocumentFileSource, DropPatternParams, FeatureKind, FeatureName, FeaturesBuilder,
    FeaturesSupported, ParsePatternParams, PatternId, ScanKind, SupportKind,
};
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::ffi::OsString;
use std::time::Duration;

pub(crate) struct SearchCommandPayload {
    pub(crate) files_configuration: Option<FilesConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) pattern: String,
    pub(crate) language: Option<GritTargetLanguage>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) vcs_configuration: Option<VcsConfiguration>,
}

struct SearchExecution {
    /// The GritQL pattern to search for.
    ///
    /// Note that the search command does not support rewrites.
    pattern: PatternId,

    /// The language to query for.
    ///
    /// Grit queries are specific to the grammar of the language they
    /// target, so we currently do not support writing queries that apply
    /// to multiple languages at once.
    ///
    /// If none given, the default language is JavaScript.
    language: Option<GritTargetLanguage>,

    /// An optional tuple.
    /// 1. The virtual path to the file
    /// 2. The content of the file
    stdin_file_path: Option<String>,
}

impl Execution for SearchExecution {
    fn wanted_features(&self) -> FeatureName {
        FeaturesBuilder::new().with_search().build()
    }

    fn not_requested_features(&self) -> FeatureName {
        FeaturesBuilder::new().with_all().without_search().build()
    }

    fn can_handle(&self, features: FeaturesSupported) -> bool {
        features.supports_search()
    }

    fn on_post_crawl(&self, workspace: &dyn Workspace) -> Result<(), WorkspaceError> {
        workspace.drop_pattern(DropPatternParams {
            pattern: self.pattern.clone(),
        })
    }

    fn is_vcs_targeted(&self) -> bool {
        false
    }

    fn supports_kind(&self, file_features: &FeaturesSupported) -> Option<SupportKind> {
        Some(file_features.support_kind_for(FeatureKind::Search))
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        self.stdin_file_path.as_deref()
    }

    fn as_diagnostic_category(&self) -> &'static Category {
        category!("search")
    }

    fn is_search(&self) -> bool {
        true
    }

    fn requires_write_access(&self) -> bool {
        false
    }

    fn analyzer_selectors(&self) -> AnalyzerSelectors {
        AnalyzerSelectors::default()
    }

    fn search_language(&self) -> Option<GritTargetLanguage> {
        self.language.clone()
    }

    fn search_pattern(&self) -> Option<&PatternId> {
        Some(&self.pattern)
    }

    fn summary_phrase(&self, files: usize, duration: &Duration) -> MarkupBuf {
        SummaryVerbExecution.summary_verb("Searched", files, duration)
    }
}

pub(crate) struct SearchProcessFile;

impl SearchProcessFile {
    fn is_file_compatible_with_pattern(
        file_source: &DocumentFileSource,
        pattern_language: &GritTargetLanguage,
    ) -> bool {
        match pattern_language {
            GritTargetLanguage::JsTargetLanguage(_) => {
                matches!(file_source, DocumentFileSource::Js(_))
            }
            GritTargetLanguage::CssTargetLanguage(_) => {
                matches!(file_source, DocumentFileSource::Css(_))
            }
            GritTargetLanguage::JsonTargetLanguage(_) => {
                matches!(file_source, DocumentFileSource::Json(_))
            }
        }
    }
}

impl ProcessFile for SearchProcessFile {
    fn process_file<Ctx>(
        ctx: &Ctx,
        workspace_file: &mut WorkspaceFile,
        _features_supported: &FeaturesSupported,
    ) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        let execution = ctx.execution();
        let file_source = DocumentFileSource::from_path(workspace_file.path.as_path(), false);
        let pattern_language = execution
            .search_language()
            .unwrap_or(GritTargetLanguage::JsTargetLanguage(JsTargetLanguage));
        // SAFETY: search_pattern is implemented in this file
        let pattern = execution.search_pattern().unwrap();

        // Ignore files that don't match the pattern's target language
        if !Self::is_file_compatible_with_pattern(&file_source, &pattern_language) {
            return Ok(FileStatus::Ignored);
        }

        let result = workspace_file
            .guard()
            .search_pattern(pattern)
            .with_file_path_and_code(workspace_file.path.to_string(), category!("search"))?;

        let input = workspace_file.input()?;
        let file_name = workspace_file.path.to_string();
        let matches_len = result.matches.len();

        let search_results = Message::Diagnostics {
            file_path: file_name,
            content: input,
            diagnostics: result
                .matches
                .into_iter()
                .map(|mat| SearchDiagnostic.with_file_span(mat))
                .collect(),
            skipped_diagnostics: 0,
        };

        Ok(FileStatus::SearchResult(matches_len, search_results))
    }

    fn process_std_in(_payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic> {
        Ok(())
    }
}

impl TraversalCommand for SearchCommandPayload {
    type ProcessFile = SearchProcessFile;

    fn command_name(&self) -> &'static str {
        "search"
    }

    fn minimal_scan_kind(&self) -> Option<ScanKind> {
        Some(ScanKind::KnownFiles)
    }
    fn get_execution(
        &self,
        _cli_options: &CliOptions,
        _console: &mut dyn Console,
        workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic> {
        let pattern = workspace
            .parse_pattern(ParsePatternParams {
                pattern: self.pattern.clone(),
                default_language: self.language.clone().unwrap_or_default(),
            })?
            .pattern_id;
        Ok(Box::new(SearchExecution {
            stdin_file_path: self.stdin_file_path.clone(),
            language: self.language.clone(),
            pattern,
        }))
    }

    fn merge_configuration(
        &mut self,
        mut loaded_configuration: Configuration,
        _loaded_directory: Option<Utf8PathBuf>,
        _loaded_file: Option<Utf8PathBuf>,
        _fs: &dyn FileSystem,
        _console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        loaded_configuration
            .files
            .merge_with(self.files_configuration.clone());
        loaded_configuration
            .vcs
            .merge_with(self.vcs_configuration.clone());

        Ok(loaded_configuration)
    }

    fn get_files_to_process(
        &self,
        _fs: &dyn FileSystem,
        _configuration: &Configuration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        Ok(self.paths.clone())
    }
}
