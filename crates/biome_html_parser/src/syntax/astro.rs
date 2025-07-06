use crate::parser::HtmlParser;
use crate::syntax::parse_error::expected_closed_fence;
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{
    FENCE, HTML_ASTRO_FRONTMATTER_ELEMENT, HTML_LITERAL, HTML_ASTRO_EXPRESSION,
    HTML_ASTRO_SHORTHAND_ATTRIBUTE, HTML_ASTRO_SPREAD_ATTRIBUTE, HTML_ASTRO_EXPRESSION_ATTRIBUTE,
    HTML_ASTRO_TEMPLATE_LITERAL_ATTRIBUTE, HTML_JS_CONTENT, HTML_TEMPLATE_LITERAL_CONTENT,
};
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;

pub(crate) fn parse_astro_fence(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at(HTML_LITERAL) && p.cur_text() == "---" {
        let m = p.start();
        p.bump_remap_with_context(FENCE, HtmlLexContext::AstroFencedCodeBlock);
        if p.at(T![<]) {
            p.error(expected_closed_fence(p, p.cur_range()));
            let c = m.complete(p, HTML_ASTRO_FRONTMATTER_ELEMENT);
            return ParsedSyntax::Present(c);
        }
        p.bump_with_context(HTML_LITERAL, HtmlLexContext::AstroFencedCodeBlock);
        p.expect_with_context(T![---], HtmlLexContext::AstroFencedCodeBlock);

        let c = m.complete(p, HTML_ASTRO_FRONTMATTER_ELEMENT);
        return ParsedSyntax::Present(c);
    }
    Absent
}

#[inline]
pub(crate) fn is_at_fence(p: &mut HtmlParser) -> bool {
    p.file_source().is_astro() && p.at(HTML_LITERAL) && p.cur_text() == "---"
}

/// Parse Astro expression like {expression}
pub(crate) fn parse_astro_expression(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['{']);
    
    // Parse the JavaScript content inside the braces
    if p.at(HTML_JS_CONTENT) {
        p.bump(HTML_JS_CONTENT);
    }
    
    p.expect(T!['}']);
    ParsedSyntax::Present(m.complete(p, HTML_ASTRO_EXPRESSION))
}

/// Parse Astro shorthand attribute like {name}
pub(crate) fn parse_astro_shorthand_attribute(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['{']);
    
    if p.at(HTML_LITERAL) {
        p.bump(HTML_LITERAL); // The identifier name
    }
    
    p.expect(T!['}']);
    ParsedSyntax::Present(m.complete(p, HTML_ASTRO_SHORTHAND_ATTRIBUTE))
}

/// Parse Astro spread attribute like {...props}
pub(crate) fn parse_astro_spread_attribute(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['{']);
    
    if p.at(T![...]) {
        p.bump(T![...]);
        if p.at(HTML_JS_CONTENT) {
            p.bump(HTML_JS_CONTENT);
        }
    }
    
    p.expect(T!['}']);
    ParsedSyntax::Present(m.complete(p, HTML_ASTRO_SPREAD_ATTRIBUTE))
}

/// Parse Astro expression attribute like name={expression}
pub(crate) fn parse_astro_expression_attribute(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['{']);
    
    if p.at(HTML_JS_CONTENT) {
        p.bump(HTML_JS_CONTENT);
    }
    
    p.expect(T!['}']);
    ParsedSyntax::Present(m.complete(p, HTML_ASTRO_EXPRESSION_ATTRIBUTE))
}

/// Parse Astro template literal attribute like name={`template ${expr}`}
pub(crate) fn parse_astro_template_literal_attribute(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['{']);
    
    if p.at(T!['`']) {
        p.bump(T!['`']);
        if p.at(HTML_TEMPLATE_LITERAL_CONTENT) {
            p.bump(HTML_TEMPLATE_LITERAL_CONTENT);
        }
        p.expect(T!['`']);
    }
    
    p.expect(T!['}']);
    ParsedSyntax::Present(m.complete(p, HTML_ASTRO_TEMPLATE_LITERAL_ATTRIBUTE))
}

/// Check if we're at an Astro expression
#[inline]
pub(crate) fn is_at_astro_expression(p: &mut HtmlParser) -> bool {
    p.file_source().is_astro() && p.at(T!['{'])
}

/// Check if we're at an Astro spread attribute
#[inline]
pub(crate) fn is_at_astro_spread_attribute(p: &mut HtmlParser) -> bool {
    p.file_source().is_astro() && p.at(T!['{']) && p.nth_at(1, T![...])
}
