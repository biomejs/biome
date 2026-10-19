use biome_configuration::ConfigurationPathHint;
use biome_diagnostics::{Diagnostic, Location, PrintDescription, Resource, Severity, serde};
use biome_fs::BiomePath;
use biome_resolver::FsWithResolverProxy;
use biome_service::configuration::{LoadedLocation, load_configuration};
use biome_service::db::DbState;
use biome_service::projects::ProjectKey;
use biome_service::settings::ModuleGraphResolutionKind;
use biome_service::workspace::{
    GetProjectFilesParams, GritSearchQuery, OpenProjectParams, OpenProjectResult,
    RetryingWorkspace, ScanKind, ScanProjectParams, ServiceNotification, UpdateSettingsParams,
};
use biome_service::{
    Watcher, WatcherInstructionChannel, WatcherKind, WatcherOptions, Workspace, WorkspaceServer,
};
use camino::{Utf8Path, Utf8PathBuf};
use std::io;
use std::sync::{Arc, Mutex, PoisonError};
use std::thread;
use tokio::sync::watch;

/// The Biome project that the server's tools work on: a workspace that holds
/// every file of the project, with the project's Biome configuration loaded.
///
/// The configuration is loaded after the server starts, and the project's files
/// are then indexed and watched, so the workspace stays up to date when files
/// change on disk. When the configuration has errors, the project stops: tools
/// report the errors, and each tool call tries to load the configuration again,
/// so the project resumes as soon as the configuration is fixed. Changes to the
/// configuration files are applied the same way.
///
/// As in the daemon, the workspace database is owned by the process, so updates
/// replace the previous content of a file instead of adding to it.
pub struct Project {
    server: Arc<WorkspaceServer>,
    db_state: Arc<DbState>,
    root: Utf8PathBuf,
    /// The project as last loaded, or `None` when the configuration hasn't
    /// been loaded yet or had errors.
    loaded: Mutex<Option<LoadedProject>>,
    #[cfg(test)]
    notifications: watch::Receiver<ServiceNotification>,
    /// Stops the watcher thread when the project is dropped.
    _watcher_instructions: WatcherInstructionChannel,
}

/// A project whose configuration loaded.
struct LoadedProject {
    key: ProjectKey,
    /// The configuration files that were loaded, with their content at that
    /// time, or `None` for a file that couldn't be read.
    configuration_files: Vec<(Utf8PathBuf, Option<String>)>,
}

impl Project {
    /// Creates the workspace for the project at `root` and starts the file
    /// watcher with `watcher_options`, or a polling watcher when that one
    /// can't start. The configuration is loaded later, by
    /// [Self::ensure_loaded].
    ///
    /// ## Errors
    ///
    /// Returns an error when neither watcher can start, or when the watcher's
    /// thread can't start.
    pub fn new(
        fs: Arc<dyn FsWithResolverProxy>,
        root: Utf8PathBuf,
        watcher_options: WatcherOptions,
    ) -> io::Result<Self> {
        // The operating system can refuse to watch more files, for example
        // when other processes use all the inotify instances on Linux. A
        // polling watcher doesn't need them and still keeps the workspace
        // current, only with a delay.
        let (watcher, instruction_channel) = Watcher::new(watcher_options.clone())
            .or_else(|_| {
                Watcher::new(WatcherOptions {
                    watcher_kind: WatcherKind::Polling,
                    ..watcher_options
                })
            })
            .map_err(io::Error::other)?;
        // The server doesn't use the notifications; tests wait on them.
        let (notification_sender, _) = watch::channel(ServiceNotification::IndexUpdated);
        #[cfg(test)]
        let notifications = notification_sender.subscribe();
        let db_state = Arc::new(DbState::lsp(fs.clone()));
        let server = Arc::new(WorkspaceServer::new(
            fs,
            instruction_channel.sender.clone(),
            notification_sender,
            Arc::new(GritSearchQuery::default()),
            None,
        ));

        let watched_server = Arc::clone(&server);
        let watched_db_state = Arc::clone(&db_state);
        thread::Builder::new()
            .name(String::from("biome::mcp::watcher"))
            .spawn(move || watched_server.start_watcher(&watched_db_state, watcher))?;

        Ok(Self {
            server,
            db_state,
            root,
            loaded: Mutex::new(None),
            #[cfg(test)]
            notifications,
            _watcher_instructions: instruction_channel,
        })
    }

    /// Returns the key of the project, after loading the configuration and
    /// indexing the project when that hasn't happened yet, when a
    /// configuration file changed, or when the configuration had errors.
    ///
    /// Calls wait for each other, so only one of them loads the configuration.
    ///
    /// ## Errors
    ///
    /// Returns a message for the agent that lists the configuration errors.
    pub(crate) fn ensure_loaded(&self) -> Result<ProjectKey, String> {
        let mut loaded = self.loaded.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(project) = loaded.as_ref()
            && !self.configuration_changed(project)
        {
            return Ok(project.key);
        }

        *loaded = None;
        let project = self.load().map_err(|reasons| {
            let reasons: Vec<_> = reasons
                .iter()
                .map(|reason| format!("- {reason}"))
                .collect();
            format!(
                "Biome stopped because the configuration has errors:\n{}\nFix the configuration; the next call loads it again.",
                reasons.join("\n")
            )
        })?;
        let key = project.key;
        *loaded = Some(project);
        Ok(key)
    }

    /// Returns a handle to the project's workspace, which retries the
    /// operations that a concurrent update from the watcher interrupts.
    ///
    /// The project has one workspace server and one database; the handle only
    /// borrows them, as the LSP session does, so every handle works on the
    /// same workspace and creating one costs nothing.
    pub(crate) fn workspace(&self) -> impl Workspace + '_ {
        RetryingWorkspace::new(self.server.with_db_state(&self.db_state))
    }

    /// The directory that relative paths in tool arguments are resolved against.
    pub(crate) fn root(&self) -> &Utf8Path {
        &self.root
    }

    /// Returns a receiver that changes each time the workspace updates its
    /// index, for example after the watcher processed a file change.
    #[cfg(test)]
    pub(crate) fn notifications(&self) -> watch::Receiver<ServiceNotification> {
        self.notifications.clone()
    }

    /// Loads the Biome configuration of the root, opens the project, and
    /// indexes and watches all of its files.
    ///
    /// Returns the descriptions of the configuration errors when there are any,
    /// including errors in the nested configurations that the scan finds.
    fn load(&self) -> Result<LoadedProject, Vec<String>> {
        let workspace = self.workspace();
        let loaded_configuration = load_configuration(
            workspace.fs(),
            ConfigurationPathHint::FromWorkspace(self.root.clone()),
        )
        .map_err(|error| vec![PrintDescription(&error).to_string()])?;
        if loaded_configuration.has_errors() {
            let configuration_path = loaded_configuration.file_path();
            return Err(loaded_configuration
                .as_diagnostics_iter()
                .map(|diagnostic| {
                    describe(
                        PrintDescription(diagnostic).to_string(),
                        diagnostic.location(),
                        &self.root,
                        configuration_path,
                    )
                })
                .collect());
        }

        let extended_configurations = loaded_configuration.extended_configurations();
        let mut configuration_files: Vec<_> = loaded_configuration
            .file_path()
            .map(Utf8Path::to_path_buf)
            .into_iter()
            .chain(extended_configurations.iter().map(|(path, _)| path.clone()))
            .collect();
        let configuration = loaded_configuration.resolved_configuration();
        // A configuration found in a parent folder makes that folder the
        // project's directory, so the project holds every file that the
        // configuration applies to. A configuration from the user's
        // configuration folder applies to the root instead.
        let project_directory = match (
            &loaded_configuration.loaded_location,
            loaded_configuration.directory_path(),
        ) {
            (LoadedLocation::ParentFolder, Some(directory)) => directory.to_path_buf(),
            _ => self.root.clone(),
        };

        // Opening the same directory again returns the key it already has.
        let OpenProjectResult { project_key } = workspace
            .open_project(OpenProjectParams {
                path: BiomePath::new(&project_directory),
                open_uninitialized: true,
            })
            .map_err(|error| vec![PrintDescription(&error).to_string()])?;

        // Indexes every file of the project, so the tools can work on the
        // workspace's documents, which the watcher keeps current.
        let scan_kind = ScanKind::Project;
        let settings = workspace
            .update_settings(UpdateSettingsParams {
                project_key,
                workspace_directory: Some(BiomePath::new(&project_directory)),
                configuration,
                extended_configurations: extended_configurations
                    .into_iter()
                    .map(|(path, configuration)| (BiomePath::from(path), configuration))
                    .collect(),
                module_graph_resolution_kind: ModuleGraphResolutionKind::from(&scan_kind),
            })
            .map_err(|error| vec![PrintDescription(&error).to_string()])?;
        configuration_errors(&settings.diagnostics, &self.root)?;
        let scan = workspace
            .scan_project(ScanProjectParams {
                project_key,
                watch: true,
                // A reload must apply the new configuration to files that are
                // already indexed and watched.
                force: true,
                scan_kind,
                verbose: false,
            })
            .map_err(|error| vec![PrintDescription(&error).to_string()])?;
        configuration_errors(&scan.diagnostics, &self.root)?;

        configuration_files.extend(
            scan.configuration_files
                .iter()
                .map(|path| path.as_path().to_path_buf()),
        );
        let fs = workspace.fs();
        let configuration_files = configuration_files
            .into_iter()
            .map(|path| {
                let content = fs.read_file_from_path(&path).ok();
                (path, content)
            })
            .collect();
        Ok(LoadedProject {
            key: project_key,
            configuration_files,
        })
    }

    /// Returns whether a configuration file of `project` changed on disk since
    /// it was loaded, or a new configuration file, such as a nested
    /// `biome.json`, appeared in the project.
    fn configuration_changed(&self, project: &LoadedProject) -> bool {
        let workspace = self.workspace();
        let fs = workspace.fs();
        let file_changed = project
            .configuration_files
            .iter()
            .any(|(path, content)| fs.read_file_from_path(path).ok() != *content);
        if file_changed {
            return true;
        }

        workspace
            .get_project_files(GetProjectFilesParams {
                project_key: project.key,
            })
            .is_ok_and(|result| {
                result.paths.iter().any(|path| {
                    path.is_config()
                        && !project
                            .configuration_files
                            .iter()
                            .any(|(known, _)| known == path.as_path())
                })
            })
    }
}

/// Returns the descriptions of the configuration errors in `diagnostics`, if
/// there are any.
///
/// Errors that aren't about the configuration, such as a failure of the parser
/// in one file, are tagged as internal and don't stop the project.
fn configuration_errors(
    diagnostics: &[serde::Diagnostic],
    root: &Utf8Path,
) -> Result<(), Vec<String>> {
    let errors: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity() >= Severity::Error && !diagnostic.tags().is_internal()
        })
        .map(|diagnostic| {
            describe(
                PrintDescription(diagnostic).to_string(),
                diagnostic.location(),
                root,
                None,
            )
        })
        .collect();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Returns `description` after the path of the file it's about, relative to
/// `root`. The path comes from `location`, or from `default_path` when
/// `location` has none.
fn describe(
    description: String,
    location: Location,
    root: &Utf8Path,
    default_path: Option<&Utf8Path>,
) -> String {
    let path = match location.resource {
        Some(Resource::File(path)) => Some(Utf8Path::new(path)),
        _ => default_path,
    };
    match path {
        Some(path) => format!("{}: {description}", path.strip_prefix(root).unwrap_or(path)),
        None => description,
    }
}
