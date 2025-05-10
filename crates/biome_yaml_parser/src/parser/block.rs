use biome_parser::{
    CompletedMarker, Parser,
    parse_lists::ParseNodeList,
    parse_recovery::ParseRecovery,
    prelude::{
        ParsedSyntax::{self, *},
        TokenSource,
    },
};
use biome_yaml_syntax::{
    T,
    YamlSyntaxKind::{self, *},
};

use crate::lexer::YamlLexContext;

use super::{
    YamlParser,
    flow::{is_at_any_flow_node, is_at_flow_yaml_node, parse_any_flow_node, parse_flow_yaml_node},
    implicit::try_parse_implicit_flow_yaml_node,
    parse_error::expected_block_mapping,
};

pub(crate) fn parse_any_block_node(p: &mut YamlParser, context: YamlLexContext) -> ParsedSyntax {
    if is_at_block_in_block_node(p) {
        Present(parse_block_in_block_node(p))
    } else if is_at_any_flow_node(p) {
        Present(parse_flow_in_block_node(p))
    } else {
        Absent
    }
}

fn parse_flow_in_block_node(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    parse_any_flow_node(p, YamlLexContext::FlowOut);
    p.eat(NEWLINE);
    m.complete(p, YAML_FLOW_IN_BLOCK_NODE)
}

fn parse_block_in_block_node(p: &mut YamlParser) -> CompletedMarker {
    if is_at_block_scalar(p) {
        parse_block_scalar(p)
    } else {
        parse_block_collection(p)
    }
}

fn parse_block_collection(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    if p.indent_level != 0 {
        p.expect(INDENT);
    }
    p.indent_level += 1;
    parse_block_mapping(p);
    p.indent_level -= 1;
    p.eat(DEDENT);
    m.complete(p, YAML_BLOCK_COLLECTION)
}

fn parse_block_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    match p.cur() {
        LITERAL_BLOCK_LITERAL => {
            parse_literal_scalar(p);
        }
        FOLDED_BLOCK_LITERAL => {
            parse_folded_scalar(p);
        }
        _ => {}
    }
    m.complete(p, YAML_BLOCK_SCALAR)
}

fn parse_literal_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(LITERAL_BLOCK_LITERAL);
    m.complete(p, YAML_LITERAL_SCALAR)
}

fn parse_folded_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(FOLDED_BLOCK_LITERAL);
    m.complete(p, YAML_FOLDED_SCALAR)
}

fn parse_block_mapping(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    BlockMapEntryList.parse_list(p);
    m.complete(p, YAML_BLOCK_MAPPING)
}

#[derive(Default)]
pub(crate) struct BlockMapEntryList;

impl ParseNodeList for BlockMapEntryList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_BLOCK_MAP_ENTRY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_block_map_entry(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(DEDENT)
        // If this is the top level block mapping
        || p.at(DOC_END)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(p, &BlockMapEntryListParseRecovery, expected_block_mapping)
    }
}

struct BlockMapEntryListParseRecovery;

impl ParseRecovery for BlockMapEntryListParseRecovery {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;
    const RECOVERED_KIND: Self::Kind = YAML_BOGUS_BLOCK_MAP_ENTRY;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(DEDENT)
    }
}

fn parse_block_map_entry(p: &mut YamlParser) -> ParsedSyntax {
    if is_at_explicit_mapping_key(p) {
        parse_block_map_explicit_entry(p)
    } else {
        parse_block_map_implicit_entry(p)
    }
}

fn parse_block_map_explicit_entry(p: &mut YamlParser) -> ParsedSyntax {
    if !is_at_explicit_mapping_key(p) {
        return Absent;
    }
    let m = p.start();
    parse_block_map_explicit_key(p);
    parse_block_map_explicit_value(p);
    Present(m.complete(p, YAML_BLOCK_MAP_EXPLICIT_ENTRY))
}

fn parse_block_map_explicit_key(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(QUESTION);
    parse_block_indented(p, YamlLexContext::BlockOut).ok();
    m.complete(p, YAML_BLOCK_MAP_EXPLICIT_KEY)
}

fn parse_block_map_explicit_value(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.expect(COLON);
    parse_block_indented(p, YamlLexContext::BlockOut).ok();
    m.complete(p, YAML_BLOCK_MAP_EXPLICIT_VALUE)
}

fn parse_block_indented(p: &mut YamlParser, context: YamlLexContext) -> ParsedSyntax {
    if is_at_block_node(p) {
        parse_any_block_node(p, context)
    } else {
        Present(parse_compact_mapping(p))
    }
}

fn parse_compact_mapping(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    BlockMapEntryList.parse_list(p);
    m.complete(p, YAML_COMPACT_MAPPING)
}

fn parse_block_map_implicit_entry(p: &mut YamlParser) -> ParsedSyntax {
    if !is_at_flow_yaml_node(p) {
        return Absent;
    }
    let m = p.start();
    parse_block_map_implicit_key(p);
    parse_block_map_implicit_value(p);
    Present(m.complete(p, YAML_BLOCK_MAP_IMPLICIT_ENTRY))
}

fn parse_block_map_implicit_key(p: &mut YamlParser) -> CompletedMarker {
    parse_flow_yaml_node(p, YamlLexContext::BlockKey)
}

fn parse_block_map_implicit_value(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(COLON);
    // Value can be completely empty according to the spec
    let value = parse_any_block_node(p, YamlLexContext::BlockOut);
    if value.is_absent() {
        p.eat(NEWLINE);
    }
    m.complete(p, YAML_BLOCK_MAP_IMPLICIT_VALUE)
}

fn is_at_block_node(p: &mut YamlParser) -> bool {
    is_at_block_in_block_node(p) || is_at_any_flow_node(p)
}

fn is_at_block_in_block_node(p: &mut YamlParser) -> bool {
    is_at_block_collection(p)
}

fn is_at_block_collection(p: &mut YamlParser) -> bool {
    if p.indent_level > 0 && p.at(INDENT) {
        let checkpoint = p.checkpoint();
        p.bump(INDENT);
        let block_mapping_predicate = is_at_block_mapping(p);
        p.rewind(checkpoint);
        block_mapping_predicate
    } else if p.indent_level == 0 {
        is_at_block_mapping(p)
    } else {
        false
    }
}

fn is_at_block_mapping(p: &mut YamlParser) -> bool {
    is_at_explicit_mapping_key(p) || is_at_implicit_mapping_key(p)
}

fn is_at_explicit_mapping_key(p: &mut YamlParser) -> bool {
    while p.at(NEWLINE) {
        p.source_mut().bump();
    }
    p.at(QUESTION)
}

// Try to parse through the implicit key and check for the existence of the `:` indicator. Since
// most hand written yaml files' keys are really short, this should not impact the editor
// experience.
// Perhaps in the future we can try to optimize this out by first parsing the implicit key then
// later attach it to the block mapping node. However, it would not solve the problem where this is
// not an implicit key but just a normal flow node. In that case the parser still has to rewind and
// parse the flow node under FLOW_OUT context.
fn is_at_implicit_mapping_key(p: &mut YamlParser) -> bool {
    let checkpoint = p.checkpoint();
    let implicit_key = try_parse_implicit_flow_yaml_node(p);
    let violated_implicit_key_constraint = implicit_key.is_err();
    let absent = implicit_key.is_ok_and(|key| matches!(key, Absent));
    let is_at_indicator = p.at(T![:]);
    p.rewind(checkpoint);
    !violated_implicit_key_constraint && is_at_indicator && !absent
}

fn is_at_block_scalar(p: &YamlParser) -> bool {
    p.at(LITERAL_BLOCK_LITERAL) || p.at(FOLDED_BLOCK_LITERAL)
}
