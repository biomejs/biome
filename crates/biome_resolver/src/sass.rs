//! Resolution of the URLs loaded by Sass `@use`, `@forward` and `@import`
//! rules, following the filesystem importer of the Sass specification:
//! <https://github.com/sass/sass/blob/main/spec/modules.md#resolving-a-file-url>
//!
//! Where the specification and dart-sass differ, this follows dart-sass
//! (`resolveImportPath` in `lib/src/importer/utils.dart`): a URL that ends in
//! one of the extensions never resolves to an index file, and `_theme` also
//! tries the partial `__theme`.

use std::borrow::Cow;

use camino::{Utf8Path, Utf8PathBuf};

use crate::{
    Resolution, ResolveError, ResolveOptions, ResolvedPathInfo, ResolverFsProxy,
    is_relative_specifier, normalize_owned_absolute_path, resolve_module, resolve_path_info,
    strip_query_and_fragment,
};

/// The Sass rule that loads a stylesheet.
///
/// Libraries that moved to the module system can ship an import-only file,
/// such as `_theme.import.scss` next to `_theme.scss`, so that stylesheets
/// still using `@import` keep getting the API they had before. Only `@import`
/// loads these files.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SassLoadRule {
    /// `@use` and `@forward`, which load a stylesheet as a module. Import-only
    /// files are ignored.
    Module,
    /// `@import`, which looks for an import-only file first, then for the
    /// files `@use` would load.
    Import,
}

/// Resolves the Sass URL `specifier` relative to `base_dir`, the way Sass finds
/// the stylesheet to load:
/// <https://sass-lang.com/documentation/at-rules/use/#finding-the-module>
///
/// `options` configures the lookup:
/// - [ResolveOptions::extensions] are tried in order, each on the URL as
///   written and as a partial with a leading `_`, such as `theme.scss` and
///   `_theme.scss`. Sass fails when both exist, and this returns
///   [ResolveError::AmbiguousSassUrl]. A URL that already ends in one of the
///   extensions only tries that one.
/// - [ResolveOptions::default_files] are tried, with the same extensions and
///   partials, in the directory that a URL without an extension names, when
///   the URL itself matches no file. A URL ending in `/` only names that
///   directory.
/// - With [SassLoadRule::Import], each lookup above first tries the
///   import-only files, such as `_theme.import.scss`, for each extension it
///   tries, and only then the regular files.
/// - With [ResolveOptions::assume_relative], a bare URL such as `theme` that
///   matches no file relative to `base_dir` is resolved as a package, as
///   bundlers do. Without it, bare URLs are only resolved as packages. The
///   package lookup doesn't apply the Sass rules, so it doesn't find partials
///   inside packages.
///
/// URLs with a scheme, such as `sass:math` or `pkg:bootstrap`, aren't
/// resolved, and neither are load paths. A query or fragment is ignored.
pub fn resolve_sass(
    specifier: &str,
    base_dir: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    rule: SassLoadRule,
    options: &ResolveOptions,
) -> Result<Utf8PathBuf, ResolveError> {
    if has_url_scheme(specifier) {
        return Err(ResolveError::NotFound);
    }
    let specifier = strip_query_and_fragment(specifier);
    let is_bare = !is_relative_specifier(specifier) && !specifier.starts_with('/');
    if is_bare && !options.assume_relative {
        return resolve_module(specifier, base_dir, fs, options).map(Resolution::into_path);
    }
    let url = normalize_owned_absolute_path(base_dir.join(specifier));
    let resolved = match url
        .extension()
        .filter(|extension| options.extensions.contains(extension))
    {
        Some(extension) => resolve_with_extension(&url, extension, fs, rule)?,
        None if specifier.ends_with('/') => resolve_directory(&url, fs, rule, options)?,
        None => match resolve_without_extension(url.as_str(), fs, rule, options.extensions)? {
            Some(path) => Some(path),
            None => resolve_directory(&url, fs, rule, options)?,
        },
    };
    match resolved {
        Some(path) => Ok(path),
        None if is_bare => {
            resolve_module(specifier, base_dir, fs, options).map(Resolution::into_path)
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
        && let Some(path) = candidates.find(".import", extension, fs)?
    {
        return Ok(Some(path));
    }
    candidates.find("", extension, fs)
}

fn resolve_without_extension(
    url: &str,
    fs: &dyn ResolverFsProxy,
    rule: SassLoadRule,
    extensions: &[&str],
) -> Result<Option<Utf8PathBuf>, ResolveError> {
    let Some(mut candidates) = Candidates::new(url) else {
        return Ok(None);
    };
    if rule == SassLoadRule::Import {
        for extension in extensions {
            if let Some(path) = candidates.find(".import", extension, fs)? {
                return Ok(Some(path));
            }
        }
    }
    for extension in extensions {
        if let Some(path) = candidates.find("", extension, fs)? {
            return Ok(Some(path));
        }
    }
    Ok(None)
}

fn resolve_directory(
    url: &Utf8Path,
    fs: &dyn ResolverFsProxy,
    rule: SassLoadRule,
    options: &ResolveOptions,
) -> Result<Option<Utf8PathBuf>, ResolveError> {
    // Without trailing slashes, so `lstat` doesn't follow a symlinked directory.
    let url = url.components().as_path();
    let Ok((ResolvedPathInfo::Directory, dir)) =
        resolve_path_info(Cow::Borrowed(url), fs, &ResolveOptions::default())
    else {
        return Ok(None);
    };
    for default_file in options.default_files {
        let url = dir.join(default_file);
        if let Some(path) = resolve_without_extension(url.as_str(), fs, rule, options.extensions)? {
            return Ok(Some(path));
        }
    }
    Ok(None)
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

    /// Returns the file among both paths with `infix` and `extension`
    /// appended, such as `_theme.import.scss`. Both candidates have equal
    /// precedence, so finding both is an error.
    fn find(
        &mut self,
        infix: &str,
        extension: &str,
        fs: &dyn ResolverFsProxy,
    ) -> Result<Option<Utf8PathBuf>, ResolveError> {
        let mut found = None;
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
