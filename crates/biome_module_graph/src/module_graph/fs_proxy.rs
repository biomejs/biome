use crate::path_info_cache::PathInfoCache;
use biome_package::{PackageJson, TsConfigJson};
use biome_project_layout::ProjectLayout;
use biome_resolver::{FsWithResolverProxy, PathInfo, ResolveError, ResolverFsProxy};
use camino::{Utf8Path, Utf8PathBuf};

pub(crate) struct ModuleGraphFsProxy<'a> {
    fs: &'a dyn FsWithResolverProxy,
    project_layout: &'a ProjectLayout,
    path_info_cache: &'a PathInfoCache,
}

impl<'a> ModuleGraphFsProxy<'a> {
    pub fn new(
        fs: &'a dyn FsWithResolverProxy,
        path_info_cache: &'a PathInfoCache,
        project_layout: &'a ProjectLayout,
    ) -> Self {
        Self {
            fs,
            project_layout,
            path_info_cache,
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
        self.path_info_cache
            .get_or_insert_with_fs(path, self.fs)
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
