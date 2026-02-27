mod visitor;

use crate::css_module_info::CssClassReference;
use biome_resolver::ResolvedPath;
use biome_rowan::TokenText;
use indexmap::IndexSet;
use std::collections::BTreeSet;
use std::ops::Deref;
use std::sync::Arc;

pub(crate) use visitor::HtmlModuleVisitor;

/// Information restricted to a single HTML module in the [ModuleGraph].
///
/// Tracks CSS classes defined in embedded `<style>` blocks, CSS classes
/// referenced in `class` attributes, and external stylesheets linked via
/// `<link rel="stylesheet">`.
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
        style_classes: IndexSet<TokenText>,
        referenced_classes: Vec<CssClassReference>,
        imported_stylesheets: Vec<ResolvedPath>,
    ) -> Self {
        let info = HtmlModuleInfoInner {
            style_classes,
            referenced_classes,
            imported_stylesheets,
        };
        Self(Arc::new(info))
    }

    pub(crate) fn dump(&self) -> SerializedHtmlModuleInfo {
        SerializedHtmlModuleInfo {
            style_classes: self
                .0
                .style_classes
                .iter()
                .map(|token| token.text().to_string())
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
    pub style_classes: IndexSet<TokenText>,

    /// CSS class references from `class="..."` attributes within this HTML file.
    ///
    /// Each entry represents one attribute occurrence (e.g., `class="foo bar"`),
    /// which may contain multiple space-separated class names.
    pub referenced_classes: Vec<CssClassReference>,

    /// Resolved paths of external stylesheets linked via
    /// `<link rel="stylesheet" href="...">`.
    pub imported_stylesheets: Vec<ResolvedPath>,
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
