use crate::RuleDiagnostic;
use biome_parser::AnyParse;
use std::{fmt::Debug, path::PathBuf};

/// Definition of an analyzer plugin.
pub trait AnalyzerPlugin: Debug {
    fn evaluate(&self, root: AnyParse, path: PathBuf) -> Vec<RuleDiagnostic>;

    fn supports_css(&self) -> bool;

    fn supports_js(&self) -> bool;
}
