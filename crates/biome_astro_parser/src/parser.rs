use crate::prelude::*;
use biome_parser::prelude::*;

pub type AstroParser<'source> = Parser<'source, AstroSyntaxKind, TokenSource<'source>, ()>;

/// Parse the root of an Astro document
pub fn parse_root(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    // Optional BOM
    p.eat(UNICODE_BOM);

    // Optional frontmatter
    if p.at(T!['---']) {
        parse_frontmatter(p);
    }

    // Parse body elements
    parse_element_list(p);

    // Expect EOF
    p.expect(EOF);

    m.complete(p, ASTRO_ROOT)
}

/// Parse frontmatter block: --- content ---
fn parse_frontmatter(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    p.expect(T!['---']);

    // Parse the content as JavaScript/TypeScript
    // For now, we'll treat it as a single token
    // In a real implementation, we'd integrate with the JS parser
    if !p.at(T!['---']) {
        let content_marker = p.start();
        
        // Consume everything until the closing ---
        while !p.at(T!['---']) && !p.at(EOF) {
            p.bump_any();
        }
        
        content_marker.complete(p, ASTRO_FRONTMATTER_CONTENT);
    }

    p.expect(T!['---']);

    m.complete(p, ASTRO_FRONTMATTER)
}

/// Parse a list of elements
fn parse_element_list(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    while !p.at(EOF) && !p.at(T!['</']) {
        parse_element(p);
    }

    m.complete(p, ASTRO_ELEMENT_LIST)
}

/// Parse a single element (HTML, component, expression, text, etc.)
fn parse_element(p: &mut AstroParser) -> Option<CompletedMarker> {
    match p.current() {
        T!['<'] if p.nth_at(1, T!['/']) => {
            // This is a closing tag, don't parse it here
            return None;
        }
        T!['<'] => Some(parse_tag(p)),
        T!['{'] => Some(parse_expression(p)),
        ASTRO_TEXT => Some(parse_text(p)),
        _ => {
            // Try to recover by consuming the token as text
            Some(parse_text(p))
        }
    }
}

/// Parse an HTML/component tag
fn parse_tag(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    p.expect(T!['<']);

    // Parse tag name
    if p.at(ASTRO_TEXT) {
        let name_marker = p.start();
        p.bump(ASTRO_TEXT);
        name_marker.complete(p, ASTRO_ELEMENT_NAME);
    }

    // Parse attributes
    parse_attribute_list(p);

    if p.eat(T!['/']) {
        // Self-closing tag
        p.expect(T!['>']);
        m.complete(p, ASTRO_SELF_CLOSING_ELEMENT)
    } else {
        p.expect(T!['>']);

        // Parse children
        parse_element_list(p);

        // Parse closing tag
        if p.at(T!['<']) && p.nth_at(1, T!['/']) {
            p.expect(T!['<']);
            p.expect(T!['/']);
            
            // Parse closing tag name
            if p.at(ASTRO_TEXT) {
                let name_marker = p.start();
                p.bump(ASTRO_TEXT);
                name_marker.complete(p, ASTRO_ELEMENT_NAME);
            }
            
            p.expect(T!['>']);
        }

        m.complete(p, ASTRO_ELEMENT)
    }
}

/// Parse a list of attributes
fn parse_attribute_list(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    while !p.at(T!['>']) && !p.at(T!['/']) && !p.at(EOF) {
        parse_attribute(p);
    }

    m.complete(p, ASTRO_ATTRIBUTE_LIST)
}

/// Parse a single attribute
fn parse_attribute(p: &mut AstroParser) -> Option<CompletedMarker> {
    if p.at(T!['{']) {
        // Could be shorthand {prop} or spread {...props}
        if p.nth_at(1, T!['...']) {
            Some(parse_spread_attribute(p))
        } else {
            Some(parse_shorthand_attribute(p))
        }
    } else if p.at(ASTRO_TEXT) {
        Some(parse_regular_attribute(p))
    } else {
        // Skip unknown tokens
        p.bump_any();
        None
    }
}

/// Parse a regular attribute: name="value" or name={expression}
fn parse_regular_attribute(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    // Parse attribute name
    let name_marker = p.start();
    p.expect(ASTRO_TEXT);
    name_marker.complete(p, ASTRO_ATTRIBUTE_NAME);

    if p.eat(T!['=']) {
        if p.at(T!['{']) {
            // Expression attribute: name={expr}
            parse_expression(p);
            m.complete(p, ASTRO_EXPRESSION_ATTRIBUTE)
        } else if p.at(T!['`']) {
            // Template literal attribute: name=`template`
            parse_template_literal(p);
            m.complete(p, ASTRO_TEMPLATE_LITERAL_ATTRIBUTE)
        } else if p.at(ASTRO_STRING_LITERAL) {
            // String attribute: name="value"
            let value_marker = p.start();
            p.bump(ASTRO_STRING_LITERAL);
            value_marker.complete(p, ASTRO_ATTRIBUTE_VALUE);
            m.complete(p, ASTRO_ATTRIBUTE)
        } else {
            // Error recovery
            m.complete(p, ASTRO_ATTRIBUTE)
        }
    } else {
        // Boolean attribute
        m.complete(p, ASTRO_ATTRIBUTE)
    }
}

/// Parse shorthand attribute: {prop}
fn parse_shorthand_attribute(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    p.expect(T!['{']);
    
    if p.at(ASTRO_TEXT) {
        let name_marker = p.start();
        p.bump(ASTRO_TEXT);
        name_marker.complete(p, ASTRO_ATTRIBUTE_NAME);
    }
    
    p.expect(T!['}']);

    m.complete(p, ASTRO_SHORTHAND_ATTRIBUTE)
}

/// Parse spread attribute: {...props}
fn parse_spread_attribute(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    p.expect(T!['{']);
    p.expect(T!['...']);

    // Parse the expression
    if p.at(ASTRO_TEXT) {
        let expr_marker = p.start();
        p.bump(ASTRO_TEXT);
        expr_marker.complete(p, ASTRO_EXPRESSION_CONTENT);
    }

    p.expect(T!['}']);

    m.complete(p, ASTRO_SPREAD_ATTRIBUTE)
}

/// Parse an expression: {expression}
fn parse_expression(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    p.expect(T!['{']);

    // For now, consume everything until }
    // In a real implementation, we'd integrate with the JS parser
    if !p.at(T!['}']) {
        let expr_marker = p.start();
        
        while !p.at(T!['}']) && !p.at(EOF) {
            p.bump_any();
        }
        
        expr_marker.complete(p, ASTRO_EXPRESSION_CONTENT);
    }

    p.expect(T!['}']);

    m.complete(p, ASTRO_EXPRESSION)
}

/// Parse a template literal: `template ${expr}`
fn parse_template_literal(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();

    p.expect(T!['`']);

    // Consume everything until closing backtick
    while !p.at(T!['`']) && !p.at(EOF) {
        p.bump_any();
    }

    p.expect(T!['`']);

    m.complete(p, ASTRO_TEMPLATE_LITERAL_ATTRIBUTE)
}

/// Parse plain text
fn parse_text(p: &mut AstroParser) -> CompletedMarker {
    let m = p.start();
    p.bump_any();
    m.complete(p, ASTRO_TEXT)
}