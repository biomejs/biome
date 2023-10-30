use crate::project_handlers::{
    ProjectAnalyzerCapabilities, ProjectCapabilities, ProjectHandler, ProjectLintResult,
};
use crate::WorkspaceError;
use biome_diagnostics::serde::Diagnostic as SerdeDiagnostic;
use biome_diagnostics::Severity;
use biome_fs::RomePath;
use biome_json_syntax::JsonRoot;
use biome_parser::AnyParse;
use biome_project::{NodeJsProject, Project};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct NodeProjectHandler {}

impl ProjectHandler for NodeProjectHandler {
    fn capabilities(&self) -> ProjectCapabilities {
        ProjectCapabilities {
            analyzer: ProjectAnalyzerCapabilities { lint: Some(lint) },
        }
    }
}

fn lint(_path: &RomePath, parse: AnyParse) -> Result<ProjectLintResult, WorkspaceError> {
    let mut node_js_project = NodeJsProject::default();
    let tree: JsonRoot = parse.tree();
    node_js_project.from_root(&tree);

    let diagnostic_count = node_js_project.diagnostics.len() as u64;
    let errors = node_js_project
        .diagnostics
        .iter()
        .filter(|diag| diag.severity() <= Severity::Error)
        .count();

    node_js_project.analyze();

    let skipped_diagnostics = diagnostic_count - node_js_project.diagnostics.len() as u64;

    Ok(ProjectLintResult {
        diagnostics: node_js_project
            .diagnostics
            .into_iter()
            .map(SerdeDiagnostic::new)
            .collect(),
        errors,
        skipped_diagnostics,
    })
}
