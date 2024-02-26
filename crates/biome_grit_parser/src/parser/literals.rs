use super::{GritParser, SUPPORTED_LANGUAGE_SET};
use biome_grit_syntax::GritSyntaxKind::{self, *};
use biome_parser::prelude::{ParsedSyntax::*, *};

const BOOLEAN_VALUE_SET: TokenSet<GritSyntaxKind> = token_set![TRUE_KW, FALSE_KW];

const CODE_SNIPPET_SET: TokenSet<GritSyntaxKind> =
    SUPPORTED_LANGUAGE_SET.union(token_set![GRIT_BACKTICK_SNIPPET, GRIT_RAW_BACKTICK_SNIPPET]);

pub(crate) fn parse_literal(p: &mut GritParser) -> ParsedSyntax {
    match p.cur() {
        TRUE_KW | FALSE_KW => parse_boolean_literal(p),
        GRIT_DOUBLE => parse_double_literal(p),
        GRIT_INT => parse_int_literal(p),
        GRIT_NEGATIVE_INT => parse_negative_int_literal(p),
        GRIT_STRING => parse_string_literal(p),
        UNDEFINED_KW => parse_undefined_literal(p),
        kind if CODE_SNIPPET_SET.contains(kind) => parse_code_snippet(p),
        // TODO: List
        // TODO: Map
        _ => Absent,
    }
}

#[inline]
fn parse_backtick_snippet_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_BACKTICK_SNIPPET) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_BACKTICK_SNIPPET);
    Present(m.complete(p, GRIT_BACKTICK_SNIPPET_LITERAL))
}

#[inline]
pub(crate) fn parse_boolean_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(BOOLEAN_VALUE_SET) {
        return Absent;
    }

    let m = p.start();
    p.bump_ts(BOOLEAN_VALUE_SET);
    Present(m.complete(p, GRIT_BOOLEAN_LITERAL))
}

#[inline]
fn parse_code_snippet(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(CODE_SNIPPET_SET) {
        return Absent;
    }

    let m = p.start();

    let syntax = match p.cur() {
        GRIT_BACKTICK_SNIPPET => parse_backtick_snippet_literal(p),
        GRIT_RAW_BACKTICK_SNIPPET => parse_raw_backtick_snippet_literal(p),
        _ => parse_language_specific_snippet(p),
    };

    match syntax {
        Present(_) => Present(m.complete(p, GRIT_CODE_SNIPPET)),
        Absent => {
            m.abandon(p);
            Absent
        }
    }
}

#[inline]
pub(crate) fn parse_double_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_DOUBLE) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_DOUBLE);
    Present(m.complete(p, GRIT_DOUBLE_LITERAL))
}

#[inline]
fn parse_int_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_INT) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_INT);
    Present(m.complete(p, GRIT_INT_LITERAL))
}

#[inline]
fn parse_language_specific_snippet(p: &mut GritParser) -> ParsedSyntax {
    if !p.at_ts(SUPPORTED_LANGUAGE_SET) {
        return Absent;
    }

    let m = p.start();

    {
        let m = p.start();
        p.bump_ts(SUPPORTED_LANGUAGE_SET);
        m.complete(p, GRIT_LANGUAGE_NAME);
    }

    p.eat(GRIT_STRING);

    Present(m.complete(p, GRIT_LANGUAGE_SPECIFIC_SNIPPET))
}

#[inline]
fn parse_negative_int_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_NEGATIVE_INT) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_NEGATIVE_INT);
    Present(m.complete(p, GRIT_NEGATIVE_INT_LITERAL))
}

#[inline]
fn parse_raw_backtick_snippet_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_RAW_BACKTICK_SNIPPET) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_RAW_BACKTICK_SNIPPET);
    Present(m.complete(p, GRIT_RAW_BACKTICK_SNIPPET_LITERAL))
}

#[inline]
fn parse_string_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(GRIT_STRING) {
        return Absent;
    }

    let m = p.start();
    p.bump(GRIT_STRING);
    Present(m.complete(p, GRIT_STRING_LITERAL))
}

#[inline]
fn parse_undefined_literal(p: &mut GritParser) -> ParsedSyntax {
    if !p.at(UNDEFINED_KW) {
        return Absent;
    }

    let m = p.start();
    p.bump(UNDEFINED_KW);
    Present(m.complete(p, GRIT_UNDEFINED_LITERAL))
}
