use crate::ConfigValue;
use biome_glob::Glob;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::AnyJsRoot;
use biome_package::PackageJson;
use biome_rowan::{Text, TextRange};
use camino::{Utf8Path, Utf8PathBuf};
use std::fmt::Debug;

/// Describes the files and names that a framework injects into a package.
///
/// See the [crate] documentation for how loaders relate to each other.
pub trait FrameworkLoader: Debug + Send + Sync {
    /// The npm package of the framework, such as `astro` or
    /// `@astrojs/starlight`.
    fn package_name(&self) -> &'static str;

    /// The versions of the framework that the loader understands, as a
    /// semver range such as `>=0.30.0`.
    ///
    /// When the installed version is outside the range, the loader doesn't
    /// run, and the package is incomplete for every [Facet].
    fn supported_versions(&self) -> &'static str;

    /// When the loader runs.
    fn activation(&self) -> Activation;

    /// Returns what the framework injects, given the `options` that
    /// configure it.
    ///
    /// For an [Activation::Dependency] loader, `options` is the default
    /// export of its configuration file, or an empty object when the package
    /// has no configuration file. For an [Activation::Integration] loader,
    /// `options` is the first argument of the call that registers the
    /// integration, or an empty object when the call has no argument.
    ///
    /// `options` is [ConfigValue::Unknown] when the configuration file can't
    /// be read as JavaScript, when it has no `export default`, or when the
    /// value can only be known by running the configuration.
    fn load(&self, ctx: &LoaderContext, options: &ConfigValue) -> Vec<Contribution>;
}

/// When a [FrameworkLoader] runs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Activation {
    /// When the manifest of a package depends on the framework.
    ///
    /// The loader reads the first of `config_files` that exists in the
    /// directory of the package.
    Dependency {
        config_files: &'static [&'static str],
    },

    /// When another loader contributes a [Contribution::Integration] for the
    /// framework.
    Integration,
}

/// Something that a framework injects into a package.
#[derive(Clone, Debug, PartialEq)]
pub enum Contribution {
    /// A file that the framework loads, even though no source file imports
    /// it. Consumers index the file and register what it exposes, depending
    /// on the kind of file.
    Visit(Utf8PathBuf),

    /// A name that the configuration creates, such as a CSS custom property
    /// that a font option defines.
    Declare(Declaration),

    /// An integration that the configuration registers.
    ///
    /// The loader of `package` loads it. Without such a loader, the
    /// stylesheets that `package` exports are visited. Like the imports of
    /// the configuration file, `package` is looked up from the directory of
    /// the package whose frameworks are loaded.
    Integration { package: Text, options: ConfigValue },

    /// The loader can't know what the framework injects for `Facet`, for
    /// example because the configuration computes a value when it runs.
    Incomplete(Facet),
}

/// A name that a framework configuration creates.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Declaration {
    pub kind: DeclarationKind,
    pub name: Text,
    pub origin: Origin,
}

/// What a [Declaration] creates.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum DeclarationKind {
    CssClass,
    CssCustomProperty,
    JsBinding,
}

/// Where a configuration creates a [Declaration].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Origin {
    pub path: Utf8PathBuf,
    pub range: TextRange,
}

/// A kind of information that a framework injects.
///
/// Each [Contribution::Incomplete] names the facet whose information is
/// missing, so that only the rules relying on that facet stop reporting.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Facet {
    /// Stylesheets that apply to every page, with their classes and custom
    /// properties.
    Styles,

    /// Elements with CSS classes in files that Biome can't index, such as
    /// MDX pages.
    ClassUsages,

    /// Modules that imports can name, such as `astro:content`.
    Modules,

    /// Variables available without an import, such as auto-imported
    /// functions.
    Bindings,
}

impl Facet {
    pub const ALL: [Self; 4] = [
        Self::Styles,
        Self::ClassUsages,
        Self::Modules,
        Self::Bindings,
    ];
}

/// A package installed in a `node_modules` directory.
#[derive(Clone, Debug)]
pub struct InstalledPackage {
    /// The directory of the package.
    pub path: Utf8PathBuf,
    pub manifest: PackageJson,
}

/// The resolution rules for [LoaderHost::resolve].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionKind {
    /// The rules for the `@import` rules of stylesheets.
    Stylesheet,

    /// The rules for the imports of JavaScript and TypeScript modules.
    Module,
}

/// Gives loaders access to the files of a project.
///
/// An implementation should read files so that a change to a file that a
/// loader read causes the loader to run again.
pub trait LoaderHost {
    /// Returns whether `path` is a file.
    fn is_file(&self, path: &Utf8Path) -> bool;

    /// Returns the JavaScript or TypeScript file at `path` with its semantic
    /// model, or `None` when the file can't be read as JavaScript or
    /// TypeScript, or has syntax errors.
    fn js_file(&self, path: &Utf8Path) -> Option<(AnyJsRoot, SemanticModel)>;

    /// Returns the package `name` installed for the package in
    /// `package_path`, or `None` when it isn't installed.
    fn installed_package(&self, package_path: &Utf8Path, name: &str) -> Option<InstalledPackage>;

    /// Resolves `specifier` as imported by a file in `directory`, or returns
    /// `None` when it doesn't resolve to a file.
    fn resolve(
        &self,
        directory: &Utf8Path,
        specifier: &str,
        kind: ResolutionKind,
    ) -> Option<Utf8PathBuf>;

    /// Returns the files under `directory` whose path relative to
    /// `directory` matches `glob`, in alphabetical order.
    fn glob(&self, directory: &Utf8Path, glob: &Glob) -> Vec<Utf8PathBuf>;
}

/// What a [FrameworkLoader] knows about the package it loads for.
pub struct LoaderContext<'a> {
    pub(crate) host: &'a dyn LoaderHost,
    pub(crate) package_path: &'a Utf8Path,
    pub(crate) framework: &'a InstalledPackage,
    pub(crate) config_path: Option<&'a Utf8Path>,
}

impl LoaderContext<'_> {
    /// Returns the directory of the package whose frameworks are loaded.
    pub fn package_path(&self) -> &Utf8Path {
        self.package_path
    }

    /// Returns the installed package of the framework.
    pub fn framework(&self) -> &InstalledPackage {
        self.framework
    }

    /// Returns the [Origin] of `range` in the configuration file that the
    /// options come from, or `None` when the package has no configuration
    /// file.
    pub fn origin(&self, range: TextRange) -> Option<Origin> {
        self.config_path.map(|path| Origin {
            path: path.to_path_buf(),
            range,
        })
    }

    /// Returns whether `path` is a file.
    pub fn is_file(&self, path: &Utf8Path) -> bool {
        self.host.is_file(path)
    }

    /// Resolves `specifier` relative to the package directory, like the
    /// paths of a configuration file.
    pub fn resolve(&self, specifier: &str, kind: ResolutionKind) -> Option<Utf8PathBuf> {
        self.host.resolve(self.package_path, specifier, kind)
    }

    /// Returns the files under `directory` whose path relative to
    /// `directory` matches `glob`, in alphabetical order.
    pub fn glob(&self, directory: &Utf8Path, glob: &Glob) -> Vec<Utf8PathBuf> {
        self.host.glob(directory, glob)
    }
}
