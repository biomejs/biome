//! Glimmer template parsing rules
//!
//! This module contains the parsing logic for Glimmer templates.

use crate::parser::GlimmerParser;
use crate::token_source::GlimmerLexContext;
use biome_glimmer_syntax::GlimmerSyntaxKind::*;
use biome_glimmer_syntax::T;
use biome_parser::Parser;

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

    while !p.at(EOF) && !at_statement_list_end(p) {
        if !parse_statement(p) {
            // If we couldn't parse a statement, create a bogus node and consume one token
            let bogus = p.start();
            p.bump_any();
            bogus.complete(p, GLIMMER_BOGUS_STATEMENT);
        }
    }

    m.complete(p, GLIMMER_STATEMENT_LIST);
}

/// Check if we're at the end of a statement list
fn at_statement_list_end(p: &mut GlimmerParser) -> bool {
    // End of statement list when we hit:
    // - End tag: </
    // - Closing mustache for block: {{/
    // - Else block: {{else
    matches!(
        (p.cur(), p.nth(1)),
        (T![<], T![/]) | (L_CURLY2, T![/]) | (L_CURLY2, ELSE_KW)
    )
}

/// Parse a single statement
///
/// Returns true if a statement was successfully parsed
fn parse_statement(p: &mut GlimmerParser) -> bool {
    match p.cur() {
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
        MUSTACHE_COMMENT => {
            parse_mustache_comment(p);
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

/// Parse a mustache comment {{! ... }} or {{!-- ... --}}
fn parse_mustache_comment(p: &mut GlimmerParser) {
    let m = p.start();
    p.bump(MUSTACHE_COMMENT);
    m.complete(p, GLIMMER_MUSTACHE_COMMENT_STATEMENT);
}

/// Parse either a mustache statement, block statement, or element modifier
fn parse_mustache_or_block(p: &mut GlimmerParser) {
    // Check if it's a block statement ({{#...) or element modifier
    if p.nth(1) == T![#] {
        parse_block_statement(p);
    } else {
        parse_mustache_statement(p);
    }
}

/// Parse a mustache statement: {{expression}} or {{{expression}}}
fn parse_mustache_statement(p: &mut GlimmerParser) {
    let m = p.start();

    p.expect(L_CURLY2); // {{

    // Check for triple mustache {{{
    let is_triple = p.at(L_CURLY2);
    if is_triple {
        p.bump(L_CURLY2);
    }

    // Parse the expression inside
    parse_expression(p);

    // Closing braces
    p.expect(R_CURLY2); // }}
    if is_triple {
        p.expect(R_CURLY2);
    }

    m.complete(p, GLIMMER_MUSTACHE_STATEMENT);
}

/// Parse a block statement: {{#if}}...{{/if}}
fn parse_block_statement(p: &mut GlimmerParser) {
    let m = p.start();

    // Opening: {{#
    p.expect(L_CURLY2);
    p.expect(T![#]);

    // Block path (helper name like 'if', 'each', 'let')
    parse_path_expression(p);

    // Params list
    parse_params_list(p);

    // Hash
    parse_hash(p);

    // Closing }}
    p.expect(R_CURLY2);

    // Block content
    parse_block(p);

    // Optional else block
    if p.at(L_CURLY2) && p.nth(1) == ELSE_KW {
        parse_else_block(p);
    }

    // Closing: {{/path}}
    p.expect(L_CURLY2);
    p.expect(T![/]);
    parse_path_expression(p);
    p.expect(R_CURLY2);

    m.complete(p, GLIMMER_BLOCK_STATEMENT);
}

/// Parse a block: optional block params + statement list
fn parse_block(p: &mut GlimmerParser) {
    let m = p.start();

    // Optional block params: as |foo bar|
    if p.at(AS_KW) {
        parse_block_params(p);
    }

    // Statement list
    parse_statement_list(p);

    m.complete(p, GLIMMER_BLOCK);
}

/// Parse block params: as |param1 param2|
fn parse_block_params(p: &mut GlimmerParser) {
    let m = p.start();

    p.expect(AS_KW);
    p.expect(T![|]);

    // Parse param name list
    let list = p.start();
    while p.at(IDENT) {
        let param = p.start();
        p.bump(IDENT);
        param.complete(p, GLIMMER_PARAM_NAME);
    }
    list.complete(p, GLIMMER_PARAM_NAME_LIST);

    p.expect(T![|]);

    m.complete(p, GLIMMER_BLOCK_PARAMS);
}

/// Parse an else block: {{else}} or {{else if condition}}
fn parse_else_block(p: &mut GlimmerParser) {
    let m = p.start();

    p.expect(L_CURLY2);
    p.expect(ELSE_KW);

    // Check for else-if
    if p.at(IF_KW) {
        p.bump(IF_KW);
        parse_expression(p);
    }

    p.expect(R_CURLY2);

    // Block content
    parse_block(p);

    m.complete(p, GLIMMER_ELSE_BLOCK);
}

/// Parse an element (HTML tag or Glimmer component)
fn parse_element(p: &mut GlimmerParser) {
    let m = p.start();

    let is_self_closing = parse_start_tag(p);

    if !is_self_closing {
        // Parse children
        parse_statement_list(p);

        // Parse closing tag if not self-closing
        if p.at(T![<]) && p.nth(1) == T![/] {
            parse_end_tag(p);
        }
    }

    m.complete(p, GLIMMER_ELEMENT_NODE);
}

/// Parse an opening tag
/// Returns true if the tag is self-closing
fn parse_start_tag(p: &mut GlimmerParser) -> bool {
    let m = p.start();

    p.expect(T![<]);

    // Tag name as path expression (allows MyComponent or ns:Component)
    parse_path_expression(p);

    // Parse attributes
    parse_attribute_list(p);

    // Parse element modifiers ({{on "click" handler}})
    parse_element_modifier_list(p);

    // Optional block params
    if p.at(AS_KW) {
        parse_block_params(p);
    }

    // Check for self-closing
    let is_self_closing = if p.at(T![/]) {
        let sc = p.start();
        p.bump(T![/]);
        sc.complete(p, SELF_CLOSING);
        true
    } else {
        false
    };

    p.expect(T![>]);

    m.complete(p, GLIMMER_START_TAG);
    is_self_closing
}

/// Parse attribute list
fn parse_attribute_list(p: &mut GlimmerParser) {
    let m = p.start();

    while p.at(IDENT) {
        parse_attribute(p);
    }

    m.complete(p, GLIMMER_ATTRIBUTE_LIST);
}

/// Parse a single attribute: name="value" or name={{value}}
fn parse_attribute(p: &mut GlimmerParser) {
    let m = p.start();

    p.expect(IDENT);
    p.expect(T![=]);

    // Parse attribute value (can be text, mustache, or concat)
    parse_attribute_value(p);

    m.complete(p, GLIMMER_ATTRIBUTE_NODE);
}

/// Parse attribute value
fn parse_attribute_value(p: &mut GlimmerParser) {
    match p.cur() {
        TEXT => {
            parse_text_node(p);
        }
        L_CURLY2 => {
            parse_mustache_statement(p);
        }
        _ => {
            // If we have a mix of text and mustaches, it's a concat statement
            // For now, treat as bogus
            let bogus = p.start();
            p.bump_any();
            bogus.complete(p, GLIMMER_BOGUS);
        }
    }
}

/// Parse element modifier list
fn parse_element_modifier_list(p: &mut GlimmerParser) {
    let m = p.start();

    while p.at(L_CURLY2) && p.nth(1) == IDENT {
        parse_element_modifier(p);
    }

    m.complete(p, GLIMMER_ELEMENT_MODIFIER_LIST);
}

/// Parse element modifier: {{on "click" handler}}
fn parse_element_modifier(p: &mut GlimmerParser) {
    let m = p.start();

    p.expect(L_CURLY2);
    parse_path_expression(p);
    parse_params_list(p);
    parse_hash(p);
    p.expect(R_CURLY2);

    m.complete(p, GLIMMER_ELEMENT_MODIFIER);
}

/// Parse a closing tag: </name>
fn parse_end_tag(p: &mut GlimmerParser) {
    let m = p.start();

    p.expect(T![<]);
    p.expect(T![/]);
    parse_path_expression(p);
    p.expect(T![>]);

    m.complete(p, GLIMMER_END_TAG);
}

///////////////
// Expressions
///////////////

/// Parse an expression (path, sub-expression, or literal)
fn parse_expression(p: &mut GlimmerParser) {
    match p.cur() {
        T!['('] => {
            parse_sub_expression(p);
        }
        STRING_LITERAL | NUMBER_LITERAL | TRUE_KW | FALSE_KW | NULL_KW | UNDEFINED_KW => {
            parse_literal(p);
        }
        THIS_KW | T![@] | IDENT => {
            parse_path_expression(p);
        }
        _ => {
            // Create a bogus expression
            let bogus = p.start();
            p.bump_any();
            bogus.complete(p, GLIMMER_BOGUS_EXPRESSION);
        }
    }
}

/// Parse a sub-expression: (helper arg1 arg2 key=value)
fn parse_sub_expression(p: &mut GlimmerParser) {
    let m = p.start();

    p.expect(T!['(']);
    parse_path_expression(p);
    parse_params_list(p);
    parse_hash(p);
    p.expect(T![')']);

    m.complete(p, GLIMMER_SUB_EXPRESSION);
}

/// Parse a path expression: this, this.foo, @arg, foo, foo.bar
fn parse_path_expression(p: &mut GlimmerParser) {
    let m = p.start();

    // Parse head
    parse_path_head(p);

    // Parse tail (segments)
    let segments = p.start();
    while p.at(T![.]) {
        parse_path_segment(p);
    }
    segments.complete(p, GLIMMER_PATH_SEGMENT_LIST);

    m.complete(p, GLIMMER_PATH_EXPRESSION);
}

/// Parse path head: this | @name | name
fn parse_path_head(p: &mut GlimmerParser) {
    match p.cur() {
        THIS_KW => {
            let m = p.start();
            p.bump(THIS_KW);
            m.complete(p, GLIMMER_THIS_HEAD);
        }
        T![@] => {
            let m = p.start();
            p.bump(T![@]);
            p.expect(IDENT);
            m.complete(p, GLIMMER_AT_HEAD);
        }
        IDENT => {
            let m = p.start();
            p.bump(IDENT);
            m.complete(p, GLIMMER_VAR_HEAD);
        }
        _ => {
            // Error: expected path head
            let bogus = p.start();
            p.bump_any();
            bogus.complete(p, GLIMMER_BOGUS);
        }
    }
}

/// Parse a path segment: .foo
fn parse_path_segment(p: &mut GlimmerParser) {
    let m = p.start();
    p.expect(T![.]);
    p.expect(IDENT);
    m.complete(p, GLIMMER_PATH_SEGMENT);
}

/// Parse params list (zero or more expressions)
fn parse_params_list(p: &mut GlimmerParser) {
    let m = p.start();

    while is_expression_start(p) {
        parse_expression(p);
    }

    m.complete(p, GLIMMER_PARAMS_LIST);
}

/// Parse hash (zero or more key=value pairs)
fn parse_hash(p: &mut GlimmerParser) {
    let m = p.start();

    // Parse hash pair list
    let pairs = p.start();
    while p.at(IDENT) && p.nth(1) == T![=] {
        parse_hash_pair(p);
    }
    pairs.complete(p, GLIMMER_HASH_PAIR_LIST);

    m.complete(p, GLIMMER_HASH);
}

/// Parse a hash pair: key=value
fn parse_hash_pair(p: &mut GlimmerParser) {
    let m = p.start();
    p.expect(IDENT);
    p.expect(T![=]);
    parse_expression(p);
    m.complete(p, GLIMMER_HASH_PAIR);
}

/// Check if current token can start an expression
fn is_expression_start(p: &mut GlimmerParser) -> bool {
    matches!(
        p.cur(),
        T!['(']
            | STRING_LITERAL
            | NUMBER_LITERAL
            | TRUE_KW
            | FALSE_KW
            | NULL_KW
            | UNDEFINED_KW
            | THIS_KW
            | T![@]
            | IDENT
    )
}

///////////////
// Literals
///////////////

/// Parse a literal (string, number, boolean, null, undefined)
fn parse_literal(p: &mut GlimmerParser) {
    let m = p.start();

    let kind = match p.cur() {
        STRING_LITERAL => {
            p.bump(STRING_LITERAL);
            GLIMMER_STRING_LITERAL
        }
        NUMBER_LITERAL => {
            p.bump(NUMBER_LITERAL);
            GLIMMER_NUMBER_LITERAL
        }
        TRUE_KW | FALSE_KW => {
            p.bump_any();
            GLIMMER_BOOLEAN_LITERAL
        }
        NULL_KW => {
            p.bump(NULL_KW);
            GLIMMER_NULL_LITERAL
        }
        UNDEFINED_KW => {
            p.bump(UNDEFINED_KW);
            GLIMMER_UNDEFINED_LITERAL
        }
        _ => {
            // Error: expected literal
            p.bump_any();
            GLIMMER_BOGUS
        }
    };

    m.complete(p, kind);
}
