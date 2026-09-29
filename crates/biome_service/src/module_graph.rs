#[cfg(feature = "module_graph")]
pub use biome_module_graph::SerializedModuleInfo;
#[cfg(all(feature = "module_graph", feature = "lang_css"))]
pub(crate) use biome_module_graph::resolve_css_module;
#[cfg(all(feature = "module_graph", feature = "lang_html"))]
pub(crate) use biome_module_graph::resolve_html_module;
#[cfg(all(feature = "module_graph", feature = "lang_js"))]
pub(crate) use biome_module_graph::resolve_js_module;
#[cfg(feature = "module_graph")]
pub(crate) use biome_module_graph::{
    ModuleDb, ModuleDependencies, ModuleInfo, ModuleInfoKind, module_dependencies,
};

#[cfg(not(feature = "module_graph"))]
use camino::Utf8PathBuf;

#[cfg(not(feature = "module_graph"))]
pub(crate) type ModuleDependencies = Vec<Utf8PathBuf>;

#[cfg(not(feature = "module_graph"))]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SerializedModuleInfo {}
