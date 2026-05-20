//! TypeScript source acquisition and default library discovery for global types codegen.

use std::collections::{BTreeMap, HashSet};
use std::fmt::Write as _;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufReader, ErrorKind, Read as _, Write as _};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::{Context, anyhow, bail};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsBindingPattern, AnyJsExpression, AnyJsLiteralExpression, JsFileSource,
    JsVariableDeclarator,
};
use biome_rowan::{AstNode, Text, TokenText};
use biome_string_case::StrLikeExtension;
use sha2::{Digest, Sha256};

use crate::generate_global_types::SourcePin;

/// Default remote used to acquire TypeScript sources.
const DEFAULT_TYPESCRIPT_REPO_URL: &str = "https://github.com/microsoft/TypeScript.git";

/// Workspace-relative cache directory for acquired TypeScript checkouts.
const TYPESCRIPT_CACHE_RELATIVE_DIR: &str = "target/xtask/typescript";

/// Directory containing per-pin temporary checkout namespaces.
const TEMP_CHECKOUT_DIR: &str = ".tmp";

/// TypeScript command line parser path relative to a checkout root.
const COMMAND_LINE_PARSER_RELATIVE_PATH: &str = "src/compiler/commandLineParser.ts";

/// TypeScript default library directory path relative to a checkout root.
const DEFAULT_LIBRARY_RELATIVE_DIR: &str = "lib";

/// TypeScript default library filenames must start with this marker before the lib key.
const DEFAULT_LIBRARY_FILE_PREFIX: &str = "lib.";

/// TypeScript default library discovery is limited to declaration files.
const DEFAULT_LIBRARY_FILE_SUFFIX: &str = ".d.ts";

/// Default base TypeScript library filenames checked for highest sort priority.
const BASE_DEFAULT_LIBRARY_FILES: &[&str] = &["lib.es6.d.ts", "lib.d.ts"];

/// Triple-slash directives are only recognized in comments that start with `///`.
const TRIPLE_SLASH_PREFIX: &str = "///";

/// TypeScript dependency directives use the XML-like `<reference ...>` form.
const REFERENCE_DIRECTIVE_PREFIX: &str = "<reference";

/// `path` references are file dependencies resolved relative to the current declaration.
const REFERENCE_PATH_ATTRIBUTE: &str = "path";

/// `lib` references are default-library dependencies resolved through TypeScript's lib map.
const REFERENCE_LIB_ATTRIBUTE: &str = "lib";

/// `types` references require external packages that this generator does not vendor.
const REFERENCE_TYPES_ATTRIBUTE: &str = "types";

/// `no-default-lib` references affect TypeScript's default library selection but add no file edge.
const REFERENCE_NO_DEFAULT_LIB_ATTRIBUTE: &str = "no-default-lib";

/// Explicit lib profile roots used as discovery starting points.
///
/// Discovery is restricted to these filenames plus their reference closure;
/// adding a new root requires an explicit selection-table change. `pub` so
/// `generate_global_types::run` can pass the production list to `discover` and
/// tests can pass scenario-specific lists.
pub const PROFILE_ROOTS: &[&str] = &[
    "lib.es5.d.ts",
    "lib.es2015.collection.d.ts",
    "lib.es2015.iterable.d.ts",
    "lib.es2015.promise.d.ts",
    "lib.es2018.promise.d.ts",
    "lib.es2020.promise.d.ts",
    "lib.es2021.promise.d.ts",
    "lib.es2024.promise.d.ts",
    "lib.es2025.promise.d.ts",
    "lib.es2015.symbol.wellknown.d.ts",
    "lib.es2015.reflect.d.ts",
    "lib.esnext.disposable.d.ts",
];

/// Priority used by TypeScript for the base default library files.
const BASE_DEFAULT_LIBRARY_PRIORITY: usize = 0;

/// Priority offset TypeScript uses for entries from `libEntries`.
const DEFAULT_LIBRARY_PRIORITY_OFFSET: usize = 1;

/// Priority offset TypeScript uses for files outside default-library ordering.
const OUTSIDE_DEFAULT_LIBRARY_PRIORITY_OFFSET: usize = 2;

/// Maximum number of unique temporary checkout names to try for one acquisition.
const MAX_TEMP_CHECKOUT_ATTEMPTS: u32 = 1024;

/// SHA-256 is 32 bytes; hex-encoded that is 64 characters.
const SHA256_HEX_LENGTH: usize = 64;

/// `git ls-files --stage` mode for a tracked regular file (`-rw-r--r--`).
const GIT_REGULAR_FILE_MODE: &str = "100644";

/// `git ls-files --stage` mode for a tracked executable regular file (`-rwxr-xr-x`).
const GIT_EXECUTABLE_FILE_MODE: &str = "100755";

/// Width of the mode prefix in a `git ls-files --stage` entry; both mode constants are 6 bytes.
const GIT_FILE_MODE_LENGTH: usize = 6;

/// Maximum number of polls when contending for the acquire lock.
///
/// Combined with [`ACQUIRE_LOCK_POLL_INTERVAL_MS`] this bounds the wait at roughly
/// 60 seconds, enough for a sibling clone+sparse-checkout to finish but short
/// enough that a leaked lock file does not stall CI indefinitely.
const ACQUIRE_LOCK_POLL_ATTEMPTS: u32 = 600;

/// Sleep between acquire-lock polls in milliseconds.
///
/// Tuned with [`ACQUIRE_LOCK_POLL_ATTEMPTS`] to total roughly 60 seconds.
const ACQUIRE_LOCK_POLL_INTERVAL_MS: u64 = 100;

/// File stored inside an acquire-lock directory with the owner PID and token.
const ACQUIRE_LOCK_OWNER_FILE: &str = "owner";

/// Lowercase nibble table used by the streaming hex encoder.
const HEX_NIBBLES: &[u8; 16] = b"0123456789abcdef";

/// Upper bound on a single `git cat-file --batch` blob payload (16 MiB).
///
/// TypeScript declaration files are kilobytes; this constant caps the
/// pre-allocation driven by the size git reports in the batch header so a
/// corrupted cache or a malicious `repo_url_override` cannot make the codegen
/// allocate an arbitrary amount of memory before we read.
const MAX_BLOB_SIZE_BYTES: usize = 16 * 1024 * 1024;

/// Upper bound on a single `git cat-file --batch` response header line.
///
/// Headers are always `<sha> blob <size>\n` or `<object> missing\n`, both
/// comfortably under 200 bytes. This cap protects the reused `header_buffer`
/// from unbounded growth if a corrupted child ever emits a pathological line.
const MAX_BATCH_HEADER_BYTES: usize = 512;

/// A canonical filesystem path that has been proven to stay within a root.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CanonicalPath(PathBuf);

impl CanonicalPath {
    /// Canonicalizes `relative` under `root`, rejecting paths that escape `root`.
    pub fn from_within(root: &Path, relative: &str) -> anyhow::Result<Self> {
        let canonical_root = fs::canonicalize(root)
            .with_context(|| format!("failed to canonicalize root {}", root.display()))?;
        let candidate = canonical_root.join(relative);
        canonicalize_within(&canonical_root, &candidate)
    }

    /// Exposes the already-canonicalized, root-checked path. The caller does
    /// not need to revalidate containment.
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

/// An acquired TypeScript checkout and metadata derived from it.
pub struct AcquiredCheckout {
    root: CanonicalPath,
    pin: SourcePin,
    /// SHA-256 of `src/compiler/commandLineParser.ts` after LF normalization.
    command_line_parser_sha256: String,
}

impl AcquiredCheckout {
    /// Canonical root of the acquired TypeScript checkout.
    pub fn root(&self) -> &CanonicalPath {
        &self.root
    }

    /// Pinned TypeScript source tag and commit.
    pub fn pin(&self) -> &SourcePin {
        &self.pin
    }

    /// SHA-256 of `src/compiler/commandLineParser.ts` after LF normalization.
    pub fn command_line_parser_sha256(&self) -> &str {
        &self.command_line_parser_sha256
    }
}

/// Parsed entries from TypeScript's `libEntries` table.
pub struct LibEntries {
    /// Library keys in source order, preserving duplicates.
    pub libs: Vec<Text>,
    /// Library key to filename map; later duplicate keys overwrite earlier ones.
    pub lib_map: BTreeMap<Text, Text>,
}

/// A discovered TypeScript declaration file after dependency traversal.
pub struct DiscoveredFile {
    /// Canonical filesystem path, proven to stay within the checkout root.
    pub path: CanonicalPath,
    /// Checkout-relative path using forward slashes.
    pub repo_relative: String,
    /// File bytes after LF normalization.
    pub bytes_lf: Vec<u8>,
    /// SHA-256 of `bytes_lf` (LF-normalized contents), not raw on-disk bytes.
    pub sha256_hex: String,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct TrackedSourceFile {
    path: CanonicalPath,
    repo_relative: String,
}

/// Options controlling TypeScript source acquisition.
pub struct SourceOptions {
    /// When true, fail on cache miss instead of cloning over the network.
    pub offline: bool,
    /// When true, re-run [`validate_checkout`] even when the cache hits, so
    /// codegen never trusts a worktree it has not just verified.
    pub verify: bool,
    /// Alternate TypeScript repository URL used by tests; production runs use
    /// [`DEFAULT_TYPESCRIPT_REPO_URL`].
    pub repo_url_override: Option<PathBuf>,
}

/// Acquires a pinned TypeScript checkout in the workspace cache.
pub fn acquire(pin: &SourcePin, opts: &SourceOptions) -> anyhow::Result<AcquiredCheckout> {
    let workspace_root = workspace_root()?;
    let cache_parent = workspace_root.join(TYPESCRIPT_CACHE_RELATIVE_DIR);
    let checkout_path = cache_parent.join(cache_key(pin));

    if checkout_path.exists() {
        remove_stale_acquire_lock(&cache_parent, pin)?;
        remove_stale_temporary_checkouts(&cache_parent, pin)?;
        validate_checkout(&checkout_path, pin)?;
    } else {
        if opts.offline {
            bail!("offline: cache miss at {}", checkout_path.display());
        }

        fs::create_dir_all(&cache_parent)
            .with_context(|| format!("failed to create {}", cache_parent.display()))?;
        clone_checkout(&cache_parent, &checkout_path, pin, opts)?;
    }

    if opts.verify {
        validate_checkout(&checkout_path, pin)?;
    }

    let canonical_root = CanonicalPath(
        fs::canonicalize(&checkout_path)
            .with_context(|| format!("failed to canonicalize {}", checkout_path.display()))?,
    );
    let command_line_parser_bytes =
        read_blob_lf_normalized(&checkout_path, pin.sha(), COMMAND_LINE_PARSER_RELATIVE_PATH)?;
    let command_line_parser_sha256 = sha256_hex_from_bytes(&command_line_parser_bytes);

    Ok(AcquiredCheckout {
        root: canonical_root,
        pin: pin.clone(),
        command_line_parser_sha256,
    })
}

/// Parses TypeScript's `libEntries` table from `commandLineParser.ts`.
pub fn parse_lib_entries(checkout: &AcquiredCheckout) -> anyhow::Result<LibEntries> {
    let bytes = read_blob_lf_normalized(
        checkout.root.as_path(),
        checkout.pin.sha(),
        COMMAND_LINE_PARSER_RELATIVE_PATH,
    )?;
    let actual_sha = sha256_hex_from_bytes(&bytes);
    if actual_sha != checkout.command_line_parser_sha256 {
        bail!(
            "commandLineParser.ts SHA mismatch at {}:{}: read {actual_sha}, expected {}",
            checkout.pin.sha(),
            COMMAND_LINE_PARSER_RELATIVE_PATH,
            checkout.command_line_parser_sha256
        );
    }
    let source = std::str::from_utf8(&bytes).context("commandLineParser.ts is not valid UTF-8")?;
    let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());

    if parsed.has_errors() {
        bail!(
            "failed to parse commandLineParser.ts: {} diagnostics",
            parsed.diagnostics().len()
        );
    }

    let root = parsed.syntax();
    for node in root.descendants() {
        let Some(declarator) = JsVariableDeclarator::cast(node) else {
            continue;
        };
        let Ok(id) = declarator.id() else {
            continue;
        };
        let Some(name) = binding_name(&id) else {
            continue;
        };
        if name != "libEntries" {
            continue;
        }

        let Some(initializer) = declarator.initializer() else {
            bail!("libEntries declarator has no initializer");
        };
        let expression = initializer
            .expression()
            .context("failed to read libEntries initializer expression")?;
        let Some(array) = array_expression(&expression) else {
            bail!("libEntries initializer is not an array expression");
        };

        let mut libs = Vec::new();
        let mut lib_map = BTreeMap::new();
        for (index, element) in array.elements().into_iter().enumerate() {
            let element = element
                .with_context(|| format!("failed to read libEntries element at index {index}"))?;
            let tuple_expression = match element {
                AnyJsArrayElement::AnyJsExpression(expression) => expression,
                AnyJsArrayElement::JsArrayHole(_) | AnyJsArrayElement::JsSpread(_) => {
                    bail!("libEntries element at index {index} is not a tuple array");
                }
            };
            let Some(tuple_array) = array_expression(&tuple_expression) else {
                bail!("libEntries element at index {index} is not a tuple array");
            };
            let (key, filename) = tuple_elements(tuple_array)
                .with_context(|| format!("failed to parse libEntries tuple at index {index}"))?;
            validate_lib_entry_filename(index, filename.text())?;

            libs.push(key.clone());
            lib_map.insert(key, filename);
        }

        if libs.is_empty() {
            bail!("libEntries array is empty");
        }

        return Ok(LibEntries { libs, lib_map });
    }

    Err(anyhow!("could not find libEntries in commandLineParser.ts"))
}

/// Discovers default library files by walking each profile root's reference
/// closure and sorting the result by TypeScript's default-library priority.
///
/// `profile_roots` is the entry-point filename list (typically [`PROFILE_ROOTS`]).
/// Tests pass scenario-specific lists to lock the resulting closure order.
pub fn discover(
    checkout: &AcquiredCheckout,
    libs: &LibEntries,
    profile_roots: &[&str],
) -> anyhow::Result<Vec<DiscoveredFile>> {
    let default_library_path =
        CanonicalPath::from_within(checkout.root.as_path(), DEFAULT_LIBRARY_RELATIVE_DIR)
            .context("failed to resolve TypeScript lib directory")?;
    let mut visited = HashSet::new();
    let mut discovered = Vec::<DiscoveredFile>::new();
    let mut batch = BatchCatFile::new(checkout.root.as_path())?;

    for filename in profile_roots {
        let relative = default_library_relative_path(filename);
        let source = validate_reference_source_file(checkout.root.as_path(), &relative)
            .with_context(|| format!("profile root {filename:?} is not a tracked regular file"))?;

        if visited.contains(&source.repo_relative) {
            continue;
        }
        visited.insert(source.repo_relative.clone());

        let mut stack = vec![make_frame(
            source,
            checkout.root.as_path(),
            checkout.pin.sha(),
            libs,
            &mut batch,
        )?];
        while !stack.is_empty() {
            let child_path = {
                let frame = stack.last_mut().expect("stack is not empty");
                frame.children_remaining.pop().map(|reference| {
                    let parent_dir = frame.source.path.as_path().parent().ok_or_else(|| {
                        anyhow!(
                            "{} has no parent directory",
                            frame.source.path.as_path().display()
                        )
                    });

                    let parent_dir = parent_dir?;
                    resolve_child_reference(checkout.root.as_path(), parent_dir, &reference, libs)
                })
            };

            let Some(child_path) = child_path else {
                let frame = stack.pop().expect("stack is not empty");
                discovered.push(discovered_file_from_frame(frame));
                continue;
            };
            let child_path = child_path?;

            if !visited.contains(&child_path.repo_relative) {
                visited.insert(child_path.repo_relative.clone());
                stack.push(make_frame(
                    child_path,
                    checkout.root.as_path(),
                    checkout.pin.sha(),
                    libs,
                    &mut batch,
                )?);
            }
        }
    }

    discovered.sort_by_cached_key(|file| {
        default_library_priority(
            file.path.as_path(),
            default_library_path.as_path(),
            &libs.libs,
        )
    });

    Ok(discovered)
}

/// Stack frame used while traversing TypeScript triple-slash dependency references.
struct Frame {
    source: TrackedSourceFile,
    bytes_lf: Vec<u8>,
    children_remaining: Vec<TripleSlashReference>,
}

/// File or default-library reference parsed from a triple-slash directive.
enum TripleSlashReference {
    File(String),
    Lib(String),
}

/// Returns the absolute path of the Biome repository root via `git rev-parse`.
fn workspace_root() -> anyhow::Result<PathBuf> {
    let mut command = new_git_command();
    command.args(["rev-parse", "--show-toplevel"]);
    let stdout = command_stdout(&mut command, "git rev-parse --show-toplevel")?;
    Ok(PathBuf::from(stdout.trim()))
}

/// Builds a fresh `git` [`Command`] with environment variables that could
/// redirect plumbing to a different repository removed. Every git invocation
/// passes through this helper so a stray `GIT_DIR` / `GIT_WORK_TREE` /
/// `GIT_INDEX_FILE` in the caller's environment cannot silently make
/// validation read from the wrong index.
fn new_git_command() -> Command {
    let mut command = Command::new("git");
    command
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_INDEX_FILE")
        .env_remove("GIT_OBJECT_DIRECTORY")
        .env_remove("GIT_ALTERNATE_OBJECT_DIRECTORIES")
        .env_remove("GIT_COMMON_DIR")
        .env_remove("GIT_NAMESPACE");
    command
}

/// Returns the per-pin cache directory basename `"<tag>-<sha>"` used to
/// namespace acquired checkouts and lock paths.
fn cache_key(pin: &SourcePin) -> String {
    format!("{}-{}", pin.tag(), pin.sha())
}

/// Asserts that the checkout's HEAD/tag match the pin and that the worktree
/// is clean (no diff, no untracked, no hidden index flags).
fn validate_checkout(root: &Path, pin: &SourcePin) -> anyhow::Result<()> {
    let head = git_rev_parse(root, "HEAD")?;
    if head != pin.sha() {
        bail!(
            "cached TypeScript checkout HEAD mismatch at {}: got {head}, expected {}",
            root.display(),
            pin.sha()
        );
    }

    let tag_ref = format!("refs/tags/{}^{{commit}}", pin.tag());
    let tag_commit = git_rev_parse(root, &tag_ref)?;
    if tag_commit != pin.sha() {
        bail!(
            "cached TypeScript checkout tag-ref mismatch at {}: refs/tags/{} resolves to {tag_commit}, expected {}",
            root.display(),
            pin.tag(),
            pin.sha()
        );
    }

    validate_no_hidden_index_flags(root)?;

    let mut diff = new_git_command();
    diff.arg("-C")
        .arg(root)
        .args(["diff-index", "--quiet", "HEAD", "--"]);
    let status = diff
        .status()
        .with_context(|| format!("failed to run git diff-index in {}", root.display()))?;
    if !status.success() {
        bail!(
            "cached TypeScript checkout has uncommitted modifications at {}; remove the cache and rerun",
            root.display()
        );
    }

    // `--exclude-standard` keeps files covered by the pinned `.gitignore`
    // out of the listing; anything else untracked aborts the run.
    let mut untracked = new_git_command();
    untracked
        .arg("-C")
        .arg(root)
        .args(["ls-files", "--others", "--exclude-standard"]);
    let untracked_stdout = command_stdout(&mut untracked, "git ls-files --others")?;
    if !untracked_stdout.trim().is_empty() {
        bail!(
            "cached TypeScript checkout has untracked files at {}; remove the cache and rerun:\n{}",
            root.display(),
            untracked_stdout.trim()
        );
    }

    Ok(())
}

/// Bails when any tracked `lib` or `src/compiler` entry carries an
/// `assume-unchanged` / `skip-worktree` index flag, which would let a dirty
/// worktree slip past `git diff-index`.
fn validate_no_hidden_index_flags(root: &Path) -> anyhow::Result<()> {
    let mut ls_files = new_git_command();
    ls_files
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-v", "-z", "--"])
        .arg(DEFAULT_LIBRARY_RELATIVE_DIR)
        .arg("src/compiler");
    let stdout = command_stdout(&mut ls_files, "git ls-files -v")?;
    for entry in stdout.split('\0').filter(|entry| !entry.is_empty()) {
        let Some(status) = entry.as_bytes().first().copied() else {
            continue;
        };
        if status == b'S' || status.is_ascii_lowercase() {
            let path = entry.get(2..).unwrap_or(entry);
            bail!(
                "cached TypeScript checkout has hidden git index flags at {}: {path}; clear assume-unchanged/skip-worktree flags or remove the cache and rerun",
                root.display()
            );
        }
    }

    Ok(())
}

/// Resolves a revision spec via `git rev-parse` and returns the trimmed output.
fn git_rev_parse(root: &Path, revision: &str) -> anyhow::Result<String> {
    let mut command = new_git_command();
    command.arg("-C").arg(root).arg("rev-parse").arg(revision);
    let stdout = command_stdout(&mut command, &format!("git rev-parse {revision}"))?;
    Ok(stdout.trim().to_owned())
}

/// Decodes the stdout captured by [`command_stdout_bytes`] as UTF-8, bailing
/// with `description` when the bytes are not valid UTF-8.
fn command_stdout(command: &mut Command, description: &str) -> anyhow::Result<String> {
    let stdout = command_stdout_bytes(command, description)?;
    String::from_utf8(stdout).with_context(|| format!("{description} wrote non-UTF-8 stdout"))
}

/// Runs `command` and returns its raw stdout, bailing with `description` and
/// the captured stderr on a non-zero exit status.
fn command_stdout_bytes(command: &mut Command, description: &str) -> anyhow::Result<Vec<u8>> {
    let output = command
        .output()
        .with_context(|| format!("failed to run {description}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("{description} failed: {}", stderr.trim());
    }

    Ok(output.stdout)
}

/// Acquires the cache lock, sparse-checks out `lib` and `src/compiler` from
/// the pinned TypeScript tag, validates the result, and atomically renames the
/// temporary checkout into the cache slot.
fn clone_checkout(
    cache_parent: &Path,
    checkout_path: &Path,
    pin: &SourcePin,
    opts: &SourceOptions,
) -> anyhow::Result<()> {
    let _lock = AcquireLock::acquire(cache_parent, pin, checkout_path)?;

    if checkout_path.exists() {
        return validate_checkout(checkout_path, pin);
    }
    remove_stale_temporary_checkouts(cache_parent, pin)?;

    let temporary_checkout =
        TemporaryCheckout::new(unique_temporary_checkout_path(cache_parent, pin)?);

    let mut clone = new_git_command();
    clone
        .arg("clone")
        .arg("--depth=1")
        .arg("--branch")
        .arg(pin.tag())
        .arg("--filter=blob:none")
        .arg("--no-checkout")
        .arg("--");
    match opts.repo_url_override.as_deref() {
        Some(path) => clone.arg(path),
        None => clone.arg(DEFAULT_TYPESCRIPT_REPO_URL),
    };
    clone.arg(temporary_checkout.path());
    command_stdout(&mut clone, "git clone TypeScript")?;

    let mut sparse_init = new_git_command();
    sparse_init
        .arg("-C")
        .arg(temporary_checkout.path())
        .arg("sparse-checkout")
        .arg("init")
        .arg("--cone");
    command_stdout(&mut sparse_init, "git sparse-checkout init")?;

    let mut sparse_set = new_git_command();
    sparse_set
        .arg("-C")
        .arg(temporary_checkout.path())
        .arg("sparse-checkout")
        .arg("set")
        .arg("lib")
        .arg("src/compiler");
    command_stdout(&mut sparse_set, "git sparse-checkout set")?;

    let mut checkout = new_git_command();
    checkout
        .arg("-C")
        .arg(temporary_checkout.path())
        .arg("checkout");
    command_stdout(&mut checkout, "git checkout TypeScript")?;
    validate_checkout(temporary_checkout.path(), pin)?;

    match fs::rename(temporary_checkout.path(), checkout_path) {
        Ok(()) => validate_checkout(checkout_path, pin),
        Err(rename_error) if checkout_path.exists() => {
            validate_checkout(checkout_path, pin).with_context(|| {
                format!(
                    "failed to validate concurrently-created checkout after rename failed: {rename_error}"
                )
            })
        }
        Err(rename_error) => {
            Err(rename_error).with_context(|| {
                format!(
                    "failed to move {} to {}",
                    temporary_checkout.path().display(),
                    checkout_path.display()
                )
            })
        }
    }
}

/// RAII guard that owns a temporary checkout directory and removes it on drop.
struct TemporaryCheckout {
    path: PathBuf,
}

impl TemporaryCheckout {
    /// Wraps `path` so its directory tree is removed when the guard drops.
    fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Borrows the temporary checkout's filesystem path.
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TemporaryCheckout {
    /// Best-effort recursive removal of the temporary checkout directory.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Filesystem lock that serializes concurrent xtask invocations cloning the same pin.
///
/// The lock is an atomically-created directory containing an owner token made
/// from the process ID and a nonce. Live owners are left alone; dead owners and
/// legacy file locks are removed before retrying. `Drop` removes the directory
/// only when its owner token still matches, so a replaced lock is not deleted.
struct AcquireLock {
    path: PathBuf,
    token: String,
}

impl AcquireLock {
    /// Acquires the lock by atomically creating the lock directory, polling
    /// while a peer holds it, and reclaiming the slot when the recorded owner
    /// PID is dead. Times out at `ACQUIRE_LOCK_POLL_ATTEMPTS × INTERVAL_MS`.
    fn acquire(cache_parent: &Path, pin: &SourcePin, checkout_path: &Path) -> anyhow::Result<Self> {
        let path = acquire_lock_path(cache_parent, pin);
        let token = acquire_lock_token();
        for _ in 0..ACQUIRE_LOCK_POLL_ATTEMPTS {
            match fs::create_dir(&path) {
                Ok(()) => {
                    let owner_path = acquire_lock_owner_path(&path);
                    match OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .open(&owner_path)
                    {
                        Ok(mut file) => {
                            // Propagate writeln errors; a partially-written
                            // owner file means Drop can never prove ownership
                            // later, so the lock directory would leak forever.
                            if let Err(error) = writeln!(file, "{token}") {
                                let _ = fs::remove_dir_all(&path);
                                return Err(error).with_context(|| {
                                    format!(
                                        "failed to write acquire lock owner at {}",
                                        owner_path.display()
                                    )
                                });
                            }
                            if let Err(error) = file.sync_all() {
                                let _ = fs::remove_dir_all(&path);
                                return Err(error).with_context(|| {
                                    format!(
                                        "failed to flush acquire lock owner at {}",
                                        owner_path.display()
                                    )
                                });
                            }
                            return Ok(Self { path, token });
                        }
                        Err(error) => {
                            let _ = fs::remove_dir_all(&path);
                            return Err(error).with_context(|| {
                                format!(
                                    "failed to write acquire lock owner at {}",
                                    owner_path.display()
                                )
                            });
                        }
                    }
                }
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                    if remove_stale_acquire_lock_if_owner_dead(&path)? {
                        continue;
                    }
                    thread::sleep(Duration::from_millis(ACQUIRE_LOCK_POLL_INTERVAL_MS));
                }
                Err(error) => {
                    return Err(error).with_context(|| {
                        format!("failed to create acquire lock at {}", path.display())
                    });
                }
            }
        }

        let holder = read_acquire_lock_owner(&path).map_or_else(
            |_| "<unreadable>".to_owned(),
            |content| content.trim().to_owned(),
        );
        bail!(
            "another xtask invocation (PID {holder}) holds the cache lock for {}; if that process is no longer running, remove {} and rerun",
            checkout_path.display(),
            path.display()
        );
    }
}

/// Returns the per-pin acquire-lock directory path `<cache_parent>/.lock-<key>`.
fn acquire_lock_path(cache_parent: &Path, pin: &SourcePin) -> PathBuf {
    cache_parent.join(format!(".lock-{}", cache_key(pin)))
}

/// Returns the owner-token file path inside an acquire-lock directory.
fn acquire_lock_owner_path(lock_path: &Path) -> PathBuf {
    lock_path.join(ACQUIRE_LOCK_OWNER_FILE)
}

/// Builds the owner token `<pid>:<nanos>` written into the lock directory so
/// `Drop` can verify ownership before deleting it.
fn acquire_lock_token() -> String {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    format!("{}:{nonce}", std::process::id())
}

/// Best-effort removal of the acquire-lock directory (or legacy lock file) for
/// `pin`, used by callers that have already verified the prior holder is gone.
fn remove_stale_acquire_lock(cache_parent: &Path, pin: &SourcePin) -> anyhow::Result<()> {
    let path = acquire_lock_path(cache_parent, pin);
    remove_acquire_lock_path(&path)
}

/// Removes the lock at `path`, tolerating either the new directory form or
/// the legacy single-file form left by older binaries.
fn remove_acquire_lock_path(path: &Path) -> anyhow::Result<()> {
    match fs::remove_dir_all(path) {
        Ok(()) => return Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
        Err(error) if error.kind() == ErrorKind::NotADirectory => {}
        Err(error) => {
            return Err(error)
                .with_context(|| format!("failed to remove stale lock {}", path.display()));
        }
    }

    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => {
            Err(error).with_context(|| format!("failed to remove stale lock {}", path.display()))
        }
    }
}

/// Removes the lock if the recorded owner PID is no longer alive (or if the
/// lock is a legacy file). Returns whether the slot was reclaimed.
fn remove_stale_acquire_lock_if_owner_dead(path: &Path) -> anyhow::Result<bool> {
    if path.is_file() {
        remove_acquire_lock_path(path)?;
        return Ok(true);
    }

    let Ok(owner) = read_acquire_lock_owner(path) else {
        return Ok(false);
    };
    let Some(pid) = owner
        .split_once(':')
        .and_then(|(pid, _)| pid.parse::<u32>().ok())
    else {
        return Ok(false);
    };

    if acquire_lock_owner_is_alive(pid) {
        return Ok(false);
    }

    remove_acquire_lock_path(path)?;
    Ok(true)
}

/// Reads the owner token from the lock directory's `owner` file, or from the
/// legacy single-file form when `path` is a regular file.
fn read_acquire_lock_owner(path: &Path) -> anyhow::Result<String> {
    let owner_path = if path.is_dir() {
        acquire_lock_owner_path(path)
    } else {
        path.to_path_buf()
    };
    fs::read_to_string(&owner_path)
        .with_context(|| format!("failed to read acquire lock owner {}", owner_path.display()))
}

/// Best-effort liveness probe for a PID. Errors are treated as "alive" so
/// stale-lock cleanup never races against a live peer; the recorded owner is
/// only reclaimed when we can prove the process is gone.
///
/// Note: `kill -0` cannot distinguish `ESRCH` from `EPERM`, so a process
/// owned by a different user looks "dead" here. Acceptable: codegen runs in
/// single-user CI/dev, and the owner-token comparison in [`Drop`] still
/// prevents deleting a lock we did not own.
#[cfg(unix)]
fn acquire_lock_owner_is_alive(pid: u32) -> bool {
    if pid == std::process::id() {
        return true;
    }

    Command::new("kill")
        .arg("-0")
        .arg(pid.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_or(true, |status| status.success())
}

/// Windows liveness probe via `tasklist`; same "errors = alive" contract as
/// the Unix variant so stale-lock recovery never races against a live peer.
#[cfg(windows)]
fn acquire_lock_owner_is_alive(pid: u32) -> bool {
    if pid == std::process::id() {
        return true;
    }

    let filter = format!("PID eq {pid}");
    let Ok(output) = Command::new("tasklist")
        .args(["/FI", &filter, "/NH"])
        .output()
    else {
        return true;
    };
    if !output.status.success() {
        return true;
    }

    let pid_text = pid.to_string();
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .any(|line| line.split_whitespace().any(|part| part == pid_text))
}

/// Fallback liveness probe for platforms without a known process-listing API:
/// conservatively assumes the owner is still alive so we never delete a lock
/// we cannot prove is stale.
#[cfg(not(any(unix, windows)))]
fn acquire_lock_owner_is_alive(_pid: u32) -> bool {
    true
}

impl Drop for AcquireLock {
    /// Releases the lock directory only when its owner token still matches the
    /// one this guard wrote, so a different process that has already taken
    /// over the lock is not disturbed.
    fn drop(&mut self) {
        let Ok(owner) = read_acquire_lock_owner(&self.path) else {
            return;
        };
        if owner.trim() == self.token {
            let _ = remove_acquire_lock_path(&self.path);
        }
    }
}

/// Removes every leftover temporary checkout directory belonging to `pin`.
fn remove_stale_temporary_checkouts(cache_parent: &Path, pin: &SourcePin) -> anyhow::Result<()> {
    let namespace = temporary_checkout_namespace_path(cache_parent, pin);
    match fs::remove_dir_all(&namespace) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error)
            .with_context(|| format!("failed to remove stale checkout {}", namespace.display())),
    }
}

/// Allocates a fresh temporary checkout path inside the pin's namespace,
/// retrying with monotonic counters when a previous path still exists.
fn unique_temporary_checkout_path(cache_parent: &Path, pin: &SourcePin) -> anyhow::Result<PathBuf> {
    let pid = std::process::id();
    let namespace = temporary_checkout_namespace_path(cache_parent, pin);
    fs::create_dir_all(&namespace)
        .with_context(|| format!("failed to create {}", namespace.display()))?;
    for counter in 0..MAX_TEMP_CHECKOUT_ATTEMPTS {
        let temporary_path = namespace.join(format!("{pid}.{counter}"));
        if !temporary_path.exists() {
            return Ok(temporary_path);
        }
    }

    bail!(
        "failed to find an unused temporary checkout path in {}",
        cache_parent.display()
    );
}

/// Returns the per-pin namespace directory `<cache_parent>/.tmp/<digest>`
/// under which temporary checkouts for `pin` are isolated.
fn temporary_checkout_namespace_path(cache_parent: &Path, pin: &SourcePin) -> PathBuf {
    cache_parent
        .join(TEMP_CHECKOUT_DIR)
        .join(temporary_checkout_namespace(pin))
}

/// Returns a SHA-256 hex digest of `tag\0sha`, used as the per-pin namespace
/// directory for temporary checkouts so different pins never collide.
fn temporary_checkout_namespace(pin: &SourcePin) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pin.tag().as_bytes());
    hasher.update([0]);
    hasher.update(pin.sha().as_bytes());
    encode_sha256_hex(&hasher.finalize())
}

/// Canonicalizes `path` and bails if the result escapes `root`.
fn canonicalize_within(root: &Path, path: &Path) -> anyhow::Result<CanonicalPath> {
    let canonical_path = fs::canonicalize(path)
        .with_context(|| format!("failed to canonicalize {}", path.display()))?;
    if !canonical_path.starts_with(root) {
        bail!(
            "path {} escapes root {}",
            canonical_path.display(),
            root.display()
        );
    }

    Ok(CanonicalPath(canonical_path))
}

/// Normalizes CR/CRLF line endings to LF in place. The fast path returns the
/// input unchanged when no `\r` is present.
fn normalize_lf(mut bytes: Vec<u8>) -> Vec<u8> {
    if !bytes.contains(&b'\r') {
        return bytes;
    }

    let mut read = 0;
    let mut write = 0;
    while read < bytes.len() {
        if bytes[read] == b'\r' {
            bytes[write] = b'\n';
            write += 1;
            read += 1;
            if bytes.get(read) == Some(&b'\n') {
                read += 1;
            }
        } else {
            bytes[write] = bytes[read];
            write += 1;
            read += 1;
        }
    }
    bytes.truncate(write);
    bytes
}

/// Reads a single pinned blob via `git cat-file blob` and applies the same LF
/// normalization as [`BatchCatFile::read_blob_lf_normalized`].
fn read_blob_lf_normalized(root: &Path, revision: &str, relative: &str) -> anyhow::Result<Vec<u8>> {
    Ok(normalize_lf(git_cat_file_blob(root, revision, relative)?))
}

/// Single-shot `git cat-file blob <revision>:<relative>` for one-off reads
/// (the discovery hot path uses [`BatchCatFile`] instead).
fn git_cat_file_blob(root: &Path, revision: &str, relative: &str) -> anyhow::Result<Vec<u8>> {
    let object = format!("{revision}:{relative}");
    let mut child = new_git_command()
        .arg("-C")
        .arg(root)
        .args(["cat-file", "blob"])
        .arg(&object)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to spawn git cat-file blob {object}"))?;
    let Some(stdout) = child.stdout.take() else {
        let _ = child.kill();
        let _ = child.wait();
        bail!("git cat-file blob {object} did not expose the piped stdout handle");
    };

    // Read at most MAX_BLOB_SIZE_BYTES + 1 so we can detect an over-cap blob
    // without buffering the entire payload into memory first.
    let mut bytes = Vec::new();
    if let Err(error) = stdout
        .take(MAX_BLOB_SIZE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
    {
        let _ = child.kill();
        let _ = child.wait();
        return Err(error)
            .with_context(|| format!("failed to read git cat-file blob {object} stdout"));
    }
    if bytes.len() > MAX_BLOB_SIZE_BYTES {
        let _ = child.kill();
        let _ = child.wait();
        bail!("git cat-file blob {object} returned more than {MAX_BLOB_SIZE_BYTES} bytes",);
    }
    let output = child
        .wait_with_output()
        .with_context(|| format!("failed to reap git cat-file blob {object}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("git cat-file blob {object} failed: {}", stderr.trim());
    }
    Ok(bytes)
}

/// Outcome of parsing a `git cat-file --batch` response header line, with all
/// borrows already consumed so the caller is free to mutate `self`.
enum BatchHeaderOutcome {
    /// Header announced a blob payload of `size` bytes.
    Blob { size: usize },
    /// Header announced a blob payload larger than [`MAX_BLOB_SIZE_BYTES`].
    OverCap { size: usize },
    /// Header announced a missing object (no payload follows).
    Missing,
}

/// Parses a `git cat-file --batch` response header into an owned outcome. The
/// returned value carries no references into `header_buffer` so the caller
/// can mutate other `BatchCatFile` fields without borrow-checker grief.
fn parse_batch_header(header_buffer: &[u8], object: &str) -> anyhow::Result<BatchHeaderOutcome> {
    let header_text = std::str::from_utf8(header_buffer).with_context(|| {
        format!("git cat-file --batch returned a non-UTF-8 header for {object}")
    })?;
    let header = header_text.trim_end_matches('\n');

    // Git emits one of two header forms per request:
    //   "<sha> blob <size>"  for a found blob
    //   "<object> missing"   when the object is unknown
    let mut parts = header.split(' ');
    let _first = parts.next();
    let Some(kind) = parts.next() else {
        bail!("git cat-file --batch returned malformed header {header:?} for {object}");
    };
    if kind == "missing" {
        if parts.next().is_some() {
            bail!(
                "git cat-file --batch returned trailing tokens after missing for {object}: {header:?}"
            );
        }
        return Ok(BatchHeaderOutcome::Missing);
    }
    if kind != "blob" {
        bail!("git cat-file --batch returned non-blob {kind:?} for {object}");
    }
    let Some(size_str) = parts.next() else {
        bail!("git cat-file --batch returned header without size for {object}: {header:?}");
    };
    if parts.next().is_some() {
        bail!("git cat-file --batch returned trailing tokens after size for {object}: {header:?}");
    }
    let size: usize = size_str.parse().with_context(|| {
        format!("git cat-file --batch returned invalid size in header {header:?} for {object}")
    })?;
    if size > MAX_BLOB_SIZE_BYTES {
        return Ok(BatchHeaderOutcome::OverCap { size });
    }
    Ok(BatchHeaderOutcome::Blob { size })
}

/// Long-running `git cat-file --batch` child process used to amortize fork
/// overhead when [`discover`] reads many pinned blobs in a single codegen run.
struct BatchCatFile {
    child: Option<Child>,
    stdin: Option<ChildStdin>,
    /// `Option` so [`BatchCatFile::poison`] and `Drop` can close the read end
    /// of git's stdout pipe before reaping, letting a child that is blocked
    /// writing an oversized payload return from its `write()` with `EPIPE`.
    stdout: Option<BufReader<ChildStdout>>,
    /// Reused buffer for the per-request `<revision>:<relative>` object spec.
    object_buffer: String,
    /// Reused buffer for the per-request `<sha> blob <size>` response header.
    header_buffer: Vec<u8>,
    /// When set, [`Drop`] kills the child instead of waiting on it: bailing
    /// mid-batch leaves git holding an unread payload and `wait()` would
    /// deadlock against the full stdout pipe.
    poisoned: bool,
}

impl BatchCatFile {
    /// Spawns a long-running `git cat-file --batch` child for the given root.
    ///
    /// Stderr is sent to `/dev/null` (or the Windows equivalent): the child's
    /// stderr buffer is bounded, and a never-drained pipe would deadlock the
    /// child as soon as git writes a warning. We do not want to interleave a
    /// drain thread with the request/response loop on stdout.
    fn new(root: &Path) -> anyhow::Result<Self> {
        let mut child = new_git_command()
            .arg("-C")
            .arg(root)
            .args(["cat-file", "--batch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to spawn git cat-file --batch")?;
        // If either handle take fails, we must not return early without
        // reaping the spawned child or it becomes a zombie.
        let Some(stdin) = child.stdin.take() else {
            let _ = child.kill();
            let _ = child.wait();
            bail!("git cat-file --batch did not expose the piped stdin handle");
        };
        let Some(stdout) = child.stdout.take() else {
            let _ = child.kill();
            let _ = child.wait();
            bail!("git cat-file --batch did not expose the piped stdout handle");
        };
        Ok(Self {
            child: Some(child),
            stdin: Some(stdin),
            stdout: Some(BufReader::new(stdout)),
            object_buffer: String::new(),
            header_buffer: Vec::new(),
            poisoned: false,
        })
    }

    /// Marks the batch as unrecoverable and closes the stdout read end so the
    /// child unblocks from any in-flight `write()`. Called before every
    /// `bail!` that aborts mid-protocol; [`Drop`] then kills + reaps the child
    /// instead of attempting an orderly shutdown.
    fn poison(&mut self) {
        self.poisoned = true;
        self.stdout.take();
        self.stdin.take();
    }

    /// Reads the raw blob bytes for `<revision>:<relative>` via the
    /// long-running git plumbing process.
    fn read_blob(&mut self, revision: &str, relative: &str) -> anyhow::Result<Vec<u8>> {
        // Build the object spec into the reused buffer before any I/O.
        self.object_buffer.clear();
        write!(self.object_buffer, "{revision}:{relative}").expect("writing to String cannot fail");

        // Send the request. Any write/flush failure desynchronizes the
        // protocol, so poison before returning.
        if let Err(error) = self.write_request() {
            self.poison();
            return Err(error);
        }

        // Read the header line with an explicit cap so a corrupted child
        // cannot grow `header_buffer` without bound, then parse it into an
        // owned outcome so subsequent `poison()` calls do not conflict with
        // borrows into `self.header_buffer`.
        if let Err(error) = self.read_header_bytes() {
            self.poison();
            return Err(error);
        }
        let outcome = match parse_batch_header(&self.header_buffer, &self.object_buffer) {
            Ok(outcome) => outcome,
            Err(error) => {
                self.poison();
                return Err(error);
            }
        };
        let size = match outcome {
            BatchHeaderOutcome::Missing => {
                // "missing" consumes no payload; protocol is still
                // synchronized so no poison is required.
                bail!("git cat-file --batch missing object {}", self.object_buffer);
            }
            BatchHeaderOutcome::OverCap { size } => {
                // We have decided to reject this payload without draining it,
                // so poison the child to short-circuit `Drop::wait()` instead
                // of deadlocking against a full stdout pipe.
                let cap = MAX_BLOB_SIZE_BYTES;
                let object = std::mem::take(&mut self.object_buffer);
                self.poison();
                bail!(
                    "git cat-file --batch returned a blob of {size} bytes for {object}, which exceeds the {cap}-byte cap",
                );
            }
            BatchHeaderOutcome::Blob { size } => size,
        };

        let stdout = self
            .stdout
            .as_mut()
            .expect("stdout retained while struct is alive");
        let mut bytes = vec![0_u8; size];
        if let Err(error) = stdout.read_exact(&mut bytes) {
            let object = std::mem::take(&mut self.object_buffer);
            self.poison();
            return Err(error).with_context(|| {
                format!("failed to read blob {object} from git cat-file --batch")
            });
        }

        // Each blob payload is followed by a single LF terminator.
        let mut trailing = [0_u8; 1];
        if let Err(error) = stdout.read_exact(&mut trailing) {
            let object = std::mem::take(&mut self.object_buffer);
            self.poison();
            return Err(error)
                .with_context(|| format!("failed to read trailing newline for {object}"));
        }
        if trailing[0] != b'\n' {
            let object = std::mem::take(&mut self.object_buffer);
            self.poison();
            bail!("git cat-file --batch missing trailing newline for {object}");
        }

        Ok(bytes)
    }

    /// Writes the current `object_buffer` to `stdin` and flushes; on success
    /// the child can begin emitting the response header.
    fn write_request(&mut self) -> anyhow::Result<()> {
        let stdin = self
            .stdin
            .as_mut()
            .expect("stdin retained while struct is alive");
        writeln!(stdin, "{}", self.object_buffer).with_context(|| {
            format!(
                "failed to write {} to git cat-file --batch stdin",
                self.object_buffer
            )
        })?;
        stdin
            .flush()
            .context("failed to flush git cat-file --batch stdin")
    }

    /// Reads the header line into `header_buffer`, refusing anything larger
    /// than [`MAX_BATCH_HEADER_BYTES`] so the reused buffer cannot grow
    /// without bound.
    fn read_header_bytes(&mut self) -> anyhow::Result<()> {
        self.header_buffer.clear();
        let stdout = self
            .stdout
            .as_mut()
            .expect("stdout retained while struct is alive");
        let bytes_read = std::io::BufRead::read_until(
            &mut stdout.by_ref().take(MAX_BATCH_HEADER_BYTES as u64 + 1),
            b'\n',
            &mut self.header_buffer,
        )
        .with_context(|| {
            format!(
                "failed to read git cat-file --batch header for {}",
                self.object_buffer
            )
        })?;
        if bytes_read == 0 {
            bail!(
                "git cat-file --batch closed before returning a header for {}",
                self.object_buffer
            );
        }
        // `read_until` may return `MAX_BATCH_HEADER_BYTES + 1` bytes when the
        // line ends with `\n` exactly at the cap, so length is the unambiguous
        // gate: anything past the cap rejects independently of whether the
        // child also emitted a newline.
        if self.header_buffer.len() > MAX_BATCH_HEADER_BYTES {
            bail!(
                "git cat-file --batch header exceeded {MAX_BATCH_HEADER_BYTES} bytes for {}",
                self.object_buffer
            );
        }
        if self.header_buffer.last() != Some(&b'\n') {
            bail!(
                "git cat-file --batch closed mid-header for {}",
                self.object_buffer
            );
        }
        Ok(())
    }

    /// Batched companion to [`read_blob_lf_normalized`] (single-shot): reads
    /// the pinned blob and folds CRLF to LF so the resulting bytes hash
    /// identically across both read paths.
    fn read_blob_lf_normalized(
        &mut self,
        revision: &str,
        relative: &str,
    ) -> anyhow::Result<Vec<u8>> {
        Ok(normalize_lf(self.read_blob(revision, relative)?))
    }
}

impl Drop for BatchCatFile {
    /// Closes both pipes before reaping. Closing stdout's read end unblocks a
    /// child that may be mid-`write()` after the batch was poisoned; closing
    /// stdin lets git exit cleanly on the happy path.
    fn drop(&mut self) {
        // Closing stdin lets git exit gracefully on the happy path; closing
        // stdout unblocks a child that is mid-write after a poison.
        self.stdin.take();
        self.stdout.take();
        if let Some(mut child) = self.child.take() {
            if self.poisoned {
                // Don't trust a poisoned child to exit on its own.
                let _ = child.kill();
            }
            let _ = child.wait();
        }
    }
}

/// Computes the lowercase SHA-256 hex of `bytes`. Used by
/// [`discovered_file_from_frame`] to digest the already LF-normalized buffer
/// without re-reading it from git.
fn sha256_hex_from_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    encode_sha256_hex(&hasher.finalize())
}

/// Lowercase hex-encodes a 32-byte SHA-256 digest using a table lookup, with
/// the output `String` pre-sized to [`SHA256_HEX_LENGTH`].
pub(super) fn encode_sha256_hex(digest: &[u8]) -> String {
    let mut output = String::with_capacity(SHA256_HEX_LENGTH);
    for &byte in digest {
        output.push(HEX_NIBBLES[(byte >> 4) as usize] as char);
        output.push(HEX_NIBBLES[(byte & 0x0f) as usize] as char);
    }
    output
}

/// Returns the identifier text when the pattern is a single identifier binding.
fn binding_name(pattern: &AnyJsBindingPattern) -> Option<TokenText> {
    let binding = pattern.as_any_js_binding()?;
    let identifier = binding.as_js_identifier_binding()?;
    Some(identifier.name_token().ok()?.token_text_trimmed())
}

/// Strips surrounding parentheses and `as`/`satisfies` wrappers to expose
/// the underlying `JsArrayExpression`, returning `None` for any other shape.
fn array_expression(expression: &AnyJsExpression) -> Option<biome_js_syntax::JsArrayExpression> {
    let mut current = expression.clone().omit_parentheses();
    loop {
        current = match current {
            AnyJsExpression::JsArrayExpression(array) => return Some(array),
            AnyJsExpression::TsAsExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsInstantiationExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsNonNullAssertionExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsSatisfiesExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                expression.expression().ok()?.omit_parentheses()
            }
            _ => return None,
        };
    }
}

/// Extracts a `[key, filename]` pair of string literals from a 2-element array.
fn tuple_elements(array: biome_js_syntax::JsArrayExpression) -> Option<(Text, Text)> {
    let mut elements = array.elements().into_iter();
    let first = tuple_element_text(elements.next()?)?;
    let second = tuple_element_text(elements.next()?)?;
    if elements.next().is_some() {
        return None;
    }

    Some((first, second))
}

/// Returns the string-literal text of one tuple element, or `None` if it is
/// missing, holed, spread, or not a string literal.
fn tuple_element_text(element: biome_rowan::SyntaxResult<AnyJsArrayElement>) -> Option<Text> {
    let element = element.ok()?;
    let AnyJsArrayElement::AnyJsExpression(expression) = element else {
        return None;
    };
    string_literal_text(&expression)
}

/// Returns the unquoted text of a direct string literal expression, or `None`
/// for any other shape (parenthesized, `as`/`satisfies`-wrapped, computed, …).
fn string_literal_text(expression: &AnyJsExpression) -> Option<Text> {
    let AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(
        string,
    )) = expression
    else {
        return None;
    };

    Some(Text::from(string.inner_string_text().ok()?))
}

/// Bails when a `libEntries` filename does not look like a `lib.*.d.ts` shape.
fn validate_lib_entry_filename(index: usize, filename: &str) -> anyhow::Result<()> {
    if filename.is_empty()
        || filename.contains('/')
        || filename.contains('\\')
        || Path::new(filename).is_absolute()
        || !filename.starts_with(DEFAULT_LIBRARY_FILE_PREFIX)
        || !filename.ends_with(DEFAULT_LIBRARY_FILE_SUFFIX)
        || Path::new(filename)
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        bail!(
            "libEntries filename at index {index} must be a default-library basename ending in .d.ts: {filename:?}"
        );
    }

    Ok(())
}

/// Reads the file's pinned blob via `batch`, parses its triple-slash references,
/// and returns a stack frame ready for the discovery DFS.
fn make_frame(
    source_file: TrackedSourceFile,
    root: &Path,
    revision: &str,
    libs: &LibEntries,
    batch: &mut BatchCatFile,
) -> anyhow::Result<Frame> {
    let bytes_lf = batch.read_blob_lf_normalized(revision, &source_file.repo_relative)?;
    let source = std::str::from_utf8(&bytes_lf).with_context(|| {
        format!(
            "{} is not valid UTF-8",
            source_file.path.as_path().display()
        )
    })?;
    let mut children_remaining =
        parse_triple_slash_references(source, source_file.path.as_path(), root, libs)?;
    children_remaining.reverse();

    Ok(Frame {
        source: source_file,
        bytes_lf,
        children_remaining,
    })
}

/// Returns every `<reference ... />` directive found in the file's leading
/// comments, in source order.
fn parse_triple_slash_references(
    source: &str,
    path: &Path,
    root: &Path,
    libs: &LibEntries,
) -> anyhow::Result<Vec<TripleSlashReference>> {
    let parsed = parse(source, JsFileSource::d_ts(), JsParserOptions::default());
    // Refuse to extract triple-slash references from a file that did not parse
    // cleanly: a missing leading-trivia token would silently produce an empty
    // reference list and let the collector run against incomplete coverage.
    if parsed.has_errors() {
        bail!(
            "{} produced parser diagnostics while extracting triple-slash references",
            path.display()
        );
    }
    let syntax = parsed.syntax();
    let Some(first_token) = syntax.first_token() else {
        return Ok(Vec::new());
    };

    first_token
        .leading_trivia()
        .pieces()
        .filter(|piece| piece.is_comments())
        .filter_map(|piece| {
            parse_triple_slash_reference(piece.text(), path, root, libs).transpose()
        })
        .collect()
}

/// Parses a single `///` comment as a TypeScript reference directive,
/// returning the resolved edge or `None` for unrelated comments.
fn parse_triple_slash_reference(
    comment: &str,
    path: &Path,
    root: &Path,
    libs: &LibEntries,
) -> anyhow::Result<Option<TripleSlashReference>> {
    let Some(directive) = reference_pragma_arguments(comment) else {
        return Ok(None);
    };

    // TypeScript's `processPragmasIntoFields` only suppresses other reference
    // attributes when the literal value is `"true"`. Any other value (including
    // `"false"`) falls through to the regular precedence chain, so we must
    // match that semantics rather than treating any presence as a suppressor.
    if matches!(
        directive_attribute(directive, REFERENCE_NO_DEFAULT_LIB_ATTRIBUTE)?,
        Some("true"),
    ) {
        Ok(None)
    } else if let Some(types) = directive_attribute(directive, REFERENCE_TYPES_ATTRIBUTE)? {
        bail!(
            "{} contains unsupported triple-slash types reference {types:?}",
            path.display()
        );
    } else if let Some(lib_reference) = directive_attribute(directive, REFERENCE_LIB_ATTRIBUTE)? {
        let lib_key = to_typescript_file_name_lowercase(lib_reference);
        if !libs.lib_map.contains_key(lib_key.as_str()) {
            bail!(
                "{} references unknown lib {lib_reference:?} under {}",
                path.display(),
                root.display()
            );
        }
        Ok(Some(TripleSlashReference::Lib(lib_key)))
    } else if let Some(path_reference) = directive_attribute(directive, REFERENCE_PATH_ATTRIBUTE)? {
        Ok(Some(TripleSlashReference::File(path_reference.to_owned())))
    } else {
        Ok(None)
    }
}

/// Returns the attribute slice of a `/// <reference ... />` comment when the
/// shape matches TypeScript's `tripleSlashXMLCommentStartRegEx`.
fn reference_pragma_arguments(comment: &str) -> Option<&str> {
    let directive = comment.trim_start().strip_prefix(TRIPLE_SLASH_PREFIX)?;
    let directive = directive.trim_start().strip_prefix('<')?;
    let end = directive.find("/>")?;
    let directive = &directive[..end];
    let name_end = directive
        .char_indices()
        .find_map(|(index, char)| char.is_ascii_whitespace().then_some(index))?;
    let name = &directive[..name_end];
    if !name.eq_ignore_ascii_case(REFERENCE_DIRECTIVE_PREFIX.trim_start_matches('<')) {
        return None;
    }

    Some(&directive[name_end..])
}

/// Returns the value of `attribute="..."` from a directive body using ASCII-
/// case-insensitive attribute-name matching with word boundaries.
///
/// `Ok(None)` means the attribute simply isn't present; `Err(_)` means a
/// matching attribute name was found but its quoted value was malformed (for
/// example, the closing quote ran past the directive body). TypeScript itself
/// reports `Invalid_reference_directive_syntax` in that case, so we surface
/// the parse error rather than silently treating it as "absent".
fn directive_attribute<'directive>(
    directive: &'directive str,
    attribute: &str,
) -> anyhow::Result<Option<&'directive str>> {
    let lower_directive = directive.to_ascii_lowercase_cow();
    let lower_attribute = attribute.to_ascii_lowercase_cow();
    let mut search_start = 0;
    while search_start < directive.len() {
        let Some(relative_index) = lower_directive[search_start..].find(lower_attribute.as_ref())
        else {
            return Ok(None);
        };
        let attribute_start = search_start + relative_index;
        let attribute_end = attribute_start + lower_attribute.len();

        if !is_attribute_boundary_before(directive, attribute_start)
            || !is_attribute_boundary_after(directive, attribute_end)
        {
            search_start = attribute_end;
            continue;
        }

        let after_name = directive[attribute_end..].trim_start();
        let Some(after_equals) = after_name.strip_prefix('=') else {
            search_start = attribute_end;
            continue;
        };
        let after_equals = after_equals.trim_start();
        let Some(quote) = after_equals.as_bytes().first().copied() else {
            bail!("triple-slash directive attribute {attribute:?} has no value in {directive:?}");
        };
        if quote != b'"' && quote != b'\'' {
            search_start = attribute_end;
            continue;
        }

        let value_start = 1;
        let Some(value_end) = after_equals[value_start..]
            .bytes()
            .position(|byte| byte == quote)
        else {
            bail!(
                "triple-slash directive attribute {attribute:?} has an unterminated quoted value in {directive:?}"
            );
        };
        return Ok(Some(&after_equals[value_start..value_start + value_end]));
    }

    Ok(None)
}

/// ASCII-lowercases `value` for TypeScript's case-insensitive lib-key lookup,
/// borrowing through `Cow` when the input is already lowercase.
fn to_typescript_file_name_lowercase(value: &str) -> String {
    value.to_ascii_lowercase_cow().into_owned()
}

/// True when the character preceding `index` is a word boundary (whitespace,
/// start of input, or the opening `<` of the directive).
fn is_attribute_boundary_before(text: &str, index: usize) -> bool {
    text[..index]
        .chars()
        .next_back()
        .is_none_or(|char| char.is_ascii_whitespace() || char == '<')
}

/// True when the character at `index` is a word boundary (whitespace, `=`,
/// `/`, or end of input).
fn is_attribute_boundary_after(text: &str, index: usize) -> bool {
    text[index..]
        .chars()
        .next()
        .is_none_or(|char| char.is_ascii_whitespace() || char == '=' || char == '/')
}

/// Resolves a `<reference>` directive to a tracked, validated source file
/// inside the pinned checkout.
fn resolve_child_reference(
    root: &Path,
    parent_dir: &Path,
    reference: &TripleSlashReference,
    libs: &LibEntries,
) -> anyhow::Result<TrackedSourceFile> {
    match reference {
        TripleSlashReference::Lib(reference) => {
            let filename = libs
                .lib_map
                .get(reference.as_str())
                .ok_or_else(|| anyhow!("unknown lib reference {reference:?}"))?;
            let relative = default_library_relative_path(filename.text());
            validate_reference_source_file(root, &relative).with_context(|| {
                format!("lib reference {reference:?} is not a tracked regular file")
            })
        }
        TripleSlashReference::File(reference) => {
            // `root` is already canonical (it comes from `AcquiredCheckout::root`, which
            // is built with `CanonicalPath::from_within`), so we skip the extra
            // `fs::canonicalize` per file reference.
            let relative =
                normalize_typescript_reference_relative_path(root, parent_dir, reference)
                    .with_context(|| format!("failed to resolve path reference {reference:?}"))?;
            validate_reference_source_file(root, &relative).with_context(|| {
                format!("path reference {reference:?} is not a tracked regular file")
            })
        }
    }
}

/// TypeScript-equivalent `normalizePath(combinePaths(...))`: folds `\` to `/`,
/// resolves `.`/`..` segments, rejects rooted paths and NUL bytes.
fn normalize_typescript_reference_relative_path(
    root: &Path,
    parent_dir: &Path,
    reference: &str,
) -> anyhow::Result<String> {
    // Only allocate when the reference actually uses Windows separators; the
    // common Unix-style path case keeps borrowing the caller's string slice.
    let reference_owned: String;
    let reference: &str = if reference.contains('\\') {
        reference_owned = reference.replace('\\', "/");
        &reference_owned
    } else {
        reference
    };
    if is_typescript_rooted_path(reference) {
        bail!(
            "path reference {reference:?} escapes root {}",
            root.display()
        );
    }

    let parent_relative = repo_relative_path(root, parent_dir)?;
    // Borrow segment slices into `parent_relative` and `reference` instead of
    // owning per-segment Strings; both source buffers outlive `parts`.
    let mut parts: Vec<&str> = Vec::new();
    if !parent_relative.is_empty() {
        parts.extend(parent_relative.split('/'));
    }

    for segment in reference.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    bail!(
                        "path reference {reference:?} escapes root {}",
                        root.display()
                    );
                }
            }
            segment => {
                if segment.contains('\0') {
                    bail!("path reference {reference:?} contains a NUL byte");
                }
                parts.push(segment);
            }
        }
    }

    if parts.is_empty() {
        bail!("path reference {reference:?} does not resolve to a file");
    }

    Ok(parts.join("/"))
}

/// True when `path` starts with `/` or a Windows drive letter prefix (`C:`).
fn is_typescript_rooted_path(path: &str) -> bool {
    let bytes = path.as_bytes();
    path.starts_with('/')
        || matches!(
            bytes.get(0..2),
            Some([drive, b':']) if drive.is_ascii_alphabetic()
        )
}

/// Validates a checkout-relative path layer by layer: lexical shape,
/// no symlink components, tracked regular file in git, then canonicalize
/// inside the checkout root.
fn validate_reference_source_file(
    root: &Path,
    relative: &str,
) -> anyhow::Result<TrackedSourceFile> {
    validate_forward_slash_relative_path(root, relative)?;
    validate_no_symlink_components(root, relative)?;
    validate_tracked_regular_file(root, relative)?;

    let candidate = forward_slash_relative_path(root, relative);
    let path = canonicalize_within(root, &candidate)?;

    let metadata = fs::metadata(path.as_path())
        .with_context(|| format!("failed to inspect {}", path.as_path().display()))?;
    if !metadata.is_file() {
        bail!(
            "resolved TypeScript source path {} is not a regular file",
            path.as_path().display()
        );
    }

    Ok(TrackedSourceFile {
        path,
        repo_relative: relative.to_owned(),
    })
}

/// Bails when `relative` is empty, rooted, contains `\`, `.`, `..`, or a
/// per-segment rooted prefix (Windows drive letter).
fn validate_forward_slash_relative_path(root: &Path, relative: &str) -> anyhow::Result<()> {
    if relative.is_empty() || is_typescript_rooted_path(relative) {
        bail!(
            "path reference {relative:?} escapes root {}",
            root.display()
        );
    }

    for segment in relative.split('/') {
        if segment.is_empty()
            || segment == "."
            || segment == ".."
            || segment.contains('\\')
            || is_typescript_rooted_path(segment)
        {
            bail!("path reference {relative:?} is not a normalized checkout-relative path");
        }
    }

    Ok(())
}

/// Walks each path segment under `root` and bails when any intermediate
/// component is a symlink, so canonicalization cannot escape the checkout.
fn validate_no_symlink_components(root: &Path, relative: &str) -> anyhow::Result<()> {
    let mut path = root.to_path_buf();
    for segment in relative.split('/') {
        path.push(segment);
        let metadata = fs::symlink_metadata(&path)
            .with_context(|| format!("failed to inspect {}", path.display()))?;
        if metadata.file_type().is_symlink() {
            bail!(
                "resolved TypeScript source path {} contains symlink component {}",
                forward_slash_relative_path(root, relative).display(),
                path.display()
            );
        }
    }

    Ok(())
}

/// Bails unless `git ls-files --stage --error-unmatch` reports `relative` as a
/// tracked regular file with mode 100644 or 100755.
fn validate_tracked_regular_file(root: &Path, relative: &str) -> anyhow::Result<()> {
    let literal_pathspec = format!(":(literal){relative}");
    let mut ls_files = new_git_command();
    ls_files
        .arg("-C")
        .arg(root)
        .args(["ls-files", "--stage", "-z", "--error-unmatch", "--"])
        .arg(literal_pathspec);
    let stdout = command_stdout(&mut ls_files, "git ls-files --stage --error-unmatch")?;
    let Some(entry) = stdout.strip_suffix('\0') else {
        bail!(
            "git did not report a tracked entry for {}",
            forward_slash_relative_path(root, relative).display()
        );
    };
    let Some((metadata, reported_path)) = entry.split_once('\t') else {
        bail!("git reported malformed ls-files entry for {relative:?}: {entry:?}");
    };
    let Some(mode) = metadata.get(..GIT_FILE_MODE_LENGTH) else {
        bail!("git reported truncated ls-files mode for {relative:?}: {entry:?}");
    };
    if mode != GIT_REGULAR_FILE_MODE && mode != GIT_EXECUTABLE_FILE_MODE {
        bail!("git reports {relative:?} as non-regular mode {mode:?}");
    }
    if reported_path != relative {
        bail!("git reported tracked path {reported_path:?}, expected {relative:?}");
    }

    Ok(())
}

/// Joins a forward-slash `relative` path onto `root`, segment by segment.
fn forward_slash_relative_path(root: &Path, relative: &str) -> PathBuf {
    let mut path = root.to_path_buf();
    for segment in relative.split('/') {
        path.push(segment);
    }
    path
}

/// Materializes a discovery stack frame into the per-file output record,
/// hashing the cached LF-normalized bytes once.
fn discovered_file_from_frame(frame: Frame) -> DiscoveredFile {
    let sha256_hex = sha256_hex_from_bytes(&frame.bytes_lf);

    DiscoveredFile {
        path: frame.source.path,
        repo_relative: frame.source.repo_relative,
        bytes_lf: frame.bytes_lf,
        sha256_hex,
    }
}

/// Returns `path` re-expressed relative to `root` and forward-slashed.
fn repo_relative_path(root: &Path, path: &Path) -> anyhow::Result<String> {
    let relative = path
        .strip_prefix(root)
        .with_context(|| format!("{} is not under {}", path.display(), root.display()))?;
    forward_slash_path(relative)
}

/// Joins a path's normal components with `/`, bailing on absolute prefixes
/// or non-UTF-8 components.
fn forward_slash_path(path: &Path) -> anyhow::Result<String> {
    path.components()
        .try_fold(String::new(), |mut acc, component| {
            let Component::Normal(part) = component else {
                bail!("path {} contains a non-normal component", path.display());
            };
            let part = part
                .to_str()
                .ok_or_else(|| anyhow!("path {} is not valid UTF-8", path.display()))?;
            if !acc.is_empty() {
                acc.push('/');
            }
            acc.push_str(part);
            Ok(acc)
        })
}

/// Builds the repo-relative path of a default library file, joining the
/// `lib/` prefix used by every TypeScript declaration in `libEntries`.
fn default_library_relative_path(filename: &str) -> String {
    format!("{DEFAULT_LIBRARY_RELATIVE_DIR}/{filename}")
}

/// TypeScript-equivalent default library sort key: base files first, then
/// `libEntries` order, then files outside the default library tree.
fn default_library_priority(path: &Path, default_library_path: &Path, libs: &[Text]) -> usize {
    let outside_priority = libs.len() + OUTSIDE_DEFAULT_LIBRARY_PRIORITY_OFFSET;
    let Ok(relative) = path.strip_prefix(default_library_path) else {
        return outside_priority;
    };

    let Some(filename) = relative.file_name().and_then(|filename| filename.to_str()) else {
        return outside_priority;
    };

    if BASE_DEFAULT_LIBRARY_FILES.contains(&filename) {
        return BASE_DEFAULT_LIBRARY_PRIORITY;
    }

    let Some(stripped) = filename.strip_prefix(DEFAULT_LIBRARY_FILE_PREFIX) else {
        return outside_priority;
    };
    let Some(stripped) = stripped.strip_suffix(DEFAULT_LIBRARY_FILE_SUFFIX) else {
        return outside_priority;
    };

    libs.iter()
        .position(|lib| lib.text() == stripped)
        .map_or(outside_priority, |index| {
            index + DEFAULT_LIBRARY_PRIORITY_OFFSET
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_library_priority_base_files_only_win_inside_default_library_dir() {
        let libs = vec![Text::from("es6")];
        let default_library_path = Path::new("checkout").join(DEFAULT_LIBRARY_RELATIVE_DIR);
        let outside_priority = libs.len() + OUTSIDE_DEFAULT_LIBRARY_PRIORITY_OFFSET;

        for filename in BASE_DEFAULT_LIBRARY_FILES {
            assert_eq!(
                default_library_priority(
                    &default_library_path.join(filename),
                    &default_library_path,
                    &libs,
                ),
                BASE_DEFAULT_LIBRARY_PRIORITY
            );
            assert_eq!(
                default_library_priority(
                    &Path::new("outside").join(filename),
                    &default_library_path,
                    &libs,
                ),
                outside_priority
            );
        }
    }
}
