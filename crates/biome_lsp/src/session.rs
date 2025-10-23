use crate::diagnostics::LspError;
use crate::documents::Document;
use crate::extension_settings::{CONFIGURATION_SECTION, ExtensionSettings};
use crate::utils;
use anyhow::Result;
use biome_analyze::RuleCategoriesBuilder;
use biome_configuration::ConfigurationPathHint;
use biome_console::markup;
use biome_deserialize::Merge;
use biome_diagnostics::PrintDescription;
use biome_fs::BiomePath;
use biome_line_index::WideEncoding;
use biome_lsp_converters::{PositionEncoding, negotiated_encoding};
use biome_service::Workspace;
use biome_service::WorkspaceError;
use biome_service::configuration::{
    LoadedConfiguration, ProjectScanComputer, load_configuration, load_editorconfig,
};
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    FeaturesBuilder, OpenProjectParams, OpenProjectResult, PullDiagnosticsParams,
    SupportsFeatureParams,
};
use biome_service::workspace::{FileFeaturesResult, ServiceNotification};
use biome_service::workspace::{RageEntry, RageParams, RageResult, UpdateSettingsParams};
use biome_service::workspace::{ScanKind, ScanProjectParams};
use camino::Utf8Path;
use camino::Utf8PathBuf;
use futures::StreamExt;
use futures::stream::futures_unordered::FuturesUnordered;
use papaya::HashMap;
use rustc_hash::FxBuildHasher;
use rustc_hash::FxHashMap;
use serde_json::Value;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicBool, AtomicU8};
use tokio::spawn;
use tokio::sync::Notify;
use tokio::sync::OnceCell;
use tokio::sync::RwLock as TokioRwLock;
use tokio::sync::watch;
use tokio::task::spawn_blocking;
use tower_lsp_server::lsp_types::{ClientCapabilities, Diagnostic, Uri};
use tower_lsp_server::lsp_types::{MessageType, Registration};
use tower_lsp_server::lsp_types::{Unregistration, WorkspaceFolder};
use tower_lsp_server::{Client, UriExt, lsp_types};
use tracing::{debug, error, info, instrument, warn};

pub(crate) struct ClientInformation {
    /// The name of the client
    pub(crate) name: String,

    /// The version of the client
    pub(crate) version: Option<String>,
}

/// Key, uniquely identifying a LSP session.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub(crate) struct SessionKey(pub u64);

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

    pub(crate) workspace: Arc<dyn Workspace>,

    configuration_status: AtomicU8,

    /// A flag to notify a message to the user when the configuration is broken, and the LSP attempts
    /// to update the diagnostics
    notified_broken_configuration: AtomicBool,

    /// Tracks whether the initialized() notification has been received
    initialized: AtomicBool,

    /// Projects opened in this session, mapped from the project's root path to
    /// the associated project key.
    projects: HashMap<BiomePath, ProjectKey>,

    /// Documents opened in this session.
    documents: HashMap<Uri, Document, FxBuildHasher>,

    pub(crate) cancellation: Arc<Notify>,

    /// Receiver for service notifications.
    ///
    /// If we receive a notification here, diagnostics for open documents are
    /// all refreshed.
    service_rx: watch::Receiver<ServiceNotification>,

    /// Lock used to synchronize multiple atomic operations to load the configuration file
    loading_operations:
        TokioRwLock<FxHashMap<Utf8PathBuf, tokio::sync::broadcast::Sender<ConfigurationStatus>>>,
}

/// The parameters provided by the client in the "initialize" request
struct InitializeParams {
    /// The capabilities provided by the client as part of [`lsp_types::InitializeParams`]
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
        workspace: Arc<dyn Workspace>,
        cancellation: Arc<Notify>,
        service_rx: watch::Receiver<ServiceNotification>,
    ) -> Self {
        let config = RwLock::new(ExtensionSettings::new());
        Self {
            key,
            client,
            initialize_params: OnceCell::default(),
            workspace,
            configuration_status: AtomicU8::new(ConfigurationStatus::Missing as u8),
            projects: Default::default(),
            documents: Default::default(),
            extension_settings: config,
            cancellation,
            notified_broken_configuration: AtomicBool::new(false),
            initialized: AtomicBool::new(false),
            service_rx,
            loading_operations: Default::default(),
            workspace_folders: Default::default(),
        }
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
            let mut this_workspace_folders = self.workspace_folders.write().unwrap();
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
            .find(|(project_path, _project_key)| path.starts_with(project_path.as_path()))
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
        self.update_diagnostics_for_document(url, doc).await
    }

    /// Computes diagnostics for the file matching the provided url and publishes
    /// them to the client. Called from [`handlers::text_document`] when a file's
    /// contents changes.
    #[tracing::instrument(level = "debug", skip_all, fields(url = display(url.as_str()), diagnostic_count), err)]
    async fn update_diagnostics_for_document(
        &self,
        url: Uri,
        doc: Document,
    ) -> Result<(), LspError> {
        let biome_path = self.file_path(&url)?;

        if !self.notified_broken_configuration() {
            if self.configuration_status().is_editorconfig_error() {
                self.set_notified_broken_configuration();
                self.client
                    .show_message(MessageType::WARNING, "The .editorconfig file has errors. Biome will report only parsing errors until the file is fixed or its usage is disabled.")
                    .await
            } else if self.configuration_status().is_error() {
                self.set_notified_broken_configuration();
                self.client
                    .show_message(MessageType::WARNING, "The configuration file has errors. Biome will report only parsing errors until the configuration is fixed.")
                    .await;
            } else if self.configuration_status().is_plugin_error() {
                self.set_notified_broken_configuration();
                self.client.show_message(MessageType::WARNING, "The plugin loading has failed. Biome will report only parsing errors until the file is fixed or its usage is disabled.").await
            }
        }

        let FileFeaturesResult {
            features_supported: file_features,
        } = self.workspace.file_features(SupportsFeatureParams {
            project_key: doc.project_key,
            features: FeaturesBuilder::new().with_linter().with_assist().build(),
            path: biome_path.clone(),
        })?;

        if !file_features.supports_lint() && !file_features.supports_assist() {
            self.client
                .publish_diagnostics(url, vec![], Some(doc.version))
                .await;
            return Ok(());
        }

        let diagnostics: Vec<Diagnostic> = {
            let mut categories = RuleCategoriesBuilder::default().with_syntax();
            if self.configuration_status().is_loaded() {
                if file_features.supports_lint() {
                    categories = categories.with_lint();
                }
                if file_features.supports_assist() {
                    categories = categories.with_assist();
                }
            }
            let result = self.workspace.pull_diagnostics(PullDiagnosticsParams {
                project_key: doc.project_key,
                path: biome_path.clone(),
                categories: categories.build(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: Vec::new(),
                pull_code_actions: false,
            })?;

            result
                .diagnostics
                .into_iter()
                .filter_map(|d| {
                    match utils::diagnostic_to_lsp(
                        d,
                        &url,
                        &doc.line_index,
                        self.position_encoding(),
                        None,
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

        info!("Diagnostics sent to the client {}", diagnostics.len());

        self.client
            .publish_diagnostics(url, diagnostics, Some(doc.version))
            .await;

        Ok(())
    }

    /// Updates diagnostics for every [`Document`] in this [`Session`]
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn update_all_diagnostics(&self) {
        let mut futures: FuturesUnordered<_> = self
            .documents
            .pin()
            .iter()
            .map(|(url, doc)| self.update_diagnostics_for_document(url.clone(), doc.clone()))
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

    /// Get the current workspace folders
    pub(crate) fn get_workspace_folders(&self) -> Option<Vec<WorkspaceFolder>> {
        self.workspace_folders.read().unwrap().clone()
    }

    pub(crate) fn update_workspace_folders(&self, folders: Vec<WorkspaceFolder>) {
        let mut workspace_folders = self.workspace_folders.write().unwrap();
        *workspace_folders = Some(folders);
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

    /// This function attempts to read the `biome.json` configuration file from
    /// the root URI and update the workspace settings accordingly
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn load_workspace_settings(self: &Arc<Self>, reload: bool) {
        if let Some(config_path) = self
            .extension_settings
            .read()
            .ok()
            .and_then(|s| s.configuration_path())
        {
            info!("Detected configuration path in the workspace settings.");
            self.set_configuration_status(ConfigurationStatus::Loading);

            let status = self
                .load_biome_configuration_file(ConfigurationPathHint::FromUser(config_path), reload)
                .await;
            debug!("Configuration status: {:?}", status);
            self.set_configuration_status(status);
        } else if let Some(folders) = self.get_workspace_folders() {
            info!("Detected workspace folder.");
            self.set_configuration_status(ConfigurationStatus::Loading);
            for folder in folders {
                info!("Attempt to load the configuration file in {:?}", folder.uri);
                let base_path = folder.uri.to_file_path().map(|p| {
                    Utf8PathBuf::from_path_buf(p.to_path_buf()).expect("To have a valid UTF-8 path")
                });
                match base_path {
                    Some(base_path) => {
                        let status = self
                            .load_biome_configuration_file(
                                ConfigurationPathHint::FromWorkspace(base_path),
                                reload,
                            )
                            .await;
                        debug!("Configuration status: {:?}", status);
                        self.set_configuration_status(status);
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
            let status = self.load_biome_configuration_file(base_path, reload).await;
            self.set_configuration_status(status);
        }
    }

    #[instrument(level = "debug", skip(self))]
    pub(crate) async fn scan_project(
        self: &Arc<Self>,
        project_key: ProjectKey,
        scan_kind: ScanKind,
        force: bool,
    ) {
        let session = self.clone();

        spawn_blocking(move || {
            let result = session.workspace.scan_project(ScanProjectParams {
                project_key,
                watch: true,
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
    }

    #[tracing::instrument(level = "debug", skip(self))]
    pub(super) async fn load_biome_configuration_file(
        self: &Arc<Self>,
        base_path: ConfigurationPathHint,
        reload: bool,
    ) -> ConfigurationStatus {
        let current_status = self.configuration_status();
        if current_status.is_loaded() && !reload {
            return current_status;
        }
        // Check if there's already an ongoing operation for this path
        let path_to_index = base_path.to_path_buf().unwrap_or_default();
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
        let loaded_configuration = match load_configuration(self.workspace.fs(), base_path.clone())
        {
            Ok(loaded_configuration) => loaded_configuration,
            Err(err) => {
                error!("Couldn't load the configuration file, reason:\n {err}");
                self.client.log_message(MessageType::ERROR, &err).await;
                return ConfigurationStatus::Error;
            }
        };

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

        let fs = self.workspace.fs();
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

        // If the configuration from the LSP or the workspace, the directory path is used as
        // the working directory. Otherwise, the base path of the session is used, then the current
        // working directory is used as the last resort.
        let path = match &base_path {
            ConfigurationPathHint::FromLsp(path) | ConfigurationPathHint::FromWorkspace(path) => {
                path.to_path_buf()
            }
            _ => self
                .base_path()
                .or_else(|| fs.working_directory())
                .unwrap_or_default(),
        };

        let project_key = match self.project_for_path(&path) {
            Some(project_key) => project_key,
            None => {
                let register_result = self.workspace.open_project(OpenProjectParams {
                    path: path.as_path().into(),
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
        let scan_kind = if scan_kind.is_none() {
            ScanKind::KnownFiles
        } else {
            scan_kind
        };

        let result = self.workspace.update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: configuration_path
                .as_ref()
                .map(Utf8PathBuf::as_path)
                .map(BiomePath::from),
            configuration,
        });

        self.insert_and_scan_project(project_key, path.into(), scan_kind, force)
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
    pub(crate) async fn load_extension_settings(&self) {
        let item = lsp_types::ConfigurationItem {
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

            let mut config = self.extension_settings.write().unwrap();
            if let Err(err) = config.set_workspace_settings(client_configuration) {
                error!("Couldn't set client configuration: {}", err);
            }
        } else {
            info!("Client did not return any configuration");
        }
    }

    /// Broadcast a shutdown signal to all active connections
    pub(crate) fn broadcast_shutdown(&self) {
        self.cancellation.notify_one();
    }

    pub(crate) fn failsafe_rage(&self, params: RageParams) -> RageResult {
        self.workspace.rage(params).unwrap_or_else(|err| {
            let entries = vec![
                RageEntry::section("Workspace"),
                RageEntry::markup(markup! {
                    <Error>"\u{2716} Rage command failed:"</Error> {&format!("{err}")}
                }),
            ];

            RageResult { entries }
        })
    }

    /// Retrieves information regarding the configuration status
    pub(crate) fn configuration_status(&self) -> ConfigurationStatus {
        self.configuration_status
            .load(Ordering::Relaxed)
            .try_into()
            .unwrap()
    }

    /// Updates the status of the configuration
    pub(super) fn set_configuration_status(&self, status: ConfigurationStatus) {
        self.notified_broken_configuration
            .store(false, Ordering::Relaxed);
        self.configuration_status
            .store(status as u8, Ordering::Relaxed);
    }

    fn notified_broken_configuration(&self) -> bool {
        self.notified_broken_configuration.load(Ordering::Relaxed)
    }
    fn set_notified_broken_configuration(&self) {
        self.notified_broken_configuration
            .store(true, Ordering::Relaxed);
    }

    pub(crate) fn requires_configuration(&self) -> bool {
        self.extension_settings
            .read()
            .unwrap()
            .requires_configuration()
    }

    pub(crate) fn is_linting_and_formatting_disabled(&self) -> bool {
        debug!("configuration status {:?}", self.configuration_status());
        match self.configuration_status() {
            ConfigurationStatus::Loaded => false,
            ConfigurationStatus::Missing => self
                .extension_settings
                .read()
                .unwrap()
                .requires_configuration(),
            ConfigurationStatus::Error
            | ConfigurationStatus::EditorConfigError
            | ConfigurationStatus::PluginError => false,
            ConfigurationStatus::Loading => true,
        }
    }

    pub fn position_encoding(&self) -> PositionEncoding {
        self.initialize_params
            .get()
            .map_or(PositionEncoding::Wide(WideEncoding::Utf16), |params| {
                negotiated_encoding(&params.client_capabilities)
            })
    }
}
