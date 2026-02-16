use crate::parser::HtmlParser;
use crate::syntax::HtmlSyntaxFeatures::Astro;
use crate::syntax::parse_error::{expected_closed_fence, expected_expression};
use crate::syntax::{TextExpression, parse_attribute_initializer, parse_single_text_expression};
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{
    ASTRO_CLASS_DIRECTIVE, ASTRO_CLIENT_DIRECTIVE, ASTRO_DIRECTIVE_VALUE, ASTRO_EMBEDDED_CONTENT,
    ASTRO_FRONTMATTER_ELEMENT, ASTRO_IS_DIRECTIVE, ASTRO_SERVER_DIRECTIVE, ASTRO_SET_DIRECTIVE,
    FENCE, HTML_ATTRIBUTE_NAME, HTML_LITERAL, HTML_SPREAD_ATTRIBUTE,
};
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::{Parser, SyntaxFeature};

pub(crate) fn parse_astro_fence(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![---]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(FENCE, HtmlLexContext::AstroFencedCodeBlock);
    if p.at(T![<]) {
        p.error(expected_closed_fence(p, p.cur_range()));
        let c = m.complete(p, ASTRO_FRONTMATTER_ELEMENT);
        return ParsedSyntax::Present(c);
    }
    if let Absent = parse_astro_embedded(p) {
        let content = p.start();
        content.complete(p, ASTRO_EMBEDDED_CONTENT);
    }
    p.expect(T![---]);

    let c = m.complete(p, ASTRO_FRONTMATTER_ELEMENT);
    ParsedSyntax::Present(c)
}

pub(crate) fn parse_astro_embedded(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_LITERAL) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(HTML_LITERAL, HtmlLexContext::AstroFencedCodeBlock);

    ParsedSyntax::Present(m.complete(p, ASTRO_EMBEDDED_CONTENT))
}

/// Parses a spread attribute or a single text expression.
pub(crate) fn parse_astro_spread_or_expression(p: &mut HtmlParser) -> ParsedSyntax {
    if !Astro.is_supported(p) {
        return Absent;
    }

    if !p.at(T!['{']) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();

    // We bump using svelte context because it's faster to lex a possible ..., which is also
    // only consumable when using the Svelte context
    p.bump_with_context(T!['{'], HtmlLexContext::Svelte);

    if p.at(T![...]) {
        p.bump_with_context(T![...], HtmlLexContext::single_expression());
        TextExpression::new_single()
            .parse_element(p)
            .or_add_diagnostic(p, expected_expression);

        p.expect_with_context(T!['}'], HtmlLexContext::InsideTag);

        Present(m.complete(p, HTML_SPREAD_ATTRIBUTE))
    } else {
        p.rewind(checkpoint);
        m.abandon(p);
        parse_single_text_expression(p, HtmlLexContext::InsideTag)
    }
}

// #region Directive parsing functions

/// Astro directive keywords
const ASTRO_DIRECTIVE_KEYWORDS: &[&str] = &["client", "set", "class", "is", "server"];

/// Check if the current position is at an Astro directive.
/// In the InsideTag context, the colon is not a separate token. The lexer produces
/// "client" and ":load" as separate HTML_LITERAL tokens. We need to check if the
/// current token is a directive keyword and the next token starts with a colon.
pub(crate) fn is_at_astro_directive_start(p: &mut HtmlParser) -> bool {
    if Astro.is_unsupported(p) {
        return false;
    }

    // Must be at an HTML_LITERAL token
    if !p.at(HTML_LITERAL) {
        return false;
    }

    let text = p.cur_text();

    // Check if the text is exactly one of the directive keywords
    if !ASTRO_DIRECTIVE_KEYWORDS.contains(&text) {
        return false;
    }

    // Check if the next token is an HTML_LITERAL starting with ":"
    let checkpoint = p.checkpoint();
    p.bump_any();
    let next_is_colon_literal = p.at(HTML_LITERAL) && p.cur_text().starts_with(':');
    p.rewind(checkpoint);

    next_is_colon_literal
}

pub(crate) fn parse_astro_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_astro_directive_start(p) {
        return Absent;
    }

    // The text should be exactly one of the directive keywords
    let directive_kind = match p.cur_text() {
        "client" => ASTRO_CLIENT_DIRECTIVE,
        "set" => ASTRO_SET_DIRECTIVE,
        "class" => ASTRO_CLASS_DIRECTIVE,
        "is" => ASTRO_IS_DIRECTIVE,
        "server" => ASTRO_SERVER_DIRECTIVE,
        _ => return Absent,
    };

    parse_directive(p, directive_kind)
}

fn parse_directive(p: &mut HtmlParser, node_kind: HtmlSyntaxKind) -> ParsedSyntax {
    let m = p.start();

    // Consume the keyword token (e.g., "client")
    // The keyword is lexed as an HTML_LITERAL with just the keyword text
    p.bump_with_context(HTML_LITERAL, HtmlLexContext::InsideTag);

    // Now parse the directive value (":load" part)
    // The value starts with an HTML_LITERAL like ":load"
    if p.at(HTML_LITERAL) {
        let text = p.cur_text();
        if text.starts_with(':') {
            // Create the directive value node as a child
            let m_value = p.start();

            // Consume the ":load" token as the name (colon is part of this token in InsideTag context)
            let m_name = p.start();
            p.bump_with_context(HTML_LITERAL, HtmlLexContext::InsideTag);
            m_name.complete(p, HTML_ATTRIBUTE_NAME);

            // Parse optional initializer if present (e.g., "={value}")
            if p.at(T![=]) {
                parse_attribute_initializer(p).ok();
            }

            m_value.complete(p, ASTRO_DIRECTIVE_VALUE);
        }
    }

    Present(m.complete(p, node_kind))
}

// #endregion
