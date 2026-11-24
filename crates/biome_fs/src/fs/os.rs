//! Implementation of the [FileSystem] and related traits for the underlying OS filesystem

use std::env::temp_dir;
use std::fs::{FileType, Metadata};
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    env, fs,
    io::{self, Read, Seek, Write},
};

use biome_diagnostics::{DiagnosticExt, Error, IoError, Severity, category};
use camino::{Utf8DirEntry, Utf8Path, Utf8PathBuf};
use rayon::{Scope, scope};
use tracing::instrument;

use crate::fs::OpenOptions;
use crate::utils::expand_symbolic_link_with_diagnostic_path;
use crate::{
    BiomePath, FileSystem, MemoryFileSystem,
    fs::{TraversalContext, TraversalScope},
};

use super::{BoxedTraversal, File, FileSystemDiagnostic, FsErrorKind, PathKind};

/// Implementation of [FileSystem] that directly calls through to the underlying OS
pub struct OsFileSystem {
    pub working_directory: Option<Utf8PathBuf>,
}

impl OsFileSystem {
    pub fn new(mut working_directory: Utf8PathBuf) -> Self {
        if working_directory.as_str().is_empty()
            && let Ok(current_directory) = env::current_dir()
            && let Ok(current_directory) = Utf8PathBuf::from_path_buf(current_directory)
        {
            working_directory = current_directory;
        }
        Self {
            working_directory: Some(working_directory),
        }
    }
}

impl Default for OsFileSystem {
    fn default() -> Self {
        let working_directory = env::current_dir()
            .map(|p| Utf8PathBuf::from_path_buf(p).expect("To be a UTF-8 path"))
            .ok();
        Self { working_directory }
    }
}

impl FileSystem for OsFileSystem {
    fn open_with_options(
        &self,
        path: &Utf8Path,
        options: OpenOptions,
    ) -> io::Result<Box<dyn File>> {
        let mut fs_options = fs::File::options();
        Ok(Box::new(OsFile {
            inner: options.into_fs_options(&mut fs_options).open(path)?,
            version: 0,
        }))
    }

    fn traversal(&self, func: BoxedTraversal) {
        let working_directory = self
            .working_directory
            .as_deref()
            .unwrap_or_else(|| Utf8Path::new(""));
        scope(|scope| {
            let traversal_scope = OsTraversalScope {
                scope,
                working_directory,
                directory_aliases: Arc::default(),
            };
            func(&traversal_scope);
        })
    }

    fn working_directory(&self) -> Option<Utf8PathBuf> {
        self.working_directory.clone()
    }

    fn path_exists(&self, path: &Utf8Path) -> bool {
        path.exists()
    }

    fn path_is_file(&self, path: &Utf8Path) -> bool {
        path.is_file()
    }

    fn path_is_dir(&self, path: &Utf8Path) -> bool {
        path.is_dir()
    }

    fn path_is_symlink(&self, path: &Utf8Path) -> bool {
        path.is_symlink()
    }

    fn path_kind(&self, path: &Utf8Path) -> Result<PathKind, FileSystemDiagnostic> {
        path_kind_from_metadata(path.metadata(), path)
    }

    fn symlink_path_kind(&self, path: &Utf8Path) -> Result<PathKind, FileSystemDiagnostic> {
        path_kind_from_metadata(path.symlink_metadata(), path)
    }

    fn read_link(&self, path: &Utf8Path) -> io::Result<Utf8PathBuf> {
        path.read_link_utf8()
    }

    fn get_changed_files(&self, base: &str) -> io::Result<Vec<String>> {
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg("--relative")
            // A: added
            // C: copied
            // M: modified
            // R: renamed
            // Source: https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---diff-filterACDMRTUXB82308203
            .arg("--diff-filter=ACMR")
            .arg(format!("{base}...HEAD"))
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.to_string())
            .collect())
    }

    fn get_staged_files(&self) -> io::Result<Vec<String>> {
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg("--relative")
            .arg("--staged")
            // A: added
            // C: copied
            // M: modified
            // R: renamed
            // Source: https://git-scm.com/docs/git-diff#Documentation/git-diff.txt---diff-filterACDMRTUXB82308203
            .arg("--diff-filter=ACMR")
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|l| l.to_string())
            .collect())
    }
}

#[derive(Debug)]
struct OsFile {
    inner: fs::File,
    version: i32,
}

impl File for OsFile {
    fn read_to_string(&mut self, buffer: &mut String) -> io::Result<()> {
        // Reset the cursor to the starting position
        self.inner.rewind()?;
        // Read the file content
        self.inner.read_to_string(buffer)?;
        Ok(())
    }

    #[instrument(level = "debug")]
    fn set_content(&mut self, content: &[u8]) -> io::Result<()> {
        // Truncate the file
        self.inner.set_len(0)?;
        // Reset the cursor to the starting position
        self.inner.rewind()?;
        // Write the byte slice
        self.inner.write_all(content)?;
        // new version stored
        self.version += 1;
        Ok(())
    }

    fn file_version(&self) -> i32 {
        self.version
    }
}

struct OsTraversalScope<'scope, 'env> {
    /// The Rayon scope used to schedule traversal work.
    scope: &'env Scope<'scope>,
    /// The immutable anchor used for relative traversal input.
    working_directory: &'env Utf8Path,
    /// Bounds directory enumerations through symbolic-link aliases across this traversal.
    directory_aliases: Arc<DirectoryAliases>,
}

/// Counts how often each physical directory is enumerated through an alias route.
///
/// Each logical alias of a directory is a distinct path with its own ignore rules, so traversal
/// enumerates a physical directory once per alias. Acyclic symbolic-link graphs can reach one
/// directory through exponentially many aliases, so enumerations through aliases are bounded per
/// physical directory. Routes that aren't aliases are never bounded, which keeps their precedence
/// when physical files are deduplicated.
#[derive(Default)]
struct DirectoryAliases(papaya::HashMap<Utf8PathBuf, AtomicUsize>);

impl DirectoryAliases {
    /// The maximum number of alias routes enumerated for one physical directory.
    const MAX_ENUMERATIONS: usize = 8;

    /// Records an alias enumeration of `physical_path`, returning `false` past the bound.
    fn try_enumerate(&self, physical_path: &Utf8Path) -> bool {
        let enumerations = self.0.pin();
        let count = match enumerations.get(physical_path) {
            Some(count) => count,
            None => {
                enumerations.get_or_insert_with(physical_path.to_path_buf(), AtomicUsize::default)
            }
        };
        count.fetch_add(1, Ordering::Relaxed) < Self::MAX_ENUMERATIONS
    }
}

/// The route traversal followed to reach the entries of a directory.
#[derive(Clone, Default)]
struct Route {
    /// The logical directory, when its entries are read through a different physical path.
    logical_parent: Option<Utf8PathBuf>,
    /// The physical directories enclosing the entries, used to detect cycles.
    physical_ancestors: Vec<Utf8PathBuf>,
    /// Whether the route followed a symbolic link below the evaluated root.
    ///
    /// The evaluated root is spelled by the caller, so following it doesn't make the entries
    /// symbolic-link paths. This is the flag reported through [PathKind].
    followed_symlink: bool,
    /// Whether the logical path differs from the physical one, including through the root.
    ///
    /// Paths that aren't aliases take precedence when physical files are deduplicated.
    is_alias: bool,
}

impl<'scope> TraversalScope<'scope> for OsTraversalScope<'scope, '_> {
    fn evaluate(&self, ctx: &'scope dyn TraversalContext, path: Utf8PathBuf) {
        let path = if path.is_absolute() {
            crate::normalize_path(&path)
        } else {
            crate::normalize_path(&self.working_directory.join(path))
        };
        let file_type = match path.symlink_metadata() {
            Ok(meta) => meta.file_type(),
            Err(err) => {
                ctx.interner().intern_path(path.clone());
                ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                    path: path.to_string(),
                    severity: Severity::Error,
                    error_kind: FsErrorKind::CantReadFile,
                    source: Some(Error::from(
                        IoError::from(err).with_category(category!("fs/io")),
                    )),
                }));
                return;
            }
        };
        handle_any_file(
            self.scope,
            ctx,
            &self.directory_aliases,
            path,
            file_type,
            &Route::default(),
        );
    }

    fn handle(&self, context: &'scope dyn TraversalContext, path: BiomePath) {
        self.scope.spawn(move |_| {
            context.handle_path(path);
        });
    }
}

/// Traverse a single directory
fn handle_dir<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    directory_aliases: &Arc<DirectoryAliases>,
    path: &Utf8Path,
    route: Route,
) {
    let diagnostic_path = route.logical_parent.as_deref().unwrap_or(path);
    let iter = match path.read_dir_utf8() {
        Ok(iter) => iter,
        Err(err) => {
            ctx.push_diagnostic(
                IoError::from(err)
                    .with_category(category!("fs/io"))
                    .with_file_path(diagnostic_path.to_string()),
            );
            return;
        }
    };

    for entry in iter {
        match entry {
            Ok(entry) => handle_dir_entry(scope, ctx, directory_aliases, entry, &route),
            Err(err) => {
                ctx.push_diagnostic(
                    IoError::from(err)
                        .with_category(category!("fs/io"))
                        .with_file_path(diagnostic_path.to_string()),
                );
            }
        }
    }
}

/// Traverse a single directory entry, scheduling any file to execute the context
/// handler and sub-directories for subsequent traversal
fn handle_dir_entry<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    directory_aliases: &Arc<DirectoryAliases>,
    entry: Utf8DirEntry,
    route: &Route,
) {
    let path = entry.path();
    let diagnostic_path = route
        .logical_parent
        .as_deref()
        .and_then(|parent| path.file_name().map(|file_name| parent.join(file_name)))
        .unwrap_or_else(|| path.to_path_buf());
    let file_type = match entry.file_type() {
        Ok(file_type) => file_type,
        Err(err) => {
            ctx.push_diagnostic(
                IoError::from(err)
                    .with_category(category!("fs/io"))
                    .with_file_path(diagnostic_path.to_string()),
            );
            return;
        }
    };
    handle_any_file(
        scope,
        ctx,
        directory_aliases,
        path.to_path_buf(),
        file_type,
        route,
    );
}

fn canonicalize_physical_path(path: &Utf8Path) -> Utf8PathBuf {
    fs::canonicalize(path)
        .ok()
        .and_then(|path| Utf8PathBuf::from_path_buf(path).ok())
        .unwrap_or_else(|| path.to_path_buf())
}

#[cfg(any(windows, test))]
fn windows_paths_have_same_spelling(left: &Utf8Path, right: &Utf8Path) -> bool {
    fn without_verbatim_prefix(path: &str) -> (&str, bool) {
        let bytes = path.as_bytes();
        let is_separator = |byte| matches!(byte, b'/' | b'\\');
        if bytes.len() >= 2 && is_separator(bytes[0]) && is_separator(bytes[1]) {
            if bytes.len() >= 6 && bytes[2] == b'?' && is_separator(bytes[3]) && bytes[5] == b':' {
                return (&path[4..], false);
            }
            if bytes.len() >= 8
                && bytes[2] == b'?'
                && is_separator(bytes[3])
                && bytes[4..7].eq_ignore_ascii_case(b"UNC")
                && is_separator(bytes[7])
            {
                return (&path[8..], true);
            }
            return (&path[2..], true);
        }
        (path, false)
    }

    let (left, left_is_unc) = without_verbatim_prefix(left.as_str());
    let (right, right_is_unc) = without_verbatim_prefix(right.as_str());
    left_is_unc == right_is_unc
        && left
            .bytes()
            .map(|byte| {
                if byte == b'/' {
                    b'\\'
                } else {
                    byte.to_ascii_lowercase()
                }
            })
            .eq(right.bytes().map(|byte| {
                if byte == b'/' {
                    b'\\'
                } else {
                    byte.to_ascii_lowercase()
                }
            }))
}

fn physical_path_differs(path: &Utf8Path, physical_path: &Utf8Path) -> bool {
    #[cfg(windows)]
    {
        !windows_paths_have_same_spelling(path, physical_path)
    }
    #[cfg(not(windows))]
    {
        path != physical_path
    }
}

fn handle_any_file<'scope>(
    scope: &Scope<'scope>,
    ctx: &'scope dyn TraversalContext,
    directory_aliases: &Arc<DirectoryAliases>,
    mut path: Utf8PathBuf,
    mut file_type: FileType,
    route: &Route,
) {
    let is_root = route.physical_ancestors.is_empty();
    let mut physical_path = route
        .physical_ancestors
        .last()
        .and_then(|parent| path.file_name().map(|file_name| parent.join(file_name)))
        .unwrap_or_else(|| canonicalize_physical_path(&path));
    let mut logical_path = match &route.logical_parent {
        Some(parent) => match path.file_name() {
            Some(file_name) => Some(parent.join(file_name)),
            None => {
                ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
                    path: path.to_string(),
                    error_kind: FsErrorKind::UnknownFileType,
                    severity: Severity::Warning,
                    source: None,
                }));
                return;
            }
        },
        None => None,
    };
    let mut followed_symlink = route.followed_symlink;
    // A root operand can be spelled through symbolic links in its parent directories.
    let mut is_alias = route.is_alias || (is_root && physical_path_differs(&path, &physical_path));

    let mut logical_path_is_interned = false;
    if file_type.is_symlink() {
        followed_symlink |= !is_root;
        is_alias = true;
        let unresolved_path = logical_path.take().unwrap_or_else(|| path.clone());
        if !ctx.interner().intern_path(unresolved_path.clone()) {
            return;
        }
        logical_path_is_interned = true;

        let (target_path, target_file_type) =
            match expand_symbolic_link_with_diagnostic_path(&path, &unresolved_path) {
                Ok((target_path, target_file_type)) => (target_path, target_file_type),
                Err(error) => {
                    ctx.push_diagnostic(error);
                    return;
                }
            };
        physical_path = canonicalize_physical_path(&target_path);

        if target_file_type.is_dir() {
            let biome_path = BiomePath::new(&unresolved_path);
            let path_kind = PathKind::Directory {
                is_symlink: followed_symlink,
            };
            if !ctx.can_handle_with_kind(&biome_path, path_kind) {
                return;
            }
            if ctx.should_store_dirs() {
                ctx.store_path(biome_path);
            }
            if route.physical_ancestors.contains(&physical_path)
                || !directory_aliases.try_enumerate(&physical_path)
            {
                return;
            }
            let mut physical_ancestors = route.physical_ancestors.clone();
            physical_ancestors.push(physical_path);
            let route = Route {
                logical_parent: Some(unresolved_path),
                physical_ancestors,
                followed_symlink,
                is_alias,
            };
            let directory_aliases = Arc::clone(directory_aliases);
            scope.spawn(move |scope| {
                handle_dir(scope, ctx, &directory_aliases, &target_path, route);
            });
            return;
        }

        logical_path = Some(unresolved_path);
        path = target_path;
        file_type = target_file_type;
    }

    let path_kind = if file_type.is_dir() {
        PathKind::Directory {
            is_symlink: followed_symlink,
        }
    } else {
        PathKind::File {
            is_symlink: followed_symlink,
        }
    };
    let traversal_logical_path = logical_path.clone();
    let logical_path = logical_path.unwrap_or_else(|| path.clone());
    let biome_path = BiomePath::new(&logical_path);

    if !ctx.can_handle_with_kind(&biome_path, path_kind) {
        return;
    }
    if !logical_path_is_interned && !ctx.interner().intern_path(logical_path.clone()) {
        return;
    }

    if file_type.is_dir() {
        if ctx.should_store_dirs() {
            ctx.store_path(biome_path);
        }
        if route.physical_ancestors.contains(&physical_path)
            || (is_alias && !directory_aliases.try_enumerate(&physical_path))
        {
            return;
        }
        let mut physical_ancestors = route.physical_ancestors.clone();
        physical_ancestors.push(physical_path);
        let route = Route {
            logical_parent: traversal_logical_path,
            physical_ancestors,
            followed_symlink,
            is_alias,
        };
        let directory_aliases = Arc::clone(directory_aliases);
        scope.spawn(move |scope| {
            handle_dir(scope, ctx, &directory_aliases, &path, route);
        });
        return;
    }

    if file_type.is_file() {
        ctx.store_physical_path(physical_path, biome_path, is_alias);
        return;
    }

    ctx.push_diagnostic(Error::from(FileSystemDiagnostic {
        path: logical_path.to_string(),
        error_kind: FsErrorKind::from(file_type),
        severity: Severity::Warning,
        source: None,
    }));
}

fn path_kind_from_metadata(
    metadata: io::Result<Metadata>,
    path: &Utf8Path,
) -> Result<PathKind, FileSystemDiagnostic> {
    match metadata {
        Ok(metadata) => {
            let is_symlink = metadata.is_symlink();
            if metadata.is_dir() {
                Ok(PathKind::Directory { is_symlink })
            } else if metadata.is_file() || is_symlink {
                Ok(PathKind::File { is_symlink })
            } else {
                Err(FileSystemDiagnostic {
                    path: path.to_string(),
                    severity: Severity::Error,
                    error_kind: FsErrorKind::UnknownFileType,
                    source: None,
                })
            }
        }
        Err(error) => Err(FileSystemDiagnostic {
            path: path.to_string(),
            severity: Severity::Error,
            error_kind: FsErrorKind::CantReadFile,
            source: Some(Error::from(
                IoError::from(error).with_category(category!("fs/io")),
            )),
        }),
    }
}

impl From<FileType> for FsErrorKind {
    fn from(_: FileType) -> Self {
        Self::UnknownFileType
    }
}

/// Testing utility that creates a working directory inside the
/// temporary OS folder.
pub struct TemporaryFs {
    /// The current working directory. It's the OS temporary folder joined with a file
    /// name passed in the [TemporaryFs::new] function
    pub working_directory: Utf8PathBuf,
    files: Vec<(Utf8PathBuf, String)>,
    project_directory: Utf8PathBuf,
}

impl TemporaryFs {
    /// Creates a temporary directory named using `directory_name`
    pub fn new(directory_name: &str) -> Self {
        let path = temp_dir().join(directory_name);
        if path.exists() {
            fs::remove_dir_all(path.as_path()).unwrap();
        }
        fs::create_dir(&path).unwrap();

        // On macOS, the temporary directory is in `/var`, which is a symlink to `/private/var`.
        // We need to get the actual path here, or we will get path inconsistency.
        #[cfg(target_os = "macos")]
        let path = fs::canonicalize(path).unwrap();

        Self {
            working_directory: Utf8PathBuf::from_path_buf(path.clone()).unwrap(),
            files: Vec::new(),
            project_directory: Utf8PathBuf::from_path_buf(path).unwrap(),
        }
    }

    /// Creates a file under the working directory
    pub fn create_file(&mut self, name: &str, content: &str) -> Utf8PathBuf {
        let path = self.working_directory.join(name);
        fs::create_dir_all(path.parent().expect("parent dir exists."))
            .expect("Temporary directory to exist and being writable");
        fs::write(path.as_std_path(), content)
            .expect("Temporary directory to exist and being writable");
        self.files.push((path.clone(), content.to_string()));
        path
    }

    pub fn create_folder(&mut self, name: &str) {
        let path = self.working_directory.join(name);
        fs::create_dir_all(path.parent().expect("parent dir exists."))
            .expect("Temporary directory to exist and being writable");
    }

    pub fn append_to_working_directory(&mut self, path: &str) {
        self.working_directory = self.project_directory.join(path);
    }

    /// Returns the path to use when running the CLI
    pub fn cli_path(&self) -> &str {
        self.working_directory.as_str()
    }

    /// Returns an instance of [OsFileSystem] given the current working directory
    pub fn create_os(&self) -> OsFileSystem {
        OsFileSystem::new(self.working_directory.clone())
    }

    /// Returns an instance of [MemoryFileSystem]. The files saved in the file system
    /// will be stripped of the working directory path, making snapshots predictable.
    pub fn create_mem(&self) -> MemoryFileSystem {
        let fs = MemoryFileSystem::default();
        for (path, _) in self.files.iter() {
            let content = fs::read_to_string(path.as_std_path()).expect("File to exist");
            fs.insert(
                path.clone()
                    .strip_prefix(self.project_directory.as_str())
                    .expect("Working directory")
                    .to_path_buf(),
                content.as_bytes(),
            );
        }

        fs
    }
}

#[cfg(test)]
mod platform_path_tests {
    use camino::Utf8Path;

    use super::windows_paths_have_same_spelling;

    #[test]
    fn compares_windows_verbatim_paths_by_their_regular_spelling() {
        assert!(windows_paths_have_same_spelling(
            Utf8Path::new(r"C:\project\file.js"),
            Utf8Path::new(r"\\?\C:\project\file.js"),
        ));
        assert!(windows_paths_have_same_spelling(
            Utf8Path::new(r"\\server\share\file.js"),
            Utf8Path::new(r"\\?\UNC\server\share\file.js"),
        ));
        assert!(windows_paths_have_same_spelling(
            Utf8Path::new("//server/share/file.js"),
            Utf8Path::new(r"\\?\UNC\server\share\file.js"),
        ));
        assert!(!windows_paths_have_same_spelling(
            Utf8Path::new(r"C:\project\file.js"),
            Utf8Path::new(r"\\?\C:\other\file.js"),
        ));
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::hash::RandomState;
    use std::os::unix::fs::symlink;

    use biome_diagnostics::Error;
    use papaya::{HashSet, HashSetRef, LocalGuard};

    use super::*;
    use crate::{PathInterner, PhysicalFileOwners, TraversalContext};

    struct TestContext {
        interner: PathInterner,
        physical_files: PhysicalFileOwners,
        visited: HashSet<BiomePath>,
        /// Files reported to [TraversalContext::can_handle_with_kind] as symbolic-link paths.
        symlink_files: HashSet<BiomePath>,
        /// The number of entries reported to [TraversalContext::can_handle_with_kind].
        handled_entries: AtomicUsize,
        ignored_path: Option<BiomePath>,
        store_dirs: bool,
    }

    impl TestContext {
        fn new(ignored_path: Option<BiomePath>, store_dirs: bool) -> Self {
            let (interner, _) = PathInterner::new();
            Self {
                interner,
                physical_files: PhysicalFileOwners::default(),
                visited: HashSet::default(),
                symlink_files: HashSet::default(),
                handled_entries: AtomicUsize::default(),
                ignored_path,
                store_dirs,
            }
        }
    }

    impl TraversalContext for TestContext {
        fn interner(&self) -> &PathInterner {
            &self.interner
        }

        fn store_physical_path(
            &self,
            physical_path: Utf8PathBuf,
            logical_path: BiomePath,
            is_symlink: bool,
        ) {
            self.physical_files.select(
                physical_path,
                logical_path,
                is_symlink,
                |previous, selected| {
                    let visited = self.visited.pin();
                    if let Some(previous) = previous {
                        visited.remove(previous);
                    }
                    visited.insert(selected.clone());
                },
            );
        }

        fn evaluated_paths(&self) -> HashSetRef<'_, BiomePath, RandomState, LocalGuard<'_>> {
            self.visited.pin()
        }

        fn push_diagnostic(&self, error: Error) {
            panic!("unexpected diagnostic: {error:?}");
        }

        fn can_handle_with_kind(&self, path: &BiomePath, path_kind: PathKind) -> bool {
            self.handled_entries.fetch_add(1, Ordering::Relaxed);
            if path_kind == (PathKind::File { is_symlink: true }) {
                self.symlink_files.pin().insert(path.clone());
            }
            self.ignored_path
                .as_ref()
                .is_none_or(|ignored| ignored != path)
        }

        fn handle_path(&self, path: BiomePath) {
            self.visited.pin().insert(path);
        }

        fn store_path(&self, path: BiomePath) {
            self.visited.pin().insert(path);
        }

        fn should_store_dirs(&self) -> bool {
            self.store_dirs
        }
    }

    #[test]
    fn ignored_symlink_alias_does_not_claim_the_physical_target() {
        let mut temporary = TemporaryFs::new("ignored_symlink_alias_does_not_claim_target");
        temporary.create_file("target/file.js", "statement();");
        let target = temporary.working_directory.join("target");
        let ignored = temporary.working_directory.join("ignored");
        let allowed = temporary.working_directory.join("allowed");
        symlink(&target, &ignored).unwrap();
        symlink(&target, &allowed).unwrap();

        let fs = temporary.create_os();
        let ctx = TestContext::new(Some(fs.biome_path(&ignored)), false);
        fs.traversal(Box::new(|scope| {
            scope.evaluate(&ctx, ignored.clone());
            scope.evaluate(&ctx, allowed.clone());
        }));

        assert!(
            ctx.visited
                .pin()
                .contains(&fs.biome_path(&allowed.join("file.js")))
        );
        std::fs::remove_dir_all(temporary.project_directory).unwrap();
    }

    #[test]
    fn ignored_file_under_one_symlink_alias_does_not_claim_the_physical_file() {
        let mut temporary = TemporaryFs::new("ignored_file_under_one_symlink_alias");
        temporary.create_file("target/nested/file.js", "statement();");
        let target = temporary.working_directory.join("target");
        let first_alias = temporary.working_directory.join("first");
        let second_alias = temporary.working_directory.join("second");
        symlink(&target, &first_alias).unwrap();
        symlink(&target, &second_alias).unwrap();

        let fs = temporary.create_os();
        let first_file = first_alias.join("nested/file.js");
        let second_file = second_alias.join("nested/file.js");
        let ctx = TestContext::new(Some(fs.biome_path(&first_file)), false);
        fs.traversal(Box::new(|scope| {
            scope.evaluate(&ctx, first_alias);
            scope.evaluate(&ctx, second_alias);
        }));

        let visited = ctx.visited.pin();
        assert!(!visited.contains(&fs.biome_path(&first_file)));
        assert!(visited.contains(&fs.biome_path(&second_file)));
        drop(visited);
        std::fs::remove_dir_all(temporary.project_directory).unwrap();
    }

    #[test]
    fn selects_a_stable_logical_alias_for_each_physical_file() {
        let mut temporary = TemporaryFs::new("selects_stable_logical_alias");
        temporary.create_file("project/root.js", "statement();");
        temporary.create_file("target/file.js", "statement();");
        let project = temporary.working_directory.join("project");
        let target = temporary.working_directory.join("target");
        let first_alias = project.join("first");
        let second_alias = project.join("second");
        symlink(&target, &first_alias).unwrap();
        symlink(&target, &second_alias).unwrap();

        let fs = temporary.create_os();
        let ctx = TestContext::new(None, false);
        fs.traversal(Box::new(|scope| {
            scope.evaluate(&ctx, project);
        }));

        let visited = ctx.visited.pin();
        assert!(visited.contains(&fs.biome_path(&first_alias.join("file.js"))));
        assert!(!visited.contains(&fs.biome_path(&second_alias.join("file.js"))));
        drop(visited);
        std::fs::remove_dir_all(temporary.project_directory).unwrap();
    }

    #[test]
    fn deduplicates_targets_through_symlinked_parent_components() {
        let mut temporary = TemporaryFs::new("deduplicates_symlinked_parent_components");
        temporary.create_file("target/nested/file.js", "statement();");
        let target = temporary.working_directory.join("target");
        let parent_alias = temporary.working_directory.join("parent");
        let nested_alias = temporary.working_directory.join("alias");
        symlink(&target, &parent_alias).unwrap();
        symlink(parent_alias.join("nested"), &nested_alias).unwrap();

        let fs = temporary.create_os();
        let ctx = TestContext::new(None, false);
        fs.traversal(Box::new(|scope| {
            scope.evaluate(&ctx, target.clone());
            scope.evaluate(&ctx, nested_alias.clone());
        }));

        let visited = ctx.visited.pin();
        assert!(visited.contains(&fs.biome_path(&target.join("nested/file.js"))));
        assert!(!visited.contains(&fs.biome_path(&nested_alias.join("file.js"))));
        drop(visited);
        std::fs::remove_dir_all(temporary.project_directory).unwrap();
    }

    #[test]
    fn bounds_enumerations_through_directory_aliases() {
        const LEVELS: usize = 12;
        let mut temporary = TemporaryFs::new("bounds_enumerations_through_directory_aliases");
        temporary.create_file(&format!("level{LEVELS}/file.js"), "statement();");
        for level in 0..LEVELS {
            let directory = temporary.working_directory.join(format!("level{level}"));
            let next = temporary
                .working_directory
                .join(format!("level{}", level + 1));
            std::fs::create_dir_all(&directory).unwrap();
            symlink(&next, directory.join("a")).unwrap();
            symlink(&next, directory.join("b")).unwrap();
        }

        let fs = temporary.create_os();
        let ctx = TestContext::new(None, false);
        let root = temporary.working_directory.join("level0");
        fs.traversal(Box::new(|scope| {
            scope.evaluate(&ctx, root.clone());
        }));

        // Without a bound, level `n` is enumerated through 2^n routes.
        let handled_entries = ctx.handled_entries.load(Ordering::Relaxed);
        assert!(
            handled_entries < 2 * LEVELS * (DirectoryAliases::MAX_ENUMERATIONS + 1),
            "handled {handled_entries} entries"
        );
        assert_eq!(ctx.visited.len(), 1);
        std::fs::remove_dir_all(temporary.project_directory).unwrap();
    }

    #[test]
    fn files_below_a_symlinked_root_are_not_symlink_paths() {
        let mut temporary = TemporaryFs::new("files_below_a_symlinked_root");
        temporary.create_file("real/project/file.js", "statement();");
        temporary.create_file("real/other/dependency.js", "statement();");
        let real = temporary.working_directory.join("real");
        let linked_parent = temporary.working_directory.join("linked");
        let linked_project = temporary.working_directory.join("project_alias");
        symlink(&real, &linked_parent).unwrap();
        symlink(real.join("project"), &linked_project).unwrap();
        symlink(real.join("other"), real.join("project/nested")).unwrap();

        let fs = temporary.create_os();
        for root in [linked_parent.join("project"), linked_project] {
            let ctx = TestContext::new(None, false);
            fs.traversal(Box::new(|scope| {
                scope.evaluate(&ctx, root.clone());
            }));

            let file = fs.biome_path(&root.join("file.js"));
            let dependency = fs.biome_path(&root.join("nested/dependency.js"));
            let visited = ctx.visited.pin();
            let symlink_files = ctx.symlink_files.pin();
            assert!(visited.contains(&file), "{file} was not visited");
            assert!(
                !symlink_files.contains(&file),
                "{file} is reported as a symlink"
            );
            assert!(
                visited.contains(&dependency),
                "{dependency} was not visited"
            );
            assert!(
                symlink_files.contains(&dependency),
                "{dependency} is not reported as a symlink"
            );
        }
        std::fs::remove_dir_all(temporary.project_directory).unwrap();
    }

    #[test]
    fn stores_every_symlink_directory_alias_while_deduplicating_files() {
        let mut temporary = TemporaryFs::new("stores_every_symlink_directory_alias");
        temporary.create_file("target/file.js", "statement();");
        let target = temporary.working_directory.join("target");
        let first_alias = temporary.working_directory.join("first");
        let second_alias = temporary.working_directory.join("second");
        symlink(&target, &first_alias).unwrap();
        symlink(&target, &second_alias).unwrap();

        let fs = temporary.create_os();
        let ctx = TestContext::new(None, true);
        fs.traversal(Box::new(|scope| {
            scope.evaluate(&ctx, target.clone());
            scope.evaluate(&ctx, first_alias.clone());
            scope.evaluate(&ctx, second_alias.clone());
        }));

        let visited = ctx.visited.pin();
        assert!(visited.contains(&fs.biome_path(&first_alias)));
        assert!(visited.contains(&fs.biome_path(&second_alias)));
        assert!(visited.contains(&fs.biome_path(&target.join("file.js"))));
        assert!(!visited.contains(&fs.biome_path(&first_alias.join("file.js"))));
        assert!(!visited.contains(&fs.biome_path(&second_alias.join("file.js"))));
        drop(visited);
        std::fs::remove_dir_all(temporary.project_directory).unwrap();
    }
}
