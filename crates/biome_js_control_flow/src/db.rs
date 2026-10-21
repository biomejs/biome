use crate::{ControlFlowModel, control_flow_model};
use biome_db::Source;
use biome_parser::db::{AnyParsedSource, ParseDb, ParsedSnippet, ParsedSourceExt};

#[salsa::tracked]
pub fn control_flow_model_from_source(db: &dyn ParseDb, file: Source) -> ControlFlowModel {
    control_flow_model(&file.parsed(db).tree())
}

#[salsa::tracked]
pub fn control_flow_model_from_snippet<'db>(
    db: &'db dyn ParseDb,
    file: ParsedSnippet<'db>,
) -> ControlFlowModel {
    control_flow_model(&file.parsed(db).tree())
}

/// Returns the cached CFG model for a parsed JavaScript source or snippet.
///
/// Returns `None` for a snippet that the current parse of its document doesn't
/// contain anymore.
pub fn js_control_flow_model<'db>(
    db: &'db dyn ParseDb,
    file: &AnyParsedSource,
) -> Option<&'db ControlFlowModel> {
    match file {
        AnyParsedSource::Source(source) => Some(control_flow_model_from_source(db, *source)),
        AnyParsedSource::ParsedSnippet(snippet) => snippet
            .resolve(db)
            .map(|snippet| control_flow_model_from_snippet(db, snippet)),
    }
}
