//! What the frameworks of a package inject into its pages.

use crate::CssPropertyDefinition;
use biome_rowan::Text;
use camino::Utf8PathBuf;

/// What the frameworks of a package inject into its pages, without any
/// source file importing it.
///
/// Queries look the scope up with [crate::ModuleDb::framework_scope] after
/// the imports and importers of a module, so that closer definitions win. A
/// stylesheet that the frameworks apply to every page also uses the scope
/// when it's outside the folder of the package.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrameworkScope {
    /// The folder of the package.
    pub package_path: Utf8PathBuf,

    /// Stylesheets that apply to every page of the package. The stylesheets
    /// that they import apply too.
    pub stylesheets: Vec<Utf8PathBuf>,

    /// HTML-like components, such as Astro components, that the frameworks
    /// render on every page. Their global styles apply to every page. Their
    /// class attributes and their local styles use classes.
    pub components: Vec<Utf8PathBuf>,

    /// CSS custom properties that the configuration of a framework defines,
    /// by name.
    pub custom_properties: Vec<(Text, CssPropertyDefinition)>,

    /// Whether the stylesheets that apply to every page can't all be known.
    /// Rules that report a missing style, such as an undeclared class, skip
    /// the package.
    pub has_unknown_styles: bool,

    /// Whether some elements that use classes can't be read, for example
    /// because they're in Markdown pages. Rules that report an unused class
    /// skip the stylesheets of the scope.
    pub has_unknown_class_usages: bool,

    /// Whether the modules that the frameworks declare can't all be known.
    /// Rules that report an unresolved import skip specifiers that can't be
    /// package names.
    pub has_unknown_modules: bool,
}
