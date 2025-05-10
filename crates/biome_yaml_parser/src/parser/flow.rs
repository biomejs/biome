use biome_parser::{
    CompletedMarker, Parser,
    prelude::{
        ParsedSyntax::{self, *},
        TokenSource,
    },
};
use biome_yaml_syntax::YamlSyntaxKind::{self, *};

use crate::lexer::YamlLexContext;

use super::YamlParser;

pub(crate) fn parse_any_flow_node(p: &mut YamlParser, context: YamlLexContext) -> CompletedMarker {
    parse_flow_yaml_node(p, context)
}

// TODO: parse node properties
pub(crate) fn parse_flow_yaml_node(p: &mut YamlParser, context: YamlLexContext) -> CompletedMarker {
    let m = p.start();
    parse_plain_scalar(p, context);
    m.complete(p, YAML_FLOW_YAML_NODE)
}

fn parse_plain_scalar(p: &mut YamlParser, context: YamlLexContext) -> CompletedMarker {
    p.re_lex(context);
    let m = p.start();
    p.bump(PLAIN_LITERAL);
    m.complete(p, YAML_PLAIN_SCALAR)
}

pub(crate) fn is_at_any_flow_node(p: &YamlParser) -> bool {
    is_at_flow_yaml_node(p)
}

pub(crate) fn is_at_flow_yaml_node(p: &YamlParser) -> bool {
    is_at_plain_scalar(p)
}

fn is_at_plain_scalar(p: &YamlParser) -> bool {
    p.at(PLAIN_LITERAL)
}
