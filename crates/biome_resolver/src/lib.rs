#![deny(clippy::use_self)]

mod errors;
mod node_builtins;
mod resolver_fs_proxy;

use std::{ops::Deref, sync::Arc};

use biome_fs::normalize_path;
use biome_json_value::{JsonObject, JsonValue};
use biome_package::{PackageJson, TsConfigJson};
use camino::{Utf8Path, Utf8PathBuf};

pub use errors::*;
pub use node_builtins::is_builtin_node_module;
pub use resolver_fs_proxy::*;

/// Resolves the given `specifier` from the given `base_dir`.
///
/// The `base_dir` is used for resolving relative specifiers, such as
/// `"./dep.ts"`, but also for automatic discovery of relevant `package.json`
/// and `tsconfig.json` files.
///
/// For reference, here is an overview of the Node.js resolution algorithm:
///   https://nodejs.org/api/modules.html#all-together
///
/// A more detailed version of the spec can be found here:
///   https://nodejs.org/api/esm.html#resolution-algorithm
pub fn resolve(
    specifier: &str,
    base_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    let specifier = strip_query_and_fragment(specifier);

    if options.resolve_node_builtins && is_builtin_node_module(specifier) {
        return Err(ResolveError::NodeBuiltIn);
    }

    if specifier.starts_with('/') {
        return resolve_absolute_path(Utf8PathBuf::from(specifier), fs, options);
    }

    if specifier == "." || specifier.starts_with("./") || specifier.starts_with("../") {
        return resolve_relative_path(specifier, base_dir, fs, options);
    }

    if options.assume_relative {
        match resolve_relative_path(specifier, base_dir, fs, options) {
            Err(ResolveError::NotFound) => { /* continue below */ }
            result => return result,
        }
    }

    resolve_module(specifier, base_dir, fs, options)
}

/// Resolves the given absolute `path`.
///
/// Absolute paths are those starting with `/` or a Windows prefix/root.
fn resolve_absolute_path(
    path: Utf8PathBuf,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    let path = normalize_owned_absolute_path(path);

    let try_extensions_with_fallback_error = |error: ResolveError| {
        for extension in options.extensions {
            let path_with_extension = Utf8PathBuf::from(format!("{path}.{extension}"));
            match resolve_path_info(path_with_extension, fs) {
                Ok((ResolvedPathInfo::Directory, _)) => {
                    // Adding an extension yielded a directory? No, thanks.
                }
                Ok((ResolvedPathInfo::File, realpath)) => return Ok(realpath),
                Err(ResolveError::NotFound) => { /* continue */ }
                Err(error) => return Err(error),
            }
        }

        Err(error)
    };

    match resolve_path_info(path.clone(), fs) {
        Ok((ResolvedPathInfo::Directory, realpath)) => {
            resolve_directory(&realpath, fs, options).or_else(try_extensions_with_fallback_error)
        }
        Ok((ResolvedPathInfo::File, realpath)) => Ok(realpath),
        Err(ResolveError::NotFound) => try_extensions_with_fallback_error(ResolveError::NotFound),
        Err(other) => Err(other),
    }
}

/// Resolves the given relative `path`.
///
/// `path` may have a `./` or `../` prefix, but this is not required.
fn resolve_relative_path(
    path: &str,
    base_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    resolve_absolute_path(base_dir.join(path), fs, options)
}

/// Resolve the directory `dir_path`.
///
/// Directories can only be resolved if [`ResolveOptions::default_files`] and
/// [`ResolveOptions::extensions`] are both specified.
fn resolve_directory(
    dir_path: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    for default_file in options.default_files {
        for extension in options.extensions {
            let default_file_path = dir_path.join(format!("{default_file}.{extension}"));
            match resolve_path_info(default_file_path, fs) {
                Ok((ResolvedPathInfo::Directory, _)) => {
                    // An index file that's a directory?
                    // Not going to fall for that...
                }
                Ok((ResolvedPathInfo::File, realpath)) => return Ok(realpath),
                Err(_) => { /* try the next one */ }
            }
        }
    }

    Err(ResolveError::DirectoryWithoutDefault)
}

/// Resolve the given module `specifier`.
fn resolve_module(
    specifier: &str,
    base_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    match &options.package_json {
        DiscoverableManifest::Auto => match fs.find_package_json(base_dir) {
            Ok((package_path, manifest)) => resolve_module_with_package_json(
                specifier,
                base_dir,
                &package_path,
                &manifest,
                fs,
                options,
            ),
            Err(_) => resolve_dependency(specifier, base_dir, fs, options),
        },
        DiscoverableManifest::Explicit {
            package_path,
            manifest,
        } => resolve_module_with_package_json(
            specifier,
            base_dir,
            package_path,
            manifest,
            fs,
            options,
        ),
        DiscoverableManifest::Off => resolve_dependency(specifier, base_dir, fs, options),
    }
}

/// Uses the given `package_json` to resolve the given module `specifier`.
///
/// The `package.json` allows us to perform alias lookups as well as
/// self-lookups (using the package's own `exports` for resolving internally).
fn resolve_module_with_package_json(
    specifier: &str,
    base_dir: &Utf8Path,
    package_path: &Utf8Path,
    package_json: &PackageJson,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    if specifier.starts_with('#') {
        return resolve_import_alias(specifier, package_path, package_json, fs, options);
    }

    if let Some(package_name) = &package_json.name {
        if specifier.starts_with(package_name)
            && specifier
                .as_bytes()
                .get(package_name.len())
                .is_some_and(|c| *c == b'/')
        {
            return resolve_export(
                &specifier[package_name.len() + 1..],
                package_path,
                package_json,
                fs,
                options,
            );
        }
    }

    resolve_dependency(specifier, base_dir, fs, options)
}

/// Resolves the given alias `specifier`.
///
/// The `specifier` must start with `#`.
fn resolve_import_alias(
    specifier: &str,
    package_dir: &Utf8Path,
    package_json: &PackageJson,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    let imports = package_json
        .get_value_by_path(&["imports"])
        .ok_or(ResolveError::NotFound)?;
    let imports = imports
        .as_object()
        .ok_or(ResolveError::InvalidMappingTarget)?;

    resolve_target_mapping(specifier, imports, package_dir, fs, options)
}

/// Resolves the given `subpath` inside the given `package_dir` using the
/// `exports` from the given `package_json`.
fn resolve_export(
    subpath: &str,
    package_dir: &Utf8Path,
    package_json: &PackageJson,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    let exports = package_json
        .get_value_by_path(&["exports"])
        .ok_or(ResolveError::NotFound)?;

    match exports {
        JsonValue::Object(mapping) => {
            resolve_target_mapping(subpath, mapping, package_dir, fs, options).or_else(|err| {
                match err {
                    ResolveError::NotFound => {
                        // exports can match directly on conditions too:
                        resolve_target_value(exports, None, package_dir, fs, options)
                    }
                    err => Err(err),
                }
            })
        }
        JsonValue::String(target) => {
            resolve_target_string(target.as_str(), package_dir, fs, options)
        }
        _ => Err(ResolveError::InvalidMappingTarget),
    }
}

/// Resolves the given `subpath` inside the given `package_dir` by looking it up
/// in the given `mapping`.
///
/// `mapping` must be taken from either `imports` or `exports` key in a
/// `package.json` file.
///
/// Keys in these mappings may contain globs with a single `*` character. If
/// present, the target value should also have a `*` character and the matching
/// characters are used as a literal replacement in the target value.
///
/// This function only implements the functionality that is common between
/// `imports` or `exports`. [`resolve_export()`] contains additional logic that
/// is specific to the `exports` map.
fn resolve_target_mapping(
    subpath: &str,
    mapping: &JsonObject,
    package_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    let subpath = normalize_subpath(subpath);
    for (key, target) in mapping.iter() {
        let key = normalize_subpath(key.as_str());
        if let Some((start, end)) = key.split_once('*') {
            if subpath.starts_with(start) && subpath.ends_with(end) {
                let glob_replacement = &subpath[start.len()..subpath.len() - end.len()];
                return resolve_target_value(
                    target,
                    Some(glob_replacement),
                    package_dir,
                    fs,
                    options,
                );
            }
        } else if key == subpath {
            return resolve_target_value(target, None, package_dir, fs, options);
        }
    }

    Err(ResolveError::NotFound)
}

/// Resolves the given `target` string from a target mapping.
///
/// When a target is found in the `imports` or `exports` mapping, it can point
/// to either a relative path or a dependency. If it's a relative path,
/// `package_dir` is used to resolve it.
fn resolve_target_string(
    target: &str,
    package_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    if target.starts_with("./") {
        let options = options.without_extensions_or_manifests();
        resolve_relative_path(target, package_dir, fs, &options)
    } else {
        resolve_dependency(target, package_dir, fs, options)
    }
}

/// Resolves the given `target` value.
///
/// Target values can be either a string to resolve, or an object with condition
/// names that need to be matched against the
/// [`ResolveOptions::condition_names`]. Matches values are resolved recursively
/// to allow nested condition objects.
///
/// If a `glob_replacement` is given, it is used as a replacement for the `*`
/// character in a string target.
fn resolve_target_value(
    target: &JsonValue,
    glob_replacement: Option<&str>,
    package_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    let resolve_string = |target| resolve_target_string(target, package_dir, fs, options);

    match target {
        JsonValue::Object(targets) => {
            for (condition, target) in targets.iter() {
                if options.condition_names.contains(&condition.as_str()) {
                    return resolve_target_value(
                        target,
                        glob_replacement,
                        package_dir,
                        fs,
                        options,
                    );
                }
            }

            Err(ResolveError::NotFound)
        }
        JsonValue::String(target) => match glob_replacement {
            Some(glob_replacement) => {
                resolve_string(&target.as_str().replace('*', glob_replacement))
            }
            None => resolve_string(target.as_str()),
        },
        _ => Err(ResolveError::InvalidMappingTarget),
    }
}

/// Resolves a dependency `specifier` by searching for `node_modules/`
/// directories from `base_dir` upwards.
///
/// If the `node_modules` subdirectory matching the package name from the
/// `specifier` contains a `package.json`, its `exports` and `main` fields can
/// be used for further resolution. Without a `package.json` file, only the
/// subpath from the `specifier` (if any) can be used for resolution.
fn resolve_dependency(
    specifier: &str,
    base_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    let (package_name, subpath) = parse_package_specifier(specifier)?;

    for dir in base_dir.ancestors() {
        let package_path = {
            let mut p = dir.to_path_buf();
            p.push("node_modules");
            p.push(package_name);
            p
        };
        let package_path = match fs.path_info(&package_path) {
            Ok(PathInfo::Directory) => package_path,
            Ok(PathInfo::Symlink {
                canonicalized_target: normalized_target,
            }) => normalized_target,
            _ => continue,
        };

        if let Ok(package_json) = fs.read_package_json(&package_path.join("package.json")) {
            if package_json.get_value_by_path(&["exports"]).is_some() {
                return resolve_export(subpath, &package_path, &package_json, fs, options);
            }

            if subpath.is_empty() {
                if let Some(main_target) = package_json
                    .get_value_by_path(&["main"])
                    .and_then(JsonValue::as_string)
                {
                    let options = options.without_extensions_or_manifests();
                    return resolve_relative_path(
                        main_target.as_str(),
                        &package_path,
                        fs,
                        &options,
                    );
                }
            }
        }

        let options = options.without_extensions_or_manifests();
        return resolve_relative_path(subpath, &package_path, fs, &options);
    }

    Err(ResolveError::NotFound)
}

enum ResolvedPathInfo {
    Directory,
    File,
}

/// Resolves the given `path` to a tuple of [`ResolvedPathInfo`] and the real
/// path being pointed to.
fn resolve_path_info(
    path: Utf8PathBuf,
    fs: &dyn ResolverFsProxy,
) -> Result<(ResolvedPathInfo, Utf8PathBuf), ResolveError> {
    match fs.path_info(&path)? {
        PathInfo::Directory => Ok((ResolvedPathInfo::Directory, path)),
        PathInfo::File => Ok((ResolvedPathInfo::File, path)),
        PathInfo::Symlink {
            canonicalized_target: normalized_target,
        } => match fs.path_info(&normalized_target)? {
            PathInfo::Directory => Ok((ResolvedPathInfo::Directory, normalized_target)),
            PathInfo::File => Ok((ResolvedPathInfo::File, normalized_target)),
            PathInfo::Symlink { .. } => Err(ResolveError::BrokenSymlink),
        },
    }
}

fn normalize_owned_absolute_path(path: Utf8PathBuf) -> Utf8PathBuf {
    let string = path.as_str();
    if string.len() < 3 {
        return path; // An absolute path this short cannot be normalized anyway.
    }

    let mut prev = string.chars().next().unwrap(); // SAFETY: We check the length above.
    for byte in string.chars().skip(1) {
        if byte == '.' && (prev == '.' || prev == std::path::MAIN_SEPARATOR) {
            return normalize_path(&path);
        }

        prev = byte;
    }

    path
}

fn normalize_subpath(subpath: &str) -> &str {
    let bytes = subpath.as_bytes();
    if bytes == b"." {
        return "";
    }

    if bytes.starts_with(b"./") {
        return &subpath[2..];
    }

    subpath
}

/// Takes a `specifier` and splits it into a `(package_name, subpath)` tuple.
///
/// Based on:
///   https://github.com/nodejs/node/blob/8f0f17e1e3b6c4e58ce748e06343c5304062c491/lib/internal/modules/esm/resolve.js#L688
fn parse_package_specifier(specifier: &str) -> Result<(&str, &str), ResolveError> {
    let bytes = specifier.as_bytes();
    let mut separator_index = bytes.iter().position(|b| *b == b'/');
    if let Some(index) = &separator_index {
        if bytes[0] == b'@' {
            separator_index = bytes[*index + 1..]
                .iter()
                .position(|b| *b == b'/')
                .map(|i| i + *index + 1);
        }
    }

    let package_name =
        separator_index.map_or(specifier, |separator_index| &specifier[..separator_index]);

    // Package name cannot have leading . and cannot have percent-encoding or
    // backslash separators.
    let bytes = package_name.as_bytes();
    if bytes.is_empty() || bytes[0] == b'.' || bytes.iter().any(|b| *b == b'\\' || *b == b'%') {
        return Err(ResolveError::InvalidPackageSpecifier);
    }

    let package_subpath =
        separator_index.map_or("", |separator_index| &specifier[separator_index + 1..]);
    Ok((package_name, package_subpath))
}

fn strip_query_and_fragment(specifier: &str) -> &str {
    let index = specifier
        .as_bytes()
        .iter()
        .skip(1)
        .position(|b| *b == b'?' || *b == b'#');
    match index {
        Some(index) => &specifier[..index + 1],
        None => specifier,
    }
}

/// Options to pass to the resolver.
#[derive(Default)]
pub struct ResolveOptions<'a> {
    /// If `true`, specifiers are assumed to be relative paths. Resolving them
    /// as a package will still be attempted if resolving as a relative path
    /// fails.
    ///
    /// For example, this makes `foo` resolve to `./foo.js` (assuming the `js`
    /// extension is also specified). Biome uses this in the `extends` field of
    /// its configuration file to allow the prefix to be optional.
    pub assume_relative: bool,

    /// Condition names to accept for the `exports` and `imports` fields.
    ///
    /// Both `exports` and `imports` support conditional mapping, where the
    /// values they map to are an object with keys that represent conditions.
    /// This option defines which keys of such objects are accepted.
    ///
    /// Note that the order in this slice doesn't matter; it is the order of the
    /// keys in the `exports` and `imports` themselves that determines which
    /// condition may be matched first.
    ///
    /// See: https://nodejs.org/api/packages.html#conditional-exports
    pub condition_names: &'a [&'a str],

    /// Default files to look for in a directory.
    ///
    /// Should be provided without extensions, as the extensions from
    /// [`Self::extensions`] will be automatically added.
    pub default_files: &'a [&'a str],

    /// List of extensions to search for in relative paths.
    ///
    /// Extensions should be provided without leading dot.
    pub extensions: &'a [&'a str],

    /// Defines which `package.json` file should be used.
    pub package_json: DiscoverableManifest<&'a PackageJson>,

    /// If `true`, the resolver will prefer to resolve to a type definition
    /// (usually a `.d.ts` file) instead of a source path.
    pub prefer_types: bool,

    /// Whether Node.js builtin modules should be resolved.
    ///
    /// Note that this setting primarily influences the kind of error returned
    /// when attempting to resolve a Node.js built-in. Built-ins cannot be
    /// resolved to a path, so if this setting is `true`, any attempt to do so
    /// will return an error of kind [`ResolveError::NodeBuiltIn`]. If `false`,
    /// the resolver may try to resolve the built-in as an ordinary dependency,
    /// which will likely fail too, but will result in a different error.
    pub resolve_node_builtins: bool,

    /// Defines which `tsconfig.json` file should be used.
    pub tsconfig: DiscoverableManifest<&'a TsConfigJson>,
}

impl ResolveOptions<'_> {
    /// Returns the instance with [`Self::assume_relative`] set to `true`.
    pub fn with_assume_relative(self) -> Self {
        Self {
            assume_relative: true,
            ..self
        }
    }

    fn without_extensions_or_manifests(&self) -> Self {
        Self {
            assume_relative: self.assume_relative,
            default_files: self.default_files,
            condition_names: self.condition_names,
            extensions: &[],
            package_json: DiscoverableManifest::Off,
            prefer_types: self.prefer_types,
            tsconfig: DiscoverableManifest::Off,
            resolve_node_builtins: self.resolve_node_builtins,
        }
    }
}

/// Wrapper for controlling how discoverable manifests should be treated.
///
/// By default, discoverable manifests such as `package.json` and
/// `tsconfig.json` will be automatically discovered, but this enum allows to
/// turn them off completely, or to provide an explicit manifest to be used
/// instead.
#[derive(Debug, Default)]
pub enum DiscoverableManifest<T> {
    #[default]
    Auto,
    Explicit {
        package_path: Utf8PathBuf,
        manifest: T,
    },
    Off,
}

/// Reference-counted resolved path wrapped in a [Result] that may contain an
/// error if the resolution failed.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ResolvedPath(Arc<Result<Utf8PathBuf, ResolveError>>);

impl Deref for ResolvedPath {
    type Target = Result<Utf8PathBuf, ResolveError>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl From<Utf8PathBuf> for ResolvedPath {
    fn from(path: Utf8PathBuf) -> Self {
        Self::new(Ok(path))
    }
}

impl From<ResolveError> for ResolvedPath {
    fn from(error: ResolveError) -> Self {
        Self::new(Err(error))
    }
}

impl From<Result<Utf8PathBuf, ResolveError>> for ResolvedPath {
    fn from(result: Result<Utf8PathBuf, ResolveError>) -> Self {
        Self::new(result)
    }
}

impl ResolvedPath {
    pub fn new(resolved_path: Result<Utf8PathBuf, ResolveError>) -> Self {
        Self(Arc::new(resolved_path))
    }

    pub fn as_path(&self) -> Option<&Utf8Path> {
        self.as_deref().ok()
    }

    pub fn error(&self) -> Option<&ResolveError> {
        self.as_deref().err()
    }

    pub fn from_path(path: impl Into<Utf8PathBuf>) -> Self {
        Self::new(Ok(path.into()))
    }
}
