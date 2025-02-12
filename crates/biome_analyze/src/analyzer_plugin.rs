use crate::RuleDiagnostic;
use biome_parser::AnyParse;
use camino::Utf8PathBuf;
use std::fmt::Debug;

/// Definition of an analyzer plugin.
pub trait AnalyzerPlugin: Debug {
    fn evaluate(&self, root: AnyParse, path: Utf8PathBuf) -> Vec<RuleDiagnostic>;

    fn supports_css(&self) -> bool;

    fn supports_js(&self) -> bool;
}
