use std::{fmt::Display, io};

use biome_console::{fmt, markup};
use biome_diagnostics::{Category, Diagnostic, category};
use camino::Utf8PathBuf;

/// List of possible errors that may occur during resolution.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResolveError {
    /// Couldn't resolve due to a broken symlink.
    BrokenSymlink,

    /// The path referenced a directory that contained no default file.
    ///
    /// Examples of default files are `index.js` or `index.ts`.
    DirectoryWithoutDefault,

    /// A manifest could not be loaded from disk.
    ErrorLoadingManifest,

    /// An alias defined in `package.json`'s `imports` field or an export
    /// defined in the `exports` field had an invalid target.
    InvalidMappingTarget,

    /// A package specifier contained one or more invalid characters.
    InvalidPackageSpecifier,

    /// No manifest could be found.
    ManifestNotFound,

    /// The specifier referenced a Node.js built-in module instead of a path.
    NodeBuiltIn,

    /// The resolver did its best, but couldn't find what you were looking for.
    NotFound,
}

impl Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BrokenSymlink => f.write_str("broken symlink"),
            Self::DirectoryWithoutDefault => f.write_str("found directory without index"),
            Self::ErrorLoadingManifest => f.write_str("error loading manifest"),
            Self::InvalidMappingTarget => {
                f.write_str("unexpected target in `imports` or `exports`")
            }
            Self::InvalidPackageSpecifier => f.write_str("invalid package name"),
            Self::ManifestNotFound => f.write_str("no package.json manifest found"),
            Self::NodeBuiltIn => f.write_str("resolved to a Node.js built-in"),
            Self::NotFound => f.write_str("module not found"),
        }
    }
}

#[derive(Debug)]
pub struct ResolveErrorDiagnostic {
    kind: ResolveError,
    path: Utf8PathBuf,
}

impl ResolveErrorDiagnostic {
    pub fn new(kind: ResolveError, path: Utf8PathBuf) -> Self {
        Self { kind, path }
    }
}

impl Diagnostic for ResolveErrorDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("internalError/fs"))
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Could not resolve \"{}\": {}", self.path, self.kind)
    }

    fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        fmt.write_markup(markup! {
            "Could not resolve "<Emphasis>{self.path.to_string()}</Emphasis>": "
            {self.kind.to_string()}
        })
    }
}
