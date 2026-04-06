mod visitor;

use crate::css_module_info::{CssClassDefinition, CssClassReference};
use biome_css_syntax::{AnyCssRoot, CssFileSource};
use biome_js_syntax::AnyJsRoot;
use biome_resolver::ResolvedPath;
use biome_rowan::Text;
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
    /// A `<style>` block with its resolved CSS source (carries [`EmbeddingApplicability`]).
    ///
    /// [`EmbeddingApplicability`]: biome_css_syntax::EmbeddingStyleApplicability
    Css(AnyCssRoot, CssFileSource),
    /// A `<script>` block parsed as JS/TS.
    Js(AnyJsRoot),
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
        imported_stylesheets: Vec<ResolvedPath>,
        static_import_paths: IndexMap<Text, ResolvedPath>,
        dynamic_import_paths: IndexMap<Text, ResolvedPath>,
    ) -> Self {
        let info = HtmlModuleInfoInner {
            style_classes,
            referenced_classes,
            imported_stylesheets,
            static_import_paths,
            dynamic_import_paths,
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

    /// Resolved paths of external stylesheets linked via
    /// `<link rel="stylesheet" href="...">`.
    pub imported_stylesheets: Vec<ResolvedPath>,

    /// Resolved paths of JS/TS modules imported from embedded `<script>` blocks.
    ///
    /// Keys are the raw import specifiers (e.g. `"./Button.vue"`); values are
    /// their resolved absolute paths. Only static imports (`import … from "…"`)
    /// are tracked here — dynamic imports are ignored for upward-traversal.
    pub static_import_paths: IndexMap<Text, ResolvedPath>,

    /// Resolved paths of JS/TS modules imported from dynamic imports.
    pub dynamic_import_paths: IndexMap<Text, ResolvedPath>,
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
