use std::{
    borrow::Cow,
    cell::RefCell,
    collections::BTreeSet,
    hash::{BuildHasherDefault, Hash, Hasher},
    io,
    ops::Deref,
    path::{Component, Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use biome_fs::{BiomePath, FileSystem, PathKind};
use biome_package::{PackageJson, TsConfigJson};
use biome_project_layout::ProjectLayout;
use camino::{FromPathBufError, Utf8Component, Utf8Path, Utf8PathBuf};
use once_cell::sync::OnceCell as OnceLock;
use oxc_resolver::{
    context::ResolveContext as Ctx, Cache, CachedPath as _, ResolveError, ResolveOptions, TsConfig,
};
use papaya::{Equivalent, HashMap, HashSet};
use rustc_hash::FxHasher;

use crate::DependencyGraph;

static THREAD_COUNT: AtomicU64 = AtomicU64::new(1);

thread_local! {
    /// Per-thread pre-allocated path that is used to perform operations on paths more quickly.
    /// Learned from parcel <https://github.com/parcel-bundler/parcel/blob/a53f8f3ba1025c7ea8653e9719e0a61ef9717079/crates/parcel-resolver/src/cache.rs#L394>
    pub static SCRATCH_PATH: RefCell<PathBuf> = RefCell::new(PathBuf::with_capacity(256));
    pub static THREAD_ID: u64 = THREAD_COUNT.fetch_add(1, Ordering::SeqCst);
}

/// Cache to be used while resolving dependencies.
///
/// This cache has a few unique requirements:
/// * During resolving, we may assume the project layout to be available, so we
///   can use it for resolving `package.json` and `tsconfig.json` files.
/// * Results will be inserted into the dependency graph, so we _cannot_ assume
///   the dependency graph to be complete when resolving regular files.
/// * However, we _do_ want to use previously built dependency graphs to be
///   as a source for resolving files when calculating _updates_ to the
///   dependency graph.
/// * Fortunately, for non-manifests, we don't actually care about the contents
///   of those files. This means that in addition to the project layout, we only
///   need to know which *paths* there are. So we can create an up-to-date cache
///   for resolving as long as we have a (possibly empty) previous dependency
///   graph and a list of paths to be added and/or removed.
pub(crate) struct ResolverCache<'a> {
    fs: &'a dyn FileSystem,
    project_layout: &'a ProjectLayout,
    dependency_graph: &'a DependencyGraph,
    added_paths: BTreeSet<&'a Utf8Path>,
    removed_paths: BTreeSet<&'a Utf8Path>,

    paths: HashSet<CachedPath, BuildHasherDefault<IdentityHasher>>,
    tsconfigs: HashMap<Utf8PathBuf, Arc<TsConfigJson>, BuildHasherDefault<FxHasher>>,
}

impl<'a> ResolverCache<'a> {
    pub fn new(
        fs: &'a dyn FileSystem,
        project_layout: &'a ProjectLayout,
        dependency_graph: &'a DependencyGraph,
        added_paths: &'a [BiomePath],
        removed_paths: &'a [BiomePath],
    ) -> Self {
        Self {
            fs,
            project_layout,
            dependency_graph,
            added_paths: added_paths.iter().map(|path| path.as_path()).collect(),
            removed_paths: removed_paths.iter().map(|path| path.as_path()).collect(),

            paths: HashSet::builder()
                .hasher(BuildHasherDefault::default())
                .resize_mode(papaya::ResizeMode::Blocking)
                .collector(seize::Collector::new().epoch_frequency(None))
                .build(),
            tsconfigs: HashMap::builder()
                .hasher(BuildHasherDefault::default())
                .resize_mode(papaya::ResizeMode::Blocking)
                .collector(seize::Collector::new().epoch_frequency(None))
                .build(),
        }
    }

    pub fn paths(&self) -> &HashSet<CachedPath, BuildHasherDefault<IdentityHasher>> {
        &self.paths
    }

    fn path_kind(&self, path: &Utf8Path) -> Option<PathKind> {
        if self.added_paths.contains(path) {
            self.fs.path_kind(path).ok()
        } else {
            self.dependency_graph.path_kind(path)
        }
    }

    /// Returns the canonical path, resolving all symbolic links.
    ///
    /// <https://github.com/parcel-bundler/parcel/blob/4d27ec8b8bd1792f536811fef86e74a31fa0e704/crates/parcel-resolver/src/cache.rs#L232>
    fn canonicalize_impl(&self, path: &CachedPath) -> Result<CachedPath, ResolveError> {
        // Check if this thread is already canonicalizing. If so, we have found a circular symlink.
        // If a different thread is canonicalizing, OnceLock will queue this thread to wait for the result.
        let tid = THREAD_ID.with(|t| *t);
        if path.canonicalizing.load(Ordering::Acquire) == tid {
            return Err(io::Error::new(io::ErrorKind::NotFound, "Circular symlink").into());
        }

        path.canonicalized
            .get_or_init(|| {
                path.canonicalizing.store(tid, Ordering::Release);

                let res = path.parent().map_or_else(
                    || Ok(path.normalize_root(self)),
                    |parent| {
                        self.canonicalize_impl(parent).and_then(|parent_canonical| {
                            let normalized = parent_canonical.normalize_with(
                                path.path().strip_prefix(parent.path()).unwrap(),
                                self,
                            );

                            let utf8_path = Utf8PathBuf::try_from(path.path().to_path_buf())
                                .map_err(FromPathBufError::into_io_error)?;
                            if self.fs.path_is_symlink(&utf8_path) {
                                let link = self.fs.read_link(&normalized.path)?;
                                if link.is_absolute() {
                                    return self.canonicalize_impl(
                                        &self.value(normalize(&link).as_std_path()),
                                    );
                                } else if let Some(dir) = normalized.parent() {
                                    // Symlink is relative `../../foo.js`, use the path directory
                                    // to resolve this symlink.
                                    return self
                                        .canonicalize_impl(&dir.normalize_with(&link, self));
                                }
                                debug_assert!(
                                    false,
                                    "Failed to get path parent for {:?}.",
                                    normalized.path()
                                );
                            }

                            Ok(normalized)
                        })
                    },
                );

                path.canonicalizing.store(0, Ordering::Release);
                res
            })
            .clone()
    }
}

impl Cache for ResolverCache<'_> {
    type Cp = CachedPath;
    type Pj = PackageJson;
    type Tc = TsConfigJson;

    fn canonicalize(&self, path: &Self::Cp) -> Result<PathBuf, ResolveError> {
        let cached_path = self.canonicalize_impl(path)?;
        let path = cached_path.to_path_buf();
        cfg_if::cfg_if! {
            if #[cfg(windows)] {
                let path = strip_windows_prefix(path);
            }
        }
        Ok(path)
    }

    fn clear(&self) {
        self.paths.pin().clear();
        self.tsconfigs.pin().clear();
    }

    fn get_package_json(
        &self,
        path: &Self::Cp,
        options: &ResolveOptions,
        ctx: &mut Ctx,
    ) -> Result<Option<(Self::Cp, Arc<PackageJson>)>, ResolveError> {
        // TODO: Change to `std::sync::OnceLock::get_or_try_init` when it is stable.
        //       See: https://github.com/rust-lang/rust/issues/109737
        let result = path
            .package_json
            .get_or_try_init(|| {
                let utf8_path = path.path().try_into().map_err(|_| {
                    ResolveError::NotFound(path.path().to_string_lossy().to_string())
                })?;
                let Some((package_json_path, mut package_json)) =
                    self.project_layout.get_node_manifest_for_path(utf8_path)
                else {
                    return Ok(None);
                };
                package_json.canonicalized_path = if options.symlinks {
                    self.canonicalize(&self.value(&path.path().join("package.json")))?
                        .try_into()
                        .map_err(FromPathBufError::into_io_error)?
                } else {
                    package_json_path
                };

                Ok(Some((path.clone(), Arc::new(package_json))))
            })
            .cloned();

        // https://github.com/webpack/enhanced-resolve/blob/58464fc7cb56673c9aa849e68e6300239601e615/lib/DescriptionFileUtils.js#L68-L82
        match &result {
            Ok(Some((_, package_json))) => {
                ctx.add_file_dependency(package_json.path.as_std_path());
            }
            Ok(None) => {
                // Avoid an allocation by making this lazy
                if let Some(deps) = &mut ctx.missing_dependencies {
                    deps.push(path.path().join("package.json"));
                }
            }
            Err(_) => {
                if let Some(deps) = &mut ctx.file_dependencies {
                    deps.push(path.path().join("package.json"));
                }
            }
        }

        result
    }

    fn get_tsconfig<F: FnOnce(&mut TsConfigJson) -> Result<(), ResolveError>>(
        &self,
        root: bool,
        path: &Path,
        callback: F,
    ) -> Result<Arc<TsConfigJson>, ResolveError> {
        let path: &Utf8Path = path
            .try_into()
            .map_err(|_| ResolveError::NotFound(path.to_string_lossy().to_string()))?;

        let tsconfigs = self.tsconfigs.pin();
        if let Some(tsconfig) = tsconfigs.get(path) {
            return Ok(Arc::clone(tsconfig));
        }

        let kind = self.path_kind(path);
        let tsconfig_path = if kind.is_some_and(PathKind::is_file) {
            Cow::Borrowed(path)
        } else if kind.is_some_and(PathKind::is_dir) {
            Cow::Owned(path.join("tsconfig.json"))
        } else {
            Cow::Owned(Utf8PathBuf::from(format!("{path}.json")))
        };

        let mut tsconfig_string = self
            .fs
            .read_file_from_path(path)
            .map_err(|_| ResolveError::TsconfigNotFound(path.as_std_path().to_path_buf()))?;
        // FIXME: We should load this from the ProjectLayout instead.
        let (mut tsconfig, diagnostics) =
            TsConfigJson::parse(root, &tsconfig_path, &mut tsconfig_string);
        if !diagnostics.is_empty() {
            // FIXME: Report diagnostics
        }

        callback(&mut tsconfig)?;

        tsconfig.expand_template_variables();
        let tsconfig = Arc::new(tsconfig);
        tsconfigs.insert(path.to_path_buf(), Arc::clone(&tsconfig));
        Ok(tsconfig)
    }

    fn is_dir(&self, path: &Self::Cp, ctx: &mut Ctx) -> bool {
        if let Some(meta) = &path.kind {
            meta.is_dir()
        } else {
            ctx.add_missing_dependency(path.path());
            false
        }
    }

    fn is_file(&self, path: &Self::Cp, ctx: &mut Ctx) -> bool {
        if let Some(meta) = &path.kind {
            ctx.add_file_dependency(path.path());
            meta.is_file()
        } else {
            ctx.add_missing_dependency(path.path());
            false
        }
    }

    fn value(&self, path: &Path) -> Self::Cp {
        // `Path::hash` is slow: https://doc.rust-lang.org/std/path/struct.Path.html#impl-Hash-for-Path
        // `path.as_os_str()` hash is not stable because we may joined a path like `foo/bar` and `foo\\bar` on windows.
        let hash = {
            let mut hasher = FxHasher::default();
            path.as_os_str().hash(&mut hasher);
            hasher.finish()
        };

        let paths = self.paths.pin();
        let path = Utf8Path::from_path(path).expect("Non UTF-8 characters in path");
        let key = BorrowedCachedPath { hash, path };
        if let Some(cached_path) = paths.get(&key) {
            return if self.removed_paths.contains(path) {
                CachedPath(Arc::new(CachedPathImpl::new(
                    hash,
                    cached_path.path.clone(),
                    cached_path.parent.clone(),
                    None,
                )))
            } else {
                cached_path.clone()
            };
        }

        let parent = path.parent().map(|p| self.value(p.as_std_path()));
        let cached_path = CachedPath(Arc::new(CachedPathImpl::new(
            hash,
            path.to_path_buf().into_boxed_path(),
            parent,
            self.path_kind(path),
        )));
        paths.insert(cached_path.clone());
        cached_path
    }
}

#[derive(Clone)]
pub(crate) struct CachedPath(Arc<CachedPathImpl>);

pub(crate) struct CachedPathImpl {
    hash: u64,
    pub(crate) path: Box<Utf8Path>,
    parent: Option<CachedPath>,
    kind: Option<PathKind>,
    canonicalized: OnceLock<Result<CachedPath, ResolveError>>,
    canonicalizing: AtomicU64,
    node_modules: OnceLock<Option<CachedPath>>,
    package_json: OnceLock<Option<(CachedPath, Arc<PackageJson>)>>,
}

impl CachedPathImpl {
    const fn new(
        hash: u64,
        path: Box<Utf8Path>,
        parent: Option<CachedPath>,
        meta: Option<PathKind>,
    ) -> Self {
        Self {
            hash,
            path,
            parent,
            kind: meta,
            canonicalized: OnceLock::new(),
            canonicalizing: AtomicU64::new(0),
            node_modules: OnceLock::new(),
            package_json: OnceLock::new(),
        }
    }

    pub fn kind(&self) -> Option<PathKind> {
        self.kind
    }
}

impl Deref for CachedPath {
    type Target = CachedPathImpl;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl oxc_resolver::CachedPath for CachedPath {
    fn path(&self) -> &Path {
        self.0.path.as_std_path()
    }

    fn to_path_buf(&self) -> PathBuf {
        self.path().to_path_buf()
    }

    fn parent(&self) -> Option<&Self> {
        self.0.parent.as_ref()
    }

    fn module_directory<C: Cache<Cp = Self>>(
        &self,
        module_name: &str,
        cache: &C,
        ctx: &mut Ctx,
    ) -> Option<Self> {
        let cached_path = cache.value(&self.path().join(module_name));
        cache.is_dir(&cached_path, ctx).then_some(cached_path)
    }

    fn cached_node_modules<C: Cache<Cp = Self>>(&self, cache: &C, ctx: &mut Ctx) -> Option<Self> {
        self.node_modules
            .get_or_init(|| self.module_directory("node_modules", cache, ctx))
            .clone()
    }

    /// Find package.json of a path by traversing parent directories.
    ///
    /// # Errors
    ///
    /// * [ResolveError::JSON]
    fn find_package_json<C: Cache<Cp = Self>>(
        &self,
        options: &ResolveOptions,
        cache: &C,
        ctx: &mut Ctx,
    ) -> Result<Option<(Self, Arc<C::Pj>)>, ResolveError> {
        let mut cache_value = self;
        // Go up directories when the querying path is not a directory
        while !cache.is_dir(cache_value, ctx) {
            if let Some(cv) = &cache_value.parent {
                cache_value = cv;
            } else {
                break;
            }
        }
        let mut cache_value = Some(cache_value);
        while let Some(cv) = cache_value {
            if let Some(package_json) = cache.get_package_json(cv, options, ctx)? {
                return Ok(Some(package_json));
            }
            cache_value = cv.parent.as_ref();
        }
        Ok(None)
    }

    fn add_extension<C: Cache<Cp = Self>>(&self, ext: &str, cache: &C) -> Self {
        SCRATCH_PATH.with_borrow_mut(|path| {
            path.clear();
            let s = path.as_mut_os_string();
            s.push(self.path.as_os_str());
            s.push(ext);
            cache.value(path)
        })
    }

    fn replace_extension<C: Cache<Cp = Self>>(&self, ext: &str, cache: &C) -> Self {
        SCRATCH_PATH.with_borrow_mut(|path| {
            path.clear();
            let s = path.as_mut_os_string();
            let self_len = self.path.as_os_str().len();
            let self_bytes = self.path.as_os_str().as_encoded_bytes();
            let slice_to_copy = self
                .path
                .extension()
                .map_or(self_bytes, |previous_extension| {
                    &self_bytes[..self_len - previous_extension.len() - 1]
                });
            // SAFETY: ???
            s.push(unsafe { std::ffi::OsStr::from_encoded_bytes_unchecked(slice_to_copy) });
            s.push(ext);
            cache.value(path)
        })
    }

    /// Returns a new path by resolving the given subpath (including "." and ".." components) with this path.
    fn normalize_with<C: Cache<Cp = Self>>(&self, subpath: impl AsRef<Path>, cache: &C) -> Self {
        let subpath = subpath.as_ref();
        let mut components = subpath.components();
        let Some(head) = components.next() else {
            return cache.value(subpath);
        };
        if matches!(head, Component::Prefix(..) | Component::RootDir) {
            return cache.value(subpath);
        }
        SCRATCH_PATH.with_borrow_mut(|path| {
            path.clear();
            path.push(self.path());
            for component in std::iter::once(head).chain(components) {
                match component {
                    Component::CurDir => {}
                    Component::ParentDir => {
                        path.pop();
                    }
                    Component::Normal(c) => {
                        cfg_if::cfg_if! {
                            if #[cfg(target_family = "wasm")] {
                                // Need to trim the extra \0 introduces by https://github.com/nodejs/uvwasi/issues/262
                                path.push(c.to_string_lossy().trim_end_matches('\0'));
                            } else {
                                path.push(c);
                            }
                        }
                    }
                    Component::Prefix(..) | Component::RootDir => {
                        unreachable!("Path {:?} Subpath {:?}", self.path, subpath)
                    }
                }
            }

            cache.value(path)
        })
    }

    #[inline]
    #[cfg(windows)]
    fn normalize_root<C: Cache<Cp = Self>>(&self, cache: &C) -> Self {
        if self.path().as_os_str().as_encoded_bytes().last() == Some(&b'/') {
            let mut path_string = self.path.to_string();
            path_string.pop();
            path_string.push('\\');
            cache.value(&PathBuf::from(path_string))
        } else {
            self.clone()
        }
    }

    #[inline]
    #[cfg(not(windows))]
    fn normalize_root<C: Cache<Cp = Self>>(&self, _cache: &C) -> Self {
        self.clone()
    }
}

impl Hash for CachedPath {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for CachedPath {
    fn eq(&self, other: &Self) -> bool {
        self.path.as_os_str() == other.path.as_os_str()
    }
}

impl Eq for CachedPath {}

struct BorrowedCachedPath<'a> {
    hash: u64,
    path: &'a Utf8Path,
}

impl Equivalent<CachedPath> for BorrowedCachedPath<'_> {
    fn equivalent(&self, other: &CachedPath) -> bool {
        self.path.as_os_str() == other.path.as_os_str()
    }
}

impl Hash for BorrowedCachedPath<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}

impl PartialEq for BorrowedCachedPath<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.path.as_os_str() == other.path.as_os_str()
    }
}

/// Since the cache key is memoized, use an identity hasher
/// to avoid double cache.
#[derive(Default)]
pub(crate) struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("Invalid use of IdentityHasher")
    }

    fn write_u64(&mut self, n: u64) {
        self.0 = n;
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

// https://github.com/parcel-bundler/parcel/blob/e0b99c2a42e9109a9ecbd6f537844a1b33e7faf5/packages/utils/node-resolver-rs/src/path.rs#L7
fn normalize(path: &Utf8Path) -> Utf8PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Utf8Component::Prefix(..)) = components.peek() {
        let buf = Utf8PathBuf::from(c.as_str());
        components.next();
        buf
    } else {
        Utf8PathBuf::new()
    };

    for component in components {
        match component {
            Utf8Component::Prefix(..) => unreachable!("Path {path:?}"),
            Utf8Component::RootDir => {
                ret.push(component.as_str());
            }
            Utf8Component::CurDir => {}
            Utf8Component::ParentDir => {
                ret.pop();
            }
            Utf8Component::Normal(c) => {
                ret.push(c);
            }
        }
    }

    ret
}

// https://github.com/oxc-project/oxc-resolver/blob/2bc5173e30adb9c30ed9aa98c083bb758ffb8d90/src/file_system.rs#L168
#[cfg(windows)]
fn strip_windows_prefix<P: AsRef<Path>>(path: P) -> PathBuf {
    const UNC_PATH_PREFIX: &[u8] = b"\\\\?\\UNC\\";
    const LONG_PATH_PREFIX: &[u8] = b"\\\\?\\";
    let path_bytes = path.as_ref().as_os_str().as_encoded_bytes();
    path_bytes
        .strip_prefix(UNC_PATH_PREFIX)
        .or_else(|| path_bytes.strip_prefix(LONG_PATH_PREFIX))
        .map_or_else(
            || path.as_ref().to_path_buf(),
            |p| {
                // SAFETY: `as_encoded_bytes` ensures `p` is valid path bytes
                unsafe { PathBuf::from(std::ffi::OsStr::from_encoded_bytes_unchecked(p)) }
            },
        )
}
