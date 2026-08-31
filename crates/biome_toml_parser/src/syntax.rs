use crate::{lexer::TomlLexContext, parser::TomlParser};
use biome_parser::{
    Parser, ParserProgress,
    diagnostic::expected_node,
    parsed_syntax::{ParsedSyntax, ParsedSyntax::Absent, ParsedSyntax::Present},
    prelude::{CompletedMarker, Marker},
};
use biome_toml_syntax::{T, TomlSyntaxKind, TomlSyntaxKind::*};

const KEY_START: biome_parser::token_set::TokenSet<TomlSyntaxKind> =
    biome_parser::token_set![TOML_BARE_KEY, TOML_BASIC_STRING, TOML_LITERAL_STRING];

pub(crate) fn parse_root(parser: &mut TomlParser) -> CompletedMarker {
    let root = parser.start();
    parser.eat(UNICODE_BOM);
    parse_item_list(parser);
    parser.expect(EOF);
    root.complete(parser, TOML_ROOT)
}

fn parse_item_list(parser: &mut TomlParser) {
    let list = parser.start();
    let mut first = true;
    let mut progress = ParserProgress::default();

    while !parser.at(EOF) {
        progress.assert_progressing(parser);
        if !first && !parser.has_preceding_line_break() {
            parser.error(parser.err_builder(
                "TOML items must be separated by a line break",
                parser.cur_range(),
            ));
        }
        first = false;

        if parse_item(parser).is_absent() {
            recover_item(parser);
        }
    }

    list.complete(parser, TOML_ITEM_LIST);
}

fn parse_item(parser: &mut TomlParser) -> ParsedSyntax {
    if parser.at(T!['[']) {
        parse_table(parser)
    } else if parser.at_ts(KEY_START) {
        parse_key_value(parser)
    } else {
        Absent
    }
}

fn parse_table(parser: &mut TomlParser) -> ParsedSyntax {
    if !parser.at(T!['[']) {
        return Absent;
    }

    let table = parser.start();
    let opening_end = parser.cur_range().end();
    parser.bump(T!['[']);
    let mut header_has_line_break = parser.has_preceding_line_break();
    let second_opening_start = parser.cur_range().start();
    let array_table = parser.eat(T!['[']);
    if array_table && opening_end != second_opening_start {
        parser.error(parser.err_builder(
            "The brackets in a TOML array table delimiter must be adjacent",
            opening_end..second_opening_start,
        ));
    }
    header_has_line_break |= parser.has_preceding_line_break();
    if header_has_line_break {
        parser.error(parser.err_builder(
            "A TOML table header must remain on one line",
            parser.cur_range(),
        ));
    }
    parse_key(parser).or_add_diagnostic(parser, expected_key);
    if parser.has_preceding_line_break() {
        parser.error(parser.err_builder(
            "A TOML table header must remain on one line",
            parser.cur_range(),
        ));
    }
    let first_closing_end = parser.at(T![']']).then_some(parser.cur_range().end());
    parser.expect(T![']']);
    if array_table {
        if parser.has_preceding_line_break() {
            parser.error(parser.err_builder(
                "A TOML table header must remain on one line",
                parser.cur_range(),
            ));
        }
        if let Some(first_closing_end) = first_closing_end
            && parser.at(T![']'])
            && first_closing_end != parser.cur_range().start()
        {
            parser.error(parser.err_builder(
                "The brackets in a TOML array table delimiter must be adjacent",
                first_closing_end..parser.cur_range().start(),
            ));
        }
        parser.expect(T![']']);
    }

    let kind = if array_table {
        TOML_ARRAY_TABLE
    } else {
        TOML_TABLE
    };
    Present(table.complete(parser, kind))
}

fn parse_key_value(parser: &mut TomlParser) -> ParsedSyntax {
    match parse_key_value_prefix(parser) {
        KeyValuePrefix::Absent => Absent,
        KeyValuePrefix::Complete(key_value) => Present(key_value),
        KeyValuePrefix::Value(key_value) => {
            parse_value(parser, false).or_add_diagnostic(parser, expected_value);
            Present(key_value.complete(parser, TOML_KEY_VALUE))
        }
    }
}

enum KeyValuePrefix {
    Absent,
    Complete(CompletedMarker),
    Value(Marker),
}

fn parse_key_value_prefix(parser: &mut TomlParser) -> KeyValuePrefix {
    if !parser.at_ts(KEY_START) {
        return KeyValuePrefix::Absent;
    }

    let key_value = parser.start();
    let key_range = parse_key(parser)
        .or_add_diagnostic(parser, expected_key)
        .map(|key| key.range(parser));
    if parser.has_preceding_line_break() {
        parser.error(parser.err_builder(
            "Expected `=` after TOML key",
            key_range.unwrap_or_else(|| parser.cur_range()),
        ));
        return KeyValuePrefix::Complete(key_value.complete(parser, TOML_KEY_VALUE));
    }
    let equals_range = if parser.at(T![=]) {
        let equals_range = parser.cur_range();
        parser.bump_with_context(T![=], TomlLexContext::Value);
        Some(equals_range)
    } else {
        parser.expect(T![=]);
        None
    };

    let Some(equals_range) = equals_range else {
        return KeyValuePrefix::Complete(key_value.complete(parser, TOML_KEY_VALUE));
    };
    if parser.has_preceding_line_break() {
        parser.error(parser.err_builder("Expected a TOML value after `=`", equals_range));
        KeyValuePrefix::Complete(key_value.complete(parser, TOML_KEY_VALUE))
    } else {
        KeyValuePrefix::Value(key_value)
    }
}

fn parse_key(parser: &mut TomlParser) -> ParsedSyntax {
    if !parser.at_ts(KEY_START) {
        return Absent;
    }

    let key = parser.start();
    let segments = parser.start();
    parse_key_segment(parser).ok();
    while parser.at(T![.]) {
        if parser.has_preceding_line_break() {
            parser.error(parser.err_builder(
                "A dotted TOML key must remain on one line",
                parser.cur_range(),
            ));
            break;
        }
        parser.bump(T![.]);
        if parser.has_preceding_line_break() {
            parser.error(parser.err_builder(
                "A dotted TOML key must remain on one line",
                parser.cur_range(),
            ));
            break;
        }
        parse_key_segment(parser).or_add_diagnostic(parser, expected_key);
    }
    segments.complete(parser, TOML_KEY_SEGMENT_LIST);

    Present(key.complete(parser, TOML_KEY))
}

fn parse_key_segment(parser: &mut TomlParser) -> ParsedSyntax {
    if !parser.at_ts(KEY_START) {
        return Absent;
    }

    let segment = parser.start();
    parser.bump_any();
    Present(segment.complete(parser, TOML_KEY_SEGMENT))
}

fn parse_value(parser: &mut TomlParser, allow_line_break: bool) -> ParsedSyntax {
    if !allow_line_break && parser.has_preceding_line_break() {
        return Absent;
    }

    match parser.cur() {
        T!['['] => parse_container(parser, ContainerKind::Array),
        T!['{'] => parse_container(parser, ContainerKind::InlineTable),
        _ => parse_scalar_value(parser, allow_line_break),
    }
}

fn parse_scalar_value(parser: &mut TomlParser, allow_line_break: bool) -> ParsedSyntax {
    if !allow_line_break && parser.has_preceding_line_break() {
        return Absent;
    }

    let kind = match parser.cur() {
        TOML_BASIC_STRING | TOML_LITERAL_STRING => TOML_STRING_VALUE,
        TOML_INTEGER => TOML_INTEGER_VALUE,
        TOML_FLOAT => TOML_FLOAT_VALUE,
        TOML_BOOLEAN => TOML_BOOLEAN_VALUE,
        TOML_OFFSET_DATE_TIME => TOML_OFFSET_DATE_TIME_VALUE,
        TOML_LOCAL_DATE_TIME => TOML_LOCAL_DATE_TIME_VALUE,
        TOML_LOCAL_DATE => TOML_LOCAL_DATE_VALUE,
        TOML_LOCAL_TIME => TOML_LOCAL_TIME_VALUE,
        ERROR_TOKEN => TOML_BOGUS_VALUE,
        _ => return Absent,
    };

    let value = parser.start();
    parser.bump_any();
    Present(value.complete(parser, kind))
}

#[derive(Clone, Copy)]
enum ContainerKind {
    Array,
    InlineTable,
}

impl ContainerKind {
    const fn closing(self) -> TomlSyntaxKind {
        match self {
            Self::Array => T![']'],
            Self::InlineTable => T!['}'],
        }
    }

    const fn list_kind(self) -> TomlSyntaxKind {
        match self {
            Self::Array => TOML_ARRAY_ELEMENT_LIST,
            Self::InlineTable => TOML_INLINE_TABLE_ELEMENT_LIST,
        }
    }

    const fn node_kind(self) -> TomlSyntaxKind {
        match self {
            Self::Array => TOML_ARRAY,
            Self::InlineTable => TOML_INLINE_TABLE,
        }
    }
}

struct Container {
    kind: ContainerKind,
    node: Marker,
    list: Marker,
    expecting_item: bool,
}

fn start_container(parser: &mut TomlParser, kind: ContainerKind) -> Container {
    let node = parser.start();
    match kind {
        ContainerKind::Array => {
            parser.bump_with_context(T!['['], TomlLexContext::ArrayValue);
        }
        ContainerKind::InlineTable => parser.bump(T!['{']),
    }
    let list = parser.start();
    Container {
        kind,
        node,
        list,
        expecting_item: true,
    }
}

fn nested_container_kind(parser: &TomlParser) -> Option<ContainerKind> {
    match parser.cur() {
        T!['['] => Some(ContainerKind::Array),
        T!['{'] => Some(ContainerKind::InlineTable),
        _ => None,
    }
}

fn parse_container(parser: &mut TomlParser, root_kind: ContainerKind) -> ParsedSyntax {
    let mut stack: Vec<(Container, Option<Marker>)> = Vec::new();
    let mut current = start_container(parser, root_kind);

    'containers: loop {
        let mut progress = ParserProgress::default();
        while !parser.at(current.kind.closing()) && !parser.at(EOF) {
            if matches!(parser.cur(), T![']'] | T!['}'])
                && stack
                    .iter()
                    .any(|(container, _)| parser.at(container.kind.closing()))
            {
                break;
            }
            progress.assert_progressing(parser);
            let preceding_line_break = parser.has_preceding_line_break();

            match current.kind {
                ContainerKind::Array => {
                    if preceding_line_break
                        && ((!current.expecting_item
                            && (parser.at(T!['[']) || parser.at_ts(KEY_START)))
                            || (current.expecting_item
                                && (parser.source().current_starts_key_value()
                                    || parser.source().current_starts_unambiguous_table_header())))
                    {
                        break;
                    }
                    if parser.at(T![,]) {
                        if current.expecting_item {
                            parser.error(expected_value(parser, parser.cur_range()));
                        }
                        parser.bump_with_context(T![,], TomlLexContext::ArrayValue);
                        current.expecting_item = true;
                        continue;
                    }
                    if !current.expecting_item {
                        parser.error(parser.err_builder(
                            "Expected a comma between array values",
                            parser.cur_range(),
                        ));
                    }

                    if let Some(kind) = nested_container_kind(parser) {
                        current.expecting_item = false;
                        stack.push((current, None));
                        current = start_container(parser, kind);
                        continue 'containers;
                    }

                    if parse_scalar_value(parser, true).is_absent() {
                        recover_separated_element(
                            parser,
                            T![']'],
                            TomlLexContext::ArrayValue,
                            TOML_BOGUS_VALUE,
                            "TOML value",
                        );
                    }
                    current.expecting_item = false;
                }
                ContainerKind::InlineTable => {
                    if preceding_line_break
                        && ((!current.expecting_item
                            && (parser.at(T!['[']) || parser.at_ts(KEY_START)))
                            || (current.expecting_item && parser.at(T!['['])))
                    {
                        break;
                    }
                    if parser.at(T![,]) {
                        if current.expecting_item {
                            parser.error(expected_value(parser, parser.cur_range()));
                        }
                        parser.bump(T![,]);
                        current.expecting_item = true;
                        continue;
                    }
                    if !current.expecting_item {
                        parser.error(parser.err_builder(
                            "Expected a comma between inline table entries",
                            parser.cur_range(),
                        ));
                    }

                    match parse_key_value_prefix(parser) {
                        KeyValuePrefix::Absent => {
                            recover_separated_element(
                                parser,
                                T!['}'],
                                TomlLexContext::Key,
                                TOML_BOGUS,
                                "TOML key-value pair",
                            );
                        }
                        KeyValuePrefix::Complete(_) => {}
                        KeyValuePrefix::Value(key_value) => {
                            if let Some(kind) = nested_container_kind(parser) {
                                current.expecting_item = false;
                                stack.push((current, Some(key_value)));
                                current = start_container(parser, kind);
                                continue 'containers;
                            }

                            parse_scalar_value(parser, false)
                                .or_add_diagnostic(parser, expected_value);
                            key_value.complete(parser, TOML_KEY_VALUE);
                        }
                    }
                    current.expecting_item = false;
                }
            }
        }

        let kind = current.kind;
        current.list.complete(parser, kind.list_kind());
        parser.expect(kind.closing());
        let node = current.node.complete(parser, kind.node_kind());

        let Some((parent, key_value)) = stack.pop() else {
            return Present(node);
        };
        if let Some(key_value) = key_value {
            key_value.complete(parser, TOML_KEY_VALUE);
        }
        current = parent;
    }
}

fn recover_item(parser: &mut TomlParser) -> CompletedMarker {
    let bogus = parser.start();
    let range = parser.cur_range();
    let missing_key = parser.at(T![=]);
    parser.bump_any();
    while !parser.at(EOF) && !parser.has_preceding_line_break() {
        parser.bump_any();
    }
    if missing_key {
        parser.error(parser.err_builder("Expected a TOML key before `=`", range));
    } else {
        parser.error(expected_node("key-value pair or table", range, parser));
    }
    bogus.complete(parser, TOML_BOGUS)
}

fn recover_separated_element(
    parser: &mut TomlParser,
    closing: TomlSyntaxKind,
    context: TomlLexContext,
    bogus_kind: TomlSyntaxKind,
    expected: &'static str,
) -> Option<CompletedMarker> {
    if parser.at(EOF) || parser.at(closing) {
        parser.error(expected_node(expected, parser.cur_range(), parser));
        return None;
    }

    let bogus = parser.start();
    let range = parser.cur_range();
    parser.bump_any_with_context(context);
    while !parser.at(EOF)
        && !parser.at(closing)
        && !parser.at(T![,])
        && !(parser.has_preceding_line_break() && (parser.at(T!['[']) || parser.at_ts(KEY_START)))
    {
        parser.bump_any_with_context(context);
    }
    parser.error(expected_node(expected, range, parser));
    Some(bogus.complete(parser, bogus_kind))
}

fn expected_key(parser: &TomlParser, range: biome_rowan::TextRange) -> ParseDiagnostic {
    expected_node("TOML key", range, parser)
}

fn expected_value(parser: &TomlParser, range: biome_rowan::TextRange) -> ParseDiagnostic {
    expected_node("TOML value", range, parser)
}

use biome_parser::prelude::ParseDiagnostic;
