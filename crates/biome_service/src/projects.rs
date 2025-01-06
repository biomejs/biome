use crate::settings::{FilesSettings, Settings};
use crate::workspace::FeatureKind;
use crate::{is_dir, WorkspaceError};
use biome_fs::BiomePath;
use biome_project::{NodeJsProject, PackageJson};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::trace;

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
    path: BiomePath,

    /// The settings of the project, usually inferred from the configuration
    /// file e.g. `biome.json`.
    settings: Settings,

    /// Optional Node.js-specific package information, if relevant for the
    /// project.
    ///
    /// TODO: This should be moved into the upcoming `ProjectLayout` service
    ///       data.
    package: Option<NodeJsProject>,
}

impl Projects {
    /// Inserts a new project with the given root path.
    ///
    /// Returns the key of the newly inserted project, or returns an existing
    /// project key if a project with the given path already existed.
    pub fn insert_project(&self, path: Utf8PathBuf) -> ProjectKey {
        let path = BiomePath::new(path);
        trace!("Insert workspace folder: {path:?}");

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
                package: None,
            },
        );
        key
    }

    /// Removes the project with the given key.
    pub fn remove_project(&self, project_key: ProjectKey) {
        self.0.pin().remove(&project_key);
    }

    /// Retrieves the settings for the given project.
    pub fn get_settings(&self, project_key: ProjectKey) -> Option<Settings> {
        self.0
            .pin()
            .get(&project_key)
            .map(|data| data.settings.clone())
    }

    /// Unsafe version of [Projects::get_settings] that return an error
    pub fn unwrap_settings(&self, project_key: ProjectKey) -> Result<Settings, WorkspaceError> {
        self.0
            .pin()
            .get(&project_key)
            .map(|data| data.settings.clone())
            .ok_or(WorkspaceError::no_project())
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
            package: project_data.package.clone(),
        };

        data.insert(project_key, project_data);
    }

    pub fn get_manifest(&self, project_key: ProjectKey) -> Option<PackageJson> {
        self.0
            .pin()
            .get(&project_key)
            .and_then(|data| data.package.as_ref())
            .map(|project| project.manifest.clone())
    }

    pub fn get_project_path(&self, project_key: ProjectKey) -> Option<BiomePath> {
        self.0.pin().get(&project_key).map(|data| data.path.clone())
    }

    pub fn insert_manifest(&self, project_key: ProjectKey, manifest: NodeJsProject) {
        let data = self.0.pin();
        let Some(project_data) = data.get(&project_key) else {
            return;
        };

        let project_data = ProjectData {
            path: project_data.path.clone(),
            settings: project_data.settings.clone(),
            package: Some(manifest),
        };

        data.insert(project_key, project_data);
    }

    /// Checks whether the given `path` belongs to project with the given path
    /// and no other project.
    pub fn path_belongs_only_to_project_with_path(
        &self,
        path: &BiomePath,
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
        let (feature_included_files, feature_ignored_files) = match feature {
            FeatureKind::Format => {
                let formatter = &settings.formatter;
                (&formatter.included_files, &formatter.ignored_files)
            }
            FeatureKind::Lint => {
                let linter = &settings.linter;
                (&linter.included_files, &linter.ignored_files)
            }

            FeatureKind::Assist => {
                let assists = &settings.assist;
                (&assists.included_files, &assists.ignored_files)
            }
            // TODO: enable once the configuration is available
            FeatureKind::Search => return false, // There is no search-specific config.
            FeatureKind::Debug => return false,
        };
        let is_feature_included = feature_included_files.is_empty()
            || is_dir(path)
            || feature_included_files.matches_path(path);
        !is_feature_included || feature_ignored_files.matches_path(path)
    }
}
