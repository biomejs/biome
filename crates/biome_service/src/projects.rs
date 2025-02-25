use crate::is_dir;
use crate::settings::{FilesSettings, Settings};
use crate::workspace::FeatureKind;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, instrument};

/// Type that holds all the settings and information for different projects
/// inside the workspace.
///
/// ## Terminology
///
/// Every project within a Biome workspace correlates with a single
/// **top-level** `biome.json`. This means that if the `biome.json` is at the
/// root of a monorepo, multiple packages (or "JavaScript projects") may reside
/// within a single project.
#[derive(Debug, Default)]
pub struct Projects(HashMap<ProjectKey, ProjectData, FxBuildHasher>);

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[repr(transparent)]
pub struct ProjectKey(NonZeroUsize);

impl Display for ProjectKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProjectKey {}", self.0.get())
    }
}

impl ProjectKey {
    #[expect(clippy::new_without_default)]
    pub fn new() -> Self {
        static KEY: AtomicUsize = AtomicUsize::new(1);
        let key = KEY.fetch_add(1, Ordering::Relaxed);
        Self(NonZeroUsize::new(key).unwrap())
    }
}

/// The information tracked for each project.
#[derive(Debug, Default)]
struct ProjectData {
    /// The root path of the project. This path should be **absolute**.
    path: Utf8PathBuf,

    /// The "root" settings of the project.
    ///
    /// Usually inferred from the **top-level** configuration file,
    /// e.g. `biome.json`.
    settings: Settings,
}

impl Projects {
    /// Inserts a new project with the given root path.
    ///
    /// Returns the key of the newly inserted project, or returns an existing
    /// project key if a project with the given path already existed.
    #[instrument(skip(self, path), fields(path))]
    pub fn insert_project(&self, path: Utf8PathBuf) -> ProjectKey {
        debug!("Insert workspace folder {}", path.as_str());

        let data = self.0.pin();
        for (key, project_data) in data.iter() {
            if project_data.path == path {
                return *key;
            }
        }

        let key = ProjectKey::new();
        self.0.pin().insert(
            key,
            ProjectData {
                path,
                settings: Settings::default(),
            },
        );
        key
    }

    /// Removes the project with the given key.
    pub fn remove_project(&self, project_key: ProjectKey) {
        self.0.pin().remove(&project_key);
    }

    /// Retrieves the settings for the given project.
    ///
    /// ## Error
    ///
    /// If the project doesn't contain any [Settings]
    pub fn get_settings(&self, project_key: ProjectKey) -> Option<Settings> {
        self.0
            .pin()
            .get(&project_key)
            .map(|data| data.settings.clone())
    }

    /// Retrieves the `files` settings for the given project.
    pub fn get_files_settings(&self, project_key: ProjectKey) -> Option<FilesSettings> {
        self.0
            .pin()
            .get(&project_key)
            .map(|data| data.settings.files.clone())
    }

    /// Sets the settings for the given project.
    pub fn set_settings(&self, project_key: ProjectKey, settings: Settings) {
        let data = self.0.pin();
        let Some(project_data) = data.get(&project_key) else {
            return;
        };

        let project_data = ProjectData {
            path: project_data.path.clone(),
            settings,
        };

        data.insert(project_key, project_data);
    }

    pub fn get_project_path(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        self.0.pin().get(&project_key).map(|data| data.path.clone())
    }

    /// Checks whether the given `path` belongs to project with the given path
    /// and no other project.
    pub fn path_belongs_only_to_project_with_path(
        &self,
        path: &Utf8Path,
        project_path: &Utf8Path,
    ) -> bool {
        let mut belongs_to_project = false;
        let mut belongs_to_other = false;
        for project_data in self.0.pin().values() {
            if path.strip_prefix(project_data.path.as_path()).is_ok() {
                if project_data.path.as_path() == project_path {
                    belongs_to_project = true;
                } else {
                    belongs_to_other = true;
                }
            }
        }

        belongs_to_project && !belongs_to_other
    }

    /// Returns the maximum file size setting for the given project.
    pub fn get_max_file_size(&self, project_key: ProjectKey) -> usize {
        let limit = self
            .0
            .pin()
            .get(&project_key)
            .and_then(|data| data.settings.files.max_size)
            .unwrap_or_default();

        usize::from(limit)
    }

    /// Checks whether a file is ignored through the feature's
    /// `ignore`/`include` settings.
    pub fn is_ignored_by_feature_config(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
        feature: FeatureKind,
    ) -> bool {
        let data = self.0.pin();
        let Some(project_data) = data.get(&project_key) else {
            return false;
        };

        let settings = &project_data.settings;
        let feature_includes_files = match feature {
            FeatureKind::Format => {
                let formatter = &settings.formatter;
                &formatter.includes
            }
            FeatureKind::Lint => {
                let linter = &settings.linter;
                &linter.includes
            }

            FeatureKind::Assist => {
                let assists = &settings.assist;
                &assists.includes
            }
            // TODO: enable once the configuration is available
            FeatureKind::Search => return false, // There is no search-specific config.
            FeatureKind::Debug => return false,
        };

        let mut is_feature_included = true;
        if !feature_includes_files.is_unset() {
            is_feature_included = if is_dir(path) {
                feature_includes_files.matches_directory_with_exceptions(path)
            } else {
                feature_includes_files.matches_with_exceptions(path)
            };
        }
        !is_feature_included
    }
}
