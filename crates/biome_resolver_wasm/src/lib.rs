#![deny(clippy::use_self)]

mod utils;

use std::sync::Arc;

use biome_deserialize::json::deserialize_from_json_str;
use biome_json_parser::JsonParserOptions;
use biome_package::{PackageJson, TsConfigJson};
use biome_resolver::{PathInfo, ResolveError, ResolveOptions, ResolverFsProxy};
use camino::{Utf8Path, Utf8PathBuf};
use js_sys::{Function, JsString, Object, Reflect};
use wasm_bindgen::prelude::*;

use crate::utils::{into_error, set_panic_hook};

#[wasm_bindgen(start)]
pub fn main() {
    set_panic_hook();
}

// #region ResolveErrorKind

// The reason why we keep a different enum here is because we want to break the implementation and documentation
// we have internally with the ones shipped to the users. The documentation we have here must be curated, and have
// an explanation in the readme of `@biomejs/resolver`.

/// Identifies the reason a resolution attempt failed.
///
/// Returned as the `errorKind` field of a failed `resolve()` result alongside
/// the human-readable `error` string. Use `errorKind` for programmatic
/// branching and `error` for display or logging.
///
/// @example
/// ```ts
/// const result = resolver.resolve("./utils.js", "/project/src");
/// if (result.errorKind === ResolveErrorKind.ModuleNotFound) {
///   // file does not exist or extension is missing from options
/// }
/// ```
#[wasm_bindgen]
pub enum ResolveErrorKind {
    /// The specifier could not be found anywhere the resolver looked.
    ///
    /// Common causes:
    /// - The file or package does not exist at the given path.
    /// - The package is not installed (`node_modules` is missing or stale).
    /// - The required extension is not listed in the `extensions` option.
    /// - The `conditionNames` option does not match any condition in the
    ///   package's `exports` map.
    /// - `baseDir` is not an absolute path to a directory.
    ModuleNotFound,

    /// The specifier resolved to a directory but no index file was found inside it.
    ///
    /// Fix by providing both `defaultFiles` and `extensions` in the resolver
    /// options. For example, `defaultFiles: ["index"]` with
    /// `extensions: ["ts", "js"]` will try `index.ts` then `index.js`.
    DirectoryWithoutIndex,

    /// The specifier names a Node.js built-in module such as `node:fs` or
    /// `node:path`.
    ///
    /// This is only returned when `resolveNodeBuiltins: true` is set. It is
    /// not a failure — it signals that the import refers to the runtime itself
    /// rather than a file on disk. Without that option, built-ins produce
    /// `ModuleNotFound` instead.
    NodeBuiltIn,

    /// No `package.json` was found walking up from `baseDir`.
    ///
    /// Confirm that `baseDir` is inside a directory tree that contains a
    /// `package.json`. This error typically means `baseDir` points outside the
    /// project root or to a temporary directory.
    ManifestNotFound,

    /// A `package.json` or `tsconfig.json` was found but could not be parsed.
    ///
    /// The file likely contains invalid JSON. Validate it with a JSON linter.
    ErrorLoadingManifest,

    /// A symlink in the resolution chain points to a target that does not exist.
    ///
    /// This usually means a broken symlink in `node_modules` left behind by an
    /// interrupted package install. Re-running the package manager's install
    /// command normally fixes it.
    BrokenSymlink,

    /// The matched condition in a `package.json` `exports` or `imports` map
    /// points to an invalid target.
    ///
    /// A valid target must be a string starting with `./`, an array of
    /// fallbacks, a conditions object, or `null`. Any other value is rejected.
    /// This is a bug in the package's `package.json`. If you control the
    /// package, fix the manifest; otherwise check for a newer version.
    InvalidExportsTarget,

    /// The specifier contains characters that are not valid in a package name.
    ///
    /// Check the specifier for typos such as uppercase letters in a scoped
    /// package name or a path segment that begins with `.`.
    InvalidPackageName,
}

impl From<ResolveError> for ResolveErrorKind {
    fn from(err: ResolveError) -> Self {
        match err {
            ResolveError::NotFound => Self::ModuleNotFound,
            ResolveError::DirectoryWithoutDefault => Self::DirectoryWithoutIndex,
            ResolveError::NodeBuiltIn => Self::NodeBuiltIn,
            ResolveError::ManifestNotFound => Self::ManifestNotFound,
            ResolveError::ErrorLoadingManifest => Self::ErrorLoadingManifest,
            ResolveError::BrokenSymlink => Self::BrokenSymlink,
            ResolveError::InvalidMappingTarget => Self::InvalidExportsTarget,
            ResolveError::InvalidPackageSpecifier => Self::InvalidPackageName,
        }
    }
}

// #endregion

// #region JsFileSystem

/// A filesystem bridge backed by two JavaScript callbacks.
///
/// This is the bridge between the Biome resolver and the host JavaScript
/// environment. Two synchronous callbacks must be provided:
///
/// - `pathInfo(path: string): "file" | "directory" | { symlink: string } | null`
///   Returns the kind of the filesystem entry at `path` **without** following
///   symlinks. For symlinks, the returned object must contain `symlink` set to
///   the fully canonicalized real path (i.e. the result of `realpathSync`).
///   Returns `null` if the path does not exist or is not accessible.
///
/// - `readFileUtf8(path: string): string | null`
///   Returns the UTF-8 content of the file at `path`, or `null` if the file
///   does not exist, is not accessible, or is not valid UTF-8.
///
/// The Node.js implementation uses `lstatSync` + `realpathSync` for
/// `pathInfo`, and `readFileSync(path, "utf8")` for `readFileUtf8`.
#[wasm_bindgen]
pub struct JsFileSystem {
    path_info_fn: Function,
    read_file_utf8_fn: Function,
}

#[wasm_bindgen]
impl JsFileSystem {
    /// Creates a new `JsFileSystem` from two JavaScript callback functions.
    ///
    /// # Arguments
    ///
    /// * `path_info_fn` - `(path: string) => "file" | "directory" | { symlink: string } | null`
    /// * `read_file_utf8_fn` - `(path: string) => string | null`
    #[wasm_bindgen(constructor)]
    pub fn new(path_info_fn: Function, read_file_utf8_fn: Function) -> Self {
        Self {
            path_info_fn,
            read_file_utf8_fn,
        }
    }
}

impl JsFileSystem {
    /// Calls the `readFileUtf8` JS callback and returns the result.
    fn read_file_utf8(&self, path: &Utf8Path) -> Result<String, ()> {
        let result = self
            .read_file_utf8_fn
            .call1(&JsValue::null(), &JsValue::from_str(path.as_str()))
            .map_err(|_| ())?;

        if result.is_null() || result.is_undefined() {
            return Err(());
        }

        result.as_string().ok_or(())
    }
}

/// Implements `ResolverFsProxy` directly on `JsFileSystem`.
///
/// We intentionally do NOT implement the `FileSystem` trait — it requires
/// `Send + Sync`, which `js_sys::Function` cannot satisfy, and exposes many
/// methods irrelevant to resolution. Implementing `ResolverFsProxy` directly
/// keeps the surface minimal and correct.
impl ResolverFsProxy for JsFileSystem {
    fn path_info(&self, path: &Utf8Path) -> Result<PathInfo, ResolveError> {
        let result = self
            .path_info_fn
            .call1(&JsValue::null(), &JsValue::from_str(path.as_str()))
            .map_err(|_| ResolveError::NotFound)?;

        if result.is_null() || result.is_undefined() {
            return Err(ResolveError::NotFound);
        }

        // String return: "file" or "directory"
        if let Some(s) = result.as_string() {
            return match s.as_str() {
                "file" => Ok(PathInfo::File),
                "directory" => Ok(PathInfo::Directory),
                _ => Err(ResolveError::NotFound),
            };
        }

        // Object return: { symlink: string } — fully canonicalized target
        if result.is_object() {
            let symlink_key = JsValue::from_str("symlink");
            let target = Reflect::get(&result, &symlink_key)
                .ok()
                .and_then(|v| v.as_string())
                .ok_or(ResolveError::BrokenSymlink)?;

            return Ok(PathInfo::Symlink {
                canonicalized_target: Utf8PathBuf::from(target),
            });
        }

        Err(ResolveError::NotFound)
    }

    fn find_package_json(
        &self,
        search_dir: &Utf8Path,
    ) -> Result<(Utf8PathBuf, PackageJson), ResolveError> {
        // Walk upward from search_dir, reading package.json at each level.
        // All walking and parsing logic is in Rust; only the file read crosses
        // the JS boundary.
        let mut dir = search_dir.to_path_buf();
        loop {
            let candidate = dir.join("package.json");
            if let Ok(content) = self.read_file_utf8(&candidate) {
                if let Some(manifest) = parse_package_json(&content) {
                    return Ok((dir, manifest));
                }
            }
            match dir.parent() {
                Some(parent) => dir = parent.to_path_buf(),
                None => return Err(ResolveError::ManifestNotFound),
            }
        }
    }

    fn read_package_json_in_directory(
        &self,
        dir_path: &Utf8Path,
    ) -> Result<PackageJson, ResolveError> {
        let path = dir_path.join("package.json");
        let content = self
            .read_file_utf8(&path)
            .map_err(|_| ResolveError::ErrorLoadingManifest)?;
        parse_package_json(&content).ok_or(ResolveError::ErrorLoadingManifest)
    }

    fn read_tsconfig_json(&self, path: &Utf8Path) -> Result<TsConfigJson, ResolveError> {
        let content = self
            .read_file_utf8(path)
            .map_err(|_| ResolveError::ErrorLoadingManifest)?;
        parse_tsconfig_json(path, &content).ok_or(ResolveError::ErrorLoadingManifest)
    }
}

// #endregion

// #region MemoryFileSystem

/// An in-memory filesystem for use in browser environments and tests.
///
/// Populate it with `insert()` before calling `Resolver.withMemoryFileSystem()`.
#[wasm_bindgen]
pub struct MemoryFileSystem {
    inner: Arc<biome_fs::MemoryFileSystem>,
}

impl Default for MemoryFileSystem {
    fn default() -> Self {
        Self {
            inner: Arc::new(biome_fs::MemoryFileSystem::default()),
        }
    }
}

#[wasm_bindgen]
impl MemoryFileSystem {
    /// Creates a new empty in-memory filesystem.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a file at `path` with the given byte content.
    pub fn insert(&self, path: &str, data: &[u8]) {
        self.inner.insert(Utf8PathBuf::from(path), data);
    }

    /// Removes the file at `path`.
    pub fn remove(&self, path: &str) {
        self.inner.remove(Utf8Path::new(path));
    }
}

// #endregion

// #region WasmResolveOptions

/// Resolver options passed as a plain JavaScript object.
///
/// All fields are optional. Unset fields use sensible defaults consistent
/// with the Node.js module resolution algorithm.
#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
struct WasmResolveOptions {
    /// Condition names to accept in `exports` / `imports` maps.
    ///
    /// Example: `["node", "import"]` for ESM, `["node", "require"]` for CJS.
    condition_names: Vec<String>,

    /// File extensions to try when resolving bare paths without an extension.
    ///
    /// Example: `[".js", ".ts", ".json"]`
    extensions: Vec<String>,

    /// Extension aliases: map an extension to one or more fallback extensions.
    ///
    /// Example: `[{ extension: ".js", aliases: [".ts", ".js"] }]`
    extension_aliases: Vec<WasmExtensionAlias>,

    /// Index file names to look for when resolving a directory.
    ///
    /// Defaults to `["index"]` when not set.
    default_files: Vec<String>,

    /// When `true`, Node.js built-in modules (e.g. `node:fs`) resolve to a
    /// `NodeBuiltIn` error instead of attempting further resolution.
    resolve_node_builtins: bool,

    /// When `true`, resolve TypeScript declaration files (`.d.ts`) instead of
    /// source files.
    resolve_types: bool,
}

#[derive(serde::Deserialize)]
struct WasmExtensionAlias {
    extension: String,
    aliases: Vec<String>,
}

/// Calls `f` with a `ResolveOptions<'_>` built from `opts`.
///
/// `ResolveOptions<'a>` requires `&'a [&'a str]` slices, but
/// `WasmResolveOptions` stores `Vec<String>`. We build the intermediate
/// `Vec<&str>` vecs on the stack here and pass borrows into `f`, ensuring
/// all lifetimes are consistent without self-referential structs.
fn with_resolve_options<F, R>(opts: &WasmResolveOptions, f: F) -> R
where
    F: FnOnce(ResolveOptions<'_>) -> R,
{
    let condition_names: Vec<&str> = opts.condition_names.iter().map(|s| s.as_str()).collect();
    let extensions: Vec<&str> = opts.extensions.iter().map(|s| s.as_str()).collect();
    let default_files: Vec<&str> = opts.default_files.iter().map(|s| s.as_str()).collect();

    // Build the inner alias vecs first so they have stable addresses before
    // we take slices of them.
    let alias_vecs: Vec<Vec<&str>> = opts
        .extension_aliases
        .iter()
        .map(|ea| ea.aliases.iter().map(|s| s.as_str()).collect())
        .collect();

    let extension_aliases: Vec<(&str, &[&str])> = opts
        .extension_aliases
        .iter()
        .enumerate()
        .map(|(i, ea)| (ea.extension.as_str(), alias_vecs[i].as_slice()))
        .collect();

    let mut options = ResolveOptions::new()
        .with_condition_names(condition_names.as_slice())
        .with_extensions(extensions.as_slice())
        .with_extension_aliases(extension_aliases.as_slice())
        .with_default_files(default_files.as_slice());

    if opts.resolve_node_builtins {
        options = options.with_resolve_node_builtins();
    }
    if opts.resolve_types {
        options = options.with_resolve_types();
    }

    f(options)
}

// #endregion

// #region Resolver

/// The filesystem backend used by the resolver.
enum ResolverFs {
    Js(JsFileSystem),
    Memory(Arc<biome_fs::MemoryFileSystem>),
}

/// A module resolver that can use either a JS filesystem bridge or an
/// in-memory filesystem.
///
/// Create with `Resolver.withJsFileSystem()` (for Node.js) or
/// `Resolver.withMemoryFileSystem()` (for browsers and tests).
#[wasm_bindgen]
pub struct Resolver {
    fs: ResolverFs,
    options: WasmResolveOptions,
}

#[wasm_bindgen]
impl Resolver {
    /// Creates a resolver backed by the provided JavaScript filesystem bridge.
    ///
    /// Use this in Node.js environments — pass a `JsFileSystem` constructed
    /// with callbacks that delegate to `lstatSync`, `realpathSync`, and
    /// `readFileSync` from `node:fs`.
    ///
    /// `options` is an optional plain JavaScript object with resolver options.
    #[wasm_bindgen(js_name = "withJsFileSystem")]
    pub fn with_js_fs(fs: JsFileSystem, options: JsValue) -> Result<Resolver, js_sys::Error> {
        let options: WasmResolveOptions = if options.is_null() || options.is_undefined() {
            WasmResolveOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options).map_err(into_error)?
        };
        Ok(Self {
            fs: ResolverFs::Js(fs),
            options,
        })
    }

    /// Creates a resolver backed by the provided in-memory filesystem.
    ///
    /// Use this in browser environments or tests. Populate the
    /// `MemoryFileSystem` with the files the resolver needs to access before
    /// calling this.
    ///
    /// `options` is an optional plain JavaScript object with resolver options.
    #[wasm_bindgen(js_name = "withMemoryFileSystem")]
    pub fn with_memory_fs(
        fs: &MemoryFileSystem,
        options: JsValue,
    ) -> Result<Resolver, js_sys::Error> {
        let options: WasmResolveOptions = if options.is_null() || options.is_undefined() {
            WasmResolveOptions::default()
        } else {
            serde_wasm_bindgen::from_value(options).map_err(into_error)?
        };
        Ok(Self {
            fs: ResolverFs::Memory(Arc::clone(&fs.inner)),
            options,
        })
    }

    /// Resolves `specifier` starting from `base_dir`.
    ///
    /// `base_dir` must be an absolute path to a **directory** (not a file).
    /// For example, pass `path.dirname(import.meta.url)` or `__dirname`.
    ///
    /// Returns a plain JavaScript object:
    /// - On success: `{ path: string }` — the resolved absolute path.
    /// - On failure: `{ error: string }` — a description of why resolution
    ///   failed.
    pub fn resolve(&self, specifier: &str, base_dir: &str) -> JsValue {
        let base_dir = Utf8Path::new(base_dir);

        let result = with_resolve_options(&self.options, |resolve_options| match &self.fs {
            ResolverFs::Js(js_fs) => {
                biome_resolver::resolve(specifier, base_dir, js_fs, &resolve_options)
            }
            ResolverFs::Memory(mem_fs) => {
                biome_resolver::resolve(specifier, base_dir, mem_fs.as_ref(), &resolve_options)
            }
        });

        match result {
            Ok(path) => {
                let obj = Object::new();
                Reflect::set(
                    &obj,
                    &JsString::from("path"),
                    &JsValue::from_str(path.as_str()),
                )
                .unwrap_or_default();
                obj.into()
            }
            Err(err) => {
                let obj = Object::new();
                Reflect::set(
                    &obj,
                    &JsString::from("error"),
                    &JsValue::from_str(&err.to_string()),
                )
                .unwrap_or_default();
                Reflect::set(
                    &obj,
                    &JsString::from("errorKind"),
                    &JsValue::from(ResolveErrorKind::from(err) as u32),
                )
                .unwrap_or_default();
                obj.into()
            }
        }
    }
}

// #endregion

// #region JSON parsing helpers

fn parse_package_json(content: &str) -> Option<PackageJson> {
    deserialize_from_json_str::<PackageJson>(content, JsonParserOptions::default(), "")
        .consume()
        .0
}

fn parse_tsconfig_json(path: &Utf8Path, content: &str) -> Option<TsConfigJson> {
    let (tsconfig, _errors) = TsConfigJson::parse(path, content);
    Some(tsconfig)
}

// #endregion
