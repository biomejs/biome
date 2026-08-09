use biome_parser::{
    CompletedMarker, Parser, TokenSet,
    parse_lists::ParseNodeList,
    parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryError, RecoveryResult},
    prelude::ParsedSyntax::{self, *},
    token_set,
};
use biome_yaml_syntax::{
    T,
    YamlSyntaxKind::{self, *},
};

use crate::parser::flow::parse_alias_node;
use crate::parser::{
    flow::parse_any_flow_node,
    parse_error::expected_header,
    property::{PropertyList, is_at_property},
};

use super::{
    YamlParser,
    flow::{
        is_at_alias_node, is_at_flow_json_node, is_at_flow_yaml_node, parse_flow_json_node,
        parse_flow_yaml_node,
    },
    parse_error::{expected_block_mapping_entry, expected_block_sequence_entry},
};

pub(crate) fn parse_any_block_node(p: &mut YamlParser) -> ParsedSyntax {
    if p.at(FLOW_START) {
        Present(parse_flow_in_block_node(p))
    } else if is_at_block_in_block_node(p) || is_at_property(p) {
        Present(parse_block_in_block_node(p))
    } else {
        Absent
    }
}

/// Parses the value of a block mapping entry, which the `:` on the key's
/// line precedes.
///
/// Bare properties on a line of their own are not part of the value: the
/// lexer wraps a scalar value in `FLOW_START`/`FLOW_END` and opens a nested
/// collection with a `MAPPING_START`/`SEQUENCE_START` in front of the
/// properties, so bare ones on their own line can only open the key of a
/// sibling entry:
///
/// ```yaml
/// b:
/// &anchor c: 3
/// ```
fn parse_block_map_entry_value(p: &mut YamlParser) -> ParsedSyntax {
    /// The flow scalar tokens that, following bare own-line properties,
    /// mark them as the next entry's key properties
    const FLOW_SCALARS: TokenSet<YamlSyntaxKind> =
        token_set![PLAIN_LITERAL, DOUBLE_QUOTED_LITERAL, SINGLE_QUOTED_LITERAL];

    if is_at_property(p)
        && p.has_preceding_line_break()
        && FLOW_SCALARS.contains(p.source_mut().kind_after_properties())
    {
        return Absent;
    }
    parse_any_block_node(p)
}

fn parse_block_in_block_node(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    PropertyList::default().parse_list(p);
    if p.at(MAPPING_START) {
        parse_block_mapping(p);
    } else if p.at(SEQUENCE_START) {
        parse_block_sequence(p);
    } else if p.at(T![|]) {
        parse_literal_scalar(p);
    } else if p.at(T![>]) {
        parse_folded_scalar(p);
    }
    m.complete(p, YAML_BLOCK_IN_BLOCK_NODE)
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
            &BlockMapEntryListParseRecovery,
            expected_block_mapping_entry,
        )
    }
}

struct BlockMapEntryListParseRecovery;

impl ParseRecovery for BlockMapEntryListParseRecovery {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;
    const RECOVERED_KIND: Self::Kind = YAML_BOGUS_BLOCK_MAP_ENTRY;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(MAPPING_END)
    }

    fn recover(&self, p: &mut Self::Parser<'_>) -> RecoveryResult {
        recover_balanced_collection(p, MAPPING_END, Self::RECOVERED_KIND)
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
        parse_block_map_entry_value(p).ok();
    }

    Present(m.complete(p, YAML_BLOCK_MAP_EXPLICIT_ENTRY))
}

fn parse_block_map_implicit_entry(p: &mut YamlParser) -> ParsedSyntax {
    let property_list = PropertyList::default().parse_list(p);
    let property_empty = property_list.range(p).is_empty();

    if is_at_flow_json_node(p) {
        let json_node = parse_flow_json_node(p, property_list);
        let m = json_node.precede(p);
        p.expect(T![:]);
        // Value can be completely empty according to the spec
        parse_block_map_entry_value(p).ok();
        Present(m.complete(p, YAML_BLOCK_MAP_IMPLICIT_ENTRY))
    } else if is_at_flow_yaml_node(p) || !property_empty {
        // plain yaml key, or empty key with properties
        let yaml_node = parse_flow_yaml_node(p, property_list);
        let m = yaml_node.precede(p);
        p.expect(T![:]);
        // Value can be completely empty according to the spec
        parse_block_map_entry_value(p).ok();
        Present(m.complete(p, YAML_BLOCK_MAP_IMPLICIT_ENTRY))
    } else if is_at_alias_node(p) {
        property_list.undo_completion(p).abandon(p);
        let alias_node = parse_alias_node(p);
        let m = alias_node.precede(p);
        p.expect(T![:]);
        // Value can be completely empty according to the spec
        parse_block_map_entry_value(p).ok();
        Present(m.complete(p, YAML_BLOCK_MAP_IMPLICIT_ENTRY))
    } else if p.at(T![:]) {
        // empty key
        property_list.undo_completion(p).abandon(p);
        let m = p.start();
        p.bump(T![:]);
        parse_block_map_entry_value(p).ok();
        Present(m.complete(p, YAML_BLOCK_MAP_IMPLICIT_ENTRY))
    } else {
        property_list.undo_completion(p).abandon(p);
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
            &BlockSequenceEntryListParseRecovery,
            expected_block_sequence_entry,
        )
    }
}

struct BlockSequenceEntryListParseRecovery;

impl ParseRecovery for BlockSequenceEntryListParseRecovery {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;
    const RECOVERED_KIND: Self::Kind = YAML_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(SEQUENCE_END)
    }

    fn recover(&self, p: &mut Self::Parser<'_>) -> RecoveryResult {
        recover_balanced_collection(p, SEQUENCE_END, Self::RECOVERED_KIND)
    }
}

fn recover_balanced_collection(
    p: &mut YamlParser,
    end_kind: YamlSyntaxKind,
    recovered_kind: YamlSyntaxKind,
) -> RecoveryResult {
    if p.at(EOF) {
        return Err(RecoveryError::Eof);
    }
    if p.at(end_kind) {
        return Err(RecoveryError::AlreadyRecovered);
    }
    if p.is_speculative_parsing() {
        return Err(RecoveryError::RecoveryDisabled);
    }

    let marker = p.start();
    let mut nested_end_kinds = Vec::new();
    while !p.at(EOF) {
        let mut closes_nested_collection = false;
        if p.at(MAPPING_START) {
            nested_end_kinds.push(MAPPING_END);
        } else if p.at(SEQUENCE_START) {
            nested_end_kinds.push(SEQUENCE_END);
        } else if nested_end_kinds.last().is_some_and(|kind| p.at(*kind)) {
            nested_end_kinds.pop();
            closes_nested_collection = nested_end_kinds.is_empty();
        } else if p.at(end_kind) && nested_end_kinds.is_empty() {
            break;
        }
        p.bump_any();
        if closes_nested_collection {
            break;
        }
    }

    Ok(marker.complete(p, recovered_kind))
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

fn parse_literal_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![|]);
    BlockHeaderList::default().parse_list(p);
    parse_block_content(p);
    m.complete(p, YAML_LITERAL_SCALAR)
}

fn parse_folded_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![>]);
    BlockHeaderList::default().parse_list(p);
    parse_block_content(p);
    m.complete(p, YAML_FOLDED_SCALAR)
}

#[derive(Default)]
pub(crate) struct BlockHeaderList {
    seen_chomping: bool,
    seen_indentation: bool,
}

impl ParseNodeList for BlockHeaderList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_BLOCK_HEADER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        match p.cur() {
            T![-] => {
                self.report_duplicate_chomping_indicator(p);
                Present(parse_strip_indicator(p))
            }
            T![+] => {
                self.report_duplicate_chomping_indicator(p);
                Present(parse_keep_indicator(p))
            }
            INDENTATION_INDICATOR => {
                if self.seen_indentation {
                    let diagnostic = p.err_builder(
                        "A block scalar can have only one indentation indicator.",
                        p.cur_range(),
                    );
                    p.error(diagnostic);
                }
                self.seen_indentation = true;
                Present(parse_indentation_indicator(p))
            }
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(BLOCK_CONTENT_LITERAL)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(YAML_BOGUS_BLOCK_HEADER, token_set![BLOCK_CONTENT_LITERAL]),
            expected_header,
        )
    }
}

impl BlockHeaderList {
    fn report_duplicate_chomping_indicator(&mut self, p: &mut YamlParser) {
        if self.seen_chomping {
            let diagnostic = p.err_builder(
                "A block scalar can have only one chomping indicator.",
                p.cur_range(),
            );
            p.error(diagnostic);
        }
        self.seen_chomping = true;
    }
}

fn parse_strip_indicator(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![-]);
    m.complete(p, YAML_BLOCK_STRIP_INDICATOR)
}

fn parse_keep_indicator(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![+]);
    m.complete(p, YAML_BLOCK_KEEP_INDICATOR)
}

fn parse_indentation_indicator(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(INDENTATION_INDICATOR);
    m.complete(p, YAML_INDENTATION_INDICATOR)
}

fn parse_block_content(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    // Could use bump here, since the lexer ensures that block content must follow the headers
    // but better be safe than sorry
    p.expect(BLOCK_CONTENT_LITERAL);
    m.complete(p, YAML_BLOCK_CONTENT)
}

fn is_at_block_in_block_node(p: &YamlParser) -> bool {
    p.at(MAPPING_START) || p.at(SEQUENCE_START) || p.at(T![|]) || p.at(T![>])
}

pub(crate) fn is_at_any_block_node(p: &YamlParser) -> bool {
    is_at_block_in_block_node(p) || p.at(FLOW_START) || is_at_property(p)
}

fn is_at_explicit_mapping_key(p: &YamlParser) -> bool {
    p.at(T![?])
}
