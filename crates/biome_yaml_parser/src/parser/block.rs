use biome_parser::{CompletedMarker, Parser};
use biome_yaml_syntax::YamlSyntaxKind::*;

use super::YamlParser;

#[expect(dead_code)]
pub(crate) fn parse_any_block(p: &mut YamlParser) -> CompletedMarker {
    parse_literal_scalar(p)
}

fn parse_literal_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();

    p.expect(LITERAL_BLOCK_LITERAL);

    m.complete(p, YAML_LITERAL_SCALAR)
}
