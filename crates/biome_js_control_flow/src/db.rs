use crate::{ControlFlowModel, control_flow_model};
use biome_db::Source;
use biome_parser::db::{AnalyzedSource, ParseDb, SourceExt, TrackedSnippet};

// The model holds the syntax tree of the file, so it has the same cache limit
// as the parse.
#[salsa::tracked(lru = 200)]
pub fn control_flow_model_from_source(db: &dyn ParseDb, file: Source) -> ControlFlowModel {
    control_flow_model(&file.parsed(db).tree())
}

// The model holds the syntax tree of the snippet, so it has the same cache
// limit as the parse.
#[salsa::tracked(lru = 200)]
pub fn control_flow_model_from_snippet<'db>(
    db: &'db dyn ParseDb,
    file: TrackedSnippet<'db>,
) -> ControlFlowModel {
    control_flow_model(&file.parsed(db).tree())
}

/// Returns the cached CFG model for a parsed JavaScript source or snippet.
///
/// Returns `None` for a snippet that the current parse of its document doesn't
/// contain anymore.
pub fn js_control_flow_model<'db>(
    db: &'db dyn ParseDb,
    file: &AnalyzedSource,
) -> Option<&'db ControlFlowModel> {
    match file {
        AnalyzedSource::Source(source) => Some(control_flow_model_from_source(db, *source)),
        AnalyzedSource::Snippet(snippet) => snippet
            .resolve(db)
            .map(|snippet| control_flow_model_from_snippet(db, snippet)),
    }
}
