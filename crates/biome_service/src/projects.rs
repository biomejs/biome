use crate::is_dir;
use crate::settings::Settings;
use crate::workspace::FeatureKind;
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

        debug!(
            "Get settings for {} {}",
            file_path.as_str(),
            data.nested_settings.len()
        );
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

    pub fn is_ignored_by_scanner(&self, project_key: ProjectKey, path: &Utf8Path) -> bool {
        self.0.pin().get(&project_key).is_none_or(|data| {
            let ignore_entries = &data.root_settings.files.scanner_ignore_entries;
            path.components().any(|component| {
                ignore_entries
                    .iter()
                    .any(|entry| entry == component.as_os_str().as_encoded_bytes())
            })
        })
    }

    pub fn is_ignored_by_top_level_config(&self, project_key: ProjectKey, path: &Utf8Path) -> bool {
        let is_included = self
            .get_settings_based_on_path(project_key, path)
            .is_some_and(|settings| {
                let includes = &settings.files.includes;

                let mut is_included = true;
                if !includes.is_unset() {
                    is_included = if is_dir(path) {
                        includes.matches_directory_with_exceptions(path)
                    } else {
                        includes.matches_with_exceptions(path)
                    };
                }

                is_included
            });

        // We store ignore matches inside the root settings, regardless of what package we are analyzing
        let is_vcs_ignored = self.get_root_settings(project_key).is_some_and(|settings| {
            if settings.vcs_settings.should_use_ignore_file() {
                settings
                    .vcs_settings
                    .ignore_matches
                    .as_ref()
                    .is_some_and(|ignored_matches| ignored_matches.is_ignored(path, is_dir(path)))
            } else {
                false
            }
        });

        !is_included || is_vcs_ignored
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
        debug!("Set nested settings for {}", path.as_str());
        self.0.pin().update(project_key, |data| {
            let mut nested_settings = data.nested_settings.clone();
            nested_settings.insert(path.clone(), settings.clone());

            ProjectData {
                path: data.path.clone(),
                root_settings: data.root_settings.clone(),
                nested_settings: nested_settings.clone(),
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
            .find(|(_, project_data)| path.starts_with(&project_data.path))
            .map(|(key, _)| *key)
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

    /// Returns the maximum file size setting for the given project.
    pub fn get_max_file_size(&self, project_key: ProjectKey, file_path: &Utf8Path) -> usize {
        let limit = self
            .0
            .pin()
            .get(&project_key)
            .and_then(|data| {
                data.root_settings
                    .override_settings
                    .patterns
                    .first()
                    .and_then(|pattern| {
                        if pattern.is_file_included(file_path) {
                            pattern.files.max_size
                        } else {
                            None
                        }
                    })
                    .or(data.root_settings.files.max_size)
            })
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

        let settings = &project_data.root_settings;
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
