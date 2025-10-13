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
//!   run on a path that doesn't correspond to any open document: either the
//!   document has been closed or the client didn't open it in the first place
//! - [WorkspaceError::SourceFileNotSupported]: This error is returned when an
//!   operation could not be completed because the language associated with the
//!   document does not implement the required capability: for instance trying to
//!   format a file with a language that does not have a formatter

mod client;
mod document;
mod server;

pub use document::{AnyEmbeddedSnippet, EmbeddedSnippet};
use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter},
    panic::RefUnwindSafe,
    str,
    sync::Arc,
    time::Duration,
};

use biome_analyze::{ActionCategory, RuleCategories};
use biome_configuration::{Configuration, analyzer::AnalyzerSelector};
use biome_console::{Markup, MarkupBuf, markup};
use biome_diagnostics::{CodeSuggestion, serde::Diagnostic};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_grit_patterns::GritTargetLanguage;
use biome_js_syntax::{TextRange, TextSize};
use biome_module_graph::SerializedJsModuleInfo;
use biome_resolver::FsWithResolverProxy;
use biome_text_edit::TextEdit;
use camino::Utf8Path;
use crossbeam::channel::bounded;
use enumflags2::{BitFlags, bitflags};
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
pub use server::WorkspaceServer;
use smallvec::SmallVec;
use tokio::sync::watch;
use tracing::debug;

pub use crate::{
    WorkspaceError,
    file_handlers::{Capabilities, DocumentFileSource},
    projects::ProjectKey,
    scanner::ScanKind,
    settings::Settings,
};
#[cfg(feature = "schema")]
use schemars::{r#gen::SchemaGenerator, schema::Schema};

pub use client::{TransportRequest, WorkspaceClient, WorkspaceTransport};
pub use server::OpenFileReason;

/// Notification regarding a workspace's service data.
#[derive(Clone, Copy, Debug)]
pub enum ServiceNotification {
    /// Notifies that some file or folder's index has been updated.
    IndexUpdated,

    /// Workspace watcher has stopped and no more service data updates are
    /// expected.
    WatcherStopped,
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FeaturesSupported([SupportKind; NUM_FEATURE_KINDS]);

impl FeaturesSupported {
    /// By default, a file does not support all features.
    const WORKSPACE_FEATURES: [SupportKind; NUM_FEATURE_KINDS] = [
        SupportKind::FileNotSupported,
        SupportKind::FileNotSupported,
        SupportKind::FileNotSupported,
        SupportKind::FileNotSupported,
        SupportKind::FileNotSupported,
        SupportKind::FileNotSupported,
    ];

    #[inline]
    fn insert(&mut self, feature: FeatureKind, support: SupportKind) {
        self.0[feature.index()] = support;
    }

    /// Adds the features that are enabled in `capabilities` to this result.
    #[inline]
    pub fn with_capabilities(mut self, capabilities: &Capabilities) -> Self {
        if capabilities.formatter.format.is_some() {
            self.insert(FeatureKind::Format, SupportKind::Supported);
        }
        if capabilities.analyzer.lint.is_some() {
            self.insert(FeatureKind::Lint, SupportKind::Supported);
        }
        if capabilities.analyzer.code_actions.is_some() {
            self.insert(FeatureKind::Assist, SupportKind::Supported);
        }
        if capabilities.search.search.is_some() {
            self.insert(FeatureKind::Search, SupportKind::Supported);
        }

        if capabilities.debug.debug_syntax_tree.is_some()
            || capabilities.debug.debug_formatter_ir.is_some()
            || capabilities.debug.debug_control_flow.is_some()
            || capabilities.debug.debug_type_info.is_some()
            || capabilities.debug.debug_registered_types.is_some()
        {
            self.insert(FeatureKind::Debug, SupportKind::Supported);
        }

        self
    }

    /// Checks if a feature is enabled for the current path.
    ///
    /// The method checks the configuration enables a certain feature for the given path.
    #[inline]
    pub(crate) fn with_settings_and_language(
        mut self,
        settings: &Settings,
        path: &Utf8Path,
        capabilities: &Capabilities,
    ) -> Self {
        // formatter
        let formatter_enabled = capabilities.enabled_for_path.formatter;
        if let Some(formatter_enabled) = formatter_enabled {
            let formatter_enabled = formatter_enabled(path, settings);

            if !formatter_enabled {
                self.insert(FeatureKind::Format, SupportKind::FeatureNotEnabled);
            }
        }

        // linter
        let linter_enabled = capabilities.enabled_for_path.linter;
        if let Some(linter_enabled) = linter_enabled {
            let linter_enabled = linter_enabled(path, settings);
            if !linter_enabled {
                self.insert(FeatureKind::Lint, SupportKind::FeatureNotEnabled);
            }
        }
        // assist
        let assist_enabled = capabilities.enabled_for_path.assist;
        if let Some(assist_enabled) = assist_enabled {
            let assist_enabled = assist_enabled(path, settings);
            if !assist_enabled {
                self.insert(FeatureKind::Assist, SupportKind::FeatureNotEnabled);
            }
        }

        // search
        let search_enabled = capabilities.enabled_for_path.search;
        if let Some(search_enabled) = search_enabled {
            let search_enabled = search_enabled(path, settings);
            if !search_enabled {
                self.insert(FeatureKind::Search, SupportKind::FeatureNotEnabled);
            }
        }

        if let Some(experimental_full_html_support) = settings.experimental_full_html_support
            && experimental_full_html_support.value()
        {
            self.insert(FeatureKind::HtmlFullSupport, SupportKind::Supported);
        }

        debug!("The file has the following feature sets: {:?}", &self);

        self
    }

    /// The file will be ignored for all features
    #[inline]
    pub fn set_ignored_for_all_features(&mut self) {
        for support_kind in self.0.iter_mut() {
            *support_kind = SupportKind::Ignored;
        }
    }

    /// The file will be protected for all features
    #[inline]
    pub fn set_protected_for_all_features(&mut self) {
        for support_kind in self.0.iter_mut() {
            *support_kind = SupportKind::Protected;
        }
    }

    #[inline]
    pub fn set_ignored(&mut self, feature: FeatureKind) {
        self.insert(feature, SupportKind::Ignored);
    }

    /// Checks whether the file support the given `feature`
    #[inline]
    fn supports(&self, feature: FeatureKind) -> bool {
        let support_kind = self.0[feature.index()];
        matches!(support_kind, SupportKind::Supported)
    }

    pub fn supports_lint(&self) -> bool {
        self.supports(FeatureKind::Lint)
    }

    pub fn supports_format(&self) -> bool {
        self.supports(FeatureKind::Format)
    }

    pub fn supports_assist(&self) -> bool {
        self.supports(FeatureKind::Assist)
    }

    pub fn supports_search(&self) -> bool {
        self.supports(FeatureKind::Search)
    }

    // TODO: remove once html full support is stable
    pub fn supports_full_html_support(&self) -> bool {
        self.supports(FeatureKind::HtmlFullSupport)
    }

    /// Returns the [`SupportKind`] for the given `feature`, but only if it is
    /// not enabled.
    #[inline(always)]
    pub fn support_kind_for(&self, feature: FeatureKind) -> SupportKind {
        self.0[feature.index()]
    }

    /// Returns the [`SupportKind`] for the given `feature`, but only if it is
    /// not enabled.
    pub fn support_kind_if_not_enabled(&self, feature: FeatureKind) -> Option<SupportKind> {
        let support_kind = self.support_kind_for(feature);
        if support_kind.is_not_enabled() {
            Some(support_kind)
        } else {
            None
        }
    }

    /// Loops through all the features of the current file, and if a feature is [SupportKind::FileNotSupported],
    /// it gets changed to [SupportKind::Ignored]
    pub fn ignore_not_supported(&mut self) {
        for support_kind in self.0.iter_mut() {
            if matches!(support_kind, SupportKind::FileNotSupported) {
                *support_kind = SupportKind::Ignored;
            }
        }
    }

    /// If at least one feature is supported, the file is supported
    pub fn is_supported(&self) -> bool {
        self.0
            .iter()
            .any(|support_kind| support_kind.is_supported())
    }

    /// The file is ignored only if all the features marked it as ignored
    pub fn is_ignored(&self) -> bool {
        self.0.iter().all(|support_kind| support_kind.is_ignored())
    }

    /// The file is protected only if all the features marked it as protected
    pub fn is_protected(&self) -> bool {
        self.0
            .iter()
            .all(|support_kind| support_kind.is_protected())
    }

    /// The file is not supported if all the features are unsupported
    pub fn is_not_supported(&self) -> bool {
        self.0
            .iter()
            .all(|support_kind| support_kind.is_not_supported())
    }

    /// The file is not enabled if all the features aren't enabled
    pub fn is_not_enabled(&self) -> bool {
        self.0
            .iter()
            .all(|support_kind| support_kind.is_not_enabled())
    }

    /// The file is not processed if for every enabled feature
    /// the file is either protected, not supported, ignored.
    #[inline]
    pub fn is_not_processed(&self) -> bool {
        self.0.iter().all(|support_kind| {
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

impl Default for FeaturesSupported {
    fn default() -> Self {
        Self(Self::WORKSPACE_FEATURES)
    }
}

impl Display for FeaturesSupported {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_map();
        for (index, support_kind) in self.0.iter().enumerate() {
            let feature = FeatureKind::from_index(index);
            dbg.key(&feature).value(&support_kind);
        }
        dbg.finish()
    }
}

impl serde::Serialize for FeaturesSupported {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeMap;

        let mut map = serializer.serialize_map(Some(NUM_FEATURE_KINDS))?;
        for (index, support_kind) in self.0.iter().enumerate() {
            let feature = FeatureKind::from_index(index);
            map.serialize_entry(&feature, support_kind)?;
        }

        map.end()
    }
}

struct FeaturesSupportedVisitor;

impl<'de> serde::de::Visitor<'de> for FeaturesSupportedVisitor {
    type Value = FeaturesSupported;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a map of supported features")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut features = FeaturesSupported::WORKSPACE_FEATURES;
        while let Some((key, value)) = map.next_entry::<FeatureKind, SupportKind>()? {
            features[key.index()] = value;
        }

        Ok(FeaturesSupported(features))
    }
}

impl<'de> serde::Deserialize<'de> for FeaturesSupported {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_map(FeaturesSupportedVisitor)
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for FeaturesSupported {
    fn schema_name() -> String {
        "FeaturesSupported".to_owned()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        use schemars::schema::*;

        Schema::Object(SchemaObject {
            instance_type: Some(InstanceType::Object.into()),
            object: Some(Box::new(ObjectValidation {
                property_names: Some(Box::new(generator.subschema_for::<FeatureKind>())),
                additional_properties: Some(Box::new(generator.subschema_for::<SupportKind>())),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FileFeaturesResult {
    pub features_supported: FeaturesSupported,
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

    pub fn new(features_supported: FeaturesSupported) -> Self {
        Self { features_supported }
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

#[derive(Clone, Copy, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
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
            Self::Supported => write!(f, "Supported"),
            Self::Ignored => write!(f, "Ignored"),
            Self::Protected => write!(f, "Protected"),
            Self::FeatureNotEnabled => write!(f, "FeatureNotEnabled"),
            Self::FileNotSupported => write!(f, "FileNotSupported"),
        }
    }
}

impl SupportKind {
    pub const fn is_supported(&self) -> bool {
        matches!(self, Self::Supported)
    }
    pub const fn is_not_enabled(&self) -> bool {
        matches!(self, Self::FeatureNotEnabled)
    }
    pub const fn is_not_supported(&self) -> bool {
        matches!(self, Self::FileNotSupported)
    }
    pub const fn is_ignored(&self) -> bool {
        matches!(self, Self::Ignored)
    }
    pub const fn is_protected(&self) -> bool {
        matches!(self, Self::Protected)
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
    // TODO: remove once full HTML support is stable
    HtmlFullSupport,
}

pub const NUM_FEATURE_KINDS: usize = 6;

impl Display for FeatureKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Format => write!(f, "Format"),
            Self::Lint => write!(f, "Lint"),
            Self::Search => write!(f, "Search"),
            Self::Assist => write!(f, "Assist"),
            Self::Debug => write!(f, "Debug"),
            Self::HtmlFullSupport => write!(f, "HtmlFullSupport"),
        }
    }
}

impl FeatureKind {
    /// Returns the feature kind from its index.
    ///
    /// ## Panics
    ///
    /// Panics if the index is higher than or equal to [`NUM_FEATURE_KINDS`].
    fn from_index(index: usize) -> Self {
        match index {
            0 => Self::Format,
            1 => Self::Lint,
            2 => Self::Search,
            3 => Self::Assist,
            4 => Self::Debug,
            5 => Self::HtmlFullSupport,
            _ => unreachable!("invalid index for FeatureKind"),
        }
    }

    /// Returns the index for the feature kind.
    #[inline]
    fn index(self) -> usize {
        match self {
            Self::Format => 0,
            Self::Lint => 1,
            Self::Search => 2,
            Self::Assist => 3,
            Self::Debug => 4,
            Self::HtmlFullSupport => 5,
        }
    }
}

#[derive(Copy, Clone, Hash, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
#[serde(
    from = "smallvec::SmallVec<[FeatureKind; 6]>",
    into = "smallvec::SmallVec<[FeatureKind; 6]>",
    rename_all = "camelCase"
)]
pub struct FeatureName(BitFlags<FeatureKind>);

impl Display for FeatureName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Debug for FeatureName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for kind in self.iter() {
            match kind {
                FeatureKind::Format => list.entry(&"Format"),
                FeatureKind::Lint => list.entry(&"Lint"),
                FeatureKind::Search => list.entry(&"Search"),
                FeatureKind::Assist => list.entry(&"Assist"),
                FeatureKind::Debug => list.entry(&"Debug"),
                FeatureKind::HtmlFullSupport => list.entry(&"HtmlFullSupport"),
            };
        }
        list.finish()
    }
}

impl FeatureName {
    pub fn iter(&self) -> enumflags2::Iter<FeatureKind> {
        self.0.iter()
    }
    pub fn empty() -> Self {
        Self(BitFlags::empty())
    }

    pub fn all() -> Self {
        Self(BitFlags::all())
    }

    pub fn insert(&mut self, kind: FeatureKind) {
        self.0.insert(kind);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<SmallVec<[FeatureKind; 6]>> for FeatureName {
    fn from(value: SmallVec<[FeatureKind; 6]>) -> Self {
        value.into_iter().fold(Self::empty(), |mut acc, kind| {
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

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        <Vec<FeatureKind>>::json_schema(generator)
    }
}

#[derive(Debug)]
pub struct FeaturesBuilder(BitFlags<FeatureKind>);

impl Default for FeaturesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FeaturesBuilder {
    pub fn new() -> Self {
        Self(BitFlags::empty())
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

    pub fn with_all(mut self) -> Self {
        self.0.insert(FeatureKind::Format);
        self.0.insert(FeatureKind::Lint);
        self.0.insert(FeatureKind::Search);
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
#[serde(rename_all = "camelCase")]
pub struct OpenFileResult {
    diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum FileContent {
    /// The client has loaded the content and submits it to the server.
    FromClient { content: String, version: i32 },

    /// The server will be responsible for loading the content from the file system.
    FromServer,
}

impl FileContent {
    pub fn from_client(content: impl Into<String>) -> Self {
        Self::FromClient {
            content: content.into(),
            version: 0,
        }
    }
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
pub struct GetTypeInfoParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetRegisteredTypesParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetSemanticModelParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetModuleGraphParams {}

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
pub struct ChangeFileResult {
    diagnostics: Vec<Diagnostic>,
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
pub struct UpdateModuleGraphParams {
    pub path: BiomePath,
    /// The kind of update to apply to the module graph
    pub update_kind: UpdateKind,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum UpdateKind {
    AddOrUpdate,
    Remove,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct PullDiagnosticsParams {
    pub project_key: ProjectKey,
    pub path: BiomePath,
    pub categories: RuleCategories,
    #[serde(default)]
    pub only: Vec<AnalyzerSelector>,
    #[serde(default)]
    pub skip: Vec<AnalyzerSelector>,
    /// Rules to apply on top of the configuration
    #[serde(default)]
    pub enabled_rules: Vec<AnalyzerSelector>,
    /// When `false` the diagnostics, don't have code frames of the code actions (fixes, suppressions, etc.)
    pub pull_code_actions: bool,
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
    pub only: Vec<AnalyzerSelector>,
    #[serde(default)]
    pub skip: Vec<AnalyzerSelector>,
    #[serde(default)]
    pub enabled_rules: Vec<AnalyzerSelector>,
    #[serde(default)]
    pub categories: RuleCategories,
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
    pub offset: Option<TextSize>,
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
    pub only: Vec<AnalyzerSelector>,
    #[serde(default)]
    pub skip: Vec<AnalyzerSelector>,
    /// Rules to apply to the file
    #[serde(default)]
    pub enabled_rules: Vec<AnalyzerSelector>,
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
pub struct ScanProjectResult {
    /// Diagnostics reported while scanning the project.
    pub diagnostics: Vec<Diagnostic>,

    /// Duration of the scan.
    pub duration: Duration,

    /// A list of child configuration files found inside the project
    pub configuration_files: Vec<BiomePath>,
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

impl Display for PatternId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
pub struct PathIsIgnoredParams {
    pub project_key: ProjectKey,
    /// The path to inspect
    pub path: BiomePath,
    /// Whether the path is ignored for specific features e.g. `formatter.includes`.
    /// When this field is empty, Biome checks only `files.includes`.
    pub features: FeatureName,
    #[serde(default)]
    /// Controls how to ignore check should be done
    pub ignore_kind: IgnoreKind,
}
#[derive(Debug, Default, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum IgnoreKind {
    /// Checks whether a path itself is explicitly ignored only.
    #[default]
    Path,

    /// Checks whether a path itself or one of its ancestors is ignored,
    /// up to the root path of the current project.
    Ancestors,
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
pub struct OpenProjectResult {
    /// A unique identifier for this project
    pub project_key: ProjectKey,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct ScanProjectParams {
    pub project_key: ProjectKey,

    /// Whether the watcher should watch this path.
    ///
    /// Does nothing if the watcher is already watching this path.
    pub watch: bool,

    /// Forces scanning of the folder, even if it is already being watched.
    pub force: bool,

    pub scan_kind: ScanKind,

    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub verbose: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct CloseProjectParams {
    pub project_key: ProjectKey,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct FileExitsParams {
    pub file_path: BiomePath,
}

impl From<BiomePath> for FileExitsParams {
    fn from(path: BiomePath) -> Self {
        Self { file_path: path }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub struct GetModuleGraphResult {
    pub data: FxHashMap<String, SerializedJsModuleInfo>,
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
    /// probably want to follow it up with a call to `scan_project()` or
    /// explicitly load settings into the project using `update_settings()`.
    ///
    /// Returns the key of the opened project and the [ScanKind] of this project.
    ///
    /// The key can be used with follow-up methods to perform actions related to the project,
    /// such as opening files or querying them.
    ///
    /// The `scan_kind` can be used to tell the scanner how it should scan the project.
    fn open_project(&self, params: OpenProjectParams) -> Result<OpenProjectResult, WorkspaceError>;

    /// Scans the given project, and initializes all settings and service data.
    ///
    /// The first time you call this method, it may take a long time since it
    /// will traverse the entire project folder recursively, parse all included
    /// files (and possibly their dependencies), and perform processing for
    /// extracting the service data.
    ///
    /// Follow-up calls may be much faster as they can reuse cached data.
    ///
    /// If [`ScanProjectParams::watch`] is `true`, this method also
    /// registers file watchers to make sure the cache remains up-to-date.
    fn scan_project(&self, params: ScanProjectParams) -> Result<ScanProjectResult, WorkspaceError>;

    /// Updates the global settings for the given project.
    ///
    /// TODO: This method should not be used in combination with
    /// `scan_project()`. When scanning is enabled, the server should
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
    /// `scan_project()`, it will also be unregistered.
    fn close_project(&self, params: CloseProjectParams) -> Result<(), WorkspaceError>;

    // #endregion

    // #region FILE-LEVEL METHODS

    /// Opens a new file in the workspace.
    ///
    /// If the file path is under a folder that belongs to an opened project
    /// other than the current one, the current project is changed accordingly.
    fn open_file(&self, params: OpenFileParams) -> Result<OpenFileResult, WorkspaceError>;

    /// Checks if `file_path` exists in the workspace.
    ///
    /// This method is useful to avoid unexpected errors before using the file method and avoid errors.
    ///
    /// ### Error
    ///
    /// It throws an error only if there's an issue with the client transport.
    fn file_exists(&self, params: FileExitsParams) -> Result<bool, WorkspaceError>;

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
    fn is_path_ignored(&self, params: PathIsIgnoredParams) -> Result<bool, WorkspaceError>;

    /// Returns the content of a given file.
    fn get_file_content(&self, params: GetFileContentParams) -> Result<String, WorkspaceError>;

    /// Returns the size of a given file, as well as the allowed maximum file
    /// size for that file.
    fn check_file_size(
        &self,
        params: CheckFileSizeParams,
    ) -> Result<CheckFileSizeResult, WorkspaceError>;

    /// Changes the content of an open file.
    fn change_file(&self, params: ChangeFileParams) -> Result<ChangeFileResult, WorkspaceError>;

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

    /// Updates the internal module graph using the provided path.
    ///
    /// ## Errors
    ///
    /// An error is emitted if the path doesn't exist inside the workspace. Use
    /// the method [Workspace::open_file] before updating the module graph.
    fn update_module_graph(&self, params: UpdateModuleGraphParams) -> Result<(), WorkspaceError>;

    /// Returns the filesystem implementation to open files with.
    ///
    /// This may be an in-memory file system.
    fn fs(&self) -> &dyn FsWithResolverProxy;

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

    // #region DEBUGGING METHODS

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

    /// Returns an IR of the type information of the document
    fn get_type_info(&self, params: GetTypeInfoParams) -> Result<String, WorkspaceError>;

    /// Returns the registered types of the document
    fn get_registered_types(
        &self,
        params: GetRegisteredTypesParams,
    ) -> Result<String, WorkspaceError>;

    /// Returns a textual, debug representation of the semantic model for the document.
    fn get_semantic_model(&self, params: GetSemanticModelParams) -> Result<String, WorkspaceError>;

    /// Returns a serializable version of the module graph
    fn get_module_graph(
        &self,
        params: GetModuleGraphParams,
    ) -> Result<GetModuleGraphResult, WorkspaceError>;

    /// Returns debug information about this workspace.
    fn rage(&self, params: RageParams) -> Result<RageResult, WorkspaceError>;

    /// Returns information about the server this workspace is connected to or
    /// `None` if the workspace isn't connected to a server.
    fn server_info(&self) -> Option<&ServerInfo>;

    // #endregion
}

/// Convenience function for constructing a server instance of [Workspace]
pub fn server(fs: Arc<dyn FsWithResolverProxy>, threads: Option<usize>) -> Box<dyn Workspace> {
    let (watcher_tx, _) = bounded(0);
    let (service_tx, _) = watch::channel(ServiceNotification::IndexUpdated);
    Box::new(WorkspaceServer::new(fs, watcher_tx, service_tx, threads))
}

/// Convenience function for constructing a client instance of [Workspace]
pub fn client<T>(
    transport: T,
    fs: Box<dyn FsWithResolverProxy>,
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

    pub fn get_type_info(&self) -> Result<String, WorkspaceError> {
        self.workspace.get_type_info(GetTypeInfoParams {
            project_key: self.project_key,
            path: self.path.clone(),
        })
    }
    pub fn get_registered_types(&self) -> Result<String, WorkspaceError> {
        self.workspace
            .get_registered_types(GetRegisteredTypesParams {
                project_key: self.project_key,
                path: self.path.clone(),
            })
    }

    pub fn get_semantic_model(&self) -> Result<String, WorkspaceError> {
        self.workspace.get_semantic_model(GetSemanticModelParams {
            project_key: self.project_key,
            path: self.path.clone(),
        })
    }

    pub fn change_file(
        &self,
        version: i32,
        content: String,
    ) -> Result<ChangeFileResult, WorkspaceError> {
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
        only: Vec<AnalyzerSelector>,
        skip: Vec<AnalyzerSelector>,
        pull_code_actions: bool,
    ) -> Result<PullDiagnosticsResult, WorkspaceError> {
        self.workspace.pull_diagnostics(PullDiagnosticsParams {
            project_key: self.project_key,
            path: self.path.clone(),
            categories,
            only,
            skip,
            enabled_rules: vec![],
            pull_code_actions,
        })
    }

    pub fn pull_actions(
        &self,
        range: Option<TextRange>,
        only: Vec<AnalyzerSelector>,
        skip: Vec<AnalyzerSelector>,
        suppression_reason: Option<String>,
        enabled_rules: Vec<AnalyzerSelector>,
        categories: RuleCategories,
    ) -> Result<PullActionsResult, WorkspaceError> {
        self.workspace.pull_actions(PullActionsParams {
            project_key: self.project_key,
            path: self.path.clone(),
            range,
            only,
            skip,
            suppression_reason,
            enabled_rules,
            categories,
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
        only: Vec<AnalyzerSelector>,
        skip: Vec<AnalyzerSelector>,
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

#[cfg(test)]
#[path = "workspace.tests.rs"]
mod tests;
