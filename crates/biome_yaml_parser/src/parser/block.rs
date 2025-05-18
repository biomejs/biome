use biome_parser::{
    CompletedMarker, Marker, Parser,
    parse_lists::ParseNodeList,
    parse_recovery::ParseRecovery,
    prelude::ParsedSyntax::{self, *},
};
use biome_yaml_syntax::{
    T,
    YamlSyntaxKind::{self, *},
};

use crate::lexer::YamlLexContext;

use super::{
    YamlParser,
    flow::{is_at_flow_yaml_node, parse_flow_yaml_node},
    parse_error::expected_block_mapping,
};

pub(crate) fn parse_any_block_node(p: &mut YamlParser) -> ParsedSyntax {
    if maybe_at_implicit_mapping_key(p) {
        Present(parse_implicit_entry_or_flow_node(p))
    } else if is_at_explicit_mapping_key(p) {
        Present(parse_block_collection(p))
    } else if is_at_block_scalar(p) {
        Present(parse_block_scalar(p))
    } else {
        Absent
    }
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

fn parse_block_collection(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    parse_block_mapping(p);
    m.complete(p, YAML_BLOCK_COLLECTION)
}

/// Since the block key context's only difference with flow context is that it disallows multiline
/// flow node, we can just parse a flow node and emit a diagnostic when we encounter a key that
/// violate the restriction.lexer
fn parse_implicit_entry_or_flow_node(p: &mut YamlParser) -> CompletedMarker {
    let implicit_key_marker = parse_block_map_implicit_key(p);
    // It's indeed an implicit key
    if p.at(T![:]) {
        let implicit_entry_marker = implicit_key_marker.precede(p);
        parse_block_map_implicit_value(p);
        let implicit_entry_marker =
            implicit_entry_marker.complete(p, YAML_BLOCK_MAP_IMPLICIT_ENTRY);
        parse_block_collection_starting_with_implicit_entry(p, implicit_entry_marker)
    }
    // It's just a flow node
    else {
        let block_node_marker = implicit_key_marker.precede(p);
        p.eat(NEWLINE);
        block_node_marker.complete(p, YAML_FLOW_IN_BLOCK_NODE)
    }
}

fn parse_block_collection_starting_with_implicit_entry(
    p: &mut YamlParser,
    implicit_entry_marker: CompletedMarker,
) -> CompletedMarker {
    let block_mapping_marker =
        parse_block_mapping_starting_with_implicit_entry(p, implicit_entry_marker);
    let block_collection_marker = block_mapping_marker.precede(p);
    block_collection_marker.complete(p, YAML_BLOCK_COLLECTION)
}

fn parse_block_mapping(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    BlockMapEntryList::default().parse_list(p);
    m.complete(p, YAML_BLOCK_MAPPING)
}

fn parse_block_mapping_starting_with_implicit_entry(
    p: &mut YamlParser,
    implicit_entry_marker: CompletedMarker,
) -> CompletedMarker {
    let block_map_entry_list_marker = implicit_entry_marker.precede(p);
    let block_map_entry_list_marker =
        BlockMapEntryList::with(block_map_entry_list_marker).parse_list(p);
    let block_map_marker = block_map_entry_list_marker.precede(p);
    block_map_marker.complete(p, YAML_BLOCK_MAPPING)
}

#[derive(Default)]
pub(crate) struct BlockMapEntryList {
    start_marker: Option<Marker>,
}

impl BlockMapEntryList {
    fn with(start_marker: Marker) -> Self {
        Self {
            start_marker: Some(start_marker),
        }
    }
}

impl ParseNodeList for BlockMapEntryList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_BLOCK_MAP_ENTRY_LIST;

    fn start_list(&mut self, p: &mut Self::Parser<'_>) -> Marker {
        self.start_marker.take().unwrap_or_else(|| p.start())
    }

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
    // Value can be omitted in an explicit entry
    parse_block_map_explicit_value(p).ok();
    Present(m.complete(p, YAML_BLOCK_MAP_EXPLICIT_ENTRY))
}

fn parse_block_map_explicit_key(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![?]);
    parse_block_indented(p).ok();
    m.complete(p, YAML_BLOCK_MAP_EXPLICIT_KEY)
}

fn parse_block_map_explicit_value(p: &mut YamlParser) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![:]);
    parse_block_indented(p).ok();
    Present(m.complete(p, YAML_BLOCK_MAP_EXPLICIT_VALUE))
}

fn parse_block_indented(p: &mut YamlParser) -> ParsedSyntax {
    parse_any_block_node(p)
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
    let value = parse_any_block_node(p);
    if value.is_absent() {
        p.eat(NEWLINE);
    }
    m.complete(p, YAML_BLOCK_MAP_IMPLICIT_VALUE)
}

fn is_at_explicit_mapping_key(p: &mut YamlParser) -> bool {
    p.at(T![?])
}

fn maybe_at_implicit_mapping_key(p: &YamlParser) -> bool {
    is_at_flow_yaml_node(p)
}

fn is_at_block_scalar(p: &YamlParser) -> bool {
    p.at(LITERAL_BLOCK_LITERAL) || p.at(FOLDED_BLOCK_LITERAL)
}
