mod astro;
mod parse_error;

use crate::parser::HtmlParser;
use crate::syntax::astro::parse_astro_fence;
use crate::syntax::parse_error::*;
use crate::token_source::{HtmlEmbeddedLanguage, HtmlLexContext, TextExpressionKind};
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

pub(crate) enum HtmlSyntaxFeatures {
    /// Exclusive to those documents that support Astro
    Astro,
    /// Exclusive to those documents that support text expressions with {{ }}
    DoubleTextExpressions,
    /// Exclusive to those documents that support text expressions with { }
    SingleTextExpressions,
}

impl SyntaxFeature for HtmlSyntaxFeatures {
    type Parser<'source> = HtmlParser<'source>;

    fn is_supported(&self, p: &HtmlParser) -> bool {
        match self {
            Self::Astro => p.options().frontmatter,
            Self::DoubleTextExpressions => {
                p.options().text_expression == Some(TextExpressionKind::Double)
            }
            Self::SingleTextExpressions => {
                p.options().text_expression == Some(TextExpressionKind::Single)
            }
        }
    }
}

const RECOVER_ATTRIBUTE_LIST: TokenSet<HtmlSyntaxKind> = token_set!(T![>], T![<], T![/]);
const RECOVER_TEXT_EXPRESSION_LIST: TokenSet<HtmlSyntaxKind> =
    token_set!(T![<], T![>], T!['}'], T!["}}"]);

/// These elements are effectively always self-closing. They should not have a closing tag (if they do, it should be a parsing error). They might not contain a `/` like in `<img />`.
static VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// For these elements, the content is treated as raw text and no parsing is done inside them. This is so that the contents of these tags can be parsed by a different parser.
pub(crate) static EMBEDDED_LANGUAGE_ELEMENTS: &[&str] = &["script", "style", "pre"];

pub(crate) fn parse_root(p: &mut HtmlParser) {
    let m = p.start();

    p.eat(UNICODE_BOM);

    if p.at(T![---]) {
        HtmlSyntaxFeatures::Astro
            .parse_exclusive_syntax(
                p,
                |p| parse_astro_fence(p),
                |p, m| {
                    p.err_builder("Frontmatter is only valid inside Astro files.", m.range(p))
                        .with_hint("Remove it or rename the file to have the .astro extension.")
                },
            )
            .ok();
    }
    parse_doc_type(p).ok();
    ElementList.parse_list(p);

    m.complete(p, HTML_ROOT);
}

fn parse_doc_type(p: &mut HtmlParser) -> ParsedSyntax {
    if !(p.at(T![<]) && p.nth_at(1, T![!])) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![<], HtmlLexContext::InsideTag);
    p.bump_with_context(T![!], HtmlLexContext::Doctype);

    if p.at(T![doctype]) {
        p.eat_with_context(T![doctype], HtmlLexContext::Doctype);
    }

    if p.at(T![html]) {
        p.eat_with_context(T![html], HtmlLexContext::Doctype);
    }

    if p.at(HTML_LITERAL) {
        p.eat_with_context(HTML_LITERAL, HtmlLexContext::Doctype);
    }

    if p.at(HTML_STRING_LITERAL) {
        p.eat_with_context(HTML_STRING_LITERAL, HtmlLexContext::Doctype);
    }

    if p.at(HTML_STRING_LITERAL) {
        p.eat_with_context(HTML_STRING_LITERAL, HtmlLexContext::Doctype);
    }

    p.eat(T![>]);

    Present(m.complete(p, HTML_DIRECTIVE))
}

fn parse_element(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }
    let m = p.start();

    p.bump_with_context(T![<], HtmlLexContext::InsideTag);
    let opening_tag_name = p.cur_text().to_string();
    let should_be_self_closing = VOID_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(opening_tag_name.as_str()));
    let is_embedded_language_tag = EMBEDDED_LANGUAGE_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(opening_tag_name.as_str()));
    parse_literal(p, HTML_TAG_NAME).or_add_diagnostic(p, expected_element_name);

    AttributeList.parse_list(p);

    if p.at(T![/]) {
        p.bump_with_context(T![/], HtmlLexContext::InsideTag);
        p.expect_with_context(T![>], HtmlLexContext::Regular);
        Present(m.complete(p, HTML_SELF_CLOSING_ELEMENT))
    } else {
        if should_be_self_closing {
            if p.at(T![/]) {
                p.bump_with_context(T![/], HtmlLexContext::InsideTag);
            }
            p.expect_with_context(T![>], HtmlLexContext::Regular);
            return Present(m.complete(p, HTML_SELF_CLOSING_ELEMENT));
        }
        p.expect_with_context(
            T![>],
            if is_embedded_language_tag {
                HtmlLexContext::EmbeddedLanguage(match opening_tag_name.as_str() {
                    tag if tag.eq_ignore_ascii_case("script") => HtmlEmbeddedLanguage::Script,
                    tag if tag.eq_ignore_ascii_case("style") => HtmlEmbeddedLanguage::Style,
                    tag if tag.eq_ignore_ascii_case("pre") => HtmlEmbeddedLanguage::Preformatted,
                    _ => unreachable!(),
                })
            } else {
                HtmlLexContext::Regular
            },
        );
        let opening = m.complete(p, HTML_OPENING_ELEMENT);
        if is_embedded_language_tag {
            // embedded language tags always have 1 element as content
            let list = p.start();
            if p.at(HTML_LITERAL) {
                let m = p.start();
                p.bump(HTML_LITERAL);
                m.complete(p, HTML_EMBEDDED_CONTENT);
            }
            list.complete(p, HTML_ELEMENT_LIST);

            parse_closing_tag(p).or_add_diagnostic(p, expected_closing_tag);
        } else {
            loop {
                ElementList.parse_list(p);
                if let Some(mut closing) =
                    parse_closing_tag(p).or_add_diagnostic(p, expected_closing_tag)
                    && !closing.text(p).contains(opening_tag_name.as_str())
                {
                    p.error(expected_matching_closing_tag(p, closing.range(p)).into_diagnostic(p));
                    closing.change_to_bogus(p);
                    continue;
                }
                break;
            }
        }
        let previous = opening.precede(p);

        Present(previous.complete(p, HTML_ELEMENT))
    }
}

fn parse_closing_tag(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<]) || !p.nth_at(1, T![/]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![<], HtmlLexContext::InsideTag);
    p.bump_with_context(T![/], HtmlLexContext::InsideTag);
    let should_be_self_closing = VOID_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(p.cur_text()));
    if should_be_self_closing {
        p.error(void_element_should_not_have_closing_tag(p, p.cur_range()).into_diagnostic(p));
    }
    let _name = parse_literal(p, HTML_TAG_NAME);

    // There shouldn't be any attributes in a closing tag.
    while p.at(HTML_LITERAL) || p.at(T!["{{"]) || p.at(T!["}}"]) {
        p.error(closing_tag_should_not_have_attributes(p, p.cur_range()));
        p.bump_remap_with_context(HTML_BOGUS, HtmlLexContext::InsideTag);
    }
    p.expect(T![>]);
    Present(m.complete(p, HTML_CLOSING_ELEMENT))
}

#[derive(Default)]
struct ElementList;

impl ParseNodeList for ElementList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = HTML_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        match p.cur() {
            T!["<![CDATA["] => parse_cdata_section(p),
            T![<] => parse_element(p),
            T!["{{"] => HtmlSyntaxFeatures::DoubleTextExpressions.parse_exclusive_syntax(
                p,
                |p| parse_double_text_expression(p, HtmlLexContext::Regular),
                |p, m| disabled_interpolation(p, m.range(p)),
            ),
            T!['{'] => parse_single_text_expression(p, HtmlLexContext::Regular).or_else(|| {
                let m = p.start();
                p.bump_remap(HTML_LITERAL);
                Present(m.complete(p, HTML_CONTENT))
            }),
            T!["}}"] | T!['}'] => {
                // The closing text expression should be handled by other functions.
                // If we're here, we assume that text expressions are enabled and
                // we remap to HTML_LITERAL
                let m = p.start();
                p.bump_remap(HTML_LITERAL);
                Present(m.complete(p, HTML_CONTENT))
            }
            HTML_LITERAL => {
                let m = p.start();
                p.bump_with_context(HTML_LITERAL, HtmlLexContext::Regular);
                Present(m.complete(p, HTML_CONTENT))
            }
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        let at_l_angle0 = p.at(T![<]);
        let at_slash1 = p.nth_at(1, T![/]);
        let at_eof = p.at(EOF);
        at_l_angle0 && at_slash1 || at_eof
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS_ELEMENT, token_set![T![<], T![>]]),
            expected_child,
        )
    }
}

#[derive(Default)]
struct AttributeList;

impl ParseNodeList for AttributeList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = HTML_ATTRIBUTE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_attribute(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![>]) || p.at(T![/]) || p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS_ELEMENT, RECOVER_ATTRIBUTE_LIST),
            expected_attribute,
        )
    }
}

fn parse_attribute(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_attribute_start(p) {
        return Absent;
    }

    let m = p.start();
    if p.at(T!["{{"]) {
        HtmlSyntaxFeatures::DoubleTextExpressions
            .parse_exclusive_syntax(
                p,
                |p| parse_double_text_expression(p, HtmlLexContext::InsideTag),
                |p, marker| disabled_interpolation(p, marker.range(p)),
            )
            .ok();

        Present(m.complete(p, HTML_ATTRIBUTE))
    } else {
        parse_literal(p, HTML_ATTRIBUTE_NAME).or_add_diagnostic(p, expected_attribute);
        if p.at(T![=]) {
            parse_attribute_initializer(p).ok();
            Present(m.complete(p, HTML_ATTRIBUTE))
        } else {
            Present(m.complete(p, HTML_ATTRIBUTE))
        }
    }
}

fn is_at_attribute_start(p: &mut HtmlParser) -> bool {
    p.at(HTML_LITERAL) || p.at(T!["{{"]) || p.at(T!['{'])
}

fn parse_literal(p: &mut HtmlParser, kind: HtmlSyntaxKind) -> ParsedSyntax {
    if !is_at_start_literal(p) {
        return Absent;
    }
    let m = p.start();

    if p.at(T!["{{"]) {
        if HtmlSyntaxFeatures::DoubleTextExpressions.is_supported(p) {
            parse_double_text_expression(p, HtmlLexContext::Regular).ok();
        } else {
            p.bump_remap_with_context(
                HTML_LITERAL,
                match kind {
                    HTML_TAG_NAME | HTML_ATTRIBUTE_NAME => HtmlLexContext::InsideTag,
                    _ => HtmlLexContext::Regular,
                },
            )
        }
    } else if p.at(T!["}}"]) {
        p.bump_remap_with_context(
            HTML_LITERAL,
            match kind {
                HTML_TAG_NAME | HTML_ATTRIBUTE_NAME => HtmlLexContext::InsideTag,
                _ => HtmlLexContext::Regular,
            },
        );
    } else {
        p.bump_with_context(
            HTML_LITERAL,
            match kind {
                HTML_TAG_NAME | HTML_ATTRIBUTE_NAME => HtmlLexContext::InsideTag,
                _ => HtmlLexContext::Regular,
            },
        );
    }

    Present(m.complete(p, kind))
}

fn is_at_start_literal(p: &mut HtmlParser) -> bool {
    p.at(HTML_LITERAL) || p.at(T!["{{"]) || p.at(T!["}}"])
}

fn parse_attribute_string_literal(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(HTML_STRING_LITERAL) {
        return Absent;
    }
    let m = p.start();

    p.bump_with_context(HTML_STRING_LITERAL, HtmlLexContext::InsideTag);

    Present(m.complete(p, HTML_STRING))
}

fn parse_attribute_initializer(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![=], HtmlLexContext::AttributeValue);
    if p.at(T!['{']) {
        HtmlSyntaxFeatures::SingleTextExpressions
            .parse_exclusive_syntax(
                p,
                |p| parse_single_text_expression(p, HtmlLexContext::InsideTag),
                |p, m| {
                    p.err_builder("Expressions are only valid inside Astro files.", m.range(p))
                        .with_hint("Remove it or rename the file to have the .astro extension.")
                },
            )
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_ATTRIBUTE_LIST),
                expected_attribute,
            )
            .ok();
    } else if p.at(T!["{{"]) {
        HtmlSyntaxFeatures::DoubleTextExpressions
            .parse_exclusive_syntax(
                p,
                |p| parse_double_text_expression(p, HtmlLexContext::InsideTag),
                |p, m| {
                    p.err_builder("Text expressions aren't supported.", m.range(p))
                        .with_hint("Remove it or add the option.")
                },
            )
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_ATTRIBUTE_LIST),
                expected_attribute,
            )
            .ok();
    } else {
        parse_attribute_string_literal(p).or_add_diagnostic(p, expected_initializer);
    }
    Present(m.complete(p, HTML_ATTRIBUTE_INITIALIZER_CLAUSE))
}

fn parse_cdata_section(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["<![CDATA["]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T!["<![CDATA["], HtmlLexContext::CdataSection);
    while !p.at(T!["]]>"]) && !p.at(EOF) {
        p.bump_with_context(HTML_LITERAL, HtmlLexContext::CdataSection);
    }
    p.expect(T!["]]>"]);
    Present(m.complete(p, HTML_CDATA_SECTION))
}

/// Parse a text expression, notably:
///
/// ```vue
/// {{ expression }}
/// ```
fn parse_double_text_expression(p: &mut HtmlParser, context: HtmlLexContext) -> ParsedSyntax {
    if !is_at_opening_double_expression(p) {
        return Absent;
    }
    let checkpoint = p.checkpoint();
    let m = p.start();
    let opening_range = p.cur_range();
    p.bump_with_context(
        T!["{{"],
        HtmlLexContext::TextExpression(TextExpressionKind::Double),
    );

    TextExpression::new_double().parse_element(p).ok();

    if p.at(T!["}}"]) {
        p.expect_with_context(T!["}}"], context);
        Present(m.complete(p, HTML_DOUBLE_TEXT_EXPRESSION))
    } else if p.at(T![<]) {
        let diagnostic = expected_text_expression(p, p.cur_range(), opening_range);
        p.error(diagnostic);
        Present(m.complete(p, HTML_BOGUS_TEXT_EXPRESSION))
    } else {
        m.abandon(p);
        p.rewind(checkpoint);

        let recovery =
            ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_TEXT_EXPRESSION_LIST);
        if let Ok(m) = recovery.enable_recovery_on_line_break().recover(p) {
            let diagnostic = expected_text_expression(p, m.range(p), opening_range);
            p.error(diagnostic);
            Present(m)
        } else {
            Absent
        }
    }
}

pub(crate) fn is_at_opening_double_expression(p: &mut HtmlParser) -> bool {
    p.at(T!["{{"])
}

// Parsers a single tag expression. `context` is applied after lexing the last token `}`
pub(crate) fn parse_single_text_expression(
    p: &mut HtmlParser,
    context: HtmlLexContext,
) -> ParsedSyntax {
    if !HtmlSyntaxFeatures::SingleTextExpressions.is_supported(p) {
        return Absent;
    }

    if !p.at(T!['{']) {
        return Absent;
    }
    let checkpoint = p.checkpoint();
    let m = p.start();
    let opening_range = p.cur_range();

    p.bump_with_context(
        T!['{'],
        HtmlLexContext::TextExpression(TextExpressionKind::Single),
    );

    TextExpression::new_single().parse_element(p).ok();

    if p.at(T!['}']) {
        p.bump_remap_with_context(T!['}'], context);
        Present(m.complete(p, HTML_SINGLE_TEXT_EXPRESSION))
    } else if p.at(T![<]) {
        let diagnostic = expected_text_expression(p, p.cur_range(), opening_range);
        p.error(diagnostic);
        Present(m.complete(p, HTML_BOGUS_TEXT_EXPRESSION))
    } else {
        m.abandon(p);
        p.rewind(checkpoint);
        let recovery =
            ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_TEXT_EXPRESSION_LIST);
        if let Ok(m) = recovery.enable_recovery_on_line_break().recover(p) {
            let diagnostic = expected_text_expression(p, m.range(p), opening_range);
            p.error(diagnostic);
            Present(m)
        } else {
            Absent
        }
    }
}

struct TextExpression {
    kind: TextExpressionKind,
}

impl TextExpression {
    pub fn new_single() -> Self {
        Self {
            kind: TextExpressionKind::Single,
        }
    }

    pub fn new_double() -> Self {
        Self {
            kind: TextExpressionKind::Double,
        }
    }
}

impl TextExpression {
    fn parse_element(&mut self, p: &mut HtmlParser) -> ParsedSyntax {
        if p.at(EOF) || p.at(T![<]) {
            return Absent;
        }

        let m = p.start();

        match self.kind {
            TextExpressionKind::Single => {
                if p.at(T!["}}"]) {
                    p.bump_remap_with_context(
                        HTML_LITERAL,
                        HtmlLexContext::TextExpression(self.kind),
                    );
                } else {
                    p.bump_remap_any_with_context(HtmlLexContext::TextExpression(self.kind));
                }
            }
            TextExpressionKind::Double => {
                if p.at(T!['}']) {
                    p.bump_remap_with_context(
                        HTML_LITERAL,
                        HtmlLexContext::TextExpression(self.kind),
                    )
                } else {
                    p.bump_remap_any_with_context(HtmlLexContext::TextExpression(self.kind))
                }
            }
        }

        Present(m.complete(p, HTML_TEXT_EXPRESSION))
    }
}
