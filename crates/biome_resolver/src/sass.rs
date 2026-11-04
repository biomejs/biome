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
        None => match resolve_without_extension(url.as_str(), fs, rule)? {
            Some(path) => Some(path),
            None if is_directory(&url, fs) => {
                resolve_without_extension(url.join("index").as_str(), fs, rule)?
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
    let Some(mut candidates) = url
        .as_str()
        .strip_suffix(extension)
        .and_then(|url| url.strip_suffix('.'))
        .and_then(Candidates::new)
    else {
        return Ok(None);
    };
    if rule == SassLoadRule::Import
        && let Some(path) = candidates.find(".import", &[extension], fs)?
    {
        return Ok(Some(path));
    }
    candidates.find("", &[extension], fs)
}

fn resolve_without_extension(
    url: &str,
    fs: &dyn ResolverFsProxy,
    rule: SassLoadRule,
) -> Result<Option<Utf8PathBuf>, ResolveError> {
    let Some(mut candidates) = Candidates::new(url) else {
        return Ok(None);
    };
    if rule == SassLoadRule::Import {
        if let Some(path) = candidates.find(".import", &["sass", "scss"], fs)? {
            return Ok(Some(path));
        }
        if let Some(path) = candidates.find(".import", &["css"], fs)? {
            return Ok(Some(path));
        }
    }
    if let Some(path) = candidates.find("", &["sass", "scss"], fs)? {
        return Ok(Some(path));
    }
    candidates.find("", &["css"], fs)
}

/// The two paths Sass tries for a URL without its extension: as written, and
/// as a partial with `_` prepended to its basename, even when the basename
/// already starts with `_`. Each path is kept in one buffer that is reused for
/// every extension.
struct Candidates {
    path: String,
    path_len: usize,
    partial: String,
    partial_len: usize,
}

impl Candidates {
    fn new(url: &str) -> Option<Self> {
        let basename_start = url.rfind(std::path::is_separator)? + 1;
        if basename_start == url.len() {
            return None;
        }
        let capacity = url.len() + ".import.scss".len() + 1;
        let mut path = String::with_capacity(capacity);
        path.push_str(url);
        let mut partial = String::with_capacity(capacity);
        partial.push_str(&url[..basename_start]);
        partial.push('_');
        partial.push_str(&url[basename_start..]);
        Some(Self {
            path_len: path.len(),
            path,
            partial_len: partial.len(),
            partial,
        })
    }

    /// Returns the file among both paths with `infix` and one of `extensions`
    /// appended, such as `_theme.import.scss`. These candidates have equal
    /// precedence, so finding more than one is an error.
    fn find(
        &mut self,
        infix: &str,
        extensions: &[&str],
        fs: &dyn ResolverFsProxy,
    ) -> Result<Option<Utf8PathBuf>, ResolveError> {
        let mut found = None;
        for extension in extensions {
            for (buffer, len) in [
                (&mut self.path, self.path_len),
                (&mut self.partial, self.partial_len),
            ] {
                buffer.truncate(len);
                buffer.push_str(infix);
                buffer.push('.');
                buffer.push_str(extension);
                if let Some(path) = find_file(Utf8Path::new(buffer), fs) {
                    if found.is_some() {
                        return Err(ResolveError::AmbiguousSassUrl);
                    }
                    found = Some(path);
                }
            }
        }
        Ok(found)
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
