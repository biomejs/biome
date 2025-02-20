use crate::RuleDiagnostic;
use biome_parser::AnyParse;
use camino::Utf8PathBuf;
use std::{fmt::Debug, sync::Arc};

/// Slice of analyzer plugins that can be cheaply cloned.
pub type AnalyzerPluginSlice<'a> = &'a [Arc<Box<dyn AnalyzerPlugin>>];

/// Vector of analyzer plugins that can be cheaply cloned.
pub type AnalyzerPluginVec = Vec<Arc<Box<dyn AnalyzerPlugin>>>;

/// Definition of an analyzer plugin.
pub trait AnalyzerPlugin: Debug + Send + Sync {
    fn evaluate(&self, root: AnyParse, path: Utf8PathBuf) -> Vec<RuleDiagnostic>;

    fn supports_css(&self) -> bool;

    fn supports_js(&self) -> bool;
}
