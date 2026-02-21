mod visitor;

use biome_css_syntax::CssPseudoClassFunctionSelector;
use biome_resolver::ResolvedPath;
use biome_rowan::{Text, TextRange, TextSize, TokenText};
use indexmap::{IndexMap, IndexSet};
use std::borrow::Borrow;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

pub(crate) use visitor::CssModuleVisitor;

/// A CSS class name together with its source location.
///
/// The class name is stored as a slice into a [`TokenText`] — the full token
/// text that contains it — identified by a [`TextRange`] that is relative to
/// the start of that token. This avoids allocating a separate `String` for
/// each class name.
///
/// For a CSS class selector like `.foo`, the token holds `"foo"` and the range
/// covers the whole token. For an HTML `class="foo bar"` attribute, the token
/// holds `"\"foo bar\""` (including quotes) and each class has its own
/// token-relative range pointing at `"foo"` or `"bar"` respectively.
///
/// Equality and hashing use only the class name text, not the range, so an
/// `IndexSet<CssClass>` deduplicates by name.
///
/// [`Borrow<str>`] is implemented so that `IndexSet::contains` accepts a plain
/// `&str` without constructing a full [`CssClass`].
#[derive(Clone, Debug)]
pub struct CssClass {
    /// The full token text that contains this class name.
    pub(crate) token: TokenText,
    /// Byte range of the class name within [`token`](CssClass::token).
    ///
    /// Applying this range to `token.text()` yields the class name as a `&str`.
    pub range: TextRange,
}

impl CssClass {
    /// Returns the class name as a string slice.
    ///
    /// This is a zero-cost operation — no allocation is performed.
    #[inline]
    pub fn text(&self) -> &str {
        let start = usize::from(self.range.start());
        let end = usize::from(self.range.end());
        &self.token.text()[start..end]
    }
}

impl PartialEq for CssClass {
    fn eq(&self, other: &Self) -> bool {
        self.text() == other.text()
    }
}

impl Eq for CssClass {}

impl Hash for CssClass {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text().hash(state);
    }
}

impl Borrow<str> for CssClass {
    fn borrow(&self) -> &str {
        self.text()
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
    pub(crate) fn new(imports: CssImports, classes: IndexSet<CssClass>) -> Self {
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
                .map(|c| c.text().to_string())
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

    /// Set of all CSS class names defined in this file (via class selectors),
    /// together with their source locations.
    ///
    /// Collected by walking all `CssClassSelector` nodes in the CST, including
    /// those inside nested rules and at-rules. Does not include classes inside
    /// `:global(...)` pseudo-class selectors.
    ///
    /// The [`TextRange`] stored in each [`CssClass`] points to the class
    /// selector token and is intended for LSP features such as go-to-definition.
    pub classes: IndexSet<CssClass>,
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

/// Splits the inner text of a `class="foo bar"` attribute into individual
/// [`CssClass`] entries.
///
/// Each entry holds the quote-stripped token text and a byte range relative
/// to the start of that text.
///
/// This function is shared by the HTML and JS visitors for splitting
/// whitespace-separated class names from attribute values.
pub(crate) fn collect_class_tokens(inner: &TokenText, classes: &mut IndexSet<CssClass>) {
    let content = inner.text();
    let mut byte_offset: u32 = 0;

    for word in content.split_ascii_whitespace() {
        let word_offset = content[byte_offset as usize..]
            .find(word)
            .map_or(byte_offset, |pos| byte_offset + pos as u32);

        let start = TextSize::from(word_offset);
        let end = start + TextSize::from(word.len() as u32);

        classes.insert(CssClass {
            token: inner.clone(),
            range: TextRange::new(start, end),
        });

        byte_offset = word_offset + word.len() as u32;
    }
}
