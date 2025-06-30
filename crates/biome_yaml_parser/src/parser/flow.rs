use biome_parser::{CompletedMarker, Parser};
use biome_yaml_syntax::YamlSyntaxKind::*;

use super::YamlParser;

// TODO: parse node properties
pub(crate) fn parse_flow_yaml_node(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    parse_plain_scalar(p);
    m.complete(p, YAML_FLOW_YAML_NODE)
}

fn parse_plain_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(PLAIN_LITERAL);
    m.complete(p, YAML_PLAIN_SCALAR)
}

pub(crate) fn is_at_flow_yaml_node(p: &mut YamlParser) -> bool {
    is_at_plain_scalar(p)
}

fn is_at_plain_scalar(p: &mut YamlParser) -> bool {
    p.at(PLAIN_LITERAL)
}
