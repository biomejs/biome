use crate::{ControlFlowModel, control_flow_model};
use biome_db::{AnyParsedSource, Db, ParsedSnippet, ParsedSource};

#[salsa::tracked]
pub fn control_flow_model_from_source(db: &dyn Db, file: ParsedSource) -> ControlFlowModel {
    control_flow_model(&file.parsed(db).tree())
}

#[salsa::tracked]
pub fn control_flow_model_from_snippet(db: &dyn Db, file: ParsedSnippet) -> ControlFlowModel {
    control_flow_model(&file.parsed(db).tree())
}

/// Returns the cached CFG model for a parsed JavaScript source or snippet.
pub fn js_control_flow_model<'db>(
    db: &'db dyn Db,
    file: &AnyParsedSource,
) -> &'db ControlFlowModel {
    match file {
        AnyParsedSource::ParsedSource(source) => control_flow_model_from_source(db, *source),
        AnyParsedSource::ParsedSnippet(snippet) => control_flow_model_from_snippet(db, *snippet),
    }
}
