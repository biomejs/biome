//! Resolution of the URLs loaded by Sass `@use`, `@forward` and `@import`
//! rules, following the filesystem importer of the Sass specification:
//! <https://github.com/sass/sass/blob/main/spec/modules.md#resolving-a-file-url>
//!
//! Where the specification and dart-sass differ, this follows dart-sass
//! (`resolveImportPath` in `lib/src/importer/utils.dart`): a URL that ends in
//! `.sass`, `.scss` or `.css` never resolves to an `index` file, and `_theme`
//! also tries the partial `__theme`.

use std::borrow::Cow;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    ResolveError, ResolveOptions, ResolvedPathInfo, ResolverFsProxy, is_relative_specifier,
    normalize_owned_absolute_path, resolve, resolve_path_info,
};

/// The Sass rule that loads a stylesheet, which decides whether import-only
/// files such as `_theme.import.scss` are considered.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SassLoadRule {
    /// `@use` and `@forward`.
    Module,
    /// `@import`, which prefers import-only files.
    Import,
}

/// Resolves the Sass URL `specifier` relative to `base_dir`.
///
/// A URL without an extension tries `.sass`, `.scss`, then `.css`, each as
/// written and as a partial with a leading `_`, and then the `index` file in
/// the directory it names. When two candidates of equal precedence exist, such
/// as `theme.scss` and `_theme.scss`, Sass fails, and this returns
/// [ResolveError::AmbiguousSassUrl].
///
/// A bare URL, such as `normalize.css/normalize`, that matches no file
/// relative to `base_dir` is resolved as a package with `package_options`, as
/// bundlers do. That lookup doesn't apply the Sass rules, so it doesn't find
/// partials inside packages.
///
/// URLs with a scheme, such as `sass:math` or `pkg:bootstrap`, aren't
/// resolved, and neither are load paths.
pub fn resolve_sass(
    specifier: &str,
    base_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    rule: SassLoadRule,
    package_options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    if has_url_scheme(specifier) {
        return Err(ResolveError::NotFound);
    }
    let url = normalize_owned_absolute_path(base_dir.join(specifier));
    let resolved = match url
        .extension()
        .filter(|extension| matches!(*extension, "sass" | "scss" | "css"))
    {
        Some(extension) => resolve_with_extension(&url, extension, fs, rule)?,
        None => match resolve_without_extension(&url, fs, rule)? {
            Some(path) => Some(path),
            None if is_directory(&url, fs) => {
                resolve_without_extension(&url.join("index"), fs, rule)?
            }
            None => None,
        },
    };
    match resolved {
        Some(path) => Ok(path),
        None if !is_relative_specifier(specifier) && !specifier.starts_with('/') => {
            resolve(specifier, base_dir, fs, package_options)
        }
        None => Err(ResolveError::NotFound),
    }
}

fn resolve_with_extension(
    url: &Utf8Path,
    extension: &str,
    fs: &dyn ResolverFsProxy,
    rule: SassLoadRule,
) -> Result<Option<Utf8PathBuf>, ResolveError> {
    if rule == SassLoadRule::Import
        && let Some(path) =
            resolve_for_partials(&url.with_extension(format!("import.{extension}")), fs)?
    {
        return Ok(Some(path));
    }
    resolve_for_partials(url, fs)
}

fn resolve_without_extension(
    url: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    rule: SassLoadRule,
) -> Result<Option<Utf8PathBuf>, ResolveError> {
    if rule == SassLoadRule::Import {
        if let Some(path) = resolve_sass_or_scss(url, ".import", fs)? {
            return Ok(Some(path));
        }
        if let Some(path) = resolve_for_partials(&with_suffix(url, ".import.css"), fs)? {
            return Ok(Some(path));
        }
    }
    if let Some(path) = resolve_sass_or_scss(url, "", fs)? {
        return Ok(Some(path));
    }
    resolve_for_partials(&with_suffix(url, ".css"), fs)
}

/// Resolves `url` with `infix` and the `.sass` or `.scss` extension appended,
/// which have equal precedence.
fn resolve_sass_or_scss(
    url: &Utf8Path,
    infix: &str,
    fs: &dyn ResolverFsProxy,
) -> Result<Option<Utf8PathBuf>, ResolveError> {
    let sass = resolve_for_partials(&with_suffix(url, &format!("{infix}.sass")), fs)?;
    let scss = resolve_for_partials(&with_suffix(url, &format!("{infix}.scss")), fs)?;
    match (sass, scss) {
        (Some(_), Some(_)) => Err(ResolveError::AmbiguousSassUrl),
        (sass, scss) => Ok(sass.or(scss)),
    }
}

/// Resolves `url` as written, or as a partial with `_` prepended to its
/// basename, even when the basename already starts with `_`.
fn resolve_for_partials(
    url: &Utf8Path,
    fs: &dyn ResolverFsProxy,
) -> Result<Option<Utf8PathBuf>, ResolveError> {
    let Some(basename) = url.file_name() else {
        return Ok(None);
    };
    let partial = find_file(&url.with_file_name(format!("_{basename}")), fs);
    match (find_file(url, fs), partial) {
        (Some(_), Some(_)) => Err(ResolveError::AmbiguousSassUrl),
        (file, partial) => Ok(file.or(partial)),
    }
}

/// Returns the real path of `path` if it is a file, following symlinks.
fn find_file(path: &Utf8Path, fs: &dyn ResolverFsProxy) -> Option<Utf8PathBuf> {
    match resolve_path_info(Cow::Borrowed(path), fs, &ResolveOptions::default()) {
        Ok((ResolvedPathInfo::File, real_path)) => Some(real_path),
        _ => None,
    }
}

fn is_directory(path: &Utf8Path, fs: &dyn ResolverFsProxy) -> bool {
    matches!(
        resolve_path_info(Cow::Borrowed(path), fs, &ResolveOptions::default()),
        Ok((ResolvedPathInfo::Directory, _))
    )
}

fn with_suffix(url: &Utf8Path, suffix: &str) -> Utf8PathBuf {
    format!("{url}{suffix}").into()
}

/// Returns whether `specifier` starts with a URL scheme, such as `sass:` or
/// `https:`. A Windows drive letter isn't one.
fn has_url_scheme(specifier: &str) -> bool {
    specifier.split_once(':').is_some_and(|(scheme, _)| {
        scheme.len() > 1
            && scheme.starts_with(|c: char| c.is_ascii_alphabetic())
            && scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
    })
}
