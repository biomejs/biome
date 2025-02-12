use crate::diagnostics::LspError;
use crate::documents::Document;
use crate::extension_settings::ExtensionSettings;
use crate::extension_settings::CONFIGURATION_SECTION;
use crate::utils;
use anyhow::Result;
use biome_analyze::RuleCategoriesBuilder;
use biome_configuration::ConfigurationPathHint;
use biome_console::markup;
use biome_deserialize::Merge;
use biome_diagnostics::{DiagnosticExt, Error, PrintDescription};
use biome_fs::BiomePath;
use biome_lsp_converters::{negotiated_encoding, PositionEncoding, WideEncoding};
use biome_service::configuration::{
    load_configuration, load_editorconfig, ConfigurationExt, LoadedConfiguration,
};
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::projects::ProjectKey;
use biome_service::workspace::ScanProjectFolderParams;
use biome_service::workspace::{
    FeaturesBuilder, GetFileContentParams, OpenProjectParams, PullDiagnosticsParams,
    SupportsFeatureParams,
};
use biome_service::workspace::{RageEntry, RageParams, RageResult, UpdateSettingsParams};
use biome_service::Workspace;
use biome_service::WorkspaceError;
use camino::Utf8Path;
use camino::Utf8PathBuf;
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use papaya::HashMap;
use rustc_hash::FxBuildHasher;
use rustc_hash::FxHashMap;
use serde_json::Value;
use std::sync::atomic::Ordering;
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::Notify;
use tokio::sync::OnceCell;
use tokio::task::spawn_blocking;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::{Diagnostic, Url};
use tower_lsp::lsp_types::{MessageType, Registration};
use tower_lsp::lsp_types::{Unregistration, WorkspaceFolder};
use tracing::{error, info, warn};

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
    pub(crate) client: tower_lsp::Client,

    /// The parameters provided by the client in the "initialize" request
    initialize_params: OnceCell<InitializeParams>,

    /// The settings of the Biome extension (under the `biome` namespace)
    pub(crate) extension_settings: RwLock<ExtensionSettings>,

    pub(crate) workspace: Arc<dyn Workspace>,

    configuration_status: AtomicU8,

    /// A flag to notify a message to the user when the configuration is broken, and the LSP attempts
    /// to update the diagnostics
    notified_broken_configuration: AtomicBool,

    /// Projects opened in this session, mapped from the project's root path to
    /// the associated project key.
    projects: HashMap<BiomePath, ProjectKey>,

    /// Documents opened in this session.
    documents: HashMap<lsp_types::Url, Document, FxBuildHasher>,

    pub(crate) cancellation: Arc<Notify>,

    pub(crate) config_path: Option<Utf8PathBuf>,
}

/// The parameters provided by the client in the "initialize" request
struct InitializeParams {
    /// The capabilities provided by the client as part of [`lsp_types::InitializeParams`]
    client_capabilities: lsp_types::ClientCapabilities,
    client_information: Option<ClientInformation>,
    root_uri: Option<Url>,
    workspace_folders: Option<Vec<WorkspaceFolder>>,
}

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
}

impl ConfigurationStatus {
    pub(crate) const fn is_error(&self) -> bool {
        matches!(self, ConfigurationStatus::Error)
    }

    pub(crate) const fn is_editorconfig_error(&self) -> bool {
        matches!(self, ConfigurationStatus::EditorConfigError)
    }

    pub(crate) const fn is_loaded(&self) -> bool {
        matches!(self, ConfigurationStatus::Loaded)
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
        client: tower_lsp::Client,
        workspace: Arc<dyn Workspace>,
        cancellation: Arc<Notify>,
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
            config_path: None,
            notified_broken_configuration: AtomicBool::new(false),
        }
    }

    pub(crate) fn set_config_path(&mut self, path: Utf8PathBuf) {
        self.config_path = Some(path);
    }

    /// Initialize this session instance with the incoming initialization parameters from the client
    pub(crate) fn initialize(
        &self,
        client_capabilities: lsp_types::ClientCapabilities,
        client_information: Option<ClientInformation>,
        root_uri: Option<Url>,
        workspace_folders: Option<Vec<WorkspaceFolder>>,
    ) {
        let result = self.initialize_params.set(InitializeParams {
            client_capabilities,
            client_information,
            root_uri,
            workspace_folders,
        });

        if let Err(err) = result {
            error!("Failed to initialize session: {err}");
        }
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

    /// Registers an open project with its root path.
    pub(crate) fn insert_project(&self, path: BiomePath, project_key: ProjectKey) {
        self.projects.pin().insert(path, project_key);
    }

    /// Get a [`Document`] matching the provided [`lsp_types::Url`]
    ///
    /// If document does not exist, result is [WorkspaceError::NotFound]
    pub(crate) fn document(&self, url: &lsp_types::Url) -> Result<Document, Error> {
        self.documents
            .pin()
            .get(url)
            .cloned()
            .ok_or_else(|| WorkspaceError::not_found().with_file_path(url.to_string()))
    }

    /// Set the [`Document`] for the provided [`lsp_types::Url`]
    ///
    /// Used by [`handlers::text_document] to synchronize documents with the client.
    pub(crate) fn insert_document(&self, url: lsp_types::Url, document: Document) {
        self.documents.pin().insert(url, document);
    }

    /// Remove the [`Document`] matching the provided [`lsp_types::Url`]
    pub(crate) fn remove_document(&self, url: &lsp_types::Url) -> Option<ProjectKey> {
        self.documents.pin().remove(url).map(|doc| doc.project_key)
    }

    pub(crate) fn file_path(&self, url: &lsp_types::Url) -> Result<BiomePath> {
        let path_to_file = match url.to_file_path() {
            Err(_) => {
                // If we can't create a path, it's probably because the file doesn't exist.
                // It can be a newly created file that it's not on disk
                Utf8PathBuf::from(url.path())
            }
            Ok(path) => Utf8PathBuf::from_path_buf(path).expect("To to have a UTF-8 path"),
        };

        Ok(BiomePath::new(path_to_file))
    }

    /// Computes diagnostics for the file matching the provided url and publishes
    /// them to the client. Called from [`handlers::text_document`] when a file's
    /// contents changes.
    #[tracing::instrument(level = "debug", skip_all, fields(url = display(&url), diagnostic_count), err)]
    pub(crate) async fn update_diagnostics(&self, url: lsp_types::Url) -> Result<(), LspError> {
        let doc = self.document(&url)?;
        self.update_diagnostics_for_document(url, doc).await
    }

    /// Computes diagnostics for the file matching the provided url and publishes
    /// them to the client. Called from [`handlers::text_document`] when a file's
    /// contents changes.
    #[tracing::instrument(level = "debug", skip_all, fields(url = display(&url), diagnostic_count), err)]
    async fn update_diagnostics_for_document(
        &self,
        url: lsp_types::Url,
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
            }
        }

        let file_features = self.workspace.file_features(SupportsFeatureParams {
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
                max_diagnostics: u64::MAX,
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: Vec::new(),
            })?;

            tracing::debug!("biome diagnostics: {:#?}", result.diagnostics);
            let content = self.workspace.get_file_content(GetFileContentParams {
                project_key: doc.project_key,
                path: biome_path.clone(),
            })?;
            let offset = match biome_path.extension() {
                Some("vue") => VueFileHandler::start(content.as_str()),
                Some("astro") => AstroFileHandler::start(content.as_str()),
                Some("svelte") => SvelteFileHandler::start(content.as_str()),
                _ => None,
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

        tracing::Span::current().record("diagnostic_count", diagnostics.len());

        self.client
            .publish_diagnostics(url, diagnostics, Some(doc.version))
            .await;

        Ok(())
    }

    /// Updates diagnostics for every [`Document`] in this [`Session`]
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
    pub(crate) fn can_register_did_change_configuration(&self) -> bool {
        self.initialize_params
            .get()
            .and_then(|c| c.client_capabilities.workspace.as_ref())
            .and_then(|c| c.did_change_configuration)
            .and_then(|c| c.dynamic_registration)
            == Some(true)
    }

    /// Get the current workspace folders
    pub(crate) fn get_workspace_folders(&self) -> Option<&Vec<WorkspaceFolder>> {
        self.initialize_params
            .get()
            .and_then(|c| c.workspace_folders.as_ref())
    }

    /// Returns the base path of the workspace on the filesystem if it has one
    pub(crate) fn base_path(&self) -> Option<Utf8PathBuf> {
        let initialize_params = self.initialize_params.get()?;

        let root_uri = initialize_params.root_uri.as_ref()?;
        match root_uri.to_file_path() {
            Ok(base_path) => {
                Some(Utf8PathBuf::from_path_buf(base_path).expect("To have a UTF-8 path"))
            }
            Err(()) => {
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

    /// This function attempts to read the `biome.json` configuration file from
    /// the root URI and update the workspace settings accordingly
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn load_workspace_settings(self: &Arc<Self>) {
        // Providing a custom configuration path will not allow to support workspaces
        if let Some(config_path) = &self.config_path {
            let base_path = ConfigurationPathHint::FromUser(config_path.clone());
            let status = self.load_biome_configuration_file(base_path).await;
            self.set_configuration_status(status);
        } else if let Some(folders) = self.get_workspace_folders() {
            info!("Detected workspace folder.");
            self.set_configuration_status(ConfigurationStatus::Loading);
            for folder in folders {
                info!("Attempt to load the configuration file in {:?}", folder.uri);
                let base_path = folder
                    .uri
                    .to_file_path()
                    .map(|p| Utf8PathBuf::from_path_buf(p).expect("To have a valid UTF-8 path"));
                match base_path {
                    Ok(base_path) => {
                        let status = self
                            .load_biome_configuration_file(ConfigurationPathHint::FromWorkspace(
                                base_path,
                            ))
                            .await;
                        self.set_configuration_status(status);
                    }
                    Err(_) => {
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
            let status = self.load_biome_configuration_file(base_path).await;
            self.set_configuration_status(status);
        }
    }

    pub(crate) async fn scan_project_folder(
        self: &Arc<Self>,
        project_key: ProjectKey,
        project_path: BiomePath,
    ) {
        let session = self.clone();
        let _ = spawn_blocking(move || {
            let result = session
                .workspace
                .scan_project_folder(ScanProjectFolderParams {
                    project_key,
                    path: Some(project_path),
                });
            if let Err(err) = result {
                error!("Failed to scan project: {err}");
            }
        })
        .await;
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn load_biome_configuration_file(
        self: &Arc<Self>,
        base_path: ConfigurationPathHint,
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

        if loaded_configuration.double_configuration_found {
            warn!("Both biome.json and biome.jsonc files were found in the same folder. Biome will use the biome.json file.");
            self.client.log_message(MessageType::WARNING, "Both biome.json and biome.jsonc files were found in the same folder. Biome will use the biome.json file.").await;
        }

        info!("Configuration loaded successfully from disk.");
        info!("Update workspace settings.");

        let LoadedConfiguration {
            configuration: fs_configuration,
            directory_path: configuration_path,
            ..
        } = loaded_configuration;

        let fs = self.workspace.fs();
        let should_use_editorconfig = fs_configuration.use_editorconfig();
        let mut configuration = if should_use_editorconfig {
            let (editorconfig, editorconfig_diagnostics) = {
                let search_path = configuration_path
                    .clone()
                    .unwrap_or_else(|| fs.working_directory().unwrap_or_default());
                match load_editorconfig(fs, search_path) {
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

        let result = configuration.retrieve_gitignore_matches(fs, configuration_path.as_deref());
        let (vcs_base_path, gitignore_matches) = match result {
            Ok(result) => result,
            Err(err) => {
                error!("Couldn't load the configuration file, reason:\n {err}");
                self.client.log_message(MessageType::ERROR, &err).await;
                return ConfigurationStatus::Error;
            }
        };

        let path = match &base_path {
            ConfigurationPathHint::FromLsp(path) | ConfigurationPathHint::FromWorkspace(path) => {
                path.as_path().into()
            }
            _ => fs
                .working_directory()
                .map(BiomePath::from)
                .unwrap_or_default(),
        };
        let register_result = self.workspace.open_project(OpenProjectParams {
            path: path.clone(),
            open_uninitialized: true,
        });
        let project_key = match register_result {
            Ok(result) => result,
            Err(error) => {
                error!("Failed to register the project folder: {error}");
                self.client.log_message(MessageType::ERROR, &error).await;
                return ConfigurationStatus::Error;
            }
        };

        let result = self.workspace.update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: configuration_path.map(BiomePath::from),
            configuration,
            vcs_base_path: vcs_base_path.map(BiomePath::from),
            gitignore_matches,
        });

        self.scan_project_folder(project_key, path).await;

        if let Err(error) = result {
            error!("Failed to set workspace settings: {error}");
            self.client.log_message(MessageType::ERROR, &error).await;
            ConfigurationStatus::Error
        } else {
            ConfigurationStatus::Loaded
        }
    }

    /// Requests "workspace/configuration" from client and updates Session config
    #[tracing::instrument(level = "debug", skip(self))]
    pub(crate) async fn load_extension_settings(&self) {
        let item = lsp_types::ConfigurationItem {
            scope_uri: None,
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
    fn set_configuration_status(&self, status: ConfigurationStatus) {
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

    pub(crate) fn is_linting_and_formatting_disabled(&self) -> bool {
        match self.configuration_status() {
            ConfigurationStatus::Loaded => false,
            ConfigurationStatus::Missing => self
                .extension_settings
                .read()
                .unwrap()
                .requires_configuration(),
            ConfigurationStatus::Error | ConfigurationStatus::EditorConfigError => false,
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
