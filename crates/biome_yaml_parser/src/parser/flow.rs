use YamlSyntaxKind::*;
use biome_parser::{
    CompletedMarker, Parser,
    parse_lists::ParseSeparatedList,
    parse_recovery::{ParseRecovery, RecoveryResult},
    prelude::ParsedSyntax::{self, *},
};
use biome_yaml_syntax::{T, YamlSyntaxKind};

use super::{
    YamlParser,
    parse_error::{
        expected_flow_mapping_closing_quote, expected_flow_mapping_entry,
        expected_flow_sequence_closing_bracket, expected_flow_sequence_entry,
    },
};

pub(crate) fn parse_any_flow_node(p: &mut YamlParser) -> ParsedSyntax {
    if is_at_flow_json_node(p) {
        Present(parse_flow_json_node(p))
    } else if is_at_flow_yaml_node(p) {
        Present(parse_flow_yaml_node(p))
    } else {
        Absent
    }
}

pub(crate) fn parse_flow_json_node(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();

    if is_at_flow_sequence(p) {
        parse_flow_sequence(p);
    } else if is_at_flow_mapping(p) {
        parse_flow_mapping(p);
    } else if is_at_double_quoted_scalar(p) {
        parse_double_quoted_scalar(p);
    } else if is_at_single_quoted_scalar(p) {
        parse_single_quoted_scalar(p);
    }

    m.complete(p, YAML_FLOW_JSON_NODE)
}

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

fn parse_double_quoted_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(DOUBLE_QUOTED_LITERAL);
    m.complete(p, YAML_DOUBLE_QUOTED_SCALAR)
}

fn parse_single_quoted_scalar(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(SINGLE_QUOTED_LITERAL);
    m.complete(p, YAML_SINGLE_QUOTED_SCALAR)
}

fn parse_flow_sequence(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();

    p.bump(T!['[']);
    FlowSequenceEntryList.parse_list(p);
    if !p.eat(T![']']) {
        p.error(expected_flow_sequence_closing_bracket(p.cur_range()));
    }

    m.complete(p, YAML_FLOW_SEQUENCE)
}

fn parse_flow_mapping(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();

    p.bump(T!['{']);
    FlowMapEntryList.parse_list(p);
    if !p.eat(T!['}']) {
        p.error(expected_flow_mapping_closing_quote(p.cur_range()));
    }

    m.complete(p, YAML_FLOW_MAPPING)
}

struct FlowSequenceEntryRecovery;

impl ParseRecovery for FlowSequenceEntryRecovery {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;
    const RECOVERED_KIND: Self::Kind = YAML_BOGUS_FLOW_NODE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,]) || p.at(T![']']) || is_at_flow_yaml_node(p) || is_at_flow_json_node(p)
    }
}

#[derive(Default)]
struct FlowSequenceEntryList;

impl ParseSeparatedList for FlowSequenceEntryList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_FLOW_SEQUENCE_ENTRY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        // Flow sequence entry allows for a compact form for a mapping of single key/value pair
        // e.g. [a, b, c: d, e: f], which is equivalent to [a, b, {c: d}, {e: f}]
        if p.at(T![?]) {
            parse_flow_map_explicit_entry(p)
        } else if is_at_flow_yaml_node(p) {
            let m = p.start();
            let flow_yaml_node = parse_flow_yaml_node(p);
            if p.at(T![:]) {
                parse_flow_map_value(p);
                Present(m.complete(p, YAML_FLOW_MAP_IMPLICIT_ENTRY))
            } else {
                m.abandon(p);
                Present(flow_yaml_node)
            }
        } else if is_at_flow_json_node(p) {
            let m = p.start();
            let flow_json_node = parse_flow_json_node(p);
            if p.at(T![:]) {
                parse_flow_map_value(p);
                Present(m.complete(p, YAML_FLOW_MAP_IMPLICIT_ENTRY))
            } else {
                m.abandon(p);
                Present(flow_json_node)
            }
        } else if p.at(T![:]) {
            let m = p.start();
            parse_flow_map_value(p);
            Present(m.complete(p, YAML_FLOW_MAP_IMPLICIT_ENTRY))
        } else {
            let entry = parse_any_flow_node(p);
            if entry.is_absent() {
                p.error(expected_flow_sequence_entry(p, p.cur_range()));
            }
            entry
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![']']) || p.at(FLOW_END)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &FlowSequenceEntryRecovery, expected_flow_sequence_entry)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

struct FlowMapEntryRecovery;

impl ParseRecovery for FlowMapEntryRecovery {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;
    const RECOVERED_KIND: Self::Kind = YAML_BOGUS_FLOW_NODE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,])
            || p.at(T!['}'])
            || p.at(T![:])
            || is_at_flow_yaml_node(p)
            || is_at_flow_json_node(p)
    }
}

#[derive(Default)]
struct FlowMapEntryList;

impl ParseSeparatedList for FlowMapEntryList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_FLOW_MAP_ENTRY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        let entry = if p.at(T![?]) {
            parse_flow_map_explicit_entry(p)
        } else {
            parse_flow_map_implicit_entry(p)
        };
        if entry.is_absent() {
            p.error(expected_flow_mapping_entry(p, p.cur_range()));
        }
        entry
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}']) || p.at(FLOW_END)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &FlowMapEntryRecovery, expected_flow_mapping_entry)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_flow_map_explicit_entry(p: &mut YamlParser) -> ParsedSyntax {
    let m = p.start();

    p.bump(T![?]);

    // The entry after '?' is optional
    if is_at_flow_yaml_node(p) {
        parse_flow_yaml_node(p);
        if p.at(T![:]) {
            parse_flow_map_value(p);
        }
    } else if is_at_flow_json_node(p) {
        parse_flow_json_node(p);
        if p.at(T![:]) {
            parse_flow_map_value(p);
        }
    } else if p.at(T![:]) {
        parse_flow_map_value(p);
    }

    Present(m.complete(p, YAML_FLOW_MAP_EXPLICIT_ENTRY))
}

fn parse_flow_map_implicit_entry(p: &mut YamlParser) -> ParsedSyntax {
    if is_at_flow_yaml_node(p) {
        let m = p.start();
        parse_flow_yaml_node(p);
        if p.at(T![:]) {
            parse_flow_map_value(p);
        }
        Present(m.complete(p, YAML_FLOW_MAP_IMPLICIT_ENTRY))
    } else if is_at_flow_json_node(p) {
        let m = p.start();
        parse_flow_json_node(p);
        if p.at(T![:]) {
            parse_flow_map_value(p);
        }
        Present(m.complete(p, YAML_FLOW_MAP_IMPLICIT_ENTRY))
    } else if p.at(T![:]) {
        let m = p.start();
        parse_flow_map_value(p);
        Present(m.complete(p, YAML_FLOW_MAP_IMPLICIT_ENTRY))
    } else {
        Absent
    }
}

fn parse_flow_map_value(p: &mut YamlParser) {
    debug_assert!(p.at(T![:]));

    p.bump(T![:]);
    // Value can be empty as long as `:` exists
    parse_any_flow_node(p).ok();
}

pub(crate) fn is_at_flow_json_node(p: &YamlParser) -> bool {
    is_at_flow_sequence(p)
        || is_at_flow_mapping(p)
        || is_at_double_quoted_scalar(p)
        || is_at_single_quoted_scalar(p)
}

fn is_at_flow_sequence(p: &YamlParser) -> bool {
    p.at(T!['['])
}

fn is_at_flow_mapping(p: &YamlParser) -> bool {
    p.at(T!['{'])
}

fn is_at_double_quoted_scalar(p: &YamlParser) -> bool {
    p.at(DOUBLE_QUOTED_LITERAL)
}

fn is_at_single_quoted_scalar(p: &YamlParser) -> bool {
    p.at(SINGLE_QUOTED_LITERAL)
}

pub(crate) fn is_at_flow_yaml_node(p: &YamlParser) -> bool {
    is_at_plain_scalar(p)
}

fn is_at_plain_scalar(p: &YamlParser) -> bool {
    p.at(PLAIN_LITERAL)
}
