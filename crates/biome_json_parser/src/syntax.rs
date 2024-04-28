use crate::prelude::*;
use biome_json_syntax::JsonSyntaxKind;
use biome_json_syntax::JsonSyntaxKind::*;
use biome_parser::diagnostic::{expected_any, expected_node};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::Absent;
use biome_parser::prelude::ParsedSyntax::Present;
use biome_parser::ParserProgress;
use biome_rowan::TextRange;

const VALUE_START: TokenSet<JsonSyntaxKind> = token_set![
    T![null],
    T![true],
    T![false],
    JSON_STRING_LITERAL,
    JSON_NUMBER_LITERAL,
    T!['['],
    T!['{'],
];

const VALUE_RECOVERY_SET: TokenSet<JsonSyntaxKind> =
    VALUE_START.union(token_set![T![']'], T!['}'], T![,]]);

pub(crate) fn parse_root(p: &mut JsonParser) {
    let m = p.start();
    p.eat(UNICODE_BOM);

    let value = match parse_value(p) {
        Present(value) => Present(value),
        Absent => {
            p.error(expected_value(p, p.cur_range()));
            match ParseRecoveryTokenSet::new(JSON_BOGUS_VALUE, VALUE_START).recover(p) {
                Ok(value) => Present(value),
                Err(_) => Absent,
            }
        }
    };

    // Process the file to the end, e.g. in cases where there have been multiple values
    if !(p.at(EOF)) {
        parse_rest(p, value);
    }

    m.complete(p, JSON_ROOT);
}

fn parse_value(p: &mut JsonParser) -> ParsedSyntax {
    match p.cur() {
        T![null] => {
            let m = p.start();
            p.bump(T![null]);
            Present(m.complete(p, JSON_NULL_VALUE))
        }

        JSON_STRING_LITERAL => {
            let m = p.start();
            p.bump(JSON_STRING_LITERAL);
            Present(m.complete(p, JSON_STRING_VALUE))
        }

        TRUE_KW | FALSE_KW => {
            let m = p.start();
            p.bump(p.cur());
            Present(m.complete(p, JSON_BOOLEAN_VALUE))
        }

        JSON_NUMBER_LITERAL => {
            let m = p.start();
            p.bump(JSON_NUMBER_LITERAL);
            Present(m.complete(p, JSON_NUMBER_VALUE))
        }

        T!['{'] => parse_sequence(p, SequenceKind::Object),
        T!['['] => parse_sequence(p, SequenceKind::Array),

        IDENT => {
            let m = p.start();
            p.error(p.err_builder("String values must be double quoted.", p.cur_range()));
            p.bump(IDENT);
            Present(m.complete(p, JSON_BOGUS_VALUE))
        }

        ERROR_TOKEN => {
            // An error is already emitted by the lexer.
            let m = p.start();
            p.bump(ERROR_TOKEN);
            Present(m.complete(p, JSON_BOGUS_VALUE))
        }

        _ => Absent,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum SequenceKind {
    Array,
    Object,
}

impl SequenceKind {
    const fn node_kind(&self) -> JsonSyntaxKind {
        match self {
            SequenceKind::Array => JSON_ARRAY_VALUE,
            SequenceKind::Object => JSON_OBJECT_VALUE,
        }
    }

    const fn list_kind(&self) -> JsonSyntaxKind {
        match self {
            SequenceKind::Array => JSON_ARRAY_ELEMENT_LIST,
            SequenceKind::Object => JSON_MEMBER_LIST,
        }
    }

    const fn open_paren(&self) -> JsonSyntaxKind {
        match self {
            SequenceKind::Array => T!['['],
            SequenceKind::Object => T!['{'],
        }
    }

    const fn close_paren(&self) -> JsonSyntaxKind {
        match self {
            SequenceKind::Array => T![']'],
            SequenceKind::Object => T!['}'],
        }
    }
}

struct Sequence {
    kind: SequenceKind,
    node: Marker,
    list: Marker,
    state: SequenceState,
}

enum SequenceState {
    Start,
    Processing,
    Suspended(Option<Marker>),
}

impl Sequence {
    fn parse_item(&self, p: &mut JsonParser) -> SequenceItem {
        match self.kind {
            SequenceKind::Array => parse_array_element(p),
            SequenceKind::Object => parse_object_member(p),
        }
    }

    const fn recovery_set(&self) -> TokenSet<JsonSyntaxKind> {
        match self.kind {
            SequenceKind::Array => VALUE_RECOVERY_SET,
            SequenceKind::Object => VALUE_RECOVERY_SET.union(token_set!(T![:])),
        }
    }
}

fn parse_sequence(p: &mut JsonParser, root_kind: SequenceKind) -> ParsedSyntax {
    let mut stack = Vec::new();
    let mut current = start_sequence(p, root_kind);

    'sequence: loop {
        let mut first = match current.state {
            SequenceState::Start => true,
            SequenceState::Processing => false,
            SequenceState::Suspended(marker) => {
                if let Some(member_marker) = marker {
                    debug_assert_eq!(current.kind, SequenceKind::Object);
                    // Complete the object member
                    member_marker.complete(p, JSON_MEMBER);
                }

                current.state = SequenceState::Processing;
                false
            }
        };

        let mut progress = ParserProgress::default();

        while !p.at(EOF) && !p.at(current.kind.close_paren()) {
            if first {
                first = false;
            } else {
                p.expect(T![,]);
            }

            progress.assert_progressing(p);

            match current.parse_item(p) {
                SequenceItem::Parsed(Absent) => {
                    if p.options().allow_trailing_commas && p.last() == Some(T![,]) {
                        break;
                    }

                    let range = if p.at(T![,]) {
                        p.cur_range()
                    } else {
                        match ParseRecoveryTokenSet::new(JSON_BOGUS_VALUE, current.recovery_set())
                            .enable_recovery_on_line_break()
                            .recover(p)
                        {
                            Ok(marker) => marker.range(p),
                            Err(_) => {
                                p.error(expected_value(p, p.cur_range()));
                                // We're done for this sequence, unclear how to proceed.
                                // Continue with parent sequence.
                                break;
                            }
                        }
                    };

                    p.error(expected_value(p, range));
                }
                SequenceItem::Parsed(Present(_)) => {
                    // continue with next item
                }

                // Nested Array or object expression
                SequenceItem::Recurse(kind, marker) => {
                    current.state = SequenceState::Suspended(marker);
                    stack.push(current);
                    current = start_sequence(p, kind);
                    continue 'sequence;
                }
            }
        }

        current.list.complete(p, current.kind.list_kind());
        p.expect(current.kind.close_paren());
        let node = current.node.complete(p, current.kind.node_kind());

        match stack.pop() {
            None => return Present(node),
            Some(next) => current = next,
        };
    }
}

fn start_sequence(p: &mut JsonParser, kind: SequenceKind) -> Sequence {
    let node = p.start();

    p.expect(kind.open_paren());

    let list = p.start();
    Sequence {
        kind,
        node,
        list,
        state: SequenceState::Start,
    }
}

#[derive(Debug)]
enum SequenceItem {
    Parsed(ParsedSyntax),
    Recurse(SequenceKind, Option<Marker>),
}

fn parse_object_member(p: &mut JsonParser) -> SequenceItem {
    let m = p.start();

    if parse_member_name(p).is_absent() {
        if !(p.options().allow_trailing_commas && p.last() == Some(T![,])) {
            p.error(expected_property(p, p.cur_range()));
        }

        if !p.at(T![:]) && !p.at_ts(VALUE_START) {
            m.abandon(p);
            return SequenceItem::Parsed(Absent);
        }
    }

    p.expect(T![:]);

    match parse_sequence_value(p) {
        Ok(value) => {
            value.or_add_diagnostic(p, expected_value);
            SequenceItem::Parsed(Present(m.complete(p, JSON_MEMBER)))
        }
        Err(kind) => SequenceItem::Recurse(kind, Some(m)),
    }
}

fn parse_member_name(p: &mut JsonParser) -> ParsedSyntax {
    match p.cur() {
        JSON_STRING_LITERAL => {
            let m = p.start();
            p.bump(JSON_STRING_LITERAL);
            Present(m.complete(p, JSON_MEMBER_NAME))
        }
        IDENT | T![null] | T![true] | T![false] => {
            let m = p.start();
            p.error(p.err_builder("Property key must be double quoted", p.cur_range()));
            p.bump_remap(IDENT);
            Present(m.complete(p, JSON_MEMBER_NAME))
        }
        _ => Absent,
    }
}

fn parse_array_element(p: &mut JsonParser) -> SequenceItem {
    match parse_sequence_value(p) {
        Ok(parsed) => SequenceItem::Parsed(parsed),
        Err(kind) => SequenceItem::Recurse(kind, None),
    }
}

fn parse_sequence_value(p: &mut JsonParser) -> Result<ParsedSyntax, SequenceKind> {
    match p.cur() {
        // Special handling for arrays and objects, suspend the current sequence and start parsing
        // the nested array or object.
        T!['['] => Err(SequenceKind::Array),
        T!['{'] => Err(SequenceKind::Object),
        _ => Ok(parse_value(p)),
    }
}

#[cold]
fn parse_rest(p: &mut JsonParser, value: ParsedSyntax) {
    // Wrap the values in an array if there are more than one.
    let list = value.precede(p);

    while !p.at(EOF) {
        let range = match parse_value(p) {
            Present(value) => value.range(p),
            Absent => ParseRecoveryTokenSet::new(JSON_BOGUS_VALUE, VALUE_START)
                .recover(p)
                .expect("Expect recovery to succeed because parser isn't at EOF nor at a value.")
                .range(p),
        };

        p.error(
            p.err_builder("End of file expected", range)
                .with_hint("Use an array for a sequence of values: `[1, 2]`"),
        );
    }

    list.complete(p, JSON_ARRAY_ELEMENT_LIST)
        .precede(p)
        .complete(p, JSON_ARRAY_VALUE);
}

fn expected_value(p: &JsonParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["array", "object", "literal"], range, p)
}

fn expected_property(p: &JsonParser, range: TextRange) -> ParseDiagnostic {
    expected_node("property", range, p)
}
