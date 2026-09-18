use biome_fs::BiomePath;
use biome_resolver::{FsWithResolverProxy, PathInfo, ResolverFsProxy};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;

#[derive(Debug, Default)]
pub struct PathInfoCache {
    cache: HashMap<Utf8PathBuf, Option<PathInfo>>,
}

impl PathInfoCache {
    pub fn get_or_insert(&self, path: &Utf8Path, fs: &dyn ResolverFsProxy) -> Option<PathInfo> {
        self.cache
            .pin()
            .get_or_insert_with(path.to_path_buf(), || fs.path_info(path).ok())
            .clone()
    }

    pub fn remove(&self, path: &Utf8Path) {
        self.cache.pin().remove(path);
    }

    pub fn get_or_insert_with_fs(
        &self,
        path: &Utf8Path,
        fs: &dyn FsWithResolverProxy,
    ) -> Option<PathInfo> {
        self.cache
            .pin()
            .get_or_insert_with(path.to_path_buf(), || fs.path_info(path).ok())
            .clone()
    }

    pub fn clear_prefix(&self, prefix: &Utf8Path) {
        let pinned = self.cache.pin();
        for key in pinned.keys() {
            if key.starts_with(prefix) {
                pinned.remove(key);
            }
        }
    }

    pub fn prepopulate_directory_path_info(
        &self,
        fs: &dyn FsWithResolverProxy,
        paths: &[&BiomePath],
    ) {
        for path in paths {
            let mut parent = path.parent();
            while let Some(dir) = parent {
                if self.cache.pin().get(dir).is_some() {
                    break; // already cached from a previous walk
                }
                self.get_or_insert(dir, fs);
                parent = dir.parent();
            }
        }
    }
}
