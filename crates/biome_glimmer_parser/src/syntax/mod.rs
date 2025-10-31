//! Glimmer template parsing rules
//!
//! This module contains the parsing logic for Glimmer templates.

use crate::parser::GlimmerParser;
use crate::token_source::GlimmerLexContext;
use biome_glimmer_syntax::GlimmerSyntaxKind::*;
use biome_parser::Parser;
use biome_parser::prelude::*;

/// Parse the root of a Glimmer template
pub(crate) fn parse_root(p: &mut GlimmerParser) {
    let m = p.start();

    // TODO: Handle Unicode BOM
    // parse_bom(p);

    // Parse template statements
    parse_statement_list(p);

    // Expect EOF
    p.expect(EOF);

    m.complete(p, GLIMMER_ROOT);
}

/// Parse a list of statements (text, mustaches, elements, etc.)
fn parse_statement_list(p: &mut GlimmerParser) {
    let m = p.start();

    while !p.at(EOF) {
        if !parse_statement(p) {
            // If we couldn't parse a statement, consume one token to avoid infinite loop
            p.bump_any();
        }
    }

    m.complete(p, GLIMMER_STATEMENT_LIST);
}

/// Parse a single statement
///
/// Returns true if a statement was successfully parsed
fn parse_statement(p: &mut GlimmerParser) -> bool {
    match p.current() {
        TEXT => {
            parse_text_node(p);
            true
        }
        L_CURLY2 => {
            parse_mustache_or_block(p);
            true
        }
        T![<] => {
            parse_element(p);
            true
        }
        COMMENT => {
            parse_comment(p);
            true
        }
        _ => false,
    }
}

/// Parse a text node
fn parse_text_node(p: &mut GlimmerParser) {
    let m = p.start();
    p.bump(TEXT);
    m.complete(p, GLIMMER_TEXT_NODE);
}

/// Parse a comment
fn parse_comment(p: &mut GlimmerParser) {
    let m = p.start();
    p.bump(COMMENT);
    m.complete(p, GLIMMER_COMMENT_STATEMENT);
}

/// Parse either a mustache statement or block statement
fn parse_mustache_or_block(p: &mut GlimmerParser) {
    // For now, just create a placeholder
    // TODO: Implement full mustache and block parsing
    let m = p.start();
    p.bump(L_CURLY2);

    // Consume tokens until we find closing }}
    while !p.at(EOF) && !p.at(R_CURLY2) {
        p.bump_any();
    }

    if p.at(R_CURLY2) {
        p.bump(R_CURLY2);
    }

    m.complete(p, GLIMMER_MUSTACHE_STATEMENT);
}

/// Parse an element (HTML tag or Glimmer component)
fn parse_element(p: &mut GlimmerParser) {
    // For now, just create a placeholder
    // TODO: Implement full element parsing
    let m = p.start();

    parse_start_tag(p);

    // Parse children (for now, skip)
    let children = p.start();
    children.complete(p, GLIMMER_STATEMENT_LIST);

    // Optional closing tag
    // TODO: Implement closing tag parsing

    m.complete(p, GLIMMER_ELEMENT_NODE);
}

/// Parse an opening tag
fn parse_start_tag(p: &mut GlimmerParser) {
    let m = p.start();

    p.expect(T![<]);

    // TODO: Parse tag name, attributes, modifiers, etc.
    // For now, just consume until >
    while !p.at(EOF) && !p.at(T![>]) {
        p.bump_any();
    }

    if p.at(T![>]) {
        p.bump(T![>]);
    }

    m.complete(p, GLIMMER_START_TAG);
}
