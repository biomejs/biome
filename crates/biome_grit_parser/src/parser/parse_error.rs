use crate::parser::GritParser;
use biome_parser::diagnostic::expected_node;
use biome_parser::prelude::ParseDiagnostic;
use biome_parser::Parser;
use biome_rowan::TextRange;

pub(crate) fn expected_language_name(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected a supported language; must be one of `js`, `json`, `css`, `html`, or `grit`",
        range,
    )
}

pub(crate) fn expected_language_flavor(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "Expected a supported language flavor; must be one of `typescript` or `jsx`",
        range,
    )
}

pub(crate) fn expected_list_index(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("list index", range, p)
}

pub(crate) fn expected_map_key(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("map key", range, p)
}

pub(crate) fn expected_pattern(p: &GritParser, range: TextRange) -> ParseDiagnostic {
    expected_node("pattern", range, p)
}
