#[cfg(feature = "module_graph")]
pub use biome_module_graph::SerializedModuleInfo;
#[cfg(all(feature = "module_graph", feature = "lang_css"))]
pub(crate) use biome_module_graph::resolve_css_module;
#[cfg(all(feature = "module_graph", feature = "lang_js"))]
pub(crate) use biome_module_graph::resolve_js_module;
#[cfg(all(feature = "module_graph", feature = "lang_html"))]
pub(crate) use biome_module_graph::{HtmlEmbeddedContent, resolve_html_module};
#[cfg(feature = "module_graph")]
pub(crate) use biome_module_graph::{
    ModuleDb, ModuleDependencies, ModuleInfo, ModuleInfoKind, PathInfoCache,
};

#[cfg(not(feature = "module_graph"))]
use camino::{Utf8Path, Utf8PathBuf};

#[cfg(not(feature = "module_graph"))]
pub(crate) type ModuleDependencies = Vec<Utf8PathBuf>;

#[cfg(not(feature = "module_graph"))]
#[derive(Default)]
pub(crate) struct PathInfoCache;

#[cfg(not(feature = "module_graph"))]
impl PathInfoCache {
    pub(crate) fn remove(&self, _path: &Utf8Path) {}
}

#[cfg(not(feature = "module_graph"))]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SerializedModuleInfo {}
