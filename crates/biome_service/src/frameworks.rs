//! Runs the framework loaders of `biome_frameworks` on the packages of a
//! project, and records what they inject.
//!
//! A framework loads files that no source file imports, such as the
//! stylesheets of Starlight, and declares modules that no file provides, such
//! as `astro:content`. The scanner indexes the files that the loaders visit,
//! and each visited file contributes by its kind to the [PackageFrameworks]
//! of its package:
//!
//! - The modules that a declaration file declares become [FrameworkModules],
//!   which module resolution falls back to.
//! - A stylesheet applies to every page of the package.
//! - An HTML-like component is rendered on every page of the package.

#[cfg(all(feature = "lang_js", feature = "lang_css"))]
use crate::diagnostics::{InvalidCssGlobal, NegatedCssGlobal, UnmatchedCssGlobal};
#[cfg(all(feature = "lang_js", feature = "lang_css"))]
use biome_fs::is_node_modules_path;
use biome_module_graph::FrameworkScope;
use biome_resolver::{match_pattern_key, pattern_key_compare};
use camino::Utf8PathBuf;
use rustc_hash::FxHashMap;
use std::sync::Arc;

#[cfg(feature = "lang_js")]
mod host;

#[cfg(feature = "lang_js")]
pub(crate) use host::{WorkspaceLoaderHost, referenced_declaration_files};

/// What the framework loaders contribute to a package.
pub(crate) struct LoadedPackage {
    pub(crate) package_path: Utf8PathBuf,
    pub(crate) contributions: biome_frameworks::FrameworkContributions,
    /// The stylesheets that the `css.globals` option of the configuration of
    /// the package names, when the option has entries.
    pub(crate) css_globals: Option<Vec<Utf8PathBuf>>,
}

/// The stylesheets that the entries of `css.globals` name, expanded once for
/// all the packages that a run of the loaders reads.
#[cfg(all(feature = "lang_js", feature = "lang_css"))]
#[derive(Default)]
pub(crate) struct CssGlobals {
    stylesheets: FxHashMap<Box<str>, Vec<Utf8PathBuf>>,
    /// Warnings about the entries that name no stylesheet.
    pub(crate) diagnostics: Vec<biome_diagnostics::Error>,
}

#[cfg(all(feature = "lang_js", feature = "lang_css"))]
impl CssGlobals {
    /// Returns the stylesheets that `entry` names, expanding it with `expand`
    /// the first time.
    ///
    /// A negated entry, which starts with `!`, names no stylesheet.
    pub(crate) fn stylesheets(
        &mut self,
        entry: &str,
        expand: impl FnOnce(&str) -> Result<Vec<Utf8PathBuf>, String>,
    ) -> &[Utf8PathBuf] {
        if !self.stylesheets.contains_key(entry) {
            let stylesheets = if entry.starts_with('!') {
                self.diagnostics
                    .push(biome_diagnostics::Error::from(NegatedCssGlobal {
                        entry: entry.to_string(),
                    }));
                Vec::new()
            } else {
                match expand(entry) {
                    Ok(stylesheets) => {
                        if stylesheets.is_empty() {
                            self.diagnostics.push(biome_diagnostics::Error::from(
                                UnmatchedCssGlobal {
                                    entry: entry.to_string(),
                                },
                            ));
                        }
                        stylesheets
                    }
                    Err(error) => {
                        self.diagnostics
                            .push(biome_diagnostics::Error::from(InvalidCssGlobal {
                                entry: entry.to_string(),
                                error,
                            }));
                        Vec::new()
                    }
                }
            };
            self.stylesheets.insert(entry.into(), stylesheets);
        }
        &self.stylesheets[entry]
    }
}

/// Returns the stylesheets that `entry`, an absolute path or glob pattern of
/// `css.globals`, names.
///
/// A glob pattern matches only stylesheets, and doesn't enter `node_modules`.
/// Returns the error of a glob pattern that isn't valid.
#[cfg(all(feature = "lang_js", feature = "lang_css"))]
pub(crate) fn css_global_stylesheets(
    host: &WorkspaceLoaderHost,
    entry: &str,
) -> Result<Vec<Utf8PathBuf>, String> {
    use biome_frameworks::LoaderHost;

    let Some((directory, glob)) = split_css_global(entry) else {
        let path = camino::Utf8Path::new(entry);
        return Ok(if host.is_file(path) {
            vec![path.to_path_buf()]
        } else {
            Vec::new()
        });
    };
    let glob = glob?;
    Ok(host
        .glob(&directory, &glob)
        .into_iter()
        .filter(is_stylesheet)
        .filter(|path| !is_node_modules_path(path))
        .collect())
}

/// Returns whether `path` is a stylesheet that `entry`, an absolute path or
/// glob pattern of `css.globals`, names. See [css_global_stylesheets].
#[cfg(all(feature = "lang_js", feature = "lang_css"))]
pub(crate) fn is_css_global(entry: &str, path: &camino::Utf8Path) -> bool {
    if entry.starts_with('!') {
        return false;
    }
    match split_css_global(entry) {
        None => camino::Utf8Path::new(entry) == path,
        Some((directory, Ok(glob))) => {
            is_stylesheet(&path)
                && !is_node_modules_path(path)
                && path
                    .strip_prefix(&directory)
                    .is_ok_and(|relative| glob.is_match(relative))
        }
        Some((_, Err(_))) => false,
    }
}

/// Splits the glob pattern `entry` into the folder before its first component
/// with a wildcard or alternatives, and the pattern of the rest. Returns
/// `None` when `entry` is a path.
#[cfg(all(feature = "lang_js", feature = "lang_css"))]
fn split_css_global(entry: &str) -> Option<(Utf8PathBuf, Result<biome_glob::Glob, String>)> {
    if !entry.contains(['*', '{']) {
        return None;
    }
    let mut directory = Utf8PathBuf::new();
    let mut pattern = Vec::new();
    for component in camino::Utf8Path::new(entry).components() {
        if pattern.is_empty() && !component.as_str().contains(['*', '{']) {
            directory.push(component);
        } else {
            pattern.push(component.as_str());
        }
    }
    let glob = pattern
        .join("/")
        .parse::<biome_glob::Glob>()
        .map_err(|error| error.to_string());
    Some((directory, glob))
}

#[cfg(all(feature = "lang_js", feature = "lang_css"))]
fn is_stylesheet(path: &impl AsRef<camino::Utf8Path>) -> bool {
    biome_languages::DocumentFileSource::from_well_known(path.as_ref(), true).is_css_like()
}

/// What the frameworks of a package inject.
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct PackageFrameworks {
    pub(crate) modules: FrameworkModules,
    pub(crate) scope: Arc<FrameworkScope>,
}

/// The modules that the frameworks of a package declare, by the specifier
/// that imports them.
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct FrameworkModules {
    /// Modules declared with a name without `*`, such as `astro:content`.
    exact: FxHashMap<String, Utf8PathBuf>,

    /// Modules declared with a name that has one `*`, such as `*.svg`, in
    /// declaration order.
    patterns: Vec<(String, Utf8PathBuf)>,
}

impl FrameworkModules {
    /// Records that `name` declares the module at `path`. The first
    /// declaration of a name wins.
    pub(crate) fn insert(&mut self, name: &str, path: Utf8PathBuf) {
        if name.contains('*') {
            if !self.patterns.iter().any(|(pattern, _)| pattern == name) {
                self.patterns.push((name.to_string(), path));
            }
        } else {
            self.exact.entry(name.to_string()).or_insert(path);
        }
    }

    /// Returns the path of the module that `specifier` imports.
    ///
    /// Like TypeScript, a name without `*` is preferred. Among the names with
    /// a `*` that match, the most specific one wins, as in the `exports` of
    /// a package.
    pub(crate) fn module_path(&self, specifier: &str) -> Option<&Utf8PathBuf> {
        if let Some(path) = self.exact.get(specifier) {
            return Some(path);
        }
        self.patterns
            .iter()
            .filter(|(pattern, _)| match_pattern_key(pattern, specifier).is_some())
            .min_by(|(left, _), (right, _)| pattern_key_compare(left, right))
            .map(|(_, path)| path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_names_win_over_patterns_and_longer_prefixes_win() {
        let mut modules = FrameworkModules::default();
        modules.insert("*.css", "/types.d.ts#0".into());
        modules.insert("*.module.css", "/types.d.ts#1".into());
        modules.insert("virtual:*", "/types.d.ts#2".into());
        modules.insert("virtual:config", "/types.d.ts#3".into());
        modules.insert("virtual:config", "/types.d.ts#4".into());

        let path = |specifier| modules.module_path(specifier).map(|path| path.as_str());
        assert_eq!(path("./a.css"), Some("/types.d.ts#0"));
        assert_eq!(path("./a.module.css"), Some("/types.d.ts#1"));
        assert_eq!(path("virtual:config"), Some("/types.d.ts#3"));
        assert_eq!(path("virtual:other"), Some("/types.d.ts#2"));
        assert_eq!(path("astro:content"), None);
    }
}
