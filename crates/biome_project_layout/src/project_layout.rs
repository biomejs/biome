use biome_package::{
    Catalogs, NodeJsPackage, Package, PackageJson, PnpmWorkspace, TsConfigJson, TurboJson,
};
use biome_rowan::SendNode;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use rustc_hash::FxBuildHasher;
use std::sync::Arc;

/// The layout used across all open projects.
///
/// Projects are comprised of zero or more packages. This arrangement is
/// intended to reflect the common usage of monorepos, where a single repository
/// may host many packages, and each package is allowed to have its own
/// settings.
///
/// For Biome, a project is where the **top-level** configuration file is, while
/// packages may have their own nested configuration files.
///
/// As a data structure, the project layout is simply a flat mapping from paths
/// to package data. This means that in order to lookup the package that is
/// most relevant for a given file, we may need to do multiple lookups from the
/// most-specific possible package path to the least. This means performance
/// degrades linearly with the depth of the path of a file. For now though, this
/// approach makes it very easy for us to invalidate part of the layout when
/// there are file system changes.
#[derive(Debug, Default)]
pub struct ProjectLayout {
    packages: HashMap<Utf8PathBuf, PackageData, FxBuildHasher>,
    pnpm_workspaces: HashMap<Utf8PathBuf, PnpmWorkspace, FxBuildHasher>,
    catalog_settings: HashMap<Utf8PathBuf, CatalogSettings, FxBuildHasher>,
}

#[derive(Debug)]
struct CatalogSettings {
    pnpm: bool,
    bun: bool,
}

/// The information tracked for each package.
///
/// Because Biome is intended to support multiple kinds of JavaScript projects,
/// the term "package" is somewhat loosely defined. It may be an NPM package,
/// a JSR package, or simply a directory with its own nested `biome.json`.
#[derive(Debug, Default)]
pub struct PackageData {
    /// Optional Node.js-specific package information, if relevant for the
    /// package.
    node_package: Option<NodeJsPackage>,
}

impl ProjectLayout {
    pub fn set_catalog_settings(&self, package_path: Utf8PathBuf, pnpm: bool, bun: bool) {
        self.catalog_settings
            .pin()
            .insert(package_path, CatalogSettings { pnpm, bun });
    }

    fn resolve_workspace_catalogs(
        &self,
        path: &Utf8Path,
        mut manifest: PackageJson,
    ) -> PackageJson {
        if let Some(settings) = self.catalog_settings.pin().get(path) {
            manifest.catalog = self.find_workspace_catalogs(path, settings.pnpm, settings.bun);
        }
        manifest
    }

    pub fn insert_pnpm_workspace(&self, path: Utf8PathBuf, manifest: PnpmWorkspace) {
        self.pnpm_workspaces.pin().insert(path, manifest);
    }

    pub fn remove_pnpm_workspace(&self, path: &Utf8Path) {
        self.pnpm_workspaces.pin().remove(path);
    }

    /// Resolves catalogs from indexed workspace manifests. pnpm takes precedence
    /// when both resolvers are enabled. Bun uses the outermost workspace manifest
    /// among the package's ancestors, even if that manifest contains no catalogs.
    fn find_workspace_catalogs(
        &self,
        package_path: &Utf8Path,
        pnpm: bool,
        bun: bool,
    ) -> Option<Catalogs> {
        if pnpm {
            let workspaces = self.pnpm_workspaces.pin();
            if let Some(catalogs) = package_path.ancestors().find_map(|path| {
                workspaces
                    .get(path)
                    .and_then(|workspace| workspace.catalogs.as_ref())
            }) {
                return Some(catalogs.clone());
            }
        }
        if !bun
            || package_path
                .components()
                .any(|component| component.as_str() == "node_modules")
        {
            return None;
        }
        let packages = self.packages.pin();
        package_path
            .ancestors()
            .filter_map(|path| {
                packages
                    .get(path)
                    .and_then(|data| data.node_package.as_ref())
                    .and_then(|package| package.manifest.as_ref())
                    .filter(|manifest| manifest.has_workspaces)
            })
            .last()
            .and_then(|manifest| manifest.bun_catalogs.clone())
    }

    /// Returns the `package.json` that should be used for the given `path`,
    /// together with the absolute path of the package in which it was found.
    ///
    /// This function will look for the closest `package.json` file in the
    /// ancestors of the given `path`, and returns the first one it finds.
    pub fn find_node_manifest_for_path(
        &self,
        path: &Utf8Path,
    ) -> Option<(Utf8PathBuf, PackageJson)> {
        let packages = self.packages.pin();
        path.ancestors().find_map(|package_path| {
            packages
                .get(package_path)
                .and_then(|data| data.node_package.as_ref())
                .and_then(|node_package| node_package.manifest.as_ref())
                .map(|manifest| {
                    (
                        package_path.to_path_buf(),
                        self.resolve_workspace_catalogs(package_path, manifest.clone()),
                    )
                })
        })
    }

    /// Returns the `package.json` inside the given `package_path`.
    ///
    /// This function does not look for the closest `package.json` file in the
    /// hierarchy, but only returns the one that is stored in the layout for
    /// the given `package_path`.
    pub fn get_node_manifest_for_package(&self, package_path: &Utf8Path) -> Option<PackageJson> {
        self.packages
            .pin()
            .get(package_path)
            .and_then(|data| data.node_package.as_ref())
            .and_then(|node_package| node_package.manifest.as_ref())
            .cloned()
            .map(|manifest| self.resolve_workspace_catalogs(package_path, manifest))
    }

    /// Returns the `package.json` for a dependency by name, walking ancestor
    /// `node_modules` directories to handle hoisted dependencies (e.g. in
    /// monorepos).
    ///
    /// Given a `project_dir` (the directory containing the consumer's
    /// `package.json`) and a `dependency_name` (e.g. `"lodash"` or
    /// `"@scope/pkg"`), this will try:
    /// - `<project_dir>/node_modules/<dependency_name>`
    /// - `<parent>/node_modules/<dependency_name>`
    /// - ... all the way up to the filesystem root
    pub fn get_dependency_manifest(
        &self,
        project_dir: &Utf8Path,
        dependency_name: &str,
    ) -> Option<PackageJson> {
        for ancestor in project_dir.ancestors() {
            let dep_path = ancestor.join("node_modules").join(dependency_name);
            if let Some(manifest) = self.get_node_manifest_for_package(&dep_path) {
                return Some(manifest);
            }
        }
        None
    }

    /// Returns the `tsconfig.json` inside the given `package_path`.
    ///
    /// This function does not look for the closest `tsconfig.json` file in the
    /// hierarchy, but only returns the one that is stored in the layout for
    /// the given `package_path`.
    pub fn get_tsconfig_json_for_package(&self, package_path: &Utf8Path) -> Option<TsConfigJson> {
        self.packages
            .pin()
            .get(package_path)
            .and_then(|data| data.node_package.as_ref())
            .and_then(|node_package| node_package.tsconfig.as_ref())
            .cloned()
    }

    /// Returns the `turbo.json` that should be used for the given `path`,
    /// together with the absolute path of the package in which it was found.
    ///
    /// This function will look for the closest `turbo.json` file in the
    /// ancestors of the given `path`, and returns the first one it finds.
    pub fn find_turbo_json_for_path(
        &self,
        path: &Utf8Path,
    ) -> Option<(Utf8PathBuf, Arc<TurboJson>)> {
        let packages = self.packages.pin();
        path.ancestors().find_map(|package_path| {
            packages
                .get(package_path)
                .and_then(|data| data.node_package.as_ref())
                .and_then(|node_package| node_package.turbo_json.as_ref())
                .map(|turbo_json| (package_path.to_path_buf(), Arc::clone(turbo_json)))
        })
    }

    /// Returns ALL `turbo.json` files that apply to the given `path`.
    ///
    /// In a Turborepo monorepo, environment variables can be declared in:
    /// 1. A package-level `turbo.json` in the package directory
    /// 2. The root `turbo.json` at the repository root
    ///
    /// This function returns all turbo.json files found in the ancestors of
    /// the given path, ordered from closest (package-level) to furthest (root).
    pub fn find_all_turbo_json_for_path(&self, path: &Utf8Path) -> Vec<Arc<TurboJson>> {
        let packages = self.packages.pin();
        path.ancestors()
            .filter_map(|package_path| {
                packages
                    .get(package_path)
                    .and_then(|data| data.node_package.as_ref())
                    .and_then(|node_package| node_package.turbo_json.as_ref())
                    .map(Arc::clone)
            })
            .collect()
    }

    /// Returns the `turbo.json` inside the given `package_path`.
    ///
    /// This function does not look for the closest `turbo.json` file in the
    /// hierarchy, but only returns the one that is stored in the layout for
    /// the given `package_path`.
    pub fn get_turbo_json_for_package(&self, package_path: &Utf8Path) -> Option<Arc<TurboJson>> {
        self.packages
            .pin()
            .get(package_path)
            .and_then(|data| data.node_package.as_ref())
            .and_then(|node_package| node_package.turbo_json.as_ref())
            .map(Arc::clone)
    }

    /// Inserts a `package.json` manifest for the package at the given `path`.
    ///
    /// `path` refers to the package directory, not the `package.json` file
    /// itself.
    pub fn insert_node_manifest(&self, path: Utf8PathBuf, manifest: PackageJson) {
        self.packages.pin().update_or_insert_with(
            path,
            |data| {
                let node_js_package = NodeJsPackage {
                    manifest: Some(manifest.clone()),
                    diagnostics: Default::default(),
                    tsconfig: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.tsconfig.clone()),
                    turbo_json: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.turbo_json.clone()),
                };

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let node_js_package = NodeJsPackage {
                    manifest: Some(manifest.clone()),
                    ..Default::default()
                };

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    /// Inserts a `tsconfig.json` manifest for the package at the given `path`.
    ///
    /// `path` refers to the package directory, not the `package.json` file
    /// itself.
    pub fn insert_tsconfig(&self, path: Utf8PathBuf, tsconfig: TsConfigJson) {
        self.packages.pin().update_or_insert_with(
            path,
            |data| {
                let node_js_package = NodeJsPackage {
                    manifest: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.manifest.clone()),
                    diagnostics: Default::default(),
                    tsconfig: Some(tsconfig.clone()),
                    turbo_json: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.turbo_json.clone()),
                };

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let node_js_package = NodeJsPackage {
                    tsconfig: Some(tsconfig.clone()),
                    ..Default::default()
                };

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    /// Inserts a `turbo.json` manifest for the package at the given `path`.
    ///
    /// `path` refers to the package directory, not the `turbo.json` file
    /// itself.
    pub fn insert_turbo_json(&self, path: Utf8PathBuf, turbo_json: TurboJson) {
        let turbo_json = Arc::new(turbo_json);
        self.packages.pin().update_or_insert_with(
            path,
            |data| {
                let node_js_package = NodeJsPackage {
                    manifest: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.manifest.clone()),
                    diagnostics: Default::default(),
                    tsconfig: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.tsconfig.clone()),
                    turbo_json: Some(Arc::clone(&turbo_json)),
                };

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let node_js_package = NodeJsPackage {
                    turbo_json: Some(Arc::clone(&turbo_json)),
                    ..Default::default()
                };

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    /// Inserts a `package.json` manifest for the package at the given `path`,
    /// parsing the manifest on demand.
    ///
    /// See also [Self::insert_node_manifest()].
    pub fn insert_serialized_node_manifest(&self, path: Utf8PathBuf, manifest: &SendNode) {
        self.packages.pin().update_or_insert_with(
            path.clone(),
            |data| {
                let mut node_js_package = NodeJsPackage {
                    manifest: Default::default(),
                    diagnostics: Default::default(),
                    tsconfig: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.tsconfig.clone()),
                    turbo_json: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.turbo_json.clone()),
                };
                node_js_package.insert_serialized_manifest(
                    &manifest.to_language_root(),
                    &path.join("package.json"),
                );

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let mut node_js_package = NodeJsPackage::default();
                node_js_package.insert_serialized_manifest(
                    &manifest.to_language_root(),
                    &path.join("package.json"),
                );

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    /// Inserts a `tsconfig.json` manifest for the package at the given `path`,
    /// parsing the manifest on demand.
    pub fn insert_serialized_tsconfig(&self, path: Utf8PathBuf, manifest: &SendNode) {
        self.packages.pin().update_or_insert_with(
            path.clone(),
            |data| {
                let mut node_js_package = NodeJsPackage {
                    manifest: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.manifest.clone()),
                    diagnostics: Default::default(),
                    tsconfig: Default::default(),
                    turbo_json: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.turbo_json.clone()),
                };
                node_js_package.insert_serialized_tsconfig(
                    &manifest.to_language_root(),
                    &path.join("tsconfig.json"),
                );

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let mut node_js_package = NodeJsPackage::default();
                node_js_package.insert_serialized_tsconfig(
                    &manifest.to_language_root(),
                    &path.join("tsconfig.json"),
                );

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    /// Inserts a `turbo.json` manifest for the package at the given `path`,
    /// parsing the manifest on demand.
    ///
    /// `filename` should be the actual filename (e.g., `"turbo.json"` or `"turbo.jsonc"`).
    pub fn insert_serialized_turbo_json(
        &self,
        path: Utf8PathBuf,
        manifest: &SendNode,
        filename: &str,
    ) {
        self.packages.pin().update_or_insert_with(
            path.clone(),
            |data| {
                let mut node_js_package = NodeJsPackage {
                    manifest: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.manifest.clone()),
                    diagnostics: Default::default(),
                    tsconfig: data
                        .node_package
                        .as_ref()
                        .and_then(|package| package.tsconfig.clone()),
                    turbo_json: Default::default(),
                };
                node_js_package.insert_serialized_turbo_json(
                    &manifest.to_language_root(),
                    &path.join(filename),
                );

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let mut node_js_package = NodeJsPackage::default();
                node_js_package.insert_serialized_turbo_json(
                    &manifest.to_language_root(),
                    &path.join(filename),
                );

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    /// Returns whether the manifest with the given `path` is indexed in the
    /// project layout.
    ///
    /// Returns `true` for indexed Node.js manifests and `pnpm-workspace.yaml`.
    pub fn is_indexed(&self, path: &Utf8Path) -> bool {
        if path.file_name() == Some("pnpm-workspace.yaml") {
            return path
                .parent()
                .is_some_and(|path| self.pnpm_workspaces.pin().contains_key(path));
        }
        path.parent()
            .and_then(|package_path| {
                self.packages
                    .pin()
                    .get(package_path)
                    .and_then(|data| data.node_package.as_ref())
                    .map(|package| match path.file_name() {
                        Some("package.json") => package.manifest.is_some(),
                        Some("tsconfig.json") => package.tsconfig.is_some(),
                        Some("turbo.json" | "turbo.jsonc") => package.turbo_json.is_some(),
                        _ => false,
                    })
            })
            .unwrap_or_default()
    }

    /// Returns all package paths currently tracked in the layout.
    pub fn package_paths(&self) -> Vec<Utf8PathBuf> {
        self.packages.pin().keys().cloned().collect()
    }

    /// Searches for the `tsconfig.json` file nearest to `path` and calls
    /// `query` on it if found.
    ///
    /// Returns the result of `query` if it was executed.
    pub fn query_tsconfig_for_path<F, R>(&self, path: &Utf8Path, query: F) -> Option<R>
    where
        F: Fn(&TsConfigJson) -> R,
    {
        let query = &query;
        let packages = self.packages.pin();
        path.ancestors().find_map(|package_path| {
            packages
                .get(package_path)
                .and_then(|data| data.node_package.as_ref())
                .and_then(|node_package| node_package.tsconfig.as_ref())
                .map(query)
        })
    }

    /// Removes a `tsconfig.json` manifest from the package with the given
    /// `path`.
    pub fn remove_tsconfig_from_package(&self, path: &Utf8Path) {
        self.packages
            .pin()
            .update(path.to_path_buf(), |data| PackageData {
                node_package: data
                    .node_package
                    .as_ref()
                    .map(NodeJsPackage::without_tsconfig),
            });
    }

    /// Searches for the `turbo.json` file nearest to `path` and calls
    /// `query` on it if found.
    ///
    /// Returns the result of `query` if it was executed.
    pub fn query_turbo_json_for_path<F, R>(&self, path: &Utf8Path, query: F) -> Option<R>
    where
        F: Fn(&TurboJson) -> R,
    {
        let query = &query;
        let packages = self.packages.pin();
        path.ancestors().find_map(|package_path| {
            packages
                .get(package_path)
                .and_then(|data| data.node_package.as_ref())
                .and_then(|node_package| node_package.turbo_json.as_ref())
                .map(|turbo_json| query(turbo_json.as_ref()))
        })
    }

    /// Removes a `turbo.json` manifest from the package with the given
    /// `path`.
    pub fn remove_turbo_json_from_package(&self, path: &Utf8Path) {
        self.packages
            .pin()
            .update(path.to_path_buf(), |data| PackageData {
                node_package: data
                    .node_package
                    .as_ref()
                    .map(NodeJsPackage::without_turbo_json),
            });
    }

    /// Removes a package and its metadata from the project layout.
    pub fn remove_package(&self, path: &Utf8Path) {
        self.catalog_settings.pin().remove(path);
        self.packages.pin().remove(path);
    }

    /// Unloads all paths from the graph within the given `path`.
    pub fn unload_folder(&self, path: &Utf8Path) {
        let settings = self.catalog_settings.pin();
        for package_path in settings.keys() {
            if package_path.starts_with(path) {
                settings.remove(package_path);
            }
        }
        let workspaces = self.pnpm_workspaces.pin();
        for workspace_path in workspaces.keys() {
            if workspace_path.starts_with(path) {
                workspaces.remove(workspace_path);
            }
        }
        let packages = self.packages.pin();
        for package_path in packages.keys() {
            if package_path.starts_with(path) {
                packages.remove(package_path);
            }
        }
    }
}
