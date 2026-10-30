//! Runs the framework loaders of `biome_frameworks` on the packages of a
//! project, and records what they inject.
//!
//! A framework loads files that no source file imports, such as the
//! stylesheets of Starlight, and declares modules that no file provides, such
//! as `astro:content`. The scanner indexes the files that the loaders visit.
//! The modules that the visited declaration files declare become
//! [FrameworkModules], which module resolution falls back to.

use biome_resolver::{match_pattern_key, pattern_key_compare};
use camino::Utf8PathBuf;
use rustc_hash::FxHashMap;

#[cfg(feature = "lang_js")]
mod host;

#[cfg(feature = "lang_js")]
pub(crate) use host::{WorkspaceLoaderHost, referenced_declaration_files};

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
