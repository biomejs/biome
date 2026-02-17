use crate::parser::HtmlParser;
use crate::syntax::HtmlSyntaxFeatures::Astro;
use crate::syntax::parse_error::{expected_closed_fence, expected_expression};
use crate::syntax::{TextExpression, parse_attribute_initializer, parse_single_text_expression};
use crate::token_source::{HtmlLexContext, HtmlReLexContext};
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
/// In the InsideTagAstro context, the colon is a separate token.
pub(crate) fn is_at_astro_directive_start(p: &mut HtmlParser) -> bool {
    if Astro.is_unsupported(p) {
        return false;
    }

    let checkpoint = p.checkpoint();
    p.re_lex(HtmlReLexContext::InsideTagAstro);

    let first_token = p.cur();
    let first_text = p.cur_text().to_string();

    p.bump_any_with_context(HtmlLexContext::InsideTagAstro);
    let second_token = p.cur();

    p.rewind(checkpoint);

    let first_is_directive = matches!(
        first_token,
        T![client] | T![set] | T![class] | T![is] | T![server]
    ) || ASTRO_DIRECTIVE_KEYWORDS.contains(&first_text.as_str());

    first_is_directive && second_token == T![:]
}

pub(crate) fn parse_astro_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_astro_directive_start(p) {
        return Absent;
    }

    p.re_lex(HtmlReLexContext::InsideTagAstro);

    let directive_kind = if p.at(HTML_LITERAL) {
        match p.cur_text() {
            "client" => ASTRO_CLIENT_DIRECTIVE,
            "set" => ASTRO_SET_DIRECTIVE,
            "class" => ASTRO_CLASS_DIRECTIVE,
            "is" => ASTRO_IS_DIRECTIVE,
            "server" => ASTRO_SERVER_DIRECTIVE,
            _ => return Absent,
        }
    } else {
        match p.cur() {
            T![client] => ASTRO_CLIENT_DIRECTIVE,
            T![set] => ASTRO_SET_DIRECTIVE,
            T![class] => ASTRO_CLASS_DIRECTIVE,
            T![is] => ASTRO_IS_DIRECTIVE,
            T![server] => ASTRO_SERVER_DIRECTIVE,
            _ => return Absent,
        }
    };

    parse_directive(p, directive_kind)
}

fn parse_directive(p: &mut HtmlParser, node_kind: HtmlSyntaxKind) -> ParsedSyntax {
    let m = p.start();
    p.bump_any_with_context(HtmlLexContext::InsideTagAstro);

    // Parse the directive value (":load" part)
    parse_directive_value(p).or_add_diagnostic(p, |p, range| {
        p.err_builder(
            "Expected a directive value after the directive keyword",
            range,
        )
    });

    Present(m.complete(p, node_kind))
}

fn parse_directive_value(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();

    // Consume the colon token
    p.expect_with_context(T![:], HtmlLexContext::InsideTagAstro);

    // Parse the directive name (e.g., "load" in "client:load")
    if p.at(HTML_LITERAL) {
        let m_name = p.start();
        p.bump_with_context(HTML_LITERAL, HtmlLexContext::InsideTagAstro);
        m_name.complete(p, HTML_ATTRIBUTE_NAME);
    } else {
        p.error(p.err_builder("Expected a directive name after ':'", p.cur_range()));
    }

    // Parse optional initializer if present (e.g., "={value}")
    if p.at(T![=]) {
        parse_attribute_initializer(p).ok();
        if Astro.is_supported(p) {
            p.re_lex(HtmlReLexContext::InsideTagAstro);
        }
    }

    Present(m.complete(p, ASTRO_DIRECTIVE_VALUE))
}

// #endregion
