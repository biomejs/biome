mod visitor;

use crate::ImportPathMap;
use crate::css_module_info::{CssClassDefinition, CssClassReference};
use biome_css_syntax::{AnyCssRoot, TextRange};
use biome_js_syntax::AnyJsRoot;
use biome_languages::CssFileSource;
use biome_languages::css::EmbeddingStyleApplicability;
use biome_resolver::ResolvedPath;
use biome_rowan::{TextSize, TokenText};
use camino::Utf8Path;
use indexmap::IndexMap;
use indexmap::IndexSet;
use std::collections::BTreeSet;
use std::ops::Deref;
use std::sync::Arc;

pub(crate) use visitor::HtmlModuleVisitor;

/// A single embedded content block extracted from an HTML-like file
/// (`*.html`, `*.vue`, `*.astro`, `*.svelte`).
///
/// This is passed to [`ModuleGraph::update_graph_for_html_paths`] so the
/// module graph can track both CSS class definitions and JS static imports
/// without the caller needing to know how they are processed internally.
///
/// The caller (workspace server or test helper) is responsible for:
/// - Resolving `file_source_index → CssFileSource` for CSS blocks.
/// - Providing already-parsed `AnyCssRoot` / `AnyJsRoot` syntax trees.
///
/// The module graph is responsible for all downstream logic (class collection,
/// import resolution, upward traversal).
pub enum HtmlEmbeddedContent {
    /// A `<style>` block with its resolved CSS source and content offset
    /// within the parent document.
    ///
    /// [`EmbeddingApplicability`]: biome_css_syntax::EmbeddingStyleApplicability
    Css(AnyCssRoot, CssFileSource, TextSize),
    /// A `<script>` block parsed as JS/TS with its content offset within the parent document.
    Js(AnyJsRoot, TextSize),
}

/// Information restricted to a single HTML module in the [ModuleGraph].
///
/// Tracks CSS classes defined in embedded `<style>` blocks, CSS classes
/// referenced in `class` attributes, external stylesheets linked via
/// `<link rel="stylesheet">`, and JS modules imported from embedded `<script>`
/// blocks.
#[derive(Clone, Debug)]
pub struct HtmlModuleInfo(pub(super) Arc<HtmlModuleInfoInner>);

impl Deref for HtmlModuleInfo {
    type Target = HtmlModuleInfoInner;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl HtmlModuleInfo {
    pub(crate) fn new(
        style_classes: IndexSet<CssClassDefinition>,
        referenced_classes: Vec<CssClassReference>,
        imported_stylesheets: Vec<HtmlImport>,
        import_paths: ImportPathMap<HtmlImport>,
    ) -> Self {
        let info = HtmlModuleInfoInner {
            style_classes,
            referenced_classes,
            imported_stylesheets,
            import_paths,
        };
        Self(Arc::new(info))
    }

    pub(crate) fn dump(&self) -> SerializedHtmlModuleInfo {
        SerializedHtmlModuleInfo {
            style_classes: self
                .0
                .style_classes
                .iter()
                .map(|c| c.name.text().to_string())
                .collect(),
            referenced_classes: self
                .0
                .referenced_classes
                .iter()
                .flat_map(|r| {
                    r.token
                        .text()
                        .split_ascii_whitespace()
                        .map(|s| s.to_string())
                })
                .collect(),
        }
    }
}

/// A stylesheet or script import at its position in an HTML-like document.
#[derive(Clone, Debug)]
pub struct HtmlImport {
    /// Absolute range of the element or JavaScript import expression.
    pub range: TextRange,
    /// Resolved import path.
    pub resolved_path: ResolvedPath,
    /// Whether the import is visible outside the containing HTML-like component.
    pub applicability: EmbeddingStyleApplicability,
}

impl HtmlImport {
    /// Returns the resolved filesystem path, when resolution succeeded.
    pub fn as_path(&self) -> Option<&Utf8Path> {
        self.resolved_path.as_path()
    }
}

impl Deref for HtmlImport {
    type Target = ResolvedPath;

    fn deref(&self) -> &Self::Target {
        &self.resolved_path
    }
}

#[derive(Clone, Debug)]
pub struct HtmlModuleInfoInner {
    /// CSS class names defined in `<style>` blocks within this HTML file.
    ///
    /// Collected by walking all `CssClassSelector` nodes in the embedded CSS
    /// ASTs (already parsed by the workspace server — no re-parsing needed).
    ///
    /// Each `TokenText` represents a single class name (e.g., "header" from `.header`).
    pub style_classes: IndexSet<CssClassDefinition>,

    /// CSS class references from `class="..."` attributes within this HTML file.
    ///
    /// Each entry represents one attribute occurrence (e.g., `class="foo bar"`),
    /// which may contain multiple space-separated class names.
    pub referenced_classes: Vec<CssClassReference>,

    /// Stylesheet imports from `<link>` elements and embedded `<style>` blocks.
    pub imported_stylesheets: Vec<HtmlImport>,

    /// Resolved paths imported from embedded `<script>` blocks in source order.
    pub import_paths: ImportPathMap<HtmlImport>,
}

impl HtmlModuleInfoInner {
    /// Returns CSS classes defined in `global` definitions
    pub(crate) fn get_global_styles(&self) -> IndexMap<TextRange, TokenText> {
        self.style_classes
            .iter()
            .filter(|declaration| declaration.applicability.is_global())
            .map(|c| (c.range, c.name.clone()))
            .collect()
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SerializedHtmlModuleInfo {
    /// CSS class names defined in `<style>` blocks.
    pub style_classes: BTreeSet<String>,
    /// CSS class names referenced in `class` attributes.
    pub referenced_classes: BTreeSet<String>,
}
