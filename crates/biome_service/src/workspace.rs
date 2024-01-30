//! The [Workspace] is the main entry point for high-level clients (the Biome
//! CLI and Language Server) into the various language-specific services of the
//! Biome toolchain (parser, formatter, analyzer ...)
//!
//! # Documents
//!
//! A [Workspace] instance operates on a set of open documents managed by one
//! or multiple clients, and provides language services for these documents
//! like diagnostics, code actions or formatting in a language independent way.
//!
//! In this regard the [Workspace] trait shares many similarities with the
//! Language Server Protocol, and in the context of the Language Server the
//! state of the [Workspace] instance is intended to closely mirror the state
//! of the actual in-editor workspace (the set of documents open in the
//! [Workspace] is the set of files currently open in the editor)
//!
//! In the context of the CLI most commands will generally work on batches of
//! files, and as such the set of "open documents" instead corresponds to the
//! list of files the CLI is currently actively processing
//!
//! # State
//!
//! A [Workspace] instance is stateful: this is not reflected on the trait (all
//! methods take an immutable `&self` borrow) because the interface is also
//! required to be thread-safe ([Send] + [Sync]), but the workspace is allowed
//! to internally cache data across calls (this is in fact the main reason for
//! the use of the "open documents" set, those documents can serve as
//! conceptual garbage collection roots to manage the caching and eviction of
//! parse trees, intermediate analysis data or diagnostics)
//!
//! # Implementations
//!
//! Currently the [Workspace] trait is implemented for a single `WorkspaceServer`
//! type. However it is eventually intended to also be implemented for a
//! potential `WorkspaceClient` type and to operate on a remote workspace
//! server through a transport layer. This would allow the CLI and Language
//! Server process to share a the same [Workspace] instance in a common daemon
//! process for instance
//!
//! # Errors
//!
//! Because of the aforementioned client-server abstraction, the [Workspace]
//! is designed to let any operation fail: all methods return a [Result] with a
//! [WorkspaceError] enum wrapping the underlying issue. Some common errors are:
//!
//! - [WorkspaceError::NotFound]: This error is returned when an operation is being
//! run on a path that doesn't correspond to any open document: either the
//! document has been closed or the client didn't open it in the first place
//! - [WorkspaceError::SourceFileNotSupported]: This error is returned when an
//! operation could not be completed because the language associated with the
//! document does not implement the required capability: for instance trying to
//! format a file with a language that does not have a formatter

use crate::file_handlers::Capabilities;
use crate::{Deserialize, Serialize, WorkspaceError};
use biome_analyze::ActionCategory;
pub use biome_analyze::RuleCategories;
use biome_console::{markup, Markup, MarkupBuf};
use biome_css_formatter::can_format_css_yet;
use biome_diagnostics::CodeSuggestion;
use biome_formatter::Printed;
use biome_fs::RomePath;
use biome_js_syntax::{TextRange, TextSize};
use biome_text_edit::TextEdit;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::{borrow::Cow, panic::RefUnwindSafe, sync::Arc};
use tracing::debug;

pub use self::client::{TransportRequest, WorkspaceClient, WorkspaceTransport};
use crate::configuration::PartialConfiguration;
pub use crate::file_handlers::Language;
use crate::settings::WorkspaceSettings;

mod client;
mod server;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SupportsFeatureParams {
    pub path: RomePath,
    pub feature: Vec<FeatureName>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SupportsFeatureResult {
    pub reason: Option<SupportKind>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FileFeaturesResult {
    pub features_supported: HashMap<FeatureName, SupportKind>,
}

impl FileFeaturesResult {
    /// Sorted array of files that should not be processed no matter the cases.
    /// These files are handled by other tools.
    const PROTECTED_FILES: &'static [&'static str; 10] = &[
        // Composer
        "composer.json",
        "composer.lock",
        // Deno
        "deno.json",
        "deno.jsonc",
        // TSC
        "jsconfig.json",
        // NPM
        "npm-shrinkwrap.json",
        "package-lock.json",
        // TSC
        "tsconfig.json",
        "typescript.json",
        // Yarn
        "yarn.lock",
    ];

    /// Checks whether this file is protected.
    /// A protected file is handled by a specific tool and should be ignored.
    pub(crate) fn is_protected_file(path: &Path) -> bool {
        path.file_name()
            .and_then(OsStr::to_str)
            .is_some_and(|file_name| {
                FileFeaturesResult::PROTECTED_FILES
                    .binary_search(&file_name)
                    .is_ok()
            })
    }

    /// By default, all features are not supported by a file.
    const WORKSPACE_FEATURES: [(FeatureName, SupportKind); 3] = [
        (FeatureName::Lint, SupportKind::FileNotSupported),
        (FeatureName::Format, SupportKind::FileNotSupported),
        (FeatureName::OrganizeImports, SupportKind::FileNotSupported),
    ];

    pub fn new() -> Self {
        Self {
            features_supported: HashMap::from(FileFeaturesResult::WORKSPACE_FEATURES),
        }
    }

    pub fn with_capabilities(mut self, capabilities: &Capabilities) -> Self {
        if capabilities.formatter.format.is_some() {
            self.features_supported
                .insert(FeatureName::Format, SupportKind::Supported);
        }
        if capabilities.analyzer.lint.is_some() {
            self.features_supported
                .insert(FeatureName::Lint, SupportKind::Supported);
        }
        if capabilities.analyzer.organize_imports.is_some() {
            self.features_supported
                .insert(FeatureName::OrganizeImports, SupportKind::Supported);
        }

        self
    }

    pub(crate) fn with_settings_and_language(
        mut self,
        settings: &WorkspaceSettings,
        language: &Language,
        path: &Path,
    ) -> Self {
        let formatter_disabled =
            if let Some(disabled) = settings.override_settings.formatter_disabled(path) {
                disabled
            } else if language.is_javascript_like() {
                !settings.formatter().enabled || settings.javascript_formatter_disabled()
            } else if language.is_json_like() {
                !settings.formatter().enabled || settings.json_formatter_disabled()
            } else if language.is_css_like() {
                !can_format_css_yet()
                    || !settings.formatter().enabled
                    || settings.css_formatter_disabled()
            } else {
                !settings.formatter().enabled
            };
        if formatter_disabled {
            self.features_supported
                .insert(FeatureName::Format, SupportKind::FeatureNotEnabled);
        }
        // linter
        if let Some(disabled) = settings.override_settings.linter_disabled(path) {
            if disabled {
                self.features_supported
                    .insert(FeatureName::Lint, SupportKind::FeatureNotEnabled);
            }
        } else if !settings.linter().enabled {
            self.features_supported
                .insert(FeatureName::Lint, SupportKind::FeatureNotEnabled);
        }

        // organize imports
        if let Some(disabled) = settings.override_settings.organize_imports_disabled(path) {
            if disabled {
                self.features_supported
                    .insert(FeatureName::OrganizeImports, SupportKind::FeatureNotEnabled);
            }
        } else if !settings.organize_imports().enabled {
            self.features_supported
                .insert(FeatureName::OrganizeImports, SupportKind::FeatureNotEnabled);
        }

        debug!(
            "The file has the following feature sets: \n{:?}",
            &self.features_supported
        );

        self
    }

    /// The file will be ignored for all features
    pub fn set_ignored_for_all_features(&mut self) {
        for support_kind in self.features_supported.values_mut() {
            *support_kind = SupportKind::Ignored;
        }
    }

    /// The file will be protected for all features
    pub fn set_protected_for_all_features(&mut self) {
        for support_kind in self.features_supported.values_mut() {
            *support_kind = SupportKind::Protected;
        }
    }

    pub fn ignored(&mut self, feature: FeatureName) {
        self.features_supported
            .insert(feature, SupportKind::Ignored);
    }

    /// Checks whether the file support the given `feature`
    pub fn supports_for(&self, feature: &FeatureName) -> bool {
        self.features_supported
            .get(feature)
            .map(|support_kind| matches!(support_kind, SupportKind::Supported))
            .unwrap_or_default()
    }

    /// Loops through all the features of the current file, and if a feature is [SupportKind::FileNotSupported],
    /// it gets changed to [SupportKind::Ignored]
    pub fn ignore_not_supported(&mut self) {
        for support_kind in self.features_supported.values_mut() {
            if matches!(support_kind, SupportKind::FileNotSupported) {
                *support_kind = SupportKind::Ignored;
            }
        }
    }

    pub fn support_kind_for(&self, feature: &FeatureName) -> Option<&SupportKind> {
        self.features_supported.get(feature)
    }

    /// If at least one feature is supported, the file is supported
    pub fn is_supported(&self) -> bool {
        self.features_supported
            .values()
            .any(|support_kind| support_kind.is_supported())
    }

    /// The file is ignored only if all the features marked it as ignored
    pub fn is_ignored(&self) -> bool {
        self.features_supported
            .values()
            .all(|support_kind| support_kind.is_ignored())
    }

    /// The file is protected only if all the features marked it as protected
    pub fn is_protected(&self) -> bool {
        self.features_supported
            .values()
            .all(|support_kind| support_kind.is_protected())
    }

    /// The file is not supported if all the featured marked it as not supported
    pub fn is_not_supported(&self) -> bool {
        self.features_supported
            .values()
            .all(|support_kind| support_kind.is_not_supported())
    }

    /// The file is not enabled if all the features marked it as not enabled
    pub fn is_not_enabled(&self) -> bool {
        self.features_supported
            .values()
            .all(|support_kind| support_kind.is_not_enabled())
    }

    /// The file is not processed if for every enabled feature
    /// the file is either protected, not supported, ignored.
    pub fn is_not_processed(&self) -> bool {
        self.features_supported.values().all(|support_kind| {
            matches!(
                support_kind,
                SupportKind::FeatureNotEnabled
                    | SupportKind::FileNotSupported
                    | SupportKind::Ignored
                    | SupportKind::Protected
            )
        })
    }
}

impl SupportsFeatureResult {
    /// Whether the feature is intentionally disabled
    pub const fn is_not_enabled(&self) -> bool {
        matches!(self.reason, Some(SupportKind::FeatureNotEnabled))
    }

    /// Whether the feature is supported
    pub const fn is_supported(&self) -> bool {
        self.reason.is_none()
    }

    /// Whether the feature is not supported, regardless of the reason
    pub const fn is_not_supported(&self) -> bool {
        self.reason.is_some()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Clone)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SupportKind {
    /// The feature is enabled for the file
    Supported,
    /// The file is ignored (configuration)
    Ignored,
    /// The file is protected, meaning that it can't be processed because other tools manage it
    Protected,
    /// The feature is not enabled (configuration or the file doesn't need it)
    FeatureNotEnabled,
    /// The file is not capable of having this feature
    FileNotSupported,
}

impl SupportKind {
    pub const fn is_supported(&self) -> bool {
        matches!(self, SupportKind::Supported)
    }
    pub const fn is_not_enabled(&self) -> bool {
        matches!(self, SupportKind::FeatureNotEnabled)
    }

    pub const fn is_not_supported(&self) -> bool {
        matches!(self, SupportKind::FileNotSupported)
    }
    pub const fn is_ignored(&self) -> bool {
        matches!(self, SupportKind::Ignored)
    }
    pub const fn is_protected(&self) -> bool {
        matches!(self, SupportKind::Protected)
    }
}

#[derive(Debug, Copy, Clone, Hash, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum FeatureName {
    Format,
    Lint,
    OrganizeImports,
}

#[derive(Debug, Default)]
pub struct FeaturesBuilder(Vec<FeatureName>);

impl FeaturesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_formatter(mut self) -> Self {
        self.0.push(FeatureName::Format);
        self
    }
    pub fn with_linter(mut self) -> Self {
        self.0.push(FeatureName::Lint);
        self
    }
    pub fn with_organize_imports(mut self) -> Self {
        self.0.push(FeatureName::OrganizeImports);
        self
    }

    pub fn build(self) -> Vec<FeatureName> {
        self.0
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct UpdateSettingsParams {
    pub configuration: PartialConfiguration,
    // @ematipico TODO: have a better data structure for this
    pub vcs_base_path: Option<PathBuf>,
    // @ematipico TODO: have a better data structure for this
    pub gitignore_matches: Vec<String>,
    pub working_directory: Option<PathBuf>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProjectFeaturesParams {
    pub manifest_path: RomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProjectFeaturesResult {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OpenFileParams {
    pub path: RomePath,
    pub content: String,
    pub version: i32,
    #[serde(default)]
    pub language_hint: Language,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GetSyntaxTreeParams {
    pub path: RomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GetSyntaxTreeResult {
    pub cst: String,
    pub ast: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GetControlFlowGraphParams {
    pub path: RomePath,
    pub cursor: TextSize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GetFormatterIRParams {
    pub path: RomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GetFileContentParams {
    pub path: RomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ChangeFileParams {
    pub path: RomePath,
    pub content: String,
    pub version: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CloseFileParams {
    pub path: RomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PullDiagnosticsParams {
    pub path: RomePath,
    pub categories: RuleCategories,
    pub max_diagnostics: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PullDiagnosticsResult {
    pub diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    pub errors: usize,
    pub skipped_diagnostics: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PullActionsParams {
    pub path: RomePath,
    pub range: TextRange,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct PullActionsResult {
    pub actions: Vec<CodeAction>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct CodeAction {
    pub category: ActionCategory,
    pub rule_name: Option<(Cow<'static, str>, Cow<'static, str>)>,
    pub suggestion: CodeSuggestion,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FormatFileParams {
    pub path: RomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FormatRangeParams {
    pub path: RomePath,
    pub range: TextRange,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FormatOnTypeParams {
    pub path: RomePath,
    pub offset: TextSize,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
/// Which fixes should be applied during the analyzing phase
pub enum FixFileMode {
    /// Applies [safe](biome_diagnostics::Applicability::Always) fixes
    SafeFixes,
    /// Applies [safe](biome_diagnostics::Applicability::Always) and [unsafe](biome_diagnostics::Applicability::MaybeIncorrect) fixes
    SafeAndUnsafeFixes,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FixFileParams {
    pub path: RomePath,
    pub fix_file_mode: FixFileMode,
    pub should_format: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FixFileResult {
    /// New source code for the file with all fixes applied
    pub code: String,
    /// List of all the code actions applied to the file
    pub actions: Vec<FixAction>,

    /// Number of errors
    pub errors: usize,

    /// number of skipped suggested fixes
    pub skipped_suggested_fixes: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct FixAction {
    /// Name of the rule group and rule that emitted this code action
    pub rule_name: Option<(Cow<'static, str>, Cow<'static, str>)>,
    /// Source range at which this action was applied
    pub range: TextRange,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RenameParams {
    pub path: RomePath,
    pub symbol_at: TextSize,
    pub new_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RenameResult {
    /// Range of source code modified by this rename operation
    pub range: TextRange,
    /// List of text edit operations to apply on the source code
    pub indels: TextEdit,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ServerInfo {
    /// The name of the server as defined by the server.
    pub name: String,

    /// The server's version as defined by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RageParams {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct RageResult {
    pub entries: Vec<RageEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum RageEntry {
    Section(String),
    Pair { name: String, value: MarkupBuf },
    Markup(MarkupBuf),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OrganizeImportsParams {
    pub path: RomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct OrganizeImportsResult {
    pub code: String,
}

impl RageEntry {
    pub fn section(name: &str) -> Self {
        Self::Section(name.to_string())
    }

    pub fn markup(markup: Markup) -> Self {
        Self::Markup(markup.to_owned())
    }

    pub fn pair(name: &str, value: &str) -> Self {
        Self::pair_markup(name, markup!({ value }))
    }

    pub fn pair_markup(name: &str, value: Markup) -> Self {
        Self::Pair {
            name: name.to_string(),
            value: value.to_owned(),
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct IsPathIgnoredParams {
    pub rome_path: RomePath,
    pub feature: FeatureName,
}

pub trait Workspace: Send + Sync + RefUnwindSafe {
    /// Checks whether a certain feature is supported. There are different conditions:
    /// - Biome doesn't recognize a file, so it can't provide the feature;
    /// - the feature is disabled inside the configuration;
    /// - the file is ignored
    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError>;

    /// Given a file, the workspace tries to understand if this file is a "manifest" of a project.
    fn project_features(
        &self,
        _params: ProjectFeaturesParams,
    ) -> Result<ProjectFeaturesResult, WorkspaceError> {
        todo!()
    }

    /// Checks if the current path is ignored by the workspace, against a particular feature.
    ///
    /// Takes as input the path of the file that workspace is currently processing and
    /// a list of paths to match against.
    ///
    /// If the file path matches, than `true` is returned and it should be considered ignored.
    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError>;

    /// Update the global settings for this workspace
    fn update_settings(&self, params: UpdateSettingsParams) -> Result<(), WorkspaceError>;

    /// Add a new file to the workspace
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError>;

    // Return a textual, debug representation of the syntax tree for a given document
    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, WorkspaceError>;

    // Return a textual, debug representation of the control flow graph at a given position in the document
    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, WorkspaceError>;

    // Return a textual, debug representation of the formatter IR for a given document
    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError>;

    /// Return the content of a file
    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError>;

    /// Change the content of an open file
    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError>;

    /// Remove a file from the workspace
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError>;

    /// Retrieves the list of diagnostics associated to a file
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError>;

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError>;

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, WorkspaceError>;

    /// Runs a range of an open document through the formatter
    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError>;

    /// Runs a "block" ending at the specified character of an open document
    /// through the formatter
    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError>;

    /// Return the content of the file with all safe code actions applied
    fn fix_file(&self, params: FixFileParams) -> Result<FixFileResult, WorkspaceError>;

    /// Return the content of the file after renaming a symbol
    fn rename(&self, params: RenameParams) -> Result<RenameResult, WorkspaceError>;

    /// Returns debug information about this workspace.
    fn rage(&self, params: RageParams) -> Result<RageResult, WorkspaceError>;

    /// Returns information about the server this workspace is connected to or `None` if the workspace isn't connected to a server.
    fn server_info(&self) -> Option<&ServerInfo>;

    /// Applies import sorting
    fn organize_imports(
        &self,
        params: OrganizeImportsParams,
    ) -> Result<OrganizeImportsResult, WorkspaceError>;
}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server() -> Box<dyn Workspace> {
    Box::new(server::WorkspaceServer::new())
}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server_sync() -> Arc<dyn Workspace> {
    Arc::new(server::WorkspaceServer::new())
}

/// Convenience function for constructing a client instance of [Workspace]
pub fn client<T>(transport: T) -> Result<Box<dyn Workspace>, WorkspaceError>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync + 'static,
{
    Ok(Box::new(client::WorkspaceClient::new(transport)?))
}

/// [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)
/// guard for an open file in a workspace, takes care of closing the file
/// automatically on drop
pub struct FileGuard<'app, W: Workspace + ?Sized> {
    workspace: &'app W,
    path: RomePath,
}

impl<'app, W: Workspace + ?Sized> FileGuard<'app, W> {
    pub fn open(workspace: &'app W, params: OpenFileParams) -> Result<Self, WorkspaceError> {
        let path = params.path.clone();
        workspace.open_file(params)?;
        Ok(Self { workspace, path })
    }

    pub fn get_syntax_tree(&self) -> Result<GetSyntaxTreeResult, WorkspaceError> {
        self.workspace.get_syntax_tree(GetSyntaxTreeParams {
            path: self.path.clone(),
        })
    }

    pub fn get_control_flow_graph(&self, cursor: TextSize) -> Result<String, WorkspaceError> {
        self.workspace
            .get_control_flow_graph(GetControlFlowGraphParams {
                path: self.path.clone(),
                cursor,
            })
    }

    pub fn change_file(&self, version: i32, content: String) -> Result<(), WorkspaceError> {
        self.workspace.change_file(ChangeFileParams {
            path: self.path.clone(),
            version,
            content,
        })
    }

    pub fn get_file_content(&self) -> Result<String, WorkspaceError> {
        self.workspace.get_file_content(GetFileContentParams {
            path: self.path.clone(),
        })
    }

    pub fn pull_diagnostics(
        &self,
        categories: RuleCategories,
        max_diagnostics: u64,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        self.workspace.pull_diagnostics(PullDiagnosticsParams {
            path: self.path.clone(),
            categories,
            max_diagnostics,
        })
    }

    pub fn pull_actions(&self, range: TextRange) -> Result<PullActionsResult, WorkspaceError> {
        self.workspace.pull_actions(PullActionsParams {
            path: self.path.clone(),
            range,
        })
    }

    pub fn format_file(&self) -> Result<Printed, WorkspaceError> {
        self.workspace.format_file(FormatFileParams {
            path: self.path.clone(),
        })
    }

    pub fn format_range(&self, range: TextRange) -> Result<Printed, WorkspaceError> {
        self.workspace.format_range(FormatRangeParams {
            path: self.path.clone(),
            range,
        })
    }

    pub fn format_on_type(&self, offset: TextSize) -> Result<Printed, WorkspaceError> {
        self.workspace.format_on_type(FormatOnTypeParams {
            path: self.path.clone(),
            offset,
        })
    }

    pub fn fix_file(
        &self,
        fix_file_mode: FixFileMode,
        should_format: bool,
    ) -> Result<FixFileResult, WorkspaceError> {
        self.workspace.fix_file(FixFileParams {
            path: self.path.clone(),
            fix_file_mode,
            should_format,
        })
    }

    pub fn organize_imports(&self) -> Result<OrganizeImportsResult, WorkspaceError> {
        self.workspace.organize_imports(OrganizeImportsParams {
            path: self.path.clone(),
        })
    }
}

impl<'app, W: Workspace + ?Sized> Drop for FileGuard<'app, W> {
    fn drop(&mut self) {
        self.workspace
            .close_file(CloseFileParams {
                path: self.path.clone(),
            })
            // `close_file` can only error if the file was already closed, in
            // this case it's generally better to silently matcher the error
            // than panic (especially in a drop handler)
            .ok();
    }
}

#[test]
fn test_order() {
    for items in FileFeaturesResult::PROTECTED_FILES.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
