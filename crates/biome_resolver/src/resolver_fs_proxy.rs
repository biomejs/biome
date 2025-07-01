use biome_fs::{FileSystem, FsErrorKind, PathKind, expand_symbolic_link};
use biome_package::{Manifest, PackageJson, TsConfigJson};
use camino::{Utf8Path, Utf8PathBuf};

use crate::errors::ResolveError;

/// Represents the kind of filesystem entry a path points at.
#[derive(Clone, Debug)]
pub enum PathInfo {
    Directory,
    File,
    Symlink {
        /// The canonicalized target of the symlink.
        ///
        /// Accessing this target is guaranteed to not return another symlink.
        canonicalized_target: Utf8PathBuf,
    },
}

impl PathInfo {
    pub const fn is_symlink(&self) -> bool {
        matches!(self, Self::Symlink { .. })
    }
}

/// A proxy for handling requests to the filesystem.
///
/// The proxy only implements a very specific subset of FS functionality in such
/// a way that we can utilise our module graph as a FS proxy instead of always
/// performing ad-hoc FS I/O.
pub trait ResolverFsProxy {
    /// Finds the `package.json` in `search_dir` or one of its parents.
    ///
    /// Returns both the parsed `PackageJson` structure as well as the path of
    /// the package in which it was found.
    fn find_package_json(
        &self,
        search_dir: &Utf8Path,
    ) -> Result<(Utf8PathBuf, PackageJson), ResolveError>;

    /// Returns information about the path.
    ///
    /// This method does not follow symlinks.
    ///
    /// Errors if the path doesn't exist , isn't accessible, or if the path
    /// points to something that is not a file, directory, or symlink.
    fn path_info(&self, path: &Utf8Path) -> Result<PathInfo, ResolveError>;

    /// Reads the `package.json` manifest that is expected to exist in the
    /// directory with the given path.
    fn read_package_json_in_directory(
        &self,
        dir_path: &Utf8Path,
    ) -> Result<PackageJson, ResolveError>;

    /// Reads the `tsconfig.json` manifest at the given path.
    fn read_tsconfig_json(&self, path: &Utf8Path) -> Result<TsConfigJson, ResolveError>;
}

pub trait FsWithResolverProxy: FileSystem + ResolverFsProxy {}

impl<Fs: FileSystem> FsWithResolverProxy for Fs {}

impl<Fs: FileSystem> ResolverFsProxy for Fs {
    fn find_package_json(
        &self,
        search_dir: &Utf8Path,
    ) -> Result<(Utf8PathBuf, PackageJson), ResolveError> {
        self.auto_search_files(search_dir, &["package.json"])
            .ok_or(ResolveError::NotFound)
            .and_then(|result| {
                self.read_package_json_in_directory(&result.directory_path)
                    .map(|manifest| (result.directory_path, manifest))
            })
    }

    fn path_info(&self, path: &Utf8Path) -> Result<PathInfo, ResolveError> {
        match self.symlink_path_kind(path) {
            Ok(PathKind::Directory { .. }) => Ok(PathInfo::Directory),
            Ok(PathKind::File { is_symlink }) if is_symlink => match expand_symbolic_link(path) {
                Ok((normalized_target, _)) => Ok(PathInfo::Symlink {
                    canonicalized_target: normalized_target,
                }),
                Err(_error) => Err(ResolveError::BrokenSymlink),
            },
            Ok(PathKind::File { .. }) => Ok(PathInfo::File),
            Err(error) => match error.error_kind {
                FsErrorKind::DereferencedSymlink | FsErrorKind::DeeplyNestedSymlinkExpansion => {
                    Err(ResolveError::BrokenSymlink)
                }
                _ => Err(ResolveError::NotFound),
            },
        }
    }

    fn read_package_json_in_directory(
        &self,
        dir_path: &Utf8Path,
    ) -> Result<PackageJson, ResolveError> {
        match PackageJson::read_manifest(self, &dir_path.join("package.json")).consume() {
            (Some(manifest), _errors) => Ok(manifest),
            _ => Err(ResolveError::ErrorLoadingManifest),
        }
    }

    fn read_tsconfig_json(&self, path: &Utf8Path) -> Result<TsConfigJson, ResolveError> {
        match TsConfigJson::read_manifest(self, path).consume() {
            (Some(manifest), _errors) => Ok(manifest),
            _ => Err(ResolveError::ErrorLoadingManifest),
        }
    }
}
