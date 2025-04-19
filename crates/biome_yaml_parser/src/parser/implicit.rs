/// To prevent unbounded lookahead, YAML spec mandates that implicit keys must not be longer that
/// 1024 characters or spread across multiple lines
use biome_parser::{
    CompletedMarker, Parser,
    prelude::{
        ParsedSyntax::{self, *},
        TokenSource,
    },
};
use biome_rowan::TextSize;
use biome_yaml_syntax::{
    T,
    YamlSyntaxKind::{self, *},
};

use crate::lexer::YamlLexContext;

use super::YamlParser;

const MAX_IMPLICIT_KEY_SIZE: u32 = 1024;

pub(crate) struct ImplicitConstraintViolation;

pub(crate) fn try_parse_implicit_flow_yaml_node(
    p: &mut YamlParser,
) -> Result<ParsedSyntax, ImplicitConstraintViolation> {
    if !p.at(PLAIN_LITERAL) {
        return Ok(Absent);
    }
    let m = p.start();
    let start_pos = p.source().position();
    try_parse_plain_scalar(p, YamlLexContext::BlockKey, start_pos)?;
    Ok(Present(m.complete(p, YAML_FLOW_YAML_NODE)))
}

fn try_parse_plain_scalar(
    p: &mut YamlParser,
    context: YamlLexContext,
    start_pos: TextSize,
) -> Result<CompletedMarker, ImplicitConstraintViolation> {
    p.re_lex(context);
    let m = p.start();
    expect_in_implicit_constrain(p, PLAIN_LITERAL, start_pos);
    Ok(m.complete(p, YAML_PLAIN_SCALAR))
}

fn expect_in_implicit_constrain(
    p: &mut YamlParser,
    kind: YamlSyntaxKind,
    start_pos: TextSize,
) -> Result<bool, ImplicitConstraintViolation> {
    if p.source().position() - start_pos > MAX_IMPLICIT_KEY_SIZE.into() {
        return Err(ImplicitConstraintViolation);
    }
    if p.at(NEWLINE) {
        return Err(ImplicitConstraintViolation);
    }
    Ok(p.expect(kind))
}
