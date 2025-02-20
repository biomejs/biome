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
//!     run on a path that doesn't correspond to any open document: either the
//!     document has been closed or the client didn't open it in the first place
//! - [WorkspaceError::SourceFileNotSupported]: This error is returned when an
//!     operation could not be completed because the language associated with the
//!     document does not implement the required capability: for instance trying to
//!     format a file with a language that does not have a formatter

mod client;
mod scanner;
mod server;

pub use self::client::{TransportRequest, WorkspaceClient, WorkspaceTransport};
use crate::file_handlers::Capabilities;
pub use crate::file_handlers::DocumentFileSource;
use crate::projects::ProjectKey;
use crate::settings::WorkspaceSettingsHandle;
use crate::{Deserialize, Serialize, WorkspaceError};
use biome_analyze::ActionCategory;
pub use biome_analyze::RuleCategories;
use biome_configuration::analyzer::RuleSelector;
use biome_configuration::Configuration;
use biome_console::{markup, Markup, MarkupBuf};
use biome_diagnostics::serde::Diagnostic;
use biome_diagnostics::CodeSuggestion;
use biome_formatter::Printed;
use biome_fs::{BiomePath, FileSystem};
use biome_grit_patterns::GritTargetLanguage;
use biome_js_syntax::{TextRange, TextSize};
use biome_text_edit::TextEdit;
use camino::Utf8Path;
use core::str;
use enumflags2::{bitflags, BitFlags};
#[cfg(feature = "schema")]
use schemars::{gen::SchemaGenerator, schema::Schema};
use smallvec::SmallVec;
use std::collections::HashMap;
use std::time::Duration;
use std::{borrow::Cow, panic::RefUnwindSafe, sync::Arc};
use tracing::{debug, instrument};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SupportsFeatureParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub features: FeatureName,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SupportsFeatureResult {
    pub reason: Option<SupportKind>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FileFeaturesResult {
    pub features_supported: HashMap<FeatureKind, SupportKind>,
}

impl std::fmt::Display for FileFeaturesResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (feature, support_kind) in self.features_supported.iter() {
            write!(f, "{}: {}, ", feature, support_kind)?;
        }
        Ok(())
    }
}

impl FileFeaturesResult {
    /// Sorted array of files that should not be processed no matter the cases.
    /// These files are handled by other tools.
    const PROTECTED_FILES: &'static [&'static str] = &[
        // Composer
        "composer.lock",
        // NPM
        "npm-shrinkwrap.json",
        "package-lock.json",
        // Yarn
        "yarn.lock",
    ];

    /// Checks whether this file is protected.
    /// A protected file is handled by a specific tool and should be ignored.
    pub(crate) fn is_protected_file(path: &Utf8Path) -> bool {
        path.file_name()
            .is_some_and(|filename| Self::PROTECTED_FILES.contains(&filename))
    }

    /// By default, all features are not supported by a file.
    const WORKSPACE_FEATURES: [(FeatureKind, SupportKind); 5] = [
        (FeatureKind::Lint, SupportKind::FileNotSupported),
        (FeatureKind::Format, SupportKind::FileNotSupported),
        (FeatureKind::Search, SupportKind::FileNotSupported),
        (FeatureKind::Assist, SupportKind::FileNotSupported),
        (FeatureKind::Debug, SupportKind::FileNotSupported),
    ];

    pub fn new() -> Self {
        Self {
            features_supported: HashMap::from(Self::WORKSPACE_FEATURES),
        }
    }

    /// Adds the features that are enabled in `capabilities` to this result.
    pub fn with_capabilities(mut self, capabilities: &Capabilities) -> Self {
        if capabilities.formatter.format.is_some() {
            self.features_supported
                .insert(FeatureKind::Format, SupportKind::Supported);
        }
        if capabilities.analyzer.lint.is_some() {
            self.features_supported
                .insert(FeatureKind::Lint, SupportKind::Supported);
        }

        if capabilities.analyzer.code_actions.is_some() {
            self.features_supported
                .insert(FeatureKind::Assist, SupportKind::Supported);
        }

        if capabilities.search.search.is_some() {
            self.features_supported
                .insert(FeatureKind::Search, SupportKind::Supported);
        }

        if capabilities.debug.debug_syntax_tree.is_some()
            || capabilities.debug.debug_formatter_ir.is_some()
            || capabilities.debug.debug_control_flow.is_some()
        {
            self.features_supported
                .insert(FeatureKind::Debug, SupportKind::Supported);
        }

        self
    }

    /// Checks if a feature is enabled for the current path.
    ///
    /// The method checks the configuration enables a certain feature for the given path.
    #[instrument(level = "debug", skip(self, handle, capabilities))]
    pub(crate) fn with_settings_and_language(
        mut self,
        handle: &WorkspaceSettingsHandle,
        path: &Utf8Path,
        capabilities: &Capabilities,
    ) -> Self {
        // formatter
        let formatter_enabled = capabilities.enabled_for_path.formatter;
        if let Some(formatter_enabled) = formatter_enabled {
            let formatter_enabled = formatter_enabled(path, handle);

            if !formatter_enabled {
                self.features_supported
                    .insert(FeatureKind::Format, SupportKind::FeatureNotEnabled);
            }
        }

        // linter
        let linter_enabled = capabilities.enabled_for_path.linter;
        if let Some(linter_enabled) = linter_enabled {
            let linter_enabled = linter_enabled(path, handle);
            if !linter_enabled {
                self.features_supported
                    .insert(FeatureKind::Lint, SupportKind::FeatureNotEnabled);
            }
        }
        // assist
        let assist_enabled = capabilities.enabled_for_path.assist;
        if let Some(assist_enabled) = assist_enabled {
            let assist_enabled = assist_enabled(path, handle);
            if !assist_enabled {
                self.features_supported
                    .insert(FeatureKind::Assist, SupportKind::FeatureNotEnabled);
            }
        }

        // search
        let search_enabled = capabilities.enabled_for_path.search;
        if let Some(search_enabled) = search_enabled {
            let search_enabled = search_enabled(path, handle);
            if !search_enabled {
                self.features_supported
                    .insert(FeatureKind::Search, SupportKind::FeatureNotEnabled);
            }
        }

        debug!(
            "The file has the following feature sets: {:?}",
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

    pub fn ignored(&mut self, feature: FeatureKind) {
        self.features_supported
            .insert(feature, SupportKind::Ignored);
    }

    /// Checks whether the file support the given `feature`
    fn supports_for(&self, feature: &FeatureKind) -> bool {
        self.features_supported
            .get(feature)
            .is_some_and(|support_kind| matches!(support_kind, SupportKind::Supported))
    }

    pub fn supports_lint(&self) -> bool {
        self.supports_for(&FeatureKind::Lint)
    }

    pub fn supports_format(&self) -> bool {
        self.supports_for(&FeatureKind::Format)
    }

    pub fn supports_assist(&self) -> bool {
        self.supports_for(&FeatureKind::Assist)
    }

    pub fn supports_search(&self) -> bool {
        self.supports_for(&FeatureKind::Search)
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

    pub fn support_kind_for(&self, feature: &FeatureKind) -> Option<&SupportKind> {
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

    /// The file is not supported if all the features are unsupported
    pub fn is_not_supported(&self) -> bool {
        self.features_supported
            .values()
            .all(|support_kind| support_kind.is_not_supported())
    }

    /// The file is not enabled if all the features aren't enabled
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
#[serde(rename_all = "camelCase")]
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

impl std::fmt::Display for SupportKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupportKind::Supported => write!(f, "Supported"),
            SupportKind::Ignored => write!(f, "Ignored"),
            SupportKind::Protected => write!(f, "Protected"),
            SupportKind::FeatureNotEnabled => write!(f, "FeatureNotEnabled"),
            SupportKind::FileNotSupported => write!(f, "FileNotSupported"),
        }
    }
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
#[bitflags]
#[repr(u8)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum FeatureKind {
    Format,
    Lint,
    Search,
    Assist,
    Debug,
}

impl std::fmt::Display for FeatureKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureKind::Format => write!(f, "Format"),
            FeatureKind::Lint => write!(f, "Lint"),
            FeatureKind::Search => write!(f, "Search"),
            FeatureKind::Assist => write!(f, "Assist"),
            FeatureKind::Debug => write!(f, "Debug"),
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(
    from = "smallvec::SmallVec<[FeatureKind; 6]>",
    into = "smallvec::SmallVec<[FeatureKind; 6]>",
    rename_all = "camelCase"
)]
pub struct FeatureName(BitFlags<FeatureKind>);

impl FeatureName {
    pub fn iter(&self) -> enumflags2::Iter<FeatureKind> {
        self.0.iter()
    }
    pub fn empty() -> Self {
        Self(BitFlags::empty())
    }

    pub fn insert(&mut self, kind: FeatureKind) {
        self.0.insert(kind);
    }
}

impl From<SmallVec<[FeatureKind; 6]>> for FeatureName {
    fn from(value: SmallVec<[FeatureKind; 6]>) -> Self {
        value
            .into_iter()
            .fold(FeatureName::empty(), |mut acc, kind| {
                acc.insert(kind);
                acc
            })
    }
}

impl From<FeatureName> for SmallVec<[FeatureKind; 6]> {
    fn from(value: FeatureName) -> Self {
        value.iter().collect()
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for FeatureName {
    fn schema_name() -> String {
        String::from("FeatureName")
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        <Vec<FeatureKind>>::json_schema(gen)
    }
}

#[derive(Debug, Default)]
pub struct FeaturesBuilder(BitFlags<FeatureKind>);

impl FeaturesBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_formatter(mut self) -> Self {
        self.0.insert(FeatureKind::Format);
        self
    }

    pub fn with_linter(mut self) -> Self {
        self.0.insert(FeatureKind::Lint);
        self
    }

    pub fn with_search(mut self) -> Self {
        self.0.insert(FeatureKind::Search);
        self
    }

    pub fn with_assist(mut self) -> Self {
        self.0.insert(FeatureKind::Assist);
        self
    }

    pub fn build(self) -> FeatureName {
        FeatureName(self.0)
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsParams {
    pub project_key: ProjectKey,
    pub configuration: Configuration,
    // @ematipico TODO: have a better data structure for this
    pub vcs_base_path: Option<BiomePath>,
    // @ematipico TODO: have a better data structure for this
    pub gitignore_matches: Vec<String>,
    pub workspace_directory: Option<BiomePath>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct UpdateSettingsResult {
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ProjectFeaturesParams {
    pub manifest_path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ProjectFeaturesResult {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct OpenFileParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub content: FileContent,
    pub version: i32,
    pub document_file_source: Option<DocumentFileSource>,

    /// Set to `true` to persist the node cache used during parsing, in order to
    /// speed up subsequent reparsing if the document has been edited.
    ///
    /// This should only be enabled if reparsing is to be expected, such as when
    /// the file is opened through the LSP Proxy.
    #[serde(default)]
    pub persist_node_cache: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", tag = "type", content = "content")]
pub enum FileContent {
    /// The client has loaded the content and submits it to the server.
    FromClient(String),

    /// The server will be responsible for loading the content from the file system.
    FromServer,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetSyntaxTreeParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetSyntaxTreeResult {
    pub cst: String,
    pub ast: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetControlFlowGraphParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub cursor: TextSize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetFormatterIRParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetFileContentParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CheckFileSizeParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CheckFileSizeResult {
    pub file_size: usize,
    pub limit: usize,
}

impl CheckFileSizeResult {
    pub fn is_too_large(&self) -> bool {
        self.file_size >= self.limit
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ChangeFileParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub content: String,
    pub version: i32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CloseFileParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PullDiagnosticsParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub categories: RuleCategories,
    pub max_diagnostics: u64,
    #[serde(default)]
    pub only: Vec<RuleSelector>,
    #[serde(default)]
    pub skip: Vec<RuleSelector>,
    /// Rules to apply on top of the configuration
    #[serde(default)]
    pub enabled_rules: Vec<RuleSelector>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PullDiagnosticsResult {
    pub diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    pub errors: usize,
    pub skipped_diagnostics: u64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PullActionsParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub range: Option<TextRange>,
    pub suppression_reason: Option<String>,
    #[serde(default)]
    pub only: Vec<RuleSelector>,
    #[serde(default)]
    pub skip: Vec<RuleSelector>,
    #[serde(default)]
    pub enabled_rules: Vec<RuleSelector>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PullActionsResult {
    pub actions: Vec<CodeAction>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CodeAction {
    pub category: ActionCategory,
    pub rule_name: Option<(Cow<'static, str>, Cow<'static, str>)>,
    pub suggestion: CodeSuggestion,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FormatFileParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FormatRangeParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub range: TextRange,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FormatOnTypeParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub offset: TextSize,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
/// Which fixes should be applied during the analyzing phase
pub enum FixFileMode {
    /// Applies [safe](biome_diagnostics::Applicability::Always) fixes
    SafeFixes,
    /// Applies [safe](biome_diagnostics::Applicability::Always) and [unsafe](biome_diagnostics::Applicability::MaybeIncorrect) fixes
    SafeAndUnsafeFixes,
    /// Applies suppression comments to existing diagnostics when using `--suppress`
    ApplySuppressions,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FixFileParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub fix_file_mode: FixFileMode,
    pub should_format: bool,
    #[serde(default)]
    pub only: Vec<RuleSelector>,
    #[serde(default)]
    pub skip: Vec<RuleSelector>,
    /// Rules to apply to the file
    #[serde(default)]
    pub enabled_rules: Vec<RuleSelector>,
    pub rule_categories: RuleCategories,
    #[serde(default)]
    pub suppression_reason: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
pub struct RenameParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub symbol_at: TextSize,
    pub new_name: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RenameResult {
    /// Range of source code modified by this rename operation
    pub range: TextRange,
    /// List of text edit operations to apply on the source code
    pub indels: TextEdit,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ScanProjectFolderResult {
    /// Diagnostics reported while scanning the project.
    pub diagnostics: Vec<Diagnostic>,

    /// Duration of the scan.
    pub duration: Duration,
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    /// The name of the server as defined by the server.
    pub name: String,

    /// The server's version as defined by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RageParams {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct RageResult {
    pub entries: Vec<RageEntry>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum RageEntry {
    Section(String),
    Pair { name: String, value: MarkupBuf },
    Markup(MarkupBuf),
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
#[serde(rename_all = "camelCase")]
pub struct ParsePatternParams {
    pub pattern: String,
    pub default_language: GritTargetLanguage,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ParsePatternResult {
    pub pattern_id: PatternId,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SearchPatternParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub pattern: PatternId,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub path: BiomePath,
    pub matches: Vec<TextRange>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct DropPatternParams {
    pub pattern: PatternId,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PatternId(String);

impl std::fmt::Display for PatternId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PatternId> for String {
    fn from(value: PatternId) -> Self {
        value.0
    }
}

impl From<String> for PatternId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for PatternId {
    fn from(value: &str) -> Self {
        Self(value.to_owned())
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct IsPathIgnoredParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub features: FeatureName,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct OpenProjectParams {
    /// The path to open
    pub path: BiomePath,

    /// Whether the folder should be opened as a project, even if no
    /// `biome.json` can be found.
    pub open_uninitialized: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ScanProjectFolderParams {
    pub project_key: ProjectKey,

    /// Optional path within the project to scan.
    ///
    /// If omitted, the project is scanned from its root folder.
    ///
    /// This is a potential optimization that allows scanning to be limited to
    /// a subset of the full project. Clients should specify it to indicate
    /// which part of the project they are interested in. The server may or may
    /// not use this to avoid scanning parts that are irrelevant to clients.
    pub path: Option<BiomePath>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CloseProjectParams {
    pub project_key: ProjectKey,
}

pub trait Workspace: Send + Sync + RefUnwindSafe {
    // #region PROJECT-LEVEL METHODS

    /// Opens a project within the workspace.
    ///
    /// The path given as part of the `params` is used to determine the root of
    /// the Biome project, but it may not have to be the root itself. The first
    /// ancestor directory of the path (including the path itself) that contains
    /// a **top-level** `biome.json` is determined to be the project root and
    /// will be used for opening the project.
    ///
    /// If no `biome.json` file can be found in any of the ancestors, and
    /// `open_uninitialized` is `true` the directory will be opened as a Biome
    /// project will default settings.
    ///
    /// Note: Opening a project does not mean the project is ready for use. You
    /// probably want to follow it up with a call to `scan_project_folder()` or
    /// explicitly load settings into the project using `update_settings()`.
    ///
    /// Returns the key of the opened project. This key can be used with
    /// follow-up methods to perform actions related to the project, such as
    /// opening files or querying them.
    fn open_project(&self, params: OpenProjectParams) -> Result<ProjectKey, WorkspaceError>;

    /// Scans the given project from a given path, and initializes all settings
    /// and service data.
    ///
    /// The first time you call this method, it may take a long time since it
    /// will traverse the entire project folder recursively, parse all included
    /// files (and possibly their dependencies), and perform processing for
    /// extracting the service data.
    ///
    /// Follow-up calls may be much faster as they can reuse cached data.
    ///
    /// TODO: This method also registers file watchers to make sure the cache
    ///       remains up-to-date.
    fn scan_project_folder(
        &self,
        params: ScanProjectFolderParams,
    ) -> Result<ScanProjectFolderResult, WorkspaceError>;

    /// Updates the global settings for the given project.
    ///
    /// TODO: This method should not be used in combination with
    /// `scan_project_folder()`. When scanning is enabled, the server should
    /// manage project settings on its own.
    fn update_settings(
        &self,
        params: UpdateSettingsParams,
    ) -> Result<UpdateSettingsResult, WorkspaceError>;

    /// Closes the project with the given key.
    ///
    /// Any settings related to the project are unloaded, and any open files
    /// that belong to the project are also closed.
    ///
    /// Projects should normally **not** be closed, so that follow-up
    /// invocations to Biome can reuse the loaded settings and open files. Only
    /// when the user explicitly asks for a project to be closed, will we do so
    /// in order to allow the user to reclaim memory. (Although in reality
    /// they'll probably just kill the daemon anyway.)
    ///
    /// If a file watcher was registered as a result of a call to
    /// `scan_project_folder()`, it will also be unregistered.
    fn close_project(&self, params: CloseProjectParams) -> Result<(), WorkspaceError>;

    // #endregion

    // #region FILE-LEVEL METHODS

    /// Opens a new file in the workspace.
    ///
    /// If the file path is under a folder that belongs to an opened project
    /// other than the current one, the current project is changed accordingly.
    fn open_file(&self, params: OpenFileParams) -> Result<(), WorkspaceError>;

    /// Checks whether a certain feature is supported for the given file path.
    ///
    /// There are different conditions:
    /// - Biome doesn't recognize a file, so it can't provide the feature;
    /// - the feature is disabled inside the configuration;
    /// - the file is ignored
    fn file_features(
        &self,
        params: SupportsFeatureParams,
    ) -> Result<FileFeaturesResult, WorkspaceError>;

    /// Checks if the given path is ignored by the project for a specific
    /// feature.
    ///
    /// If the file path matches, `true` is returned, and it should be
    /// considered ignored.
    fn is_path_ignored(&self, params: IsPathIgnoredParams) -> Result<bool, WorkspaceError>;

    /// Returns a textual, debug representation of the syntax tree for a given
    /// document.
    fn get_syntax_tree(
        &self,
        params: GetSyntaxTreeParams,
    ) -> Result<GetSyntaxTreeResult, WorkspaceError>;

    /// Returns a textual, debug representation of the control flow graph at a
    /// given position in the document.
    fn get_control_flow_graph(
        &self,
        params: GetControlFlowGraphParams,
    ) -> Result<String, WorkspaceError>;

    /// Returns a textual, debug representation of the formatter IR for a given
    /// document.
    fn get_formatter_ir(&self, params: GetFormatterIRParams) -> Result<String, WorkspaceError>;

    /// Returns the content of a given file.
    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError>;

    /// Returns the size of a given file, as well as the allowed maximum file
    /// size for that file.
    fn check_file_size(
        &self,
        params: CheckFileSizeParams,
    ) -> Result<CheckFileSizeResult, WorkspaceError>;

    /// Changes the content of an open file.
    fn change_file(&self, params: ChangeFileParams) -> Result<(), WorkspaceError>;

    /// Retrieves the list of diagnostics associated with a file.
    fn pull_diagnostics(
        &self,
        params: PullDiagnosticsParams,
    ) -> Result<PullDiagnosticsResult, WorkspaceError>;

    /// Retrieves the list of code actions available for a given cursor
    /// position within a file.
    fn pull_actions(&self, params: PullActionsParams) -> Result<PullActionsResult, WorkspaceError>;

    /// Runs the given file through the formatter using the provided options
    /// and returns the resulting source code.
    fn format_file(&self, params: FormatFileParams) -> Result<Printed, WorkspaceError>;

    /// Runs a range of an open document through the formatter.
    fn format_range(&self, params: FormatRangeParams) -> Result<Printed, WorkspaceError>;

    /// Runs a "block" ending at the specified character of an open document
    /// through the formatter.
    fn format_on_type(&self, params: FormatOnTypeParams) -> Result<Printed, WorkspaceError>;

    /// Returns the content of the file with all safe code actions applied.
    fn fix_file(&self, params: FixFileParams) -> Result<FixFileResult, WorkspaceError>;

    /// Returns the content of the file after renaming a symbol.
    fn rename(&self, params: RenameParams) -> Result<RenameResult, WorkspaceError>;

    /// Closes a file that is opened in the workspace.
    ///
    /// This only unloads the document from the workspace if the file is NOT
    /// opened by the scanner as well. If the scanner has opened the file, it
    /// may still be required for multi-file analysis.
    fn close_file(&self, params: CloseFileParams) -> Result<(), WorkspaceError>;

    /// Returns the filesystem implementation to open files with.
    ///
    /// This may be an in-memory file system.
    fn fs(&self) -> &dyn FileSystem;

    // #endregion

    // #region SEARCH-RELATED METHODS

    /// Parses a pattern to be used in follow-up [`Self::search_pattern`]
    /// requests.
    ///
    /// Clients should call [`Self::drop_pattern()`] when they no need longer
    /// need it.
    fn parse_pattern(
        &self,
        params: ParsePatternParams,
    ) -> Result<ParsePatternResult, WorkspaceError>;

    /// Searches a file for matches of the given pattern.
    fn search_pattern(&self, params: SearchPatternParams) -> Result<SearchResults, WorkspaceError>;

    /// Used to indicate a client no longer needs a specific pattern.
    fn drop_pattern(&self, params: DropPatternParams) -> Result<(), WorkspaceError>;

    // #endregion

    // #region MISC METHODS

    /// Returns debug information about this workspace.
    fn rage(&self, params: RageParams) -> Result<RageResult, WorkspaceError>;

    /// Returns information about the server this workspace is connected to or
    /// `None` if the workspace isn't connected to a server.
    fn server_info(&self) -> Option<&ServerInfo>;

    // #endregion
}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server(fs: Box<dyn FileSystem>) -> Box<dyn Workspace> {
    Box::new(server::WorkspaceServer::new(fs))
}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server_sync(fs: Box<dyn FileSystem>) -> Arc<dyn Workspace> {
    Arc::new(server::WorkspaceServer::new(fs))
}

/// Convenience function for constructing a client instance of [Workspace]
pub fn client<T>(
    transport: T,
    fs: Box<dyn FileSystem>,
) -> Result<Box<dyn Workspace>, WorkspaceError>
where
    T: WorkspaceTransport + RefUnwindSafe + Send + Sync + 'static,
{
    Ok(Box::new(client::WorkspaceClient::new(transport, fs)?))
}

/// [RAII](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)
/// guard for an open file in a workspace, takes care of closing the file
/// automatically on drop
pub struct FileGuard<'app, W: Workspace + ?Sized> {
    workspace: &'app W,
    project_key: ProjectKey,
    path: BiomePath,
}

impl<'app, W: Workspace + ?Sized> FileGuard<'app, W> {
    pub fn open(workspace: &'app W, params: OpenFileParams) -> Result<Self, WorkspaceError> {
        let project_key = params.project_key;
        let path = params.path.clone();
        workspace.open_file(params)?;
        Ok(Self {
            workspace,
            project_key,
            path,
        })
    }

    pub fn get_syntax_tree(&self) -> Result<GetSyntaxTreeResult, WorkspaceError> {
        self.workspace.get_syntax_tree(GetSyntaxTreeParams {
            project_key: self.project_key,
            path: self.path.clone(),
        })
    }

    pub fn get_control_flow_graph(&self, cursor: TextSize) -> Result<String, WorkspaceError> {
        self.workspace
            .get_control_flow_graph(GetControlFlowGraphParams {
                project_key: self.project_key,
                path: self.path.clone(),
                cursor,
            })
    }

    pub fn change_file(&self, version: i32, content: String) -> Result<(), WorkspaceError> {
        self.workspace.change_file(ChangeFileParams {
            project_key: self.project_key,
            path: self.path.clone(),
            version,
            content,
        })
    }

    pub fn get_file_content(&self) -> Result<String, WorkspaceError> {
        self.workspace.get_file_content(GetFileContentParams {
            project_key: self.project_key,
            path: self.path.clone(),
        })
    }

    pub fn pull_diagnostics(
        &self,
        categories: RuleCategories,
        max_diagnostics: u32,
        only: Vec<RuleSelector>,
        skip: Vec<RuleSelector>,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        self.workspace.pull_diagnostics(PullDiagnosticsParams {
            project_key: self.project_key,
            path: self.path.clone(),
            categories,
            max_diagnostics: max_diagnostics.into(),
            only,
            skip,
            enabled_rules: vec![],
        })
    }

    pub fn pull_actions(
        &self,
        range: Option<TextRange>,
        only: Vec<RuleSelector>,
        skip: Vec<RuleSelector>,
        suppression_reason: Option<String>,
        enabled_rules: Vec<RuleSelector>,
    ) -> Result<PullActionsResult, WorkspaceError> {
        self.workspace.pull_actions(PullActionsParams {
            project_key: self.project_key,
            path: self.path.clone(),
            range,
            only,
            skip,
            suppression_reason,
            enabled_rules,
        })
    }

    pub fn format_file(&self) -> Result<Printed, WorkspaceError> {
        self.workspace.format_file(FormatFileParams {
            project_key: self.project_key,
            path: self.path.clone(),
        })
    }

    pub fn check_file_size(&self) -> Result<CheckFileSizeResult, WorkspaceError> {
        self.workspace.check_file_size(CheckFileSizeParams {
            project_key: self.project_key,
            path: self.path.clone(),
        })
    }

    pub fn format_range(&self, range: TextRange) -> Result<Printed, WorkspaceError> {
        self.workspace.format_range(FormatRangeParams {
            project_key: self.project_key,
            path: self.path.clone(),
            range,
        })
    }

    pub fn format_on_type(&self, offset: TextSize) -> Result<Printed, WorkspaceError> {
        self.workspace.format_on_type(FormatOnTypeParams {
            project_key: self.project_key,
            path: self.path.clone(),
            offset,
        })
    }

    pub fn fix_file(
        &self,
        fix_file_mode: FixFileMode,
        should_format: bool,
        rule_categories: RuleCategories,
        only: Vec<RuleSelector>,
        skip: Vec<RuleSelector>,
        suppression_reason: Option<String>,
    ) -> Result<FixFileResult, WorkspaceError> {
        self.workspace.fix_file(FixFileParams {
            project_key: self.project_key,
            path: self.path.clone(),
            fix_file_mode,
            should_format,
            only,
            skip,
            rule_categories,
            suppression_reason,
            enabled_rules: vec![],
        })
    }

    pub fn search_pattern(&self, pattern: &PatternId) -> Result<SearchResults, WorkspaceError> {
        self.workspace.search_pattern(SearchPatternParams {
            project_key: self.project_key,
            path: self.path.clone(),
            pattern: pattern.clone(),
        })
    }
}

impl<W: Workspace + ?Sized> Drop for FileGuard<'_, W> {
    fn drop(&mut self) {
        self.workspace
            .close_file(CloseFileParams {
                project_key: self.project_key,
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
