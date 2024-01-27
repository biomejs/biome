use crate::converters::{negotiated_encoding, PositionEncoding, WideEncoding};
use crate::documents::Document;
use crate::extension_settings::ExtensionSettings;
use crate::extension_settings::CONFIGURATION_SECTION;
use crate::utils;
use anyhow::Result;
use biome_analyze::RuleCategories;
use biome_console::markup;
use biome_diagnostics::PrintDescription;
use biome_fs::{FileSystem, RomePath};
use biome_service::configuration::{load_configuration, LoadedConfiguration};
use biome_service::workspace::{
    FeatureName, FeaturesBuilder, PullDiagnosticsParams, SupportsFeatureParams,
};
use biome_service::workspace::{RageEntry, RageParams, RageResult, UpdateSettingsParams};
use biome_service::{ConfigurationBasePath, Workspace};
use biome_service::{DynRef, WorkspaceError};
use futures::stream::futures_unordered::FuturesUnordered;
use futures::StreamExt;
use rustc_hash::FxHashMap;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::Notify;
use tokio::sync::OnceCell;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::Unregistration;
use tower_lsp::lsp_types::Url;
use tower_lsp::lsp_types::{MessageType, Registration};
use tracing::{debug, error, info};

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

    /// File system to read files inside the workspace
    pub(crate) fs: DynRef<'static, dyn FileSystem>,

    documents: RwLock<FxHashMap<lsp_types::Url, Document>>,

    pub(crate) cancellation: Arc<Notify>,

    pub(crate) config_path: Option<PathBuf>,
}

/// The parameters provided by the client in the "initialize" request
struct InitializeParams {
    /// The capabilities provided by the client as part of [`lsp_types::InitializeParams`]
    client_capabilities: lsp_types::ClientCapabilities,
    client_information: Option<ClientInformation>,
    root_uri: Option<Url>,
}

#[repr(u8)]
enum ConfigurationStatus {
    /// The configuration file was properly loaded
    Loaded = 0,
    /// The configuration file does not exist
    Missing = 1,
    /// The configuration file exists but could not be loaded
    Error = 2,
}

impl TryFrom<u8> for ConfigurationStatus {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Loaded),
            1 => Ok(Self::Missing),
            2 => Ok(Self::Error),
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
        fs: DynRef<'static, dyn FileSystem>,
    ) -> Self {
        let documents = Default::default();
        let config = RwLock::new(ExtensionSettings::new());
        Self {
            key,
            client,
            initialize_params: OnceCell::default(),
            workspace,
            configuration_status: AtomicU8::new(ConfigurationStatus::Missing as u8),
            documents,
            extension_settings: config,
            fs,
            cancellation,
            config_path: None,
        }
    }

    pub(crate) fn set_config_path(&mut self, path: PathBuf) {
        self.config_path = Some(path);
    }

    /// Initialize this session instance with the incoming initialization parameters from the client
    pub(crate) fn initialize(
        &self,
        client_capabilities: lsp_types::ClientCapabilities,
        client_information: Option<ClientInformation>,
        root_uri: Option<Url>,
    ) {
        let result = self.initialize_params.set(InitializeParams {
            client_capabilities,
            client_information,
            root_uri,
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

    /// Get a [`Document`] matching the provided [`lsp_types::Url`]
    ///
    /// If document does not exist, result is [SessionError::DocumentNotFound]
    pub(crate) fn document(&self, url: &lsp_types::Url) -> Result<Document, WorkspaceError> {
        self.documents
            .read()
            .unwrap()
            .get(url)
            .cloned()
            .ok_or_else(WorkspaceError::not_found)
    }

    /// Set the [`Document`] for the provided [`lsp_types::Url`]
    ///
    /// Used by [`handlers::text_document] to synchronize documents with the client.
    pub(crate) fn insert_document(&self, url: lsp_types::Url, document: Document) {
        self.documents.write().unwrap().insert(url, document);
    }

    /// Remove the [`Document`] matching the provided [`lsp_types::Url`]
    pub(crate) fn remove_document(&self, url: &lsp_types::Url) {
        self.documents.write().unwrap().remove(url);
    }

    pub(crate) fn file_path(&self, url: &lsp_types::Url) -> Result<RomePath> {
        let mut path_to_file = match url.to_file_path() {
            Err(_) => {
                // If we can't create a path, it's probably because the file doesn't exist.
                // It can be a newly created file that it's not on disk
                PathBuf::from(url.path())
            }
            Ok(path) => path,
        };

        let relative_path = self.initialize_params.get().and_then(|initialize_params| {
            let root_uri = initialize_params.root_uri.as_ref()?;
            let root_path = root_uri.to_file_path().ok()?;
            path_to_file.strip_prefix(&root_path).ok()
        });

        if let Some(relative_path) = relative_path {
            path_to_file = relative_path.into();
        }

        Ok(RomePath::new(path_to_file))
    }

    /// Computes diagnostics for the file matching the provided url and publishes
    /// them to the client. Called from [`handlers::text_document`] when a file's
    /// contents changes.
    #[tracing::instrument(level = "trace", skip_all, fields(url = display(&url), diagnostic_count), err)]
    pub(crate) async fn update_diagnostics(&self, url: lsp_types::Url) -> Result<()> {
        let rome_path = self.file_path(&url)?;
        let doc = self.document(&url)?;
        let file_features = self.workspace.file_features(SupportsFeatureParams {
            feature: FeaturesBuilder::new()
                .with_linter()
                .with_organize_imports()
                .build(),
            path: rome_path.clone(),
        })?;

        let diagnostics = if self.is_linting_and_formatting_disabled() {
            tracing::trace!("Linting disabled because Biome configuration is missing and `requireConfiguration` is true.");
            vec![]
        } else if !file_features.supports_for(&FeatureName::Lint)
            && !file_features.supports_for(&FeatureName::OrganizeImports)
        {
            tracing::trace!("linting and import sorting are not supported: {file_features:?}");
            // Sending empty vector clears published diagnostics
            vec![]
        } else {
            let mut categories = RuleCategories::SYNTAX;
            if file_features.supports_for(&FeatureName::Lint) {
                categories |= RuleCategories::LINT
            }
            if file_features.supports_for(&FeatureName::OrganizeImports) {
                categories |= RuleCategories::ACTION
            }
            let result = self.workspace.pull_diagnostics(PullDiagnosticsParams {
                path: rome_path,
                categories,
                max_diagnostics: u64::MAX,
            })?;

            tracing::trace!("biome diagnostics: {:#?}", result.diagnostics);

            let result = result
                .diagnostics
                .into_iter()
                .filter_map(|d| {
                    match utils::diagnostic_to_lsp(
                        d,
                        &url,
                        &doc.line_index,
                        self.position_encoding(),
                    ) {
                        Ok(diag) => Some(diag),
                        Err(err) => {
                            tracing::error!("failed to convert diagnostic to LSP: {err:?}");
                            None
                        }
                    }
                })
                .collect();

            tracing::trace!("lsp diagnostics: {:#?}", result);

            result
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
            .read()
            .unwrap()
            .keys()
            .map(|url| self.update_diagnostics(url.clone()))
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

    /// Returns the base path of the workspace on the filesystem if it has one
    pub(crate) fn base_path(&self) -> Option<PathBuf> {
        let initialize_params = self.initialize_params.get()?;

        let root_uri = initialize_params.root_uri.as_ref()?;
        match root_uri.to_file_path() {
            Ok(base_path) => Some(base_path),
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
    #[tracing::instrument(level = "trace", skip(self))]
    pub(crate) async fn load_workspace_settings(&self) {
        let base_path = if let Some(config_path) = &self.config_path {
            ConfigurationBasePath::FromUser(config_path.clone())
        } else {
            match self.base_path() {
                None => ConfigurationBasePath::default(),
                Some(path) => ConfigurationBasePath::Lsp(path),
            }
        };

        let status = match load_configuration(&self.fs, base_path) {
            Ok(loaded_configuration) => {
                if loaded_configuration.has_errors() {
                    error!("Couldn't load the configuration file, reasons:");
                    for diagnostic in loaded_configuration.as_diagnostics_iter() {
                        let message = PrintDescription(diagnostic).to_string();
                        self.client.log_message(MessageType::ERROR, message).await;
                    }
                    ConfigurationStatus::Error
                } else {
                    let LoadedConfiguration {
                        configuration,
                        directory_path: configuration_path,
                        ..
                    } = loaded_configuration;
                    info!("Loaded workspace setting");
                    debug!("{configuration:#?}");
                    let fs = &self.fs;

                    let result =
                        configuration.retrieve_gitignore_matches(fs, configuration_path.as_deref());

                    match result {
                        Ok((vcs_base_path, gitignore_matches)) => {
                            let result = self.workspace.update_settings(UpdateSettingsParams {
                                working_directory: fs.working_directory(),
                                configuration,
                                vcs_base_path,
                                gitignore_matches,
                            });

                            if let Err(error) = result {
                                error!("Failed to set workspace settings: {}", error);
                                ConfigurationStatus::Error
                            } else {
                                ConfigurationStatus::Loaded
                            }
                        }
                        Err(err) => {
                            error!("Couldn't load the configuration file, reason:\n {}", err);
                            ConfigurationStatus::Error
                        }
                    }
                }
            }

            Err(err) => {
                error!("Couldn't load the configuration file, reason:\n {}", err);
                ConfigurationStatus::Error
            }
        };

        self.set_configuration_status(status);
    }

    /// Requests "workspace/configuration" from client and updates Session config
    #[tracing::instrument(level = "trace", skip(self))]
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
        match self.workspace.rage(params) {
            Ok(result) => result,
            Err(err) => {
                let entries = vec![
                    RageEntry::section("Workspace"),
                    RageEntry::markup(markup! {
                        <Error>"\u{2716} Rage command failed:"</Error> {&format!("{err}")}
                    }),
                ];

                RageResult { entries }
            }
        }
    }

    fn configuration_status(&self) -> ConfigurationStatus {
        self.configuration_status
            .load(Ordering::Relaxed)
            .try_into()
            .unwrap()
    }

    fn set_configuration_status(&self, status: ConfigurationStatus) {
        self.configuration_status
            .store(status as u8, Ordering::Relaxed);
    }

    pub(crate) fn is_linting_and_formatting_disabled(&self) -> bool {
        match self.configuration_status() {
            ConfigurationStatus::Loaded => false,
            ConfigurationStatus::Missing => self
                .extension_settings
                .read()
                .unwrap()
                .requires_configuration(),
            ConfigurationStatus::Error => true,
        }
    }

    pub fn position_encoding(&self) -> PositionEncoding {
        self.initialize_params
            .get()
            .map(|params| negotiated_encoding(&params.client_capabilities))
            .unwrap_or(PositionEncoding::Wide(WideEncoding::Utf16))
    }
}
