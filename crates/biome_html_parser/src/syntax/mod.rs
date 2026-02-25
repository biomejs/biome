mod astro;
mod parse_error;
mod svelte;
mod vue;

use crate::parser::HtmlParser;
use crate::syntax::HtmlSyntaxFeatures::{
    Astro, DoubleTextExpressions, SingleTextExpressions, Svelte, Vue,
};
use crate::syntax::astro::{
    is_at_astro_directive_keyword, is_at_astro_directive_start, parse_astro_directive,
    parse_astro_fence, parse_astro_spread_or_expression,
};
use crate::syntax::parse_error::*;
use crate::syntax::svelte::{
    is_at_svelte_directive_start, is_at_svelte_keyword, parse_attach_attribute,
    parse_svelte_at_block, parse_svelte_directive, parse_svelte_hash_block,
    parse_svelte_spread_or_expression,
};
use crate::syntax::vue::{
    parse_vue_directive, parse_vue_v_bind_shorthand_directive, parse_vue_v_on_shorthand_directive,
    parse_vue_v_slot_shorthand_directive,
};
use crate::token_source::{
    HtmlEmbeddedLanguage, HtmlLexContext, HtmlReLexContext, TextExpressionKind,
};
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
    /// Exclusive to Svelte files (for Svelte-specific directives)
    Svelte,
    /// Exclusive to those documents that support Vue
    Vue,
}

impl SyntaxFeature for HtmlSyntaxFeatures {
    type Parser<'source> = HtmlParser<'source>;

    fn is_supported(&self, p: &HtmlParser) -> bool {
        match self {
            Astro => p.options().frontmatter,
            DoubleTextExpressions => {
                p.options().text_expression == Some(TextExpressionKind::Double)
            }
            SingleTextExpressions => {
                p.options().text_expression == Some(TextExpressionKind::Single)
            }
            Svelte => p.options().svelte,
            Vue => p.options().vue,
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

    // Whether or not frontmatter was present, once we're past the frontmatter
    // position `---` can no longer start a fence. This prevents `---` in HTML
    // content from being incorrectly lexed as a FENCE token.
    if p.options().frontmatter {
        p.set_after_frontmatter(true);
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

/// We need to treat `:`, `.` and `@` differently if we are in a Vue or Astro context.
///
/// Normally, we would do this using [`HtmlSyntaxFeatures`], and we do this elsewhere.
/// However, this makes it so that these characters are disallowed and using them
/// will emit diagnostics. We want to allow them if they have no special meaning.
#[inline(always)]
fn inside_tag_context(p: &HtmlParser) -> HtmlLexContext {
    if Vue.is_supported(p) {
        HtmlLexContext::InsideTagWithDirectives { svelte: false }
    } else if Svelte.is_supported(p) {
        HtmlLexContext::InsideTagSvelte
    } else {
        HtmlLexContext::InsideTag
    }
}

fn is_possible_component(p: &HtmlParser, tag_name: &str) -> bool {
    tag_name
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_uppercase())
        && !p.options().is_html()
}

/// Returns the lexer context to use when parsing component names and member expressions.
/// This allows `.` to be lexed as a token for member expressions like Component.Member
/// We reuse [HtmlLexContext::InsideTagWithDirectives] context because it supports `.` lexing, but this is ONLY used
/// for parsing component names, not for parsing attributes.
#[inline(always)]
fn component_name_context(p: &HtmlParser) -> HtmlLexContext {
    if Vue.is_supported(p) || Svelte.is_supported(p) || Astro.is_supported(p) {
        // Use HtmlLexContext::InsideTagWithDirectives for all component-supporting files when parsing component names
        // This allows `.` to be lexed properly for member expressions
        // Note: This is safe because we only use this context for tag names, not attributes
        HtmlLexContext::InsideTagWithDirectives {
            svelte: Svelte.is_supported(p),
        }
    } else {
        HtmlLexContext::InsideTag
    }
}

/// Parse a tag name, which returns AnyHtmlTagName (one of: HtmlTagName, HtmlComponentName, or HtmlMemberName)
/// This follows the JSX parser pattern for handling member expressions like Component.Member
fn parse_any_tag_name(p: &mut HtmlParser) -> ParsedSyntax {
    if !is_at_start_literal(p) {
        return Absent;
    }

    let tag_text = p.cur_text();

    // Step 1: Parse base name (either component or regular tag)
    let name = if is_possible_component(p, tag_text) {
        // Parse as component name - use component_name_context to allow `.` for member expressions
        let m = p.start();
        p.bump_with_context(HTML_LITERAL, component_name_context(p));
        Present(m.complete(p, HTML_COMPONENT_NAME))
    } else {
        // Parse as regular HTML tag
        parse_literal(p, HTML_TAG_NAME)
    };

    // Step 2: Extend with member access if present (using .map() pattern from JSX parser)
    name.map(|mut name| {
        while p.at(T![.]) {
            let m = name.precede(p); // Create marker BEFORE already-parsed name
            p.bump_with_context(T![.], component_name_context(p)); // Use component context for `.`

            // Parse member name - must use component_name_context to maintain `.` lexing
            if is_at_start_literal(p) {
                let member_m = p.start();
                p.bump_with_context(HTML_LITERAL, component_name_context(p));
                member_m.complete(p, HTML_TAG_NAME);
            } else {
                p.error(expected_element_name(p, p.cur_range()));
            }

            name = m.complete(p, HTML_MEMBER_NAME); // Wrap previous name
        }
        name
    })
}

fn parse_element(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }
    let m = p.start();

    p.bump_with_context(T![<], inside_tag_context(p));
    let opening_tag_name = p.cur_text().to_string();
    let should_be_self_closing = VOID_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(opening_tag_name.as_str()))
        && !is_possible_component(p, opening_tag_name.as_str());
    let is_embedded_language_tag = EMBEDDED_LANGUAGE_ELEMENTS
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case(opening_tag_name.as_str()));

    parse_any_tag_name(p).or_add_diagnostic(p, expected_element_name);

    if Astro.is_supported(p) {
        p.re_lex(HtmlReLexContext::InsideTagAstro);
    }

    AttributeList.parse_list(p);

    if p.at(T![/]) {
        p.bump_with_context(T![/], inside_tag_context(p));
        p.expect_with_context(T![>], HtmlLexContext::Regular);
        Present(m.complete(p, HTML_SELF_CLOSING_ELEMENT))
    } else {
        if should_be_self_closing {
            if p.at(T![/]) {
                p.bump_with_context(T![/], inside_tag_context(p));
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

        // if the lexer found a keyword, rewind and lex as text
        if is_at_keyword(p) {
            p.re_lex(HtmlReLexContext::HtmlText);
        }

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
        .any(|tag| tag.eq_ignore_ascii_case(p.cur_text()))
        && !is_possible_component(p, p.cur_text());
    if should_be_self_closing {
        p.error(void_element_should_not_have_closing_tag(p, p.cur_range()).into_diagnostic(p));
    }
    let _name = parse_any_tag_name(p);

    // There shouldn't be any attributes in a closing tag.
    while p.at(HTML_LITERAL) || p.at(T!["{{"]) || p.at(T!["}}"]) {
        p.error(closing_tag_should_not_have_attributes(p, p.cur_range()));
        p.bump_remap_with_context(HTML_BOGUS, HtmlLexContext::InsideTag);
    }
    p.expect(T![>]);
    Present(m.complete(p, HTML_CLOSING_ELEMENT))
}

pub(crate) fn parse_html_element(p: &mut HtmlParser) -> ParsedSyntax {
    match p.cur() {
        T!["<![CDATA["] => parse_cdata_section(p),
        T![<] => parse_element(p),
        T!["{{"] => HtmlSyntaxFeatures::DoubleTextExpressions.parse_exclusive_syntax(
            p,
            |p| parse_double_text_expression(p, HtmlLexContext::Regular),
            |p, m| disabled_interpolation(p, m.range(p)),
        ),
        T!["{@"] => parse_svelte_at_block(p),
        T!["{#"] => parse_svelte_hash_block(p),
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
        // At this position, we shouldn't have svelte keyword, so we relex everything
        // as text
        _ if is_at_svelte_keyword(p) => {
            let m = p.start();
            p.re_lex(HtmlReLexContext::HtmlText);
            p.bump_with_context(HTML_LITERAL, HtmlLexContext::Regular);
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

#[derive(Default)]
struct ElementList;

impl ParseNodeList for ElementList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = HTML_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_html_element(p)
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
        p.at(T![>]) || p.at(T![/]) || p.at(T!['}'])
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

    match p.cur() {
        T!["{{"] => {
            let m = p.start();
            DoubleTextExpressions
                .parse_exclusive_syntax(
                    p,
                    |p| parse_double_text_expression(p, HtmlLexContext::InsideTag),
                    |p, marker| disabled_interpolation(p, marker.range(p)),
                )
                .ok();
            Present(m.complete(p, HTML_ATTRIBUTE))
        }
        T!["{{"] => {
            let m = p.start();
            HtmlSyntaxFeatures::DoubleTextExpressions
                .parse_exclusive_syntax(
                    p,
                    |p| parse_double_text_expression(p, HtmlLexContext::InsideTag),
                    |p, marker| disabled_interpolation(p, marker.range(p)),
                )
                .ok();

            Present(m.complete(p, HTML_ATTRIBUTE))
        }
        // Check for Astro directives before Vue colon shorthand
        // This must come first because in Astro files, colons are lexed as separate tokens
        _ if Astro.is_supported(p) && is_at_astro_directive_start(p) => parse_astro_directive(p),
        T![:] => HtmlSyntaxFeatures::Vue.parse_exclusive_syntax(
            p,
            parse_vue_v_bind_shorthand_directive,
            |p, m| disabled_vue(p, m.range(p)),
        ),
        T![@] => HtmlSyntaxFeatures::Vue.parse_exclusive_syntax(
            p,
            parse_vue_v_on_shorthand_directive,
            |p, m| disabled_vue(p, m.range(p)),
        ),
        T![#] => HtmlSyntaxFeatures::Vue.parse_exclusive_syntax(
            p,
            parse_vue_v_slot_shorthand_directive,
            |p, m| disabled_vue(p, m.range(p)),
        ),
        T!['{'] if SingleTextExpressions.is_supported(p) => parse_svelte_spread_or_expression(p),
        T!['{'] if Astro.is_supported(p) => parse_astro_spread_or_expression(p),
        // Keep previous behaviour so that invalid documents are still parsed.
        T!['{'] => Svelte.parse_exclusive_syntax(
            p,
            |p| parse_svelte_spread_or_expression(p),
            |p: &HtmlParser<'_>, m: &CompletedMarker| disabled_svelte(p, m.range(p)),
        ),
        T!["{@"] => Svelte.parse_exclusive_syntax(
            p,
            |p| parse_attach_attribute(p),
            |p: &HtmlParser<'_>, m: &CompletedMarker| disabled_svelte(p, m.range(p)),
        ),
        _ if p.cur_text().starts_with("v-") => {
            Vue.parse_exclusive_syntax(p, parse_vue_directive, |p, m| disabled_vue(p, m.range(p)))
        }
        _ if Svelte.is_supported(p) && is_at_svelte_directive_start(p) => Svelte
            .parse_exclusive_syntax(p, parse_svelte_directive, |p, m| {
                disabled_svelte(p, m.range(p))
            }),
        _ => {
            let m = p.start();
            // we've already determined that this isn't a valid astro directive, so if it looks like one, we should remap it as a literal.
            if Astro.is_supported(p) && is_at_astro_directive_keyword(p) {
                let name = p.start();
                p.bump_remap_with_context(HTML_LITERAL, inside_tag_context(p));
                name.complete(p, HTML_ATTRIBUTE_NAME);
            } else {
                parse_literal(p, HTML_ATTRIBUTE_NAME).or_add_diagnostic(p, expected_attribute);
            }

            if p.at(T![=]) {
                parse_attribute_initializer(p).ok();
            }
            Present(m.complete(p, HTML_ATTRIBUTE))
        }
    }
}

fn is_at_attribute_start(p: &mut HtmlParser) -> bool {
    p.at_ts(token_set![
        HTML_LITERAL,
        T!["{{"],
        T!['{'],
        T![:],
        T![@],
        T![#],
    ]) || (Svelte.is_supported(p) && p.at(T!["{@"]))
        || (Astro.is_supported(p) && is_at_astro_directive_keyword(p))
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
                    HTML_TAG_NAME | HTML_ATTRIBUTE_NAME | HTML_COMPONENT_NAME
                    | HTML_MEMBER_NAME => inside_tag_context(p),
                    _ => HtmlLexContext::Regular,
                },
            )
        }
    } else if p.at(T!["}}"]) {
        p.bump_remap_with_context(
            HTML_LITERAL,
            match kind {
                HTML_TAG_NAME | HTML_ATTRIBUTE_NAME | HTML_COMPONENT_NAME | HTML_MEMBER_NAME => {
                    inside_tag_context(p)
                }
                _ => HtmlLexContext::Regular,
            },
        );
    } else {
        p.bump_with_context(
            HTML_LITERAL,
            match kind {
                HTML_TAG_NAME | HTML_ATTRIBUTE_NAME | HTML_COMPONENT_NAME | HTML_MEMBER_NAME => {
                    inside_tag_context(p)
                }
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

    p.bump_with_context(HTML_STRING_LITERAL, inside_tag_context(p));

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
                |p| parse_single_text_expression(p, inside_tag_context(p)),
                |p, m| {
                    p.err_builder(
                        "Text expressions are not supported in this context.",
                        m.range(p),
                    )
                    .with_hint("Remove the expression or use a supported file type.")
                },
            )
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_ATTRIBUTE_LIST),
                expected_attribute,
            )
            .ok();

        if Astro.is_supported(p) {
            p.re_lex(HtmlReLexContext::InsideTagAstro);
        }
    } else if p.at(T!["{{"]) {
        HtmlSyntaxFeatures::DoubleTextExpressions
            .parse_exclusive_syntax(
                p,
                |p| parse_double_text_expression(p, inside_tag_context(p)),
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
    p.bump_with_context(T!["{{"], HtmlLexContext::double_expression());

    TextExpression::new_double().parse_element(p).ok();

    if p.at(T!["}}"]) {
        p.expect_with_context(T!["}}"], context);
        if context == HtmlLexContext::InsideTag
            || matches!(context, HtmlLexContext::InsideTagWithDirectives { .. })
        {
            Present(m.complete(p, HTML_ATTRIBUTE_DOUBLE_TEXT_EXPRESSION))
        } else {
            Present(m.complete(p, HTML_DOUBLE_TEXT_EXPRESSION))
        }
    } else if p.at(T![<]) {
        let diagnostic = expected_closing_text_expression(p, p.cur_range(), opening_range);
        p.error(diagnostic);
        Present(m.complete(p, HTML_BOGUS_TEXT_EXPRESSION))
    } else {
        m.abandon(p);
        p.rewind(checkpoint);

        let recovery =
            ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_TEXT_EXPRESSION_LIST);
        if let Ok(m) = recovery.enable_recovery_on_line_break().recover(p) {
            let diagnostic = expected_closing_text_expression(p, m.range(p), opening_range);
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

/// Parsers a single tag expression. `context` is applied after lexing the last token `}`
pub(crate) fn parse_single_text_expression(
    p: &mut HtmlParser,
    context: HtmlLexContext,
) -> ParsedSyntax {
    if !SingleTextExpressions.is_supported(p) {
        return Absent;
    }

    if !p.at(T!['{']) {
        return Absent;
    }
    let checkpoint = p.checkpoint();
    let m = p.start();
    let opening_range = p.cur_range();

    p.bump_with_context(T!['{'], HtmlLexContext::single_expression());

    TextExpression::new_single().parse_element(p).ok();

    if p.at(T!['}']) {
        p.bump_remap_with_context(T!['}'], context);
        if context == HtmlLexContext::InsideTag
            || matches!(context, HtmlLexContext::InsideTagWithDirectives { .. })
            || context == HtmlLexContext::InsideTagAstro
            || context == HtmlLexContext::InsideTagSvelte
        {
            Present(m.complete(p, HTML_ATTRIBUTE_SINGLE_TEXT_EXPRESSION))
        } else {
            Present(m.complete(p, HTML_SINGLE_TEXT_EXPRESSION))
        }
    } else if p.at(T![<]) {
        let diagnostic = expected_closing_text_expression(p, p.cur_range(), opening_range);
        p.error(diagnostic);
        Present(m.complete(p, HTML_BOGUS_TEXT_EXPRESSION))
    } else {
        m.abandon(p);
        p.rewind(checkpoint);
        let recovery =
            ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_TEXT_EXPRESSION_LIST);
        if let Ok(m) = recovery.enable_recovery_on_line_break().recover(p) {
            let diagnostic = expected_closing_text_expression(p, m.range(p), opening_range);
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

fn parse_single_text_expression_content(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at(EOF) || p.at(T![<]) || p.at(T!['}']) || p.cur_text().trim().is_empty() {
        return Absent;
    }
    let m = p.start();

    p.bump_remap(HTML_LITERAL);

    Present(m.complete(p, HTML_TEXT_EXPRESSION))
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
                } else if !p.at(T!['}']) {
                    p.bump_remap(HTML_LITERAL);
                } else {
                    m.abandon(p);
                    return Absent;
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

fn is_at_keyword(p: &mut HtmlParser) -> bool {
    is_at_svelte_keyword(p) || is_at_html_keyword(p)
}

fn is_at_html_keyword(p: &mut HtmlParser) -> bool {
    matches!(p.cur(), T![html] | T![doctype])
}
