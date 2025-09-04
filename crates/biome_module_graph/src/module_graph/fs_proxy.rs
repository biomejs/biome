use biome_package::{PackageJson, TsConfigJson};
use biome_project_layout::ProjectLayout;
use biome_resolver::{FsWithResolverProxy, PathInfo, ResolveError, ResolverFsProxy};
use camino::{Utf8Path, Utf8PathBuf};

use super::ModuleGraph;

pub(crate) struct ModuleGraphFsProxy<'a> {
    fs: &'a dyn FsWithResolverProxy,
    module_graph: &'a ModuleGraph,
    project_layout: &'a ProjectLayout,
}

impl<'a> ModuleGraphFsProxy<'a> {
    pub fn new(
        fs: &'a dyn FsWithResolverProxy,
        module_graph: &'a ModuleGraph,
        project_layout: &'a ProjectLayout,
    ) -> Self {
        Self {
            fs,
            module_graph,
            project_layout,
        }
    }
}

impl ResolverFsProxy for ModuleGraphFsProxy<'_> {
    fn find_package_json(
        &self,
        search_dir: &Utf8Path,
    ) -> Result<(Utf8PathBuf, PackageJson), ResolveError> {
        self.project_layout
            .find_node_manifest_for_path(search_dir)
            .ok_or(ResolveError::ManifestNotFound)
    }

    fn path_info(&self, path: &Utf8Path) -> Result<PathInfo, ResolveError> {
        self.module_graph
            .get_or_insert_path_info(path, self.fs)
            .ok_or(ResolveError::NotFound)
    }

    fn read_package_json_in_directory(
        &self,
        dir_path: &Utf8Path,
    ) -> Result<PackageJson, ResolveError> {
        self.project_layout
            .get_node_manifest_for_package(dir_path)
            .ok_or(ResolveError::ErrorLoadingManifest)
    }

    fn read_tsconfig_json(&self, path: &Utf8Path) -> Result<TsConfigJson, ResolveError> {
        self.project_layout
            .get_tsconfig_json_for_package(path.parent().expect("path should have a parent"))
            .ok_or(ResolveError::ErrorLoadingManifest)
    }
}
