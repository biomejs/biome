use crate::WorkspaceError;
use crate::file_handlers::Capabilities;
use crate::settings::Settings;
use crate::workspace::{
    DocumentFileSource, FeatureName, FeaturesSupported, FileFeaturesResult, IgnoreKind,
};
use biome_fs::{ConfigName, FileSystem};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::num::NonZeroUsize;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{debug, instrument};

/// The information tracked for each project.
#[derive(Debug, Default)]
struct ProjectData {
    /// The root path of the project. This path should be **absolute**.
    path: Utf8PathBuf,

    /// The "root" settings of the project.
    ///
    /// Usually inferred from the **top-level** configuration file,
    /// e.g. `biome.json`.
    root_settings: Settings,

    /// Optional nested settings, usually populated in monorepo
    /// projects.
    nested_settings: BTreeMap<Utf8PathBuf, Settings>,
}

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
        data.insert(
            key,
            ProjectData {
                path,
                root_settings: Settings::default(),
                nested_settings: Default::default(),
            },
        );
        key
    }

    /// Removes the project with the given key.
    pub fn remove_project(&self, project_key: ProjectKey) {
        self.0.pin().remove(&project_key);
    }

    /// Retrieves the correct settings for the given project.
    pub fn get_settings_based_on_path(
        &self,
        project_key: ProjectKey,
        file_path: &Utf8Path,
    ) -> Option<Settings> {
        let projects = self.0.pin();
        let data = projects.get(&project_key)?;

        for (project_path, settings) in &data.nested_settings {
            if file_path.starts_with(project_path) {
                return Some(settings.clone());
            }
        }

        Some(data.root_settings.clone())
    }

    /// Retrieves the correct settings for the given project.
    pub fn get_nested_settings(
        &self,
        project_key: ProjectKey,
        file_path: &Utf8Path,
    ) -> Option<Settings> {
        let projects = self.0.pin();
        let data = projects.get(&project_key)?;

        data.nested_settings
            .iter()
            .find_map(|(project_path, settings)| {
                file_path
                    .starts_with(project_path)
                    .then(|| settings.clone())
            })
    }

    /// Whether the project has been registered
    pub fn is_project_registered(&self, project_key: ProjectKey) -> bool {
        self.0.pin().get(&project_key).is_some()
    }

    pub fn get_root_settings(&self, project_key: ProjectKey) -> Option<Settings> {
        self.0
            .pin()
            .get(&project_key)
            .map(|data| data.root_settings.clone())
    }

    /// Returns whether a path is force-ignored using a forced negation (`!!`)
    /// as part of `files.includes`.
    pub fn is_force_ignored(&self, project_key: ProjectKey, path: &Utf8Path) -> bool {
        let data = self.0.pin();
        let Some(project_data) = data.get(&project_key) else {
            return false;
        };

        // Deprecated: Check `experimentalScannerIgnores` too.
        let ignore_entries = &project_data.root_settings.files.scanner_ignore_entries;
        if path.components().any(|component| {
            ignore_entries
                .iter()
                .any(|entry| entry == component.as_os_str().as_encoded_bytes())
        }) {
            return true;
        }

        let includes = project_data
            .nested_settings
            .iter()
            .find(|(project_path, _)| path.starts_with(project_path))
            .map_or(
                &project_data.root_settings.files.includes,
                |(_, settings)| &settings.files.includes,
            );
        includes.is_force_ignored(path)
    }

    pub fn is_ignored_by_top_level_config(
        &self,
        fs: &dyn FileSystem,
        project_key: ProjectKey,
        path: &Utf8Path,
        ignore_kind: IgnoreKind,
    ) -> bool {
        match self.0.pin().get(&project_key) {
            Some(project_data) => {
                is_ignored_by_top_level_config(fs, project_data, path, ignore_kind)
            }
            None => false,
        }
    }

    #[inline]
    pub fn is_ignored(
        &self,
        fs: &dyn FileSystem,
        project_key: ProjectKey,
        path: &Utf8Path,
        features: FeatureName,
        ignore_kind: IgnoreKind,
    ) -> bool {
        let data = self.0.pin();
        let Some(project_data) = data.get(&project_key) else {
            return false;
        };

        let is_ignored_by_top_level_config =
            is_ignored_by_top_level_config(fs, project_data, path, ignore_kind);

        // If there are specific features enabled, but all of them ignore the
        // path, then we treat the path as ignored too.
        let is_ignored_by_features = !features.is_empty()
            && features.iter().all(|feature| {
                project_data
                    .root_settings
                    .is_path_ignored_for_feature(path, feature)
            });

        is_ignored_by_top_level_config || is_ignored_by_features
    }

    #[inline(always)]
    pub fn get_file_features(
        &self,
        fs: &dyn FileSystem,
        project_key: ProjectKey,
        path: &Utf8Path,
        features: FeatureName,
        language: DocumentFileSource,
        capabilities: &Capabilities,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        let data = self.0.pin();
        let project_data = data
            .get(&project_key)
            .ok_or_else(WorkspaceError::no_project)?;

        let settings = project_data
            .nested_settings
            .iter()
            .find(|(project_path, _)| path.starts_with(project_path))
            .map_or(&project_data.root_settings, |(_, settings)| settings);

        let mut file_features = FeaturesSupported::default();
        file_features = file_features.with_capabilities(capabilities);
        file_features = file_features.with_settings_and_language(settings, path, capabilities);
        if settings.ignore_unknown_enabled() && language == DocumentFileSource::Unknown {
            file_features.ignore_not_supported();
        } else if path.file_name().is_some_and(|file_name| {
            file_name == ConfigName::biome_json() || file_name == ConfigName::biome_jsonc()
        }) && path
            .parent()
            .is_some_and(|dir_path| dir_path == project_data.path)
        {
            // Never ignore Biome's top-level config file
        } else if self.is_ignored(fs, project_key, path, features, IgnoreKind::Ancestors) {
            file_features.set_ignored_for_all_features();
        } else {
            for feature in features.iter() {
                if project_data
                    .root_settings
                    .is_path_ignored_for_feature(path, feature)
                    || settings.is_path_ignored_for_feature(path, feature)
                {
                    file_features.set_ignored(feature);
                }
            }
        }

        drop(data);

        // If the file is not ignored by at least one feature, then check that
        // the file is not protected.
        //
        // Protected files must be ignored.
        if !file_features.is_not_processed() && FileFeaturesResult::is_protected_file(path) {
            file_features.set_protected_for_all_features();
        }

        Ok(FileFeaturesResult {
            features_supported: file_features,
        })
    }

    /// Sets the root settings for the given project.
    ///
    /// Does nothing if the project doesn't exist.
    pub fn set_root_settings(&self, project_key: ProjectKey, settings: Settings) {
        self.0.pin().update(project_key, |data| ProjectData {
            path: data.path.clone(),
            root_settings: settings.clone(),
            nested_settings: data.nested_settings.clone(),
        });
    }

    /// Inserts a nested setting.
    ///
    /// Does nothing if the project doesn't exist.
    pub fn set_nested_settings(
        &self,
        project_key: ProjectKey,
        path: Utf8PathBuf,
        settings: Settings,
    ) {
        debug!("Set nested settings for {path}");
        self.0.pin().update(project_key, |data| {
            let mut nested_settings = data.nested_settings.clone();
            nested_settings.insert(path.clone(), settings.clone());

            ProjectData {
                path: data.path.clone(),
                root_settings: data.root_settings.clone(),
                nested_settings,
            }
        });
    }

    pub fn get_project_path(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        self.0.pin().get(&project_key).map(|data| data.path.clone())
    }

    /// Finds the key of the project to which a given path belongs, if any.
    pub fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey> {
        self.0
            .pin()
            .iter()
            .find_map(|(key, project_data)| path.starts_with(&project_data.path).then_some(*key))
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
            if path.starts_with(&project_data.path) {
                if project_data.path.as_path() == project_path {
                    belongs_to_project = true;
                } else {
                    belongs_to_other = true;
                }
            }
        }

        belongs_to_project && !belongs_to_other
    }
}

#[inline]
fn is_ignored_by_top_level_config(
    fs: &dyn FileSystem,
    project_data: &ProjectData,
    path: &Utf8Path,
    ignore_kind: IgnoreKind,
) -> bool {
    // First check if the path is ignored by the `files.includes` setting
    // relevant to the given `path`.
    let includes = project_data
        .nested_settings
        .iter()
        .find(|(project_path, _)| path.starts_with(project_path))
        .map_or(
            &project_data.root_settings.files.includes,
            |(_, settings)| &settings.files.includes,
        );
    let mut is_included = if fs.path_is_dir(path) {
        includes.is_dir_included(path)
    } else {
        includes.is_file_included(path)
    };

    // If necessary, check all the ancestors too.
    if ignore_kind == IgnoreKind::Ancestors {
        for ancestor in path.ancestors().skip(1) {
            if !is_included || ancestor == project_data.path {
                break;
            }

            is_included = is_included && includes.is_dir_included(ancestor)
        }
    }

    let root_path = match ignore_kind {
        IgnoreKind::Ancestors => Some(project_data.path.as_path()),
        IgnoreKind::Path => None,
    };
    // VCS settings are used from the root settings, regardless of what
    // package we are analyzing, so we ignore the `path` for those.
    let is_ignored_by_vcs = project_data
        .root_settings
        .vcs_settings
        .is_ignored(path, root_path);

    !is_included || is_ignored_by_vcs
}

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
