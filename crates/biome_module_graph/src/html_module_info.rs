mod visitor;

use crate::css_module_info::CssClass;
use biome_resolver::ResolvedPath;
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
        style_classes: IndexSet<CssClass>,
        referenced_classes: IndexSet<CssClass>,
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
                .map(|c| c.text().to_string())
                .collect(),
            referenced_classes: self
                .0
                .referenced_classes
                .iter()
                .map(|c| c.text().to_string())
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HtmlModuleInfoInner {
    /// CSS class names defined in `<style>` blocks within this HTML file,
    /// together with their source locations.
    ///
    /// Collected by walking all `CssClassSelector` nodes in the embedded CSS
    /// ASTs (already parsed by the workspace server — no re-parsing needed).
    ///
    /// The [`TextRange`](biome_rowan::TextRange) in each [`CssClass`] points to
    /// the class selector token within the embedded stylesheet and is intended
    /// for LSP features such as go-to-definition.
    pub style_classes: IndexSet<CssClass>,

    /// CSS class names referenced in `class="..."` attributes within this
    /// HTML file, together with the source range of each individual class
    /// token in the attribute value.
    ///
    /// The [`TextRange`](biome_rowan::TextRange) in each [`CssClass`] can be
    /// used by LSP features to locate the exact token within the attribute
    /// string.
    pub referenced_classes: IndexSet<CssClass>,

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
