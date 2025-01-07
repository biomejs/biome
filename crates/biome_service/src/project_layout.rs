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
/// most relevant for a given file, we may need to do a dumb iteration over all
/// entries to find which is the closest match. This means performance becomes
/// O(N) with the number of open packages, so if this becomes a bottleneck, we
/// may want to reconsider this approach. For now though, it makes sense because
/// it makes it very easy for us to invalidate part of the layout when there are
/// file system changes.
#[derive(Debug, Default)]
pub struct ProjectLayout(HashMap<Utf8PathBuf, PackageData, FxBuildHasher>);

/// The information tracked for each package.
///
/// Because Biome is intended to support multiple kinds of JavaScript projects,
/// the term "package" is somewhat loosely defined. It may be an NPM package,
/// a JSR package, or simply a directory with its own nested `biome.json`.
#[derive(Debug, Default)]
pub struct PackageData {
    /// The settings of the package.
    ///
    /// Usually inferred from a configuration file, e.g. `biome.json`.
    // TODO: Uncomment this.
    // Probably best done when Ema has finished with https://github.com/biomejs/biome/pull/4845
    //settings: Settings,

    /// Optional Node.js-specific package information, if relevant for the
    /// package.
    node_package: Option<NodeJsPackage>,
}

impl ProjectLayout {
    pub fn get_node_manifest_for_path(&self, path: &Utf8Path) -> Option<PackageJson> {
        self.0
            .pin()
            .iter()
            .fold(
                None::<(&Utf8PathBuf, PackageJson)>,
                |result, (package_path, data)| {
                    let node_manifest = data
                        .node_package
                        .as_ref()
                        .map(|node_package| &node_package.manifest)?;
                    if path.strip_prefix(package_path).is_err() {
                        return None;
                    }

                    result
                        .is_none_or(|(matched_package_path, _)| {
                            package_path.as_str().len() > matched_package_path.as_str().len()
                        })
                        .then(|| (package_path, node_manifest.clone()))
                },
            )
            .map(|(_, package_json)| package_json)
    }

    pub fn insert_node_manifest(&self, path: Utf8PathBuf, manifest: AnyParse) {
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
                node_js_package.deserialize_manifest(&manifest.tree());

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
            || {
                let mut node_js_package = NodeJsPackage::default();
                node_js_package.deserialize_manifest(&manifest.tree());

                PackageData {
                    node_package: Some(node_js_package),
                }
            },
        );
    }

    pub fn remove_package(&self, path: &Utf8Path) {
        self.0.pin().remove(path);
    }
}
