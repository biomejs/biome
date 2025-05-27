use biome_package::{NodeJsPackage, Package, PackageJson, TsConfigJson};
use biome_parser::AnyParse;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use rustc_hash::FxBuildHasher;

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
pub struct ProjectLayout(HashMap<Utf8PathBuf, PackageData, FxBuildHasher>);

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
    /// Returns the `package.json` that should be used for the given `path`,
    /// together with the absolute path of the package in which it was found.
    ///
    /// This function will look for the closest `package.json` file in the
    /// ancestors of the given `path`, and returns the first one it finds.
    pub fn find_node_manifest_for_path(
        &self,
        path: &Utf8Path,
    ) -> Option<(Utf8PathBuf, PackageJson)> {
        let packages = self.0.pin();
        path.ancestors().skip(1).find_map(|package_path| {
            packages
                .get(package_path)
                .and_then(|data| data.node_package.as_ref())
                .and_then(|node_package| node_package.manifest.as_ref())
                .map(|manifest| (package_path.to_path_buf(), manifest.clone()))
        })
    }

    /// Returns the `package.json` inside the given `package_path`.
    ///
    /// This function does not look for the closest `package.json` file in the
    /// hierarchy, but only returns the one that is stored in the layout for
    /// the given `package_path`.
    pub fn get_node_manifest_for_package(&self, package_path: &Utf8Path) -> Option<PackageJson> {
        self.0
            .pin()
            .get(package_path)
            .and_then(|data| data.node_package.as_ref())
            .and_then(|node_package| node_package.manifest.as_ref())
            .cloned()
    }

    /// Returns the `tsconfig.json` inside the given `package_path`.
    ///
    /// This function does not look for the closest `tsconfig.json` file in the
    /// hierarchy, but only returns the one that is stored in the layout for
    /// the given `package_path`.
    pub fn get_tsconfig_json_for_package(&self, package_path: &Utf8Path) -> Option<TsConfigJson> {
        self.0
            .pin()
            .get(package_path)
            .and_then(|data| data.node_package.as_ref())
            .and_then(|node_package| node_package.tsconfig.as_ref())
            .cloned()
    }

    /// Inserts a `package.json` manifest for the package at the given `path`.
    ///
    /// `path` refers to the package directory, not the `package.json` file
    /// itself.
    pub fn insert_node_manifest(&self, path: Utf8PathBuf, manifest: PackageJson) {
        self.0.pin().update_or_insert_with(
            path,
            |data| {
                let mut node_js_package = NodeJsPackage {
                    manifest: Default::default(),
                    diagnostics: Default::default(),
                    tsconfig: data
                        .node_package
                        .as_ref()
                        .map(|package| package.tsconfig.clone())
                        .unwrap_or_default(),
                };
                node_js_package.manifest = Some(manifest.clone());

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

    /// Inserts a `package.json` manifest for the package at the given `path`,
    /// parsing the manifest on demand.
    ///
    /// See also [Self::insert_node_manifest()].
    pub fn insert_serialized_node_manifest(&self, path: Utf8PathBuf, manifest: AnyParse) {
        self.0.pin().update_or_insert_with(
            path,
            |data| {
                let mut node_js_package = NodeJsPackage {
                    manifest: Default::default(),
                    diagnostics: Default::default(),
                    tsconfig: data
                        .node_package
                        .as_ref()
                        .map(|package| package.tsconfig.clone())
                        .unwrap_or_default(),
                };
                node_js_package.insert_serialized_manifest(&manifest.tree());

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let mut node_js_package = NodeJsPackage::default();
                node_js_package.insert_serialized_manifest(&manifest.tree());

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    /// Inserts a `tsconfig.json` manifest for the package at the given `path`,
    /// parsing the manifest on demand.
    pub fn insert_serialized_tsconfig(&self, path: Utf8PathBuf, manifest: AnyParse) {
        self.0.pin().update_or_insert_with(
            path,
            |data| {
                let mut node_js_package = NodeJsPackage {
                    manifest: data
                        .node_package
                        .as_ref()
                        .map(|package| package.manifest.clone())
                        .unwrap_or_default(),
                    diagnostics: Default::default(),
                    tsconfig: Default::default(),
                };
                node_js_package.insert_serialized_tsconfig(&manifest.tree());

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let mut node_js_package = NodeJsPackage::default();
                node_js_package.insert_serialized_tsconfig(&manifest.tree());

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    /// Removes a `tsconfig.json` manifest from the package with the given
    /// `path`.
    pub fn remove_tsconfig_from_package(&self, path: &Utf8Path) {
        self.0.pin().update(path.to_path_buf(), |data| PackageData {
            node_package: data
                .node_package
                .as_ref()
                .map(NodeJsPackage::without_tsconfig),
        });
    }

    /// Removes a package and its metadata from the project layout.
    pub fn remove_package(&self, path: &Utf8Path) {
        self.0.pin().remove(path);
    }
}
