//! Loaders for the files and modules that frameworks inject into a project.
//!
//! A framework can load files that no source file of the project imports. For
//! example, Starlight adds its own stylesheets to every page, plus the
//! stylesheets listed in the `customCss` option of `astro.config.mjs`. Biome
//! can't see these relations in the source code, so a [FrameworkLoader]
//! describes them.
//!
//! A loader reads the configuration of its framework statically, as a
//! [ConfigValue], and never runs it. It returns [Contribution]s: files that
//! Biome should visit, names that the configuration declares, and the
//! integrations that the configuration registers. Each integration is loaded
//! by its own loader, so loaders form a tree: Astro loads Starlight, and
//! Starlight loads its plugins.
//!
//! When a loader can't know what its framework injects, it contributes
//! [Contribution::Incomplete]. Consumers must then treat what the package
//! injects as unknown, so that rules that report something missing, such as
//! an undeclared CSS class, don't report false positives.
//!
//! [load_frameworks] runs the loaders of a package and collects their
//! contributions. It reads files through a [LoaderHost].

mod config_value;
mod driver;
mod loader;

pub use config_value::{ConfigCall, ConfigObject, ConfigValue, package_name};
pub use driver::{FrameworkContributions, MAX_INTEGRATION_DEPTH, load_frameworks};
pub use loader::{
    Activation, Contribution, Declaration, DeclarationKind, Facet, FrameworkLoader,
    InstalledPackage, LoaderContext, LoaderHost, Origin, ResolutionKind,
};

/// The loaders that Biome runs for every package.
pub static LOADERS: &[&dyn FrameworkLoader] = &[];
