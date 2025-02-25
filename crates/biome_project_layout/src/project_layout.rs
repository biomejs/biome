use biome_package::{NodeJsPackage, Package, PackageJson};
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
    // The settings of the package.
    //
    // Usually inferred from a configuration file, e.g. `biome.json`.
    // TODO: Uncomment this.
    // Probably best done when Ema has finished with https://github.com/biomejs/biome/pull/4845
    // settings: Settings,
    /// Optional Node.js-specific package information, if relevant for the
    /// package.
    node_package: Option<NodeJsPackage>,
}

impl ProjectLayout {
    /// Returns the `package.json` that should be used for the given `path`,
    /// together with the absolute path of the manifest file.
    pub fn get_node_manifest_for_path(
        &self,
        path: &Utf8Path,
    ) -> Option<(Utf8PathBuf, PackageJson)> {
        // Note I also tried an alternative approach where instead of iterating
        // over all entries and finding the closest match, I would do repeated
        // lookups like this:
        //
        // ```rs
        // let packages = self.0.pin();
        // path.ancestors().skip(1).find_map(|package_path| {
        //     packages
        //         .get(package_path)
        //         .and_then(|data| data.node_package.as_ref())
        //         .map(|node_package| (package_path.to_path_buf(), node_package.manifest.clone()))
        // })
        // ```
        //
        // Contrary to what I expected however, the below implementation
        // appeared significantly faster (tested on the `unleash` repository).

        let mut result: Option<(&Utf8PathBuf, &PackageJson)> = None;

        let packages = self.0.pin();
        for (package_path, data) in packages.iter() {
            let Some(node_manifest) = data
                .node_package
                .as_ref()
                .and_then(|node_package| node_package.manifest.as_ref())
            else {
                continue;
            };

            let is_closest_match = path.strip_prefix(package_path).is_ok()
                && result.is_none_or(|(matched_package_path, _)| {
                    package_path.as_str().len() > matched_package_path.as_str().len()
                });

            if is_closest_match {
                result = Some((package_path, node_manifest));
            }
        }

        result.map(|(package_path, package_json)| {
            (package_path.join("package.json"), package_json.clone())
        })
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

    /// Removes a package and its metadata from the project layout.
    pub fn remove_package(&self, path: &Utf8Path) {
        self.0.pin().remove(path);
    }
}
