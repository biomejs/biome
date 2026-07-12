use crate::diagnostics::LspError;
use crate::documents::Document;
use crate::extension_settings::{CONFIGURATION_SECTION, ExtensionSettings};
use crate::utils;
use anyhow::Result;
use biome_analyze::RuleCategoriesBuilder;
use biome_configuration::{Configuration, ConfigurationPathHint};
use biome_console::markup;
use biome_deserialize::Merge;
use biome_diagnostics::{PrintDescription, Severity};
use biome_fs::{BiomePath, normalize_path};
use biome_line_index::WideEncoding;
use biome_lsp_converters::{PositionEncoding, negotiated_encoding};
use biome_service::WorkspaceError;
use biome_service::configuration::{
    LoadedConfiguration, ProjectScanComputer, load_configuration, load_editorconfig,
};
use biome_service::diagnostics::ConfigurationOutsideProject;
use biome_service::file_handlers::astro::AstroFileHandler;
use biome_service::file_handlers::svelte::SvelteFileHandler;
use biome_service::file_handlers::vue::VueFileHandler;
use biome_service::projects::ProjectKey;
use biome_service::settings::{EditorFeature, ModuleGraphResolutionKind};
use biome_service::workspace::db::DbState;
use biome_service::workspace::{
    FeaturesBuilder, GetFileContentParams, OpenProjectParams, OpenProjectResult,
    PullDiagnosticsParams, RetryingWorkspace, SupportsFeatureParams,
};
use biome_service::workspace::{FileFeaturesResult, ServiceNotification};
use biome_service::workspace::{RageEntry, RageParams, RageResult, UpdateSettingsParams};
use biome_service::workspace::{ScanKind, ScanProjectParams};
use biome_service::{Workspace, WorkspaceServer};
use camino::Utf8Path;
use camino::Utf8PathBuf;
use futures::StreamExt;
use futures::stream::futures_unordered::FuturesUnordered;
use papaya::{HashMap, HashSet};
use parking_lot::RwLock;
use rustc_hash::{FxBuildHasher, FxHashMap};
use serde_json::Value;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::time::Duration;
use tokio::spawn;
use tokio::sync::Notify;
use tokio::sync::OnceCell;
use tokio::sync::RwLock as TokioRwLock;
use tokio::sync::watch;
use tokio::task::spawn_blocking;
use tokio::time::sleep;
use tower_lsp_server::Client;
use tower_lsp_server::ls_types::{
    self as lsp, ClientCapabilities, Diagnostic, MessageType, ProgressToken, Registration,
    Unregistration, Uri, WorkDoneProgressCreateParams, WorkspaceFolder,
    request::WorkDoneProgressCreate,
};
use tracing::{debug, error, info, instrument, warn};
use uuid::Uuid;

pub(crate) struct ClientInformation {
    /// The name of the client
    pub(crate) name: String,

    /// The version of the client
    pub(crate) version: Option<String>,
}

/// Key, uniquely identifying a LSP session.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub(crate) struct SessionKey(pub u64);

#[derive(Clone, Hash, PartialEq, Eq)]
struct ConfigurationCacheKey {
    base_path: Utf8PathBuf,
    settings_path: Option<Utf8PathBuf>,
}

const DIAGNOSTICS_DEBOUNCE: Duration = Duration::from_millis(250);

struct DiagnosticsEntry {
    /// The version of the document that was last scheduled for diagnostics.
    scheduled_version: AtomicI32,
    /// Whether diagnostics are currently being computed for this document.
    running: AtomicBool,
    /// Whether diagnostics should be recomputed after the current one completes.
    /// This is used when multiple calls are queued, and we need to re-compute diagnostics for the last version of the document.
    rerun_after_current: AtomicBool,
    /// Whether the diagnostics entry has been closed.
    closed: AtomicBool,
}

impl DiagnosticsEntry {
    fn new(version: i32) -> Self {
        Self {
            scheduled_version: AtomicI32::new(version),
            running: AtomicBool::new(false),
            rerun_after_current: AtomicBool::new(false),
            closed: AtomicBool::new(false),
        }
    }
}

/// Represents the state of an LSP server session.
pub(crate) struct Session {
    /// The unique key identifying this session.
    pub(crate) key: SessionKey,

    /// The LSP client for this session.
    pub(crate) client: Client,

    /// The parameters provided by the client in the "initialize" request
    initialize_params: OnceCell<InitializeParams>,

    /// Tracks workspace folders. They can change during the lifecycle of the server
    workspace_folders: RwLock<Option<Vec<WorkspaceFolder>>>,

    /// The settings of the Biome extension (under the `biome` namespace)
    pub(crate) extension_settings: RwLock<ExtensionSettings>,

    pub(crate) workspace: Arc<WorkspaceServer>,
    db_state: Arc<DbState>,

    /// Configuration status tracked independently per project key.
    configuration_status: HashMap<ProjectKey, ConfigurationStatus>,

    /// Remembers which projects we have already warned the user about having a
    /// broken configuration, so we don't show the same warning again every time
    /// we refresh that project's diagnostics.
    notified_broken_configuration: HashSet<ProjectKey>,

    /// Tracks whether the initialized() notification has been received
    initialized: AtomicBool,

    /// Projects opened in this session, mapped from the project's root path to
    /// the associated project key.
    projects: HashMap<BiomePath, ProjectKey>,

    /// Documents opened in this session.
    documents: HashMap<Uri, Document, FxBuildHasher>,

    /// Pending and active diagnostic refreshes for open documents.
    diagnostics: HashMap<Uri, Arc<DiagnosticsEntry>, FxBuildHasher>,

    pub(crate) cancellation: Arc<Notify>,

    /// Receiver for service notifications.
    ///
    /// If we receive a notification here, diagnostics for open documents are
    /// all refreshed.
    service_rx: watch::Receiver<ServiceNotification>,

    /// Lock used to synchronize multiple atomic operations to load the configuration file
    loading_operations:
        TokioRwLock<FxHashMap<Utf8PathBuf, tokio::sync::broadcast::Sender<ConfigurationStatus>>>,

    /// Cached configuration status per base path to avoid reloading on every request.
    configuration_status_by_path:
        TokioRwLock<FxHashMap<ConfigurationCacheKey, ConfigurationStatus>>,
}

/// The parameters provided by the client in the "initialize" request
struct InitializeParams {
    /// The capabilities provided by the client as part of [`ls_types::InitializeParams`]
    client_capabilities: ClientCapabilities,
    client_information: Option<ClientInformation>,
    root_uri: Option<Uri>,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub(crate) enum ConfigurationStatus {
    /// The configuration file was properly loaded
    Loaded = 0,
    /// The configuration file does not exist
    Missing = 1,
    /// The configuration file exists but could not be loaded
    Error = 2,
    /// The `.editorconfig` file has some errors that biome can't solve
    EditorConfigError = 3,
    /// Currently loading the configuration
    Loading = 4,
    /// The configuration file is correct, but the plugins cannot be loaded
    PluginError = 5,
}

impl ConfigurationStatus {
    pub(crate) const fn is_error(&self) -> bool {
        matches!(self, Self::Error)
    }

    pub(crate) const fn is_editorconfig_error(&self) -> bool {
        matches!(self, Self::EditorConfigError)
    }

    pub(crate) const fn is_plugin_error(&self) -> bool {
        matches!(self, Self::PluginError)
    }

    pub(crate) const fn is_loaded(&self) -> bool {
        matches!(self, Self::Loaded)
    }
}

impl TryFrom<u8> for ConfigurationStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Loaded),
            1 => Ok(Self::Missing),
            2 => Ok(Self::Error),
            3 => Ok(Self::EditorConfigError),
            4 => Ok(Self::Loading),
            5 => Ok(Self::PluginError),
            _ => Err(()),
        }
    }
}

pub(crate) type SessionHandle = Arc<Session>;

/// Holds the set of capabilities supported by the Language Server
/// instance and whether they are enabled or not
#[derive(Default)]
pub(crate) struct CapabilitySet {
    registry: FxHashMap<&'static str, (&'static str, CapabilityStatus)>,
}

/// Represents whether a capability is enabled or not, optionally holding the
/// configuration associated with the capability
pub(crate) enum CapabilityStatus {
    Enable(Option<Value>),
    Disable,
}

impl CapabilitySet {
    /// Insert a capability in the set
    pub(crate) fn add_capability(
        &mut self,
        id: &'static str,
        method: &'static str,
        status: CapabilityStatus,
    ) {
        self.registry.insert(id, (method, status));
    }
}

impl Session {
    pub(crate) fn new(
        key: SessionKey,
        client: Client,
        workspace: Arc<WorkspaceServer>,
        db_state: Arc<DbState>,
        cancellation: Arc<Notify>,
        service_rx: watch::Receiver<ServiceNotification>,
    ) -> Self {
        let config = RwLock::new(ExtensionSettings::new());
        Self {
            key,
            client,
            initialize_params: OnceCell::default(),
            workspace,
            db_state,
            configuration_status: Default::default(),
            projects: Default::default(),
            documents: Default::default(),
            diagnostics: Default::default(),
            extension_settings: config,
            cancellation,
            notified_broken_configuration: Default::default(),
            initialized: AtomicBool::new(false),
            service_rx,
            loading_operations: Default::default(),
            configuration_status_by_path: Default::default(),
            workspace_folders: Default::default(),
        }
    }

    /// Returns the workspace, wrapped so that any call interrupted by a
    /// concurrent update to the workspace database is automatically retried.
    ///
    /// Use this accessor in code where nobody would re-send the work if we
    /// dropped it:
    ///
    /// - **LSP notifications**, i.e. one-way messages from the editor, such
    ///   as `textDocument/didOpen`, `textDocument/didChange` and
    ///   `textDocument/didClose`. The editor sends them once and never asks
    ///   again: dropping a `didChange` would leave our copy of the document
    ///   permanently out of sync with the editor.
    /// - **Background tasks** the server starts on its own, such as
    ///   refreshing the diagnostics of the open documents, loading the
    ///   configuration file, or scanning the project folder.
    ///
    /// LSP request handlers (formatting, code actions, ...) should use
    /// [Self::workspace_for_request] instead.
    pub(crate) fn workspace(&self) -> impl Workspace + '_ {
        RetryingWorkspace::new(self.workspace.with_db_state(&self.db_state))
    }

    /// Returns the workspace without automatic retries.
    ///
    /// Use this accessor in **LSP request handlers**, i.e. handlers for
    /// messages the editor sends and awaits an answer for, such as
    /// `textDocument/formatting` or `textDocument/codeAction`. These handlers
    /// run inside `catch_lsp_operation`, which turns an interrupted call into
    /// a `ContentModified` response.
    ///
    /// Requests must not retry on their own, because their positions and
    /// ranges are only meaningful for the document version the editor sent
    /// them for. If the user types while we compute code actions for line 10,
    /// a retry would compute them for whatever moved to line 10 after the
    /// edit. Answering with `ContentModified` instead makes the editor
    /// re-send the request with fresh positions, if it still needs it.
    pub(crate) fn workspace_for_request(&self) -> impl Workspace + '_ {
        self.workspace.with_db_state(&self.db_state)
    }

    /// Initialize this session instance with the incoming initialization parameters from the client
    #[instrument(level = "debug", skip_all)]
    pub(crate) fn initialize(
        self: &Arc<Self>,
        client_capabilities: ClientCapabilities,
        client_information: Option<ClientInformation>,
        root_uri: Option<Uri>,
        workspace_folders: Option<Vec<WorkspaceFolder>>,
    ) {
        let result = self.initialize_params.set(InitializeParams {
            client_capabilities,
            client_information,
            root_uri,
        });

        {
            let mut this_workspace_folders = self.workspace_folders.write();
            *this_workspace_folders = workspace_folders;
        }

        if let Err(err) = result {
            error!("Failed to initialize session: {err}");
        }

        let session = self.clone();
        spawn(async move {
            let mut service_data_rx = session.service_rx.clone();
            while let Ok(()) = service_data_rx.changed().await {
                match *session.service_rx.borrow() {
                    ServiceNotification::IndexUpdated => {
                        let session = session.clone();
                        spawn(async move {
                            session.update_all_diagnostics().await;
                        });
                    }
                    ServiceNotification::WatcherStopped => {
                        break;
                    }
                }
            }
        });
    }

    /// Register a set of capabilities with the client
    pub(crate) async fn register_capabilities(&self, capabilities: CapabilitySet) {
        let mut registrations = Vec::new();
        let mut unregistrations = Vec::new();

        let mut register_methods = String::new();
        let mut unregister_methods = String::new();

        for (id, (method, status)) in capabilities.registry {
            unregistrations.push(Unregistration {
                id: id.to_string(),
                method: method.to_string(),
            });

            if !unregister_methods.is_empty() {
                unregister_methods.push_str(", ");
            }

            unregister_methods.push_str(method);

            if let CapabilityStatus::Enable(register_options) = status {
                registrations.push(Registration {
                    id: id.to_string(),
                    method: method.to_string(),
                    register_options,
                });

                if !register_methods.is_empty() {
                    register_methods.push_str(", ");
                }

                register_methods.push_str(method);
            }
        }

        if let Err(e) = self.client.unregister_capability(unregistrations).await {
            error!(
                "Error unregistering {unregister_methods:?} capabilities: {}",
                e
            );
        } else {
            info!("Unregister capabilities {unregister_methods:?}");
        }

        if let Err(e) = self.client.register_capability(registrations).await {
            error!("Error registering {register_methods:?} capabilities: {}", e);
        } else {
            info!("Register capabilities {register_methods:?}");
        }
    }

    /// Returns the key for the project that should be used for a given path.
    pub(crate) fn project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.projects
            .pin()
            .iter()
            .filter(|(project_path, _project_key)| path.starts_with(project_path.as_path()))
            .max_by_key(|(project_path, _project_key)| project_path.as_str().len())
            .map(|(_project_path, project_key)| *project_key)
    }

    /// Registers an open project with its root path and scans the folder.
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn insert_and_scan_project(
        self: &Arc<Self>,
        project_key: ProjectKey,
        path: BiomePath,
        scan_kind: ScanKind,
        force: bool,
    ) {
        self.projects.pin().insert(path.clone(), project_key);

        // Spawn the scan in the background, to avoid timing out the LSP request.
        let session = self.clone();
        spawn(async move { session.scan_project(project_key, scan_kind, force).await })
            .await
            .expect("Scanning task to complete successfully");
    }

    /// Get a [`Document`] matching the provided [`Uri`]
    ///
    /// If document does not exist, result is [WorkspaceError::NotFound]
    pub(crate) fn document(&self, url: &Uri) -> Option<Document> {
        self.documents.pin().get(url).cloned()
    }

    /// Set the [`Document`] for the provided [`Uri`]
    ///
    /// Used by [`handlers::text_document] to synchronize documents with the client.
    pub(crate) fn insert_document(&self, url: Uri, document: Document) {
        self.documents.pin().insert(url, document);
    }

    /// Remove the [`Document`] matching the provided [`Uri`]
    pub(crate) fn remove_document(&self, url: &Uri) -> Option<ProjectKey> {
        self.documents.pin().remove(url).map(|doc| doc.project_key)
    }

    fn diagnostics_entry(&self, url: Uri, version: i32) -> Arc<DiagnosticsEntry> {
        Arc::clone(
            self.diagnostics
                .pin()
                .get_or_insert_with(url, || Arc::new(DiagnosticsEntry::new(version))),
        )
    }

    pub(crate) fn close_diagnostics(&self, url: &Uri) {
        let entry = self.diagnostics.pin().remove(url).cloned();
        if let Some(entry) = entry {
            entry.closed.store(true, Ordering::Release);
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    pub(crate) fn schedule_diagnostics(self: &Arc<Self>, url: Uri, version: i32) {
        let entry = self.diagnostics_entry(url.clone(), version);
        entry.closed.store(false, Ordering::Release);
        entry.scheduled_version.store(version, Ordering::Release);

        self.spawn_delayed_diagnostics(url, entry, version);
    }

    #[tracing::instrument(level = "debug", skip_all)]
    fn spawn_delayed_diagnostics(
        self: &Arc<Self>,
        url: Uri,
        entry: Arc<DiagnosticsEntry>,
        version: i32,
    ) {
        let session = Arc::clone(self);
        spawn(async move {
            sleep(DIAGNOSTICS_DEBOUNCE).await;
            session.run_debounced_diagnostics(url, entry, version).await;
        });
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn run_debounced_diagnostics(
        self: Arc<Self>,
        url: Uri,
        entry: Arc<DiagnosticsEntry>,
        version: i32,
    ) {
        if entry.closed.load(Ordering::Acquire)
            || entry.scheduled_version.load(Ordering::Acquire) != version
        {
            return;
        }

        // Only one diagnostics update should run for a document at a time.
        // This tries to change `running` from `false` to `true`. If it works,
        // this task owns the update. `AcqRel` is used for that successful claim,
        // so this task sees the latest state, and other tasks see that this task
        // claimed the update. If it fails, another task already owns the update,
        // so `Acquire` is enough because we only need to read that state.
        if entry
            .running
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            entry.rerun_after_current.store(true, Ordering::Release);
            return;
        }

        if let Err(err) = self
            .update_diagnostics_for_version(url.clone(), version, &entry)
            .await
        {
            error!("Failed to update diagnostics: {err}");
        }

        entry.running.store(false, Ordering::Release);

        // A newer change may have arrived while this update was running. This
        // resets the flag back to `false` and tells us whether another update
        // was requested. `AcqRel` makes the reset visible to other tasks and
        // also lets this task see the latest request before deciding whether to
        // schedule another diagnostics update.
        if entry.rerun_after_current.swap(false, Ordering::AcqRel) {
            let latest_version = entry.scheduled_version.load(Ordering::Acquire);
            if latest_version != version && !entry.closed.load(Ordering::Acquire) {
                self.spawn_delayed_diagnostics(url, entry, latest_version);
            }
        }
    }

    pub(crate) fn file_path(&self, url: &Uri) -> Result<BiomePath> {
        let path_to_file = match url.to_file_path() {
            None => {
                // If we can't create a path, it's probably because the file doesn't exist.
                // It can be a newly created file that it's not on disk
                Utf8PathBuf::from(url.path().to_string())
            }
            Some(path) => Utf8PathBuf::from_path_buf(path.as_ref().to_path_buf())
                .expect("To to have a UTF-8 path"),
        };

        Ok(BiomePath::new(path_to_file))
    }

    /// Computes diagnostics for the file matching the provided url and publishes
    /// them to the client. Called from [`handlers::text_document`] when a file's
    /// contents changes.
    #[tracing::instrument(level = "debug", skip_all, fields(url = display(url.as_str()), diagnostic_count), err)]
    pub(crate) async fn update_diagnostics(&self, url: Uri) -> Result<(), LspError> {
        let Some(doc) = self.document(&url) else {
            return Ok(());
        };
        self.update_diagnostics_for_document(url.clone(), doc, None)
            .await
    }

    async fn update_diagnostics_for_version(
        &self,
        url: Uri,
        version: i32,
        entry: &DiagnosticsEntry,
    ) -> Result<(), LspError> {
        let Some(doc) = self.document(&url) else {
            return Ok(());
        };
        if doc.version != version || entry.closed.load(Ordering::Acquire) {
            return Ok(());
        }
        self.update_diagnostics_for_document(url.clone(), doc, Some(entry))
            .await
    }

    /// Computes diagnostics for the file matching the provided url and publishes
    /// them to the client. Called from [`handlers::text_document`] when a file's
    /// contents changes.
    #[tracing::instrument(level = "debug", skip_all, fields(url = display(url.as_str()), diagnostic_count), err)]
    async fn update_diagnostics_for_document(
        &self,
        url: Uri,
        doc: Document,
        entry: Option<&DiagnosticsEntry>,
    ) -> Result<(), LspError> {
        let biome_path = self.file_path(&url)?;

        let configuration_status = self.configuration_status(doc.project_key);
        if !self.notified_broken_configuration(doc.project_key) {
            if configuration_status.is_editorconfig_error() {
                self.set_notified_broken_configuration(doc.project_key);
                self.client
                    .show_message(MessageType::WARNING, "The .editorconfig file has errors. Biome will report only parsing errors until the file is fixed or its usage is disabled.")
                    .await
            } else if configuration_status.is_error() {
                self.set_notified_broken_configuration(doc.project_key);
                self.client
                    .show_message(MessageType::WARNING, "The configuration file has errors. Biome will report only parsing errors until the configuration is fixed.")
                    .await;
            } else if configuration_status.is_plugin_error() {
                self.set_notified_broken_configuration(doc.project_key);
                self.client.show_message(MessageType::WARNING, "The plugin loading has failed. Biome will report only parsing errors until the file is fixed or its usage is disabled.").await
            }
        }
        let FileFeaturesResult {
            features_supported: file_features,
        } = self.workspace().file_features(SupportsFeatureParams {
            project_key: doc.project_key,
            features: FeaturesBuilder::new().with_linter().with_assist().build(),
            path: biome_path.clone(),
            inline_config: self.inline_config(),
            skip_ignore_check: false,
            not_requested_features: FeaturesBuilder::new().with_search().build(),
        })?;

        if !file_features.supports_lint() && !file_features.supports_assist() {
            if !self.can_publish_diagnostics(&url, doc.version, entry) {
                return Ok(());
            }
            self.client
                .publish_diagnostics(url.clone(), vec![], Some(doc.version))
                .await;
            return Ok(());
        }
        let diagnostics: Vec<Diagnostic> = {
            let mut categories = RuleCategoriesBuilder::default().with_syntax();
            if configuration_status.is_loaded() {
                if file_features.supports_lint() {
                    categories = categories.with_lint();
                }
                if file_features.supports_assist() {
                    categories = categories.with_assist();
                }
            }
            let result = self.workspace().pull_diagnostics(PullDiagnosticsParams {
                project_key: doc.project_key,
                path: biome_path.clone(),
                categories: categories.build(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: Vec::new(),
                include_code_fix: false,
                inline_config: self.inline_config(),
                max_diagnostics: None,
                diagnostic_level: Severity::Information,
                enforce_assist: false,
            })?;

            let offset = if file_features.supports_full_html_support() {
                None
            } else {
                let get_start: Option<fn(&str) -> Option<u32>> = match biome_path.extension() {
                    Some("vue") => Some(VueFileHandler::start),
                    Some("astro") => Some(AstroFileHandler::start),
                    Some("svelte") => Some(SvelteFileHandler::start),
                    _ => None,
                };
                get_start.and_then(|f| {
                    let content = self
                        .workspace()
                        .get_file_content(GetFileContentParams {
                            project_key: doc.project_key,
                            path: biome_path.clone(),
                        })
                        .ok()?;
                    f(content.as_str())
                })
            };

            result
                .diagnostics
                .into_iter()
                .filter_map(|d| {
                    match utils::diagnostic_to_lsp(
                        d,
                        &url,
                        &doc.line_index,
                        self.position_encoding(),
                        offset,
                    ) {
                        Ok(diag) => Some(diag),
                        Err(err) => {
                            error!("failed to convert diagnostic to LSP: {err:?}");
                            None
                        }
                    }
                })
                .collect()
        };

        if !self.can_publish_diagnostics(&url, doc.version, entry) {
            return Ok(());
        }

        self.client
            .publish_diagnostics(url.clone(), diagnostics, Some(doc.version))
            .await;

        Ok(())
    }

    fn is_document_current(&self, url: &Uri, version: i32) -> bool {
        self.document(url)
            .is_some_and(|document| document.version == version)
    }

    fn can_publish_diagnostics(
        &self,
        url: &Uri,
        version: i32,
        entry: Option<&DiagnosticsEntry>,
    ) -> bool {
        !entry.is_some_and(|entry| entry.closed.load(Ordering::Acquire))
            && self.is_document_current(url, version)
    }

    /// Updates diagnostics for every [`Document`] in this [`Session`]
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn update_all_diagnostics(&self) {
        let mut futures: FuturesUnordered<_> = self
            .documents
            .pin()
            .iter()
            .map(|(url, doc)| self.update_diagnostics_for_document(url.clone(), doc.clone(), None))
            .collect();

        while let Some(result) = futures.next().await {
            if let Err(e) = result {
                error!("Error while updating diagnostics: {}", e);
            }
        }
    }

    /// True if the client supports dynamic registration of "workspace/didChangeConfiguration" requests
    #[instrument(level = "info", skip(self))]
    pub(crate) fn can_register_did_change_configuration(&self) -> bool {
        let result = self
            .initialize_params
            .get()
            .and_then(|c| c.client_capabilities.workspace.as_ref())
            .and_then(|c| c.did_change_configuration)
            .and_then(|c| c.dynamic_registration)
            == Some(true);

        info!("Can register didChangeConfiguration: {result}");
        result
    }

    #[instrument(level = "info", skip(self))]
    pub(crate) fn can_register_on_type_formatting(&self) -> bool {
        let result = self
            .initialize_params
            .get()
            .and_then(|c| c.client_capabilities.text_document.as_ref())
            .and_then(|c| c.on_type_formatting)
            .and_then(|c| c.dynamic_registration)
            == Some(true);

        info!("Can register onTypeFormatting: {result}");
        result
    }

    #[instrument(level = "info", skip(self))]
    pub(crate) fn can_register_formatting(&self) -> bool {
        let result = self
            .initialize_params
            .get()
            .and_then(|c| c.client_capabilities.text_document.as_ref())
            .and_then(|c| c.formatting)
            .and_then(|c| c.dynamic_registration)
            == Some(true);

        info!("Can register formatting: {result}");
        result
    }

    #[instrument(level = "info", skip(self))]
    pub(crate) fn can_register_range_formatting(&self) -> bool {
        let result = self
            .initialize_params
            .get()
            .and_then(|c| c.client_capabilities.text_document.as_ref())
            .and_then(|c| c.range_formatting)
            .and_then(|c| c.dynamic_registration)
            == Some(true);

        info!("Can register rangeFormatting: {result}");
        result
    }

    #[instrument(level = "info", skip(self))]
    pub(crate) fn can_register_did_change_watched_files(&self) -> bool {
        let result = self
            .initialize_params
            .get()
            .and_then(|c| c.client_capabilities.workspace.as_ref())
            .and_then(|c| c.did_change_watched_files)
            .and_then(|c| c.dynamic_registration)
            == Some(true);

        info!("Can register didChangeWatchedFiles: {result}");
        result
    }

    /// Whether the client supports `codeAction/resolve` for deferred edit computation.
    /// Whether the client supports `codeAction/resolve` for deferred edit computation.
    ///
    /// Per LSP spec, `resolveSupport.properties` lists which specific properties
    /// the client can resolve. We only defer when `"edit"` is in that list.
    pub(crate) fn supports_code_action_resolve(&self) -> bool {
        self.initialize_params
            .get()
            .and_then(|c| c.client_capabilities.text_document.as_ref())
            .and_then(|c| c.code_action.as_ref())
            .and_then(|c| c.resolve_support.as_ref())
            .is_some_and(|support| support.properties.iter().any(|p| p == "edit"))
    }

    #[instrument(level = "info", skip(self))]
    pub(crate) fn can_register_code_action(&self) -> bool {
        let result = self
            .initialize_params
            .get()
            .and_then(|c| c.client_capabilities.text_document.as_ref())
            .and_then(|c| c.code_action.as_ref())
            .and_then(|c| c.dynamic_registration)
            == Some(true);

        info!("Can register codeAction: {result}");
        result
    }

    pub(crate) fn can_register_goto_definition(&self) -> bool {
        let result = self
            .initialize_params
            .get()
            .and_then(|c| c.client_capabilities.text_document.as_ref())
            .and_then(|c| c.definition.as_ref())
            .and_then(|c| c.dynamic_registration)
            == Some(true)
            && self
                .extension_settings
                .read()
                .editor_features()
                .contains(EditorFeature::GotoDefinition);
        info!("Can register gotoDefinition: {result}");

        result
    }

    /// Get the current workspace folders
    pub(crate) fn get_workspace_folders(&self) -> Option<Vec<WorkspaceFolder>> {
        self.workspace_folders.read().clone()
    }

    pub(crate) fn update_workspace_folders(
        &self,
        added: Vec<WorkspaceFolder>,
        removed: Vec<WorkspaceFolder>,
    ) {
        let mut workspace_folders = self.workspace_folders.write();

        if let Some(ref mut folders) = *workspace_folders {
            if !removed.is_empty() {
                folders.retain(|folder| !removed.contains(folder));
            }

            if !added.is_empty() {
                folders.extend(added);
            }
        } else {
            *workspace_folders = Some(added);
        }
    }

    /// Returns the root URI of the workspace as provided by the client
    pub(crate) fn base_uri(&self) -> Option<Uri> {
        let initialize_params = self.initialize_params.get()?;
        initialize_params.root_uri.clone()
    }

    /// Returns the base path of the workspace on the filesystem if it has one
    pub(crate) fn base_path(&self) -> Option<Utf8PathBuf> {
        let initialize_params = self.initialize_params.get()?;

        let root_uri = initialize_params.root_uri.as_ref()?;
        match root_uri.to_file_path() {
            Some(base_path) => Some(
                Utf8PathBuf::from_path_buf(base_path.to_path_buf()).expect("To have a UTF-8 path"),
            ),
            None => {
                error!(
                    "The Workspace root URI {root_uri:?} could not be parsed as a filesystem path"
                );
                None
            }
        }
    }

    /// Returns a reference to the client information for this session
    pub(crate) fn client_information(&self) -> Option<&ClientInformation> {
        self.initialize_params.get()?.client_information.as_ref()
    }

    /// Mark that the initialized() notification has been received
    pub(crate) fn set_initialized(&self) {
        self.initialized.store(true, Ordering::Relaxed);
    }

    /// Returns true if the initialized() notification has been received
    pub(crate) fn has_initialized(&self) -> bool {
        self.initialized.load(Ordering::Relaxed)
    }

    /// Returns the configuration path set by the user in the extension settings
    pub(crate) fn get_settings_configuration_path(&self) -> Option<Utf8PathBuf> {
        self.extension_settings.read().configuration_path()
    }

    /// Resolves the user-provided `configurationPath` setting to an absolute path.
    ///
    /// If the path is already absolute, it is returned as-is. If it is relative,
    /// it is resolved against the appropriate workspace root:
    ///
    /// - When `file_path` is provided (e.g. from `did_open`), the workspace folder
    ///   that contains the file is used as the base for resolution. This ensures
    ///   that in a multi-root workspace, the relative path is resolved against the
    ///   correct root rather than an arbitrary one.
    /// - When `file_path` is `None` (e.g. from `load_workspace_settings`), each
    ///   workspace folder is tried in order. The closest path to the `file_path` is used.
    /// - If no workspace folders are registered, the session's `base_path()`
    ///   (derived from the deprecated `root_uri` initialization parameter) is used
    ///   as a fallback to keep backwards compatibility.
    ///
    /// Returns `None` if no `configurationPath` is set in the extension settings.
    pub(crate) fn resolve_configuration_path(
        &self,
        file_path: Option<&Utf8PathBuf>,
    ) -> Option<ConfigurationPathHint> {
        let config_path = self.get_settings_configuration_path()?;

        // Collect workspace folder roots as absolute Utf8PathBufs.
        let workspace_roots: Vec<Utf8PathBuf> = self
            .get_workspace_folders()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|folder| {
                folder
                    .uri
                    .to_file_path()
                    .and_then(|p| Utf8PathBuf::from_path_buf(p.to_path_buf()).ok())
            })
            .collect();

        let hint_for_path = |path: Utf8PathBuf| {
            Some(if workspace_roots.iter().any(|r| path.starts_with(r)) {
                ConfigurationPathHint::FromUser(path)
            } else {
                ConfigurationPathHint::FromUserExternal(path)
            })
        };

        if config_path.is_absolute() {
            return hint_for_path(config_path);
        }

        if let Some(file_path) = file_path {
            // Find the workspace folder that contains this file and resolve
            // the relative config path against that folder.
            if let Some(root) = workspace_roots
                .iter()
                .filter(|root| file_path.starts_with(*root))
                .max_by_key(|root| root.as_str().len())
            {
                let joined = normalize_path(&root.join(config_path));
                return hint_for_path(joined);
            }
        }

        // No file context, or the file doesn't belong to any known workspace
        // folder. Without a file to anchor against, we cannot determine which
        // folder the relative path belongs to, so we fall back to the first
        // registered workspace folder. This is a best-effort guess; the
        // correct per-file resolution happens in `did_open` where the file
        // path is available.
        if let Some(root) = workspace_roots.first() {
            let joined = normalize_path(&root.join(&config_path));
            return hint_for_path(joined);
        }

        // Fall back to the (deprecated) root_uri base path.
        if let Some(base) = self.base_path() {
            let joined = normalize_path(&base.join(&config_path));
            return hint_for_path(joined);
        }

        // Nothing to resolve against; return the path unchanged and let the
        // downstream loader produce a meaningful error.
        Some(ConfigurationPathHint::FromUserExternal(config_path))
    }

    /// This function attempts to read the `biome.json` configuration file from
    /// the root URI and update the workspace settings accordingly
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn load_workspace_settings(self: &Arc<Self>, reload: bool) {
        if let Some(config_path) = self.resolve_configuration_path(None) {
            info!("Detected configuration path in the workspace settings.");
            let config_file_path = config_path.to_path_buf();
            let status = self
                .load_biome_configuration_file(config_path, reload)
                .await;
            debug!("Configuration status: {:?}", status);
            self.record_configuration_status(config_file_path.as_deref(), status);
        } else if let Some(folders) = self.get_workspace_folders() {
            info!("Detected workspace folder.");
            for folder in folders {
                info!("Attempt to load the configuration file in {:?}", folder.uri);
                let base_path = folder.uri.to_file_path().map(|p| {
                    Utf8PathBuf::from_path_buf(p.to_path_buf()).expect("To have a valid UTF-8 path")
                });
                match base_path {
                    Some(base_path) => {
                        let status = self
                            .load_biome_configuration_file(
                                ConfigurationPathHint::FromWorkspace(base_path.clone()),
                                reload,
                            )
                            .await;
                        debug!("Configuration status: {:?}", status);
                        self.record_configuration_status(Some(&base_path), status);
                    }
                    None => {
                        error!(
                            "The Workspace root URI {:?} could not be parsed as a filesystem path",
                            folder.uri
                        );
                    }
                }
            }
        } else {
            let base_path = match self.base_path() {
                None => ConfigurationPathHint::default(),
                Some(path) => ConfigurationPathHint::FromLsp(path),
            };
            let config_file_path = base_path.to_path_buf();
            let status = self.load_biome_configuration_file(base_path, reload).await;
            self.record_configuration_status(config_file_path.as_deref(), status);
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn scan_project(
        self: &Arc<Self>,
        project_key: ProjectKey,
        scan_kind: ScanKind,
        force: bool,
    ) {
        let progress_token = ProgressToken::String(Uuid::new_v4().to_string());
        let is_work_done_progress_supported = self.initialize_params.get().is_some_and(|params| {
            params
                .client_capabilities
                .window
                .as_ref()
                .is_some_and(|window| window.work_done_progress == Some(true))
        });

        // Let the user know the scanning is taking long time using the $/progress notification.
        let progress = if is_work_done_progress_supported
            && self
                .client
                .send_request::<WorkDoneProgressCreate>(WorkDoneProgressCreateParams {
                    token: progress_token.clone(),
                })
                .await
                .is_ok()
        {
            let progress = self
                .client
                .progress(progress_token.clone(), "Biome is scanning the project");

            Some(progress.begin().await)
        } else {
            None
        };

        let session = self.clone();

        spawn_blocking(move || {
            let result = session.workspace().scan_project(ScanProjectParams {
                project_key,
                watch: scan_kind.is_project() || scan_kind.is_type_aware(),
                force,
                scan_kind,
                verbose: false,
            });

            match result {
                Ok(result) => {
                    spawn(async move {
                        for diagnostic in result.diagnostics {
                            let message = PrintDescription(&diagnostic).to_string();
                            session
                                .client
                                .log_message(MessageType::ERROR, message)
                                .await;
                        }
                    });
                }
                Err(err) => {
                    let message = PrintDescription(&err).to_string();
                    spawn(async move {
                        session
                            .client
                            .log_message(MessageType::ERROR, message)
                            .await;
                    });
                }
            }
        })
        .await
        .unwrap();

        if let Some(progress) = progress {
            progress.finish().await;
        }
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub(super) async fn load_biome_configuration_file(
        self: &Arc<Self>,
        base_path: ConfigurationPathHint,
        reload: bool,
    ) -> ConfigurationStatus {
        // Check if there's already an ongoing operation for this path
        let path_to_index = base_path.to_path_buf().unwrap_or_default();
        let cache_key = ConfigurationCacheKey {
            base_path: path_to_index.clone(),
            settings_path: self.get_settings_configuration_path(),
        };
        if !reload {
            let cached = self.configuration_status_by_path.read().await;
            if let Some(status) = cached.get(&cache_key) {
                return *status;
            }
        }
        {
            let operations = self.loading_operations.read().await;

            if let Some(sender) = operations.get(&path_to_index).cloned() {
                drop(operations); // Release read lock

                // Wait for the ongoing operation to complete
                let mut receiver = sender.subscribe();
                match receiver.recv().await {
                    Ok(status) => {
                        debug!(
                            "Reused configuration loading result for path: {:?}",
                            base_path
                        );
                        return status;
                    }
                    Err(_) => {
                        // The sender was dropped or no more messages, continue to load
                        debug!(
                            "Configuration loading broadcast ended for path: {:?}",
                            base_path
                        );
                    }
                }
            }
        }

        // Create a broadcast channel for this operation
        let (tx, _rx) = tokio::sync::broadcast::channel(1);

        // Store the receiver for other tasks to subscribe to
        {
            let mut operations = self.loading_operations.write().await;
            operations.insert(path_to_index.clone(), tx.clone());
        }

        // Perform the actual loading
        let status = self
            .load_biome_configuration_file_internal(base_path.clone(), reload)
            .await;

        {
            let mut cached = self.configuration_status_by_path.write().await;
            cached.insert(cache_key, status);
        }

        // Broadcast the result to any waiting tasks
        let _ = tx.send(status);

        // Clean up the operation
        {
            let mut operations = self.loading_operations.write().await;
            operations.remove(&path_to_index);
        }

        status
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub(super) async fn load_biome_configuration_file_internal(
        self: &Arc<Self>,
        base_path: ConfigurationPathHint,
        force: bool,
    ) -> ConfigurationStatus {
        let loaded_configuration =
            match load_configuration(self.workspace().fs(), base_path.clone()) {
                Ok(loaded_configuration) => loaded_configuration,
                Err(err) => {
                    error!("Couldn't load the configuration file, reason:\n {err}");
                    self.client.log_message(MessageType::ERROR, &err).await;
                    return ConfigurationStatus::Error;
                }
            };
        if !loaded_configuration.loaded_location.is_in_project() {
            let config_path = loaded_configuration
                .file_path
                .as_ref()
                .map_or_else(|| "<unknown>".to_string(), |p| p.to_string());
            let working_directory = match &base_path {
                ConfigurationPathHint::FromLsp(path)
                | ConfigurationPathHint::FromWorkspace(path) => path.to_string(),
                ConfigurationPathHint::FromUser(path) => {
                    let workspace = self.workspace();
                    let fs = workspace.fs();
                    if fs.path_is_file(path) {
                        path.parent()
                            .map_or("<unknown>".to_string(), |p| p.to_string())
                    } else {
                        path.to_string()
                    }
                }
                ConfigurationPathHint::FromUserExternal(_) => self
                    .base_path()
                    .map_or("<unknown>".to_string(), |p| p.to_string()),
                ConfigurationPathHint::None => "<unknown>".to_string(),
            };
            let message = PrintDescription(&ConfigurationOutsideProject {
                config_path,
                working_directory,
            })
            .to_string();
            self.client.log_message(MessageType::INFO, message).await;
        }

        if loaded_configuration.has_errors() {
            error!("Couldn't load the configuration file, reasons:");
            for diagnostic in loaded_configuration.as_diagnostics_iter() {
                let message = PrintDescription(diagnostic).to_string();
                self.client.log_message(MessageType::ERROR, message).await;
            }
            return ConfigurationStatus::Error;
        }

        info!("Configuration loaded successfully from disk.");
        info!("Update workspace settings.");

        let LoadedConfiguration {
            configuration: fs_configuration,
            directory_path: configuration_path,
            ..
        } = loaded_configuration;

        if configuration_path.is_none() && self.requires_configuration() {
            return ConfigurationStatus::Missing;
        }

        let workspace = self.workspace();
        let fs = workspace.fs();
        let should_use_editorconfig = fs_configuration.use_editorconfig();
        let mut configuration = if should_use_editorconfig {
            let (editorconfig, editorconfig_diagnostics) = {
                let search_path = configuration_path
                    .clone()
                    .unwrap_or_else(|| fs.working_directory().unwrap_or_default());
                match load_editorconfig(fs, search_path, None) {
                    Ok(result) => result,
                    Err(error) => {
                        error!("Failed load the `.editorconfig` file. Reason: {error}");
                        self.client.log_message(MessageType::ERROR, &error).await;
                        return ConfigurationStatus::EditorConfigError;
                    }
                }
            };
            for diagnostic in editorconfig_diagnostics {
                let message = PrintDescription(&diagnostic).to_string();
                self.client.log_message(MessageType::ERROR, message).await;
            }
            editorconfig.unwrap_or_default()
        } else {
            Default::default()
        };

        configuration.merge_with(fs_configuration);

        let inline_configuration = self.inline_config();
        if let Some(inline_configuration) = inline_configuration {
            configuration.merge_with(inline_configuration);
        }

        // If the configuration from the LSP or the workspace, the directory path is used as
        // the working directory. Otherwise, the base path of the session is used, then the current
        // working directory is used as the last resort.
        debug!("Configuration path provided {:?}", &base_path);
        let project_path = match &base_path {
            ConfigurationPathHint::FromLsp(path) | ConfigurationPathHint::FromWorkspace(path) => {
                path.to_path_buf()
            }
            ConfigurationPathHint::FromUser(path) => {
                let workspace_root = self
                    .get_workspace_folders()
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|folder| {
                        folder
                            .uri
                            .to_file_path()
                            .and_then(|p| Utf8PathBuf::from_path_buf(p.to_path_buf()).ok())
                    })
                    .filter(|root| path.starts_with(root))
                    .max_by_key(|root| root.as_str().len());

                if let Some(root) = workspace_root {
                    root
                } else if fs.path_is_file(path) {
                    path.parent()
                        .map_or(fs.working_directory().unwrap_or_default(), |p| {
                            p.to_path_buf()
                        })
                } else {
                    path.to_path_buf()
                }
            }
            _ => self
                .base_path()
                .or_else(|| fs.working_directory())
                .unwrap_or_default(),
        };

        let project_key = match self.project_for_path(&project_path) {
            Some(project_key) => project_key,
            None => {
                let register_result = self.workspace().open_project(OpenProjectParams {
                    path: project_path.as_path().into(),
                    open_uninitialized: true,
                });
                let OpenProjectResult { project_key } = match register_result {
                    Ok(result) => result,
                    Err(error) => {
                        error!("Failed to register the project folder: {error}");
                        self.client.log_message(MessageType::ERROR, &error).await;
                        return ConfigurationStatus::Error;
                    }
                };
                project_key
            }
        };

        let scan_kind = ProjectScanComputer::new(&configuration).compute();
        // We give priority to the scan kind requested by the user.
        let scan_kind = if scan_kind.is_project() || scan_kind.is_type_aware() {
            scan_kind
        } else if !self.scan_kind_from_editor_features().is_none() {
            self.scan_kind_from_editor_features()
        } else if scan_kind.is_none() {
            ScanKind::KnownFiles
        } else {
            scan_kind
        };

        let result = self.workspace().update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: configuration_path
                .as_ref()
                .map(Utf8PathBuf::as_path)
                .map(BiomePath::from),
            configuration,
            extended_configurations: Default::default(),
            module_graph_resolution_kind: ModuleGraphResolutionKind::from(&scan_kind),
        });

        self.insert_and_scan_project(project_key, project_path.into(), scan_kind, force)
            .await;

        if let Err(WorkspaceError::PluginErrors(error)) = result {
            error!("Failed to load plugins: {error:?}");
            self.client
                .log_message(MessageType::ERROR, format!("{error:?}"))
                .await;
            ConfigurationStatus::PluginError
        } else if let Err(error) = result {
            error!("Failed to set workspace settings: {error}");
            self.client.log_message(MessageType::ERROR, &error).await;
            ConfigurationStatus::Error
        } else {
            ConfigurationStatus::Loaded
        }
    }

    /// Requests "workspace/configuration" from client and updates Session config.
    /// It must be done before loading workspace settings in [`Self::load_workspace_settings`].
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn load_extension_settings(&self, settings: Option<Value>) {
        match settings {
            None => {
                let item = lsp::ConfigurationItem {
                    scope_uri: match self.initialize_params.get() {
                        Some(params) => params.root_uri.clone(),
                        None => None,
                    },
                    section: Some(String::from(CONFIGURATION_SECTION)),
                };

                let client_configurations = match self.client.configuration(vec![item]).await {
                    Ok(client_configurations) => client_configurations,
                    Err(err) => {
                        error!("Couldn't read configuration from the client: {err}");
                        return;
                    }
                };

                let client_configuration = client_configurations.into_iter().next();

                if let Some(client_configuration) = client_configuration {
                    info!("Loaded client configuration: {client_configuration:#?}");

                    let mut config = self.extension_settings.write();
                    if let Err(err) = config.set_workspace_settings(client_configuration) {
                        error!("Couldn't set client configuration: {}", err);
                    }
                } else {
                    info!("Client did not return any configuration");
                }
            }
            Some(settings) => {
                let settings = settings
                    .get(CONFIGURATION_SECTION)
                    .cloned()
                    .unwrap_or(settings);
                let mut config = self.extension_settings.write();
                if let Err(err) = config.set_workspace_settings(settings) {
                    error!("Couldn't set client configuration: {}", err);
                }
            }
        }
        self.clear_configuration_cache().await;
    }

    pub(crate) async fn clear_configuration_cache(&self) {
        self.configuration_status_by_path.write().await.clear();
    }

    /// Broadcast a shutdown signal to all active connections
    pub(crate) fn broadcast_shutdown(&self) {
        self.cancellation.notify_one();
    }

    pub(crate) fn failsafe_rage(&self, params: RageParams) -> RageResult {
        let result = salsa::Cancelled::catch(std::panic::AssertUnwindSafe(|| {
            self.workspace().rage(params)
        }));
        let err = match result {
            Ok(Ok(result)) => return result,
            Ok(Err(err)) => err.to_string(),
            Err(cancelled) => cancelled.to_string(),
        };

        let entries = vec![
            RageEntry::section("Workspace"),
            RageEntry::markup(markup! {
                <Error>"\u{2716} Rage command failed:"</Error> {&err}
            }),
        ];

        RageResult { entries }
    }

    /// Retrieves the configuration status of the given project, defaulting to
    /// [`ConfigurationStatus::Missing`] when the project has no recorded status.
    pub(crate) fn configuration_status(&self, project_key: ProjectKey) -> ConfigurationStatus {
        self.configuration_status
            .pin()
            .get(&project_key)
            .copied()
            .unwrap_or(ConfigurationStatus::Missing)
    }

    /// Updates the status of the configuration for the given project.
    pub(super) fn set_configuration_status(
        &self,
        project_key: ProjectKey,
        status: ConfigurationStatus,
    ) {
        self.notified_broken_configuration
            .pin()
            .remove(&project_key);
        self.configuration_status.pin().insert(project_key, status);
    }

    /// Records the configuration status for the project that owns `path`.
    fn record_configuration_status(&self, path: Option<&Utf8Path>, status: ConfigurationStatus) {
        if let Some(project_key) = path.and_then(|path| self.project_for_path(path)) {
            self.set_configuration_status(project_key, status);
        }
    }

    fn notified_broken_configuration(&self, project_key: ProjectKey) -> bool {
        self.notified_broken_configuration
            .pin()
            .contains(&project_key)
    }
    fn set_notified_broken_configuration(&self, project_key: ProjectKey) {
        self.notified_broken_configuration.pin().insert(project_key);
    }

    pub(crate) fn requires_configuration(&self) -> bool {
        self.extension_settings.read().requires_configuration()
    }

    pub(crate) fn scan_kind_from_editor_features(&self) -> ScanKind {
        self.extension_settings
            .read()
            .scan_kind_from_editor_features()
    }

    pub(crate) fn inline_config(&self) -> Option<Configuration> {
        self.extension_settings.read().inline_config()
    }

    pub(crate) fn is_linting_and_formatting_disabled(&self) -> bool {
        let disabled_for = |status: ConfigurationStatus| match status {
            ConfigurationStatus::Loaded => false,
            ConfigurationStatus::Missing => self.extension_settings.read().requires_configuration(),
            ConfigurationStatus::Error
            | ConfigurationStatus::EditorConfigError
            | ConfigurationStatus::PluginError => false,
            ConfigurationStatus::Loading => true,
        };

        // This is a single on/off switch for the whole editor: formatting and
        // code actions are turned on once, not per file. Each request later checks
        // the file's own project again, so we keep them on as long as at least one
        // project can use them, and only turn them off when every project would.
        let statuses = self.configuration_status.pin();
        if statuses.is_empty() {
            return self.requires_configuration();
        }
        statuses.values().all(|status| disabled_for(*status))
    }

    pub fn position_encoding(&self) -> PositionEncoding {
        self.initialize_params
            .get()
            .map_or(PositionEncoding::Wide(WideEncoding::Utf16), |params| {
                negotiated_encoding(&params.client_capabilities)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_fs::MemoryFileSystem;
    use biome_service::WorkspaceServer;
    use biome_service::workspace::NoopQueryProvider;
    use crossbeam::channel::bounded;
    use std::sync::Mutex;
    use tokio::sync::Notify;
    use tower_lsp_server::LanguageServer;
    use tower_lsp_server::LspService;
    use tower_lsp_server::jsonrpc::Result as JsonRpcResult;
    use tower_lsp_server::ls_types::{InitializeParams, InitializeResult};

    struct DummyServer;

    impl LanguageServer for DummyServer {
        async fn initialize(&self, _params: InitializeParams) -> JsonRpcResult<InitializeResult> {
            Ok(InitializeResult::default())
        }

        async fn shutdown(&self) -> JsonRpcResult<()> {
            Ok(())
        }
    }

    fn create_test_session() -> Arc<Session> {
        let (watcher_tx, _) = bounded(0);
        let (service_tx, service_rx) = watch::channel(ServiceNotification::IndexUpdated);
        let workspace = Arc::new(WorkspaceServer::new(
            Arc::new(MemoryFileSystem::default()),
            watcher_tx,
            service_tx,
            Arc::new(NoopQueryProvider {}),
            None,
        ));
        let db_state = Arc::new(DbState::lsp());

        let cancellation = Arc::new(Notify::new());
        let session_slot: Arc<Mutex<Option<Arc<Session>>>> = Arc::new(Mutex::new(None));

        let slot = session_slot.clone();
        let workspace = workspace.clone();
        let cancellation = cancellation.clone();
        let service_rx = service_rx.clone();

        let (_service, _socket) = LspService::build(move |client| {
            let session = Arc::new(Session::new(
                SessionKey(0),
                client,
                workspace.clone(),
                db_state.clone(),
                cancellation.clone(),
                service_rx.clone(),
            ));
            *slot.lock().unwrap() = Some(session.clone());
            DummyServer
        })
        .finish();

        session_slot
            .lock()
            .unwrap()
            .take()
            .expect("session should be initialized")
    }

    #[tokio::test]
    async fn project_for_path_prefers_most_specific_root() {
        let session = create_test_session();

        let root = BiomePath::new(Utf8PathBuf::from("workspace"));
        let nested = BiomePath::new(Utf8PathBuf::from("workspace/packages/app"));
        let root_key = ProjectKey::new();
        let nested_key = ProjectKey::new();

        session.projects.pin().insert(root, root_key);
        session.projects.pin().insert(nested, nested_key);

        let file_path = Utf8PathBuf::from("workspace/packages/app/src/main.js");
        let selected = session
            .project_for_path(file_path.as_path())
            .expect("expected a project match");

        assert_eq!(selected, nested_key);
    }
}
