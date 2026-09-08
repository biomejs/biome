use crate::WorkspaceError;
use crate::file_handlers::Capabilities;
use crate::settings::{Settings, SettingsIdentity, SettingsSelectionKey, SettingsWithEditor};
use crate::workspace::{FeatureName, FeaturesSupported, FileFeaturesResult, IgnoreKind};
use biome_fs::{ConfigName, FileSystem};
use biome_languages::DocumentFileSource;
use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

pub struct GetFileFeaturesParams<'a> {
    pub fs: &'a dyn FileSystem,
    pub project_key: ProjectKey,
    pub path: &'a Utf8Path,
    pub requested_features: FeatureName,
    pub language: DocumentFileSource,
    pub capabilities: &'a Capabilities,
    pub handle: &'a SettingsWithEditor<'a>,
    pub skip_ignore_check: bool,
    pub not_requested_features: FeatureName,
}

/// Type that holds all the settings and information for a project
/// inside the workspace.
///
/// This type identifies a single project, while the map that tracks
/// multiple projects resides in the salsa databse.
///
/// ## Terminology
///
/// Every project within a Biome workspace correlates with a single
/// **top-level** `biome.json`. This means that if the `biome.json` is at the
/// root of a monorepo, multiple packages (or "JavaScript projects") may reside
/// within a single project.
#[salsa::input]
pub struct ProjectInput {
    #[returns(copy)]
    project_key: ProjectKey,

    /// The root path of the project. This path should be **absolute**.
    #[returns(ref)]
    pub(crate) path: Utf8PathBuf,

    /// The "root" settings of the project.
    ///
    /// Usually inferred from the **top-level** configuration file,
    /// e.g. `biome.json`.
    #[returns(clone)]
    pub(crate) root_settings: SettingsIdentity,

    /// Optional nested settings, usually populated in monorepo
    /// projects.
    #[returns(ref)]
    pub(crate) nested_settings: BTreeMap<NestedPath, SettingsIdentity>,
}

#[salsa::db]
pub trait ProjectDb: biome_db::Db {
    fn get_project(&self, project_key: &ProjectKey) -> Option<ProjectInput>;

    fn find_project_for_path(&self, path: &Utf8Path) -> Option<ProjectKey>;

    fn for_each_project(&self, f: &mut dyn FnMut(ProjectInput));

    fn get_settings_context_for_path(
        &self,
        project_key: ProjectKey,
        file_path: &Utf8Path,
    ) -> Option<(
        ProjectInput,
        SettingsSelectionKey,
        Utf8PathBuf,
        SettingsIdentity,
    )> {
        let project = self.get_project(&project_key)?;

        for (settings_path, settings) in project.nested_settings(self) {
            if file_path.starts_with(settings_path.as_ref()) {
                return Some((
                    project,
                    SettingsSelectionKey::Nested(settings_path.clone()),
                    settings_path.as_ref().to_path_buf(),
                    settings.clone(),
                ));
            }
        }

        Some((
            project,
            SettingsSelectionKey::Root,
            project.path(self).to_path_buf(),
            project.root_settings(self),
        ))
    }

    fn get_project_path(&self, project_key: ProjectKey) -> Option<Utf8PathBuf> {
        Some(self.get_project(&project_key)?.path(self).to_path_buf())
    }

    /// Retrieves the correct settings for the given path.
    fn get_settings_based_on_path(
        &self,
        project_key: ProjectKey,
        file_path: &Utf8Path,
    ) -> Option<Arc<Settings>> {
        self.get_settings_context_for_path(project_key, file_path)
            .map(|(_, _, _, settings)| settings.clone_arc())
    }

    /// Retrieves the correct settings and working directory for the given project.
    fn get_settings_and_wd_based_on_path(
        &self,
        project_key: ProjectKey,
        file_path: &Utf8Path,
    ) -> Option<(Utf8PathBuf, Arc<Settings>)> {
        self.get_settings_context_for_path(project_key, file_path)
            .map(|(_, _, working_directory, settings)| (working_directory, settings.clone_arc()))
    }

    /// Retrieves the correct settings for the given project.
    fn get_nested_settings(
        &self,
        project_key: ProjectKey,
        file_path: &Utf8Path,
    ) -> Option<Arc<Settings>> {
        let data = self.get_project(&project_key)?;

        data.nested_settings(self)
            .iter()
            .find_map(|(project_path, settings)| {
                file_path
                    .starts_with(project_path.as_ref())
                    .then(|| settings.clone_arc())
            })
    }

    /// Whether the project has been registered
    fn is_project_registered(&self, project_key: ProjectKey) -> bool {
        self.get_project(&project_key).is_some()
    }

    fn get_root_settings(&self, project_key: ProjectKey) -> Option<Arc<Settings>> {
        self.get_project(&project_key)
            .map(|data| data.root_settings(self).clone_arc())
    }

    /// Returns whether a path is force-ignored using a forced negation (`!!`)
    /// as part of `files.includes`.
    fn is_force_ignored(&self, project_key: ProjectKey, path: &Utf8Path) -> bool {
        let Some(project_data) = self.get_project(&project_key) else {
            return false;
        };

        // Deprecated: Check `experimentalScannerIgnores` too.
        let root_settings = project_data.root_settings(self);
        let ignore_entries = &root_settings.as_ref().files.scanner_ignore_entries;
        if path.components().any(|component| {
            ignore_entries
                .iter()
                .any(|entry| entry == component.as_os_str().as_encoded_bytes())
        }) {
            return true;
        }

        let nested_settings = project_data.nested_settings(self);
        let root_settings = project_data.root_settings(self);

        let includes = nested_settings
            .iter()
            .find(|(project_path, _)| path.starts_with(project_path.as_ref()))
            .map_or(&root_settings.as_ref().files.includes, |(_, settings)| {
                &settings.as_ref().files.includes
            });
        includes.is_force_ignored(path)
    }

    fn is_ignored_by_top_level_config(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
        is_dir: bool,
        ignore_kind: IgnoreKind,
    ) -> bool {
        match self.get_project(&project_key) {
            Some(project_data) => {
                self.is_ignored_by_top_level_config_inner(&project_data, path, is_dir, ignore_kind)
            }
            None => false,
        }
    }

    fn is_ignored(
        &self,
        project_key: ProjectKey,
        path: &Utf8Path,
        is_dir: bool,
        features: FeatureName,
        ignore_kind: IgnoreKind,
    ) -> bool {
        let data = self.get_project(&project_key);
        let Some(project_data) = data else {
            return false;
        };

        let is_ignored_by_top_level_config =
            self.is_ignored_by_top_level_config_inner(&project_data, path, is_dir, ignore_kind);

        // If there are specific features enabled, but all of them ignore the
        // path, then we treat the path as ignored too.
        let is_ignored_by_features = !features.is_empty()
            && features.iter().all(|feature| {
                project_data
                    .root_settings(self)
                    .as_ref()
                    .is_path_ignored_for_feature(path, feature)
            });

        is_ignored_by_top_level_config || is_ignored_by_features
    }

    fn get_file_features(
        &self,
        GetFileFeaturesParams {
            fs: _,
            project_key,
            path,
            requested_features,
            language,
            capabilities,
            handle,
            skip_ignore_check,
            not_requested_features: denied_features,
        }: GetFileFeaturesParams<'_>,
    ) -> Result<FileFeaturesResult, WorkspaceError> {
        let project_data = self
            .get_project(&project_key)
            .ok_or_else(WorkspaceError::no_project)?;
        let settings = handle.as_ref();
        let mut file_features = FeaturesSupported::default()
            .with_capabilities(capabilities)
            .with_not_requested_features(denied_features)
            .with_settings_and_language(handle, path, capabilities);

        if settings.ignore_unknown_enabled() && language == DocumentFileSource::Unknown {
            file_features.ignore_not_supported();
        } else if path.file_name().is_some_and(|file_name| {
            file_name == ConfigName::biome_json() || file_name == ConfigName::biome_jsonc()
        }) && path
            .parent()
            .is_some_and(|dir_path| dir_path == project_data.path(self))
        {
            // Never ignore Biome's top-level config file
        } else if !skip_ignore_check {
            let is_ignored = {
                let is_ignored_by_top_level_config = self.is_ignored_by_top_level_config_inner(
                    &project_data,
                    path,
                    false,
                    IgnoreKind::Ancestors,
                );

                // If there are specific features enabled, but all of them ignore the
                // path, then we treat the path as ignored too.
                let is_ignored_by_features = !requested_features.is_empty()
                    && requested_features.iter().all(|feature| {
                        project_data
                            .root_settings(self)
                            .as_ref()
                            .is_path_ignored_for_feature(path, feature)
                    });

                is_ignored_by_top_level_config || is_ignored_by_features
            };

            if is_ignored {
                file_features.set_ignored_for_all_features();
            } else {
                for feature in requested_features.iter() {
                    if project_data
                        .root_settings(self)
                        .as_ref()
                        .is_path_ignored_for_feature(path, feature)
                        || settings.is_path_ignored_for_feature(path, feature)
                    {
                        file_features.set_ignored(feature);
                    }
                }
            }
        }

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

    fn is_ignored_by_top_level_config_inner(
        &self,
        project_data: &ProjectInput,
        path: &Utf8Path,
        is_dir: bool,
        ignore_kind: IgnoreKind,
    ) -> bool {
        // First check if the path is ignored by the `files.includes` setting
        // relevant to the given `path`.
        let nested_settings = project_data.nested_settings(self);
        let root_settings = project_data.root_settings(self);
        let includes = nested_settings
            .iter()
            .find(|(project_path, _)| path.starts_with(project_path.as_ref()))
            .map_or(&root_settings.as_ref().files.includes, |(_, settings)| {
                &settings.as_ref().files.includes
            });
        let mut is_included = if is_dir {
            includes.is_dir_included(path)
        } else {
            includes.is_file_included(path)
        };

        // If necessary, check all the ancestors too.
        if ignore_kind == IgnoreKind::Ancestors {
            for ancestor in path.ancestors().skip(1) {
                if !is_included || ancestor == project_data.path(self) {
                    break;
                }

                is_included = is_included && includes.is_dir_included(ancestor)
            }
        }

        let root_path = match ignore_kind {
            IgnoreKind::Ancestors => Some(project_data.path(self).as_path()),
            IgnoreKind::Path => None,
        };
        // VCS settings are used from the root settings, regardless of what
        // package we are analyzing, so we ignore the `path` for those.
        let is_ignored_by_vcs = project_data
            .root_settings(self)
            .as_ref()
            .vcs_settings
            .is_ignored(path, is_dir, root_path);

        !is_included || is_ignored_by_vcs
    }

    /// Checks whether the given `path` belongs to project with the given path
    /// and no other project.
    fn path_belongs_only_to_project_with_path(
        &self,
        path: &Utf8Path,
        project_path: &Utf8Path,
    ) -> bool {
        let mut belongs_to_project = false;
        let mut belongs_to_other = false;
        self.for_each_project(&mut |project| {
            if path.starts_with(project.path(self)) {
                if project.path(self).as_path() == project_path {
                    belongs_to_project = true;
                } else {
                    belongs_to_other = true;
                }
            }
        });

        belongs_to_project && !belongs_to_other
    }
}

/// A project path ordered by how deeply it is nested.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct NestedPath(Utf8PathBuf);

impl NestedPath {
    pub fn new(path: impl Into<Utf8PathBuf>) -> Self {
        Self(path.into())
    }

    /// The number of components in the path. `/repo/packages` counts 3: the
    /// root, `repo` and `packages`.
    fn depth(&self) -> usize {
        self.0.components().count()
    }
}

impl Ord for NestedPath {
    fn cmp(&self, other: &Self) -> Ordering {
        // Deepest first. Two projects at the same depth fall back to the path
        // order, which is only needed to keep the ordering total.
        other
            .depth()
            .cmp(&self.depth())
            .then_with(|| self.0.cmp(&other.0))
    }
}

impl PartialOrd for NestedPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AsRef<Utf8Path> for NestedPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.0
    }
}

impl From<&Utf8Path> for NestedPath {
    fn from(path: &Utf8Path) -> Self {
        Self(path.into())
    }
}

impl From<Utf8PathBuf> for NestedPath {
    fn from(path: Utf8PathBuf) -> Self {
        Self(path)
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
        let key = KEY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(NonZeroUsize::new(key).unwrap())
    }
}
