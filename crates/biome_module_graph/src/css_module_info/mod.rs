pub(crate) mod traverse;
mod visitor;

use biome_css_syntax::CssPseudoClassFunctionSelector;
use biome_resolver::ResolvedPath;
use biome_rowan::{Text, TokenText};
use camino::Utf8PathBuf;
use indexmap::{IndexMap, IndexSet};
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub use traverse::{CssClassStep, CssTraversalStep, ImportTreeDisplay, ImportTreeNode};
pub(crate) use visitor::CssModuleVisitor;

/// A CSS class reference from an HTML or JSX attribute.
///
/// Represents one `class="..."` or `className="..."` attribute occurrence.
/// The token holds the full attribute value (e.g., "foo bar baz"), which may
/// contain multiple space-separated class names.
///
/// Equality is based on both the file path and the token text, since the same
/// class attribute in different files represents different references (important
/// for CSS modules).
#[derive(Clone, Debug)]
pub struct CssClassReference {
    /// The full attribute value token (e.g., "foo bar baz")
    pub token: TokenText,
    /// The file where this reference appears
    pub file_path: Utf8PathBuf,
}

impl CssClassReference {
    /// Creates a new CSS class reference.
    pub fn new(token: TokenText, file_path: Utf8PathBuf) -> Self {
        Self { token, file_path }
    }

    /// Checks if this reference matches the given class name.
    ///
    /// Splits the attribute value by whitespace and checks if any word matches
    /// the given class name.
    pub fn matches(&self, class_name: &str) -> bool {
        self.token
            .text()
            .split_ascii_whitespace()
            .any(|word| word == class_name)
    }
}

impl PartialEq for CssClassReference {
    fn eq(&self, other: &Self) -> bool {
        self.file_path == other.file_path && self.token.text() == other.token.text()
    }
}

impl Eq for CssClassReference {}

impl Hash for CssClassReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.file_path.hash(state);
        self.token.text().hash(state);
    }
}

/// Information restricted to a single module in the [ModuleGraph].
#[derive(Clone, Debug)]
pub struct CssModuleInfo(pub(super) Arc<CssModuleInfoInner>);

impl Deref for CssModuleInfo {
    type Target = CssModuleInfoInner;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl CssModuleInfo {
    pub(crate) fn new(imports: CssImports, classes: IndexSet<TokenText>) -> Self {
        let info = CssModuleInfoInner { imports, classes };
        Self(Arc::new(info))
    }

    pub(crate) fn dump(&self) -> SerializedCssModuleInfo {
        SerializedCssModuleInfo {
            imports: self
                .0
                .imports
                .iter()
                .map(|(_, static_import)| static_import.specifier.to_string())
                .collect(),
            classes: self
                .0
                .classes
                .iter()
                .map(|token| token.text().to_string())
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CssModuleInfoInner {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the import specifier to a [CssImport] with the absolute path
    /// it resolves to. The resolved path may be looked up as key in the
    /// [ModuleGraph::data] map, although it is not required to exist
    /// (for instance, if the path is outside the project's scope).
    pub imports: CssImports,

    /// Set of all CSS class names defined in this file (via class selectors).
    ///
    /// Collected by walking all `CssClassSelector` nodes in the CST, including
    /// those inside nested rules and at-rules. Does not include classes inside
    /// `:global(...)` pseudo-class selectors.
    ///
    /// Each `TokenText` represents a single class name (e.g., "header" from `.header`).
    pub classes: IndexSet<TokenText>,
}

#[derive(Debug, Default, Clone)]
pub struct CssImports(pub(crate) IndexMap<Text, CssImport>);

impl Deref for CssImports {
    type Target = IndexMap<Text, CssImport>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CssImports {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Represents an import to one or more symbols from an external path.
///
/// It could point to any kind of resource, such as JavaScript files, CSS files,
/// images, and so on.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct CssImport {
    /// The specifier for the imported as it appeared in the source text.
    pub specifier: Text,

    /// Absolute path of the resource being imported, if it can be resolved.
    ///
    /// If the import statement referred to a package dependency, the path will
    /// point towards the resolved entry point of the package.
    ///
    /// If `None`, import resolution failed.
    pub resolved_path: ResolvedPath,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SerializedCssModuleInfo {
    /// Map of all static imports found in the module.
    ///
    /// Maps from the local imported name to the absolute path it resolves to.
    pub imports: BTreeSet<String>,

    /// Set of all CSS class names defined in this file.
    pub classes: BTreeSet<String>,
}

/// Returns `true` if the given pseudo-class function selector is `:global(...)`.
///
/// This is used by CSS and HTML module visitors to skip class selectors that
/// are globally scoped and cannot be traced to specific `class="..."` attribute
/// references.
pub(crate) fn is_global_pseudo(node: &CssPseudoClassFunctionSelector) -> bool {
    node.name()
        .ok()
        .and_then(|name| name.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == "global")
}
