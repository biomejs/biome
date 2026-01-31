use crate::parser::HtmlParser;
use crate::syntax::HtmlSyntaxFeatures::Astro;
use crate::syntax::parse_error::{expected_closed_fence, expected_expression};
use crate::syntax::{TextExpression, parse_single_text_expression};
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{
    ASTRO_EMBEDDED_CONTENT, ASTRO_FRONTMATTER_ELEMENT, FENCE, HTML_LITERAL, HTML_SPREAD_ATTRIBUTE,
};
use biome_html_syntax::T;
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
