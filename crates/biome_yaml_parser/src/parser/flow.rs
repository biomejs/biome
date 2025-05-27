use biome_parser::{CompletedMarker, Parser};
use biome_yaml_syntax::YamlSyntaxKind::*;

use crate::lexer::YamlLexContext;

use super::YamlParser;

// TODO: parse node properties
pub(crate) fn parse_flow_yaml_node(p: &mut YamlParser, context: YamlLexContext) -> CompletedMarker {
    let m = p.start();
    parse_plain_scalar(p, context);
    m.complete(p, YAML_FLOW_YAML_NODE)
}

fn parse_plain_scalar(p: &mut YamlParser, context: YamlLexContext) -> CompletedMarker {
    // The current plain token was lexed during the last `bump` or `bump_with_context` call, which
    // might have used a different context than the current one
    p.re_lex(context);
    let m = p.start();
    p.bump(PLAIN_LITERAL);
    m.complete(p, YAML_PLAIN_SCALAR)
}

pub(crate) fn is_at_flow_yaml_node(p: &YamlParser) -> bool {
    is_at_plain_scalar(p)
}

fn is_at_plain_scalar(p: &YamlParser) -> bool {
    p.at(PLAIN_LITERAL)
}
