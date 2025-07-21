use std::cell::RefCell;

use biome_parser::{
    CompletedMarker, Parser,
    parse_lists::ParseNodeList,
    parse_recovery::ParseRecovery,
    prelude::ParsedSyntax::{self, *},
};
use biome_yaml_syntax::{
    T,
    YamlSyntaxKind::{self, *},
};

use crate::parser::flow::parse_any_flow_node;

use super::{
    YamlParser,
    flow::{
        is_at_flow_json_node, is_at_flow_yaml_node, parse_flow_json_node, parse_flow_yaml_node,
    },
    parse_error::{expected_block_mapping_entry, expected_block_sequence_entry},
};

pub(crate) fn parse_any_block_node(p: &mut YamlParser) -> ParsedSyntax {
    if p.at(MAPPING_START) {
        Present(parse_block_mapping(p))
    } else if p.at(SEQUENCE_START) {
        Present(parse_block_sequence(p))
    } else if p.at(FLOW_START) {
        Present(parse_flow_in_block_node(p))
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

fn parse_block_mapping(p: &mut YamlParser) -> CompletedMarker {
    debug_assert!(p.at(MAPPING_START));
    let m = p.start();
    p.bump(MAPPING_START);
    BlockMapEntryList.parse_list(p);
    debug_assert!(p.at(MAPPING_END));
    p.expect(MAPPING_END);
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
        p.at(MAPPING_END)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &BlockMapEntryListParseRecovery::new(),
            expected_block_mapping_entry,
        )
    }
}

struct BlockMapEntryListParseRecovery {
    /// Track the number of nested mapping encountered, so that the parser can always deal with
    /// `MAPPING_START` and `MAPPING_END` in pair
    num_nested_mapping: RefCell<usize>,
}

impl BlockMapEntryListParseRecovery {
    fn new() -> Self {
        Self {
            num_nested_mapping: RefCell::new(0),
        }
    }
}

impl ParseRecovery for BlockMapEntryListParseRecovery {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;
    const RECOVERED_KIND: Self::Kind = YAML_BOGUS_BLOCK_MAP_ENTRY;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        if p.at(MAPPING_START) {
            self.num_nested_mapping.replace_with(|nested| *nested + 1);
            false
        } else if p.at(MAPPING_END) {
            self.num_nested_mapping
                .replace_with(|nested| nested.saturating_sub(1));
            *self.num_nested_mapping.borrow() == 0
        } else {
            false
        }
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
    p.bump(T![?]);
    // Explicit mapping key can be omitted as long as `?` exists
    parse_any_block_node(p).ok();

    // Value can be omitted in an explicit entry
    if p.at(T![:]) {
        p.bump(T![:]);
        parse_any_block_node(p).ok();
    }

    Present(m.complete(p, YAML_BLOCK_MAP_EXPLICIT_ENTRY))
}

fn parse_block_map_implicit_entry(p: &mut YamlParser) -> ParsedSyntax {
    if is_at_flow_yaml_node(p) {
        let m = p.start();
        parse_flow_yaml_node(p);

        p.bump(COLON);
        // Value can be completely empty according to the spec
        parse_any_block_node(p).ok();
        Present(m.complete(p, YAML_BLOCK_MAP_IMPLICIT_ENTRY))
    } else if is_at_flow_json_node(p) {
        let m = p.start();
        parse_flow_json_node(p);

        p.bump(COLON);
        // Value can be completely empty according to the spec
        parse_any_block_node(p).ok();
        Present(m.complete(p, YAML_BLOCK_MAP_IMPLICIT_ENTRY))
    } else {
        Absent
    }
}

fn parse_block_sequence(p: &mut YamlParser) -> CompletedMarker {
    debug_assert!(p.at(SEQUENCE_START));
    let m = p.start();
    p.bump(SEQUENCE_START);
    BlockSequenceEntryList.parse_list(p);
    debug_assert!(p.at(SEQUENCE_END));
    p.expect(SEQUENCE_END);
    m.complete(p, YAML_BLOCK_SEQUENCE)
}

#[derive(Default)]
pub(crate) struct BlockSequenceEntryList;

impl ParseNodeList for BlockSequenceEntryList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_BLOCK_SEQUENCE_ENTRY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_block_sequence_entry(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(SEQUENCE_END)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover(
            p,
            &BlockSequenceEntryListParseRecovery::new(),
            expected_block_sequence_entry,
        )
    }
}

struct BlockSequenceEntryListParseRecovery {
    /// Track the number of nested sequence encountered, so that the parser can always deal with
    /// `SEQUENCE_START` and `SEQUENCE_END` in pair
    num_nested: RefCell<usize>,
}

impl BlockSequenceEntryListParseRecovery {
    fn new() -> Self {
        Self {
            // Since the lexer must have been inside a mapping
            num_nested: RefCell::new(0),
        }
    }
}

impl ParseRecovery for BlockSequenceEntryListParseRecovery {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;
    const RECOVERED_KIND: Self::Kind = YAML_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        if p.at(SEQUENCE_START) {
            self.num_nested.replace_with(|nested| *nested + 1);
            false
        } else if p.at(SEQUENCE_END) {
            self.num_nested
                .replace_with(|nested| nested.saturating_sub(1));
            *self.num_nested.borrow() == 0
        } else {
            false
        }
    }
}

fn parse_block_sequence_entry(p: &mut YamlParser) -> ParsedSyntax {
    if !p.at(T![-]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![-]);
    // A sequence entry's value can be empty, as long as `-` exists
    parse_any_block_node(p).ok();
    Present(m.complete(p, YAML_BLOCK_SEQUENCE_ENTRY))
}

fn parse_flow_in_block_node(p: &mut YamlParser) -> CompletedMarker {
    debug_assert!(p.at(FLOW_START));
    let m = p.start();
    p.expect(FLOW_START);
    parse_any_flow_node(p).ok();
    p.expect(FLOW_END);
    m.complete(p, YAML_FLOW_IN_BLOCK_NODE)
}

pub(crate) fn is_at_any_block_node(p: &YamlParser) -> bool {
    p.at(MAPPING_START) || p.at(SEQUENCE_START) || p.at(FLOW_START) || is_at_block_scalar(p)
}

fn is_at_explicit_mapping_key(p: &YamlParser) -> bool {
    p.at(T![?])
}

fn is_at_block_scalar(p: &YamlParser) -> bool {
    p.at(LITERAL_BLOCK_LITERAL) || p.at(FOLDED_BLOCK_LITERAL)
}
