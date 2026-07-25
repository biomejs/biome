mod angular;
mod astro;
mod parse_error;
mod svelte;
mod vue;

use crate::parser::{HtmlParser, HtmlParserCheckpoint};
use crate::syntax::HtmlSyntaxFeatures::{
    Angular, Astro, DoubleTextExpressions, SingleTextExpressions, Svelte, Vue,
};
use crate::syntax::angular::{
    parse_angular_event_binding, parse_angular_property_binding,
    parse_angular_structural_directive, parse_angular_template_ref, parse_angular_two_way_binding,
};
use crate::syntax::astro::{
    is_at_astro_directive_keyword, is_at_astro_directive_start, parse_astro_directive,
    parse_astro_fence, parse_astro_spread_or_expression,
};
use crate::syntax::parse_error::*;
use crate::syntax::svelte::{
    SVELTE_KEYWORDS, is_at_svelte_directive_start, parse_attach_attribute, parse_svelte_at_block,
    parse_svelte_declaration_or_expression, parse_svelte_directive, parse_svelte_hash_block,
    parse_svelte_spread_or_expression,
};
use crate::syntax::vue::{
    VUE_KEYWORDS, parse_vue_directive, parse_vue_v_bind_shorthand_directive, parse_vue_v_for_value,
    parse_vue_v_on_shorthand_directive, parse_vue_v_slot_shorthand_directive,
};
use crate::token_source::{
    HtmlEmbeddedLanguage, HtmlFramework, HtmlLexContext, HtmlReLexContext, PreformattedElement,
    TextExpressionKind,
};
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use biome_parser::{Marker, Parser};
use biome_rowan::{TextRange, TextSize};
use biome_string_case::StrLikeExtension;

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
    /// Exclusive to those documents that support Angular
    Angular,
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
            Angular => p.options().angular,
        }
    }
}

const RECOVER_ATTRIBUTE_LIST: TokenSet<HtmlSyntaxKind> = token_set!(T![>], T![<], T![/]);
const RECOVER_TEXT_EXPRESSION_LIST: TokenSet<HtmlSyntaxKind> =
    token_set!(T![<], T![>], T!['}'], T!["}}"]);

/// HTML [void elements](https://html.spec.whatwg.org/#void-elements): they never
/// have content or a closing tag. Tag names are keywords, so this is an `O(1)`
/// token-kind set.
const VOID_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(
    T![area],
    T![base],
    T![br],
    T![col],
    T![embed],
    T![hr],
    T![img],
    T![input],
    T![link],
    T![meta],
    T![param],
    T![source],
    T![track],
    T![wbr]
);

/// Elements whose content is treated as raw text / an embedded language. `script`
/// and `style` share their kinds between HTML and SVG. The rest are
/// preformatted: their text is whitespace-sensitive, and lexing it as one
/// literal keeps the whitespace in the token instead of losing it to trivia.
const EMBEDDED_LANGUAGE_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(
    T![script],
    T![style],
    T![pre],
    T![textarea],
    T![xmp],
    T![plaintext]
);

/// The subset of [`EMBEDDED_LANGUAGE_ELEMENTS`] that is raw text because it is
/// preformatted rather than because it hosts another language.
const PREFORMATTED_ELEMENTS: TokenSet<HtmlSyntaxKind> =
    token_set!(T![pre], T![textarea], T![xmp], T![plaintext]);

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
    p.set_after_frontmatter(true);

    parse_doc_type(p).ok();
    // Only a real `.vue` document has single-file-component blocks. Plain HTML
    // parsed with the Vue extensions turned on keeps ordinary element nesting,
    // where an unknown top-level tag is a custom element rather than a block.
    p.set_at_vue_sfc_top_level(Vue.is_supported(p) && !p.options().is_html());
    ElementList.parse_list(p);
    p.set_at_vue_sfc_top_level(false);

    m.complete(p, HTML_ROOT);
}

fn parse_doc_type(p: &mut HtmlParser) -> ParsedSyntax {
    if !(p.at(T![<]) && p.nth_at(1, T![!])) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![<], inside_tag_context(p));
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

/// The framework flavor of the file currently being parsed.
#[inline(always)]
fn html_framework(p: &HtmlParser) -> HtmlFramework {
    if Vue.is_supported(p) {
        HtmlFramework::Vue
    } else if Svelte.is_supported(p) {
        HtmlFramework::Svelte
    } else if Astro.is_supported(p) {
        HtmlFramework::Astro
    } else if Angular.is_supported(p) {
        HtmlFramework::Angular
    } else {
        HtmlFramework::Plain
    }
}

/// The lexer context to use inside a `<...>` tag. `framework` selects the directive
/// lexing and whether PascalCase tag names are treated as component names.
///
/// We need to treat `:`, `.` and `@` differently if we are in a Vue or Astro context.
///
/// Normally, we would do this using [`HtmlSyntaxFeatures`], and we do this elsewhere.
/// However, this makes it so that these characters are disallowed and using them
/// will emit diagnostics. We want to allow them if they have no special meaning.
#[inline(always)]
fn inside_tag_context(p: &HtmlParser) -> HtmlLexContext {
    HtmlLexContext::InsideTag {
        framework: html_framework(p),
    }
}

/// Returns the lexer context to use when parsing component names and member
/// expressions (e.g. `Foo` / `Foo.Bar`). The tag-name token is always emitted as
/// `HTML_COMPONENT_LITERAL` and `.` is lexed as a token for member access. This is
/// only used once the parser already knows the name is a component.
#[inline(always)]
fn component_name_context(p: &HtmlParser) -> HtmlLexContext {
    if Angular.is_supported(p) {
        return inside_tag_context(p);
    }

    HtmlLexContext::InsideTagWithDirectives {
        svelte: Svelte.is_supported(p),
    }
}

/// Parse a tag name, returning one of `HtmlTagName`, `HtmlComponentName`, or
/// `HtmlMemberName`. The lexer decides whether the name is a component (emitting
/// `HTML_COMPONENT_LITERAL`); the parser only assembles member expressions like
/// `Component.Member`, whose parts are always component names.
fn parse_any_tag_name(p: &mut HtmlParser) -> ParsedSyntax {
    if p.cur() == HTML_COMPONENT_LITERAL {
        // Component name, possibly the base of a member expression.
        let m = p.start();
        p.bump_with_context(HTML_COMPONENT_LITERAL, component_name_context(p));
        let mut name = m.complete(p, HTML_COMPONENT_NAME);

        while p.at(T![.]) {
            let m = name.precede(p);
            p.bump_with_context(T![.], component_name_context(p));
            // The member is lexed after a `.`, so the lexer emits `HTML_LITERAL`
            // (or a tag kind) rather than `HTML_COMPONENT_LITERAL`. A member name
            // is always a component, so remap it.
            if p.at(HTML_LITERAL) || p.at(HTML_COMPONENT_LITERAL) || p.cur().is_html_tag_name() {
                let member_m = p.start();
                p.bump_remap_with_context(HTML_COMPONENT_LITERAL, component_name_context(p));
                member_m.complete(p, HTML_COMPONENT_NAME);
            } else {
                p.error(expected_element_name(p, p.cur_range()));
            }
            name = m.complete(p, HTML_MEMBER_NAME);
        }
        Present(name)
    } else if p.cur().is_html_tag_name() {
        // A known HTML/SVG tag, or the unknown-tag fallback.
        let m = p.start();
        p.bump_with_context(p.cur(), inside_tag_context(p));
        Present(m.complete(p, HTML_TAG_NAME))
    } else if is_at_start_literal(p) {
        // A `{{ }}` text expression standing in for a tag name.
        parse_literal(p, HTML_TAG_NAME)
    } else {
        Absent
    }
}

/// Parses `<?instruction foo="bar" ?>
fn parse_processing_instruction(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<?]) {
        return Absent;
    }

    let m = p.start();

    let start_range = p.cur_range();
    p.bump_with_context(T![<?], inside_tag_context(p));

    let name_range = p.cur_range();
    parse_any_tag_name(p).or_add_diagnostic(p, expected_element_name);

    if start_range.end() != name_range.start() {
        p.error(expected_no_spaces(
            p,
            TextRange::new(start_range.end(), name_range.start()),
        ));
    }

    AttributeList.parse_list(p);

    p.expect(T![?>]);

    Present(m.complete(p, HTML_PROCESSING_INSTRUCTION))
}

/// Whether a top-level block of a Vue single-file component holds something
/// other than HTML, and so has to be read as opaque text.
///
/// `<script>` and `<style>` are excluded because the embedded-language path
/// already handles them. A `<template>` is markup unless a `lang` attribute
/// hands it to a preprocessor such as Pug. Everything else is a custom block
/// like `<i18n>` or `<docs>`, whose content the SFC spec leaves entirely to
/// whichever tool consumes it, so it may not even be markup.
///
/// `<html>` is excluded as well: a document that opens with it is a plain HTML
/// page that happens to carry a `.vue` extension, not a component.
///
/// See <https://vuejs.org/api/sfc-spec.html#custom-blocks>.
fn is_vue_raw_text_block(name_kind: HtmlSyntaxKind, attributes: &str) -> bool {
    match name_kind {
        T![script] | T![style] | T![html] => false,
        T![template] => find_attribute_value(attributes, "lang")
            .is_some_and(|lang| !lang.is_empty() && !lang.eq_ignore_ascii_case("html")),
        _ => true,
    }
}

/// Whether a closing tag for `name` appears anywhere after `from`.
///
/// A raw-text block runs to its closing tag, so one that is never closed would
/// swallow the rest of the file and turn a single typo into a cascade of
/// errors. Custom blocks can be named anything, which makes that easy to hit
/// by accident, so an unclosed block falls back to ordinary element parsing
/// and its diagnostics stay where the mistake is.
fn has_closing_tag(source: &str, from: TextSize, name: &str) -> bool {
    let Some(rest) = source.get(usize::from(from)..) else {
        return false;
    };

    rest.match_indices("</").any(|(index, _)| {
        let after_slash = &rest[index + "</".len()..];
        after_slash
            .get(..name.len())
            .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
            && after_slash[name.len()..]
                .trim_start_matches([' ', '\t', '\n', '\r', '\x0C'])
                .starts_with('>')
    })
}

/// Reads the value of `name` out of the attribute region of an opening tag.
///
/// Returns `None` when the attribute is absent, and an empty string when it is
/// present without a value. The region has already been delimited by the
/// attribute parser, so this only has to walk name/value pairs to avoid
/// matching a `name` that appears inside some other attribute's value.
fn find_attribute_value<'a>(attributes: &'a str, name: &str) -> Option<&'a str> {
    let mut rest = attributes;
    loop {
        rest = rest.trim_start();
        if rest.is_empty() {
            return None;
        }

        let name_end = rest
            .find(|c: char| c == '=' || c.is_whitespace())
            .unwrap_or(rest.len());
        let (attribute_name, after_name) = rest.split_at(name_end);
        let after_name = after_name.trim_start();

        let Some(after_eq) = after_name.strip_prefix('=') else {
            // A valueless attribute; the rest of `after_name` is the next one.
            if attribute_name.eq_ignore_ascii_case(name) {
                return Some("");
            }
            rest = after_name;
            continue;
        };

        let after_eq = after_eq.trim_start();
        let (value, remainder) = match after_eq.chars().next() {
            Some(quote @ ('"' | '\'')) => {
                let inner = &after_eq[quote.len_utf8()..];
                match inner.find(quote) {
                    Some(end) => (&inner[..end], &inner[end + quote.len_utf8()..]),
                    None => (inner, ""),
                }
            }
            _ => {
                let end = after_eq.find(char::is_whitespace).unwrap_or(after_eq.len());
                after_eq.split_at(end)
            }
        };

        if attribute_name.eq_ignore_ascii_case(name) {
            return Some(value);
        }
        rest = remainder;
    }
}

fn parse_element(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }
    let m = p.start();

    p.bump_with_context(T![<], inside_tag_context(p));
    // The tag-name token has already been lexed and classified by the lexer, so
    // these checks are now `O(1)` on the token kind.
    let name_kind = p.cur();
    let name_range = p.cur_range();
    let opening_tag_name = p.cur_text().to_string();
    let should_be_self_closing = VOID_ELEMENTS.contains(name_kind);
    // Consumed below, once it is known whether this element opens a block of a
    // Vue single-file component. Nested elements are ordinary markup.
    let is_sfc_block = p.at_sfc_top_level();
    p.set_at_vue_sfc_top_level(false);
    // In Svelte files, the preformatted elements must be parsed as regular
    // elements so that Svelte expressions inside them ({@html expr}, {expr})
    // are visible as AST nodes for variable-reference tracking. The formatter
    // decides verbatim printing on its own, so keeping them off the
    // embedded-language path here costs nothing but the leading and trailing
    // whitespace, which ends up as trivia.
    let is_embedded_language_tag = EMBEDDED_LANGUAGE_ELEMENTS.contains(name_kind)
        && !(PREFORMATTED_ELEMENTS.contains(name_kind) && Svelte.is_supported(p));

    parse_any_tag_name(p).or_add_diagnostic(p, expected_element_name);

    match html_framework(p) {
        HtmlFramework::Svelte => {
            p.re_lex(HtmlReLexContext::InsideTagSvelte);
        }
        HtmlFramework::Astro => {
            p.re_lex(HtmlReLexContext::InsideTagAstro);
        }
        _ => {}
    }

    // The attribute region runs from here to the `>` the list stops at, so the
    // `lang` lookup below reads a span the parser has already delimited.
    let attributes_start = p.cur_range().start();
    AttributeList.parse_list(p);
    let attributes = p
        .source_text()
        .get(usize::from(attributes_start)..usize::from(p.cur_range().start()))
        .unwrap_or_default();
    let is_raw_text_block = is_sfc_block
        && is_vue_raw_text_block(name_kind, attributes)
        && has_closing_tag(p.source_text(), p.cur_range().end(), &opening_tag_name);
    let is_raw_text = is_embedded_language_tag || is_raw_text_block;

    if p.at(T![/]) {
        p.bump_with_context(T![/], inside_tag_context(p));
        p.expect_with_context(T![>], HtmlLexContext::Regular);
        p.set_at_vue_sfc_top_level(is_sfc_block);
        Present(m.complete(p, HTML_SELF_CLOSING_ELEMENT))
    } else {
        if should_be_self_closing {
            if p.at(T![/]) {
                p.bump_with_context(T![/], inside_tag_context(p));
            }
            p.expect_with_context(T![>], HtmlLexContext::Regular);
            p.set_at_vue_sfc_top_level(is_sfc_block);
            return Present(m.complete(p, HTML_SELF_CLOSING_ELEMENT));
        }
        p.expect_with_context(
            T![>],
            if is_raw_text_block {
                HtmlLexContext::EmbeddedLanguage(HtmlEmbeddedLanguage::RawTextBlock {
                    name: name_range,
                })
            } else if is_embedded_language_tag {
                HtmlLexContext::EmbeddedLanguage(match name_kind {
                    T![script] => HtmlEmbeddedLanguage::Script,
                    T![style] => HtmlEmbeddedLanguage::Style,
                    T![pre] => HtmlEmbeddedLanguage::Preformatted(PreformattedElement::Pre),
                    T![textarea] => {
                        HtmlEmbeddedLanguage::Preformatted(PreformattedElement::Textarea)
                    }
                    T![xmp] => HtmlEmbeddedLanguage::Preformatted(PreformattedElement::Xmp),
                    T![plaintext] => {
                        HtmlEmbeddedLanguage::Preformatted(PreformattedElement::Plaintext)
                    }
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

        if is_raw_text {
            // raw text tags always have 1 element as content
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
                {
                    if is_void_closing_tag(p, &closing) {
                        closing.change_to_bogus(p);
                        continue;
                    }

                    if !closing.text(p).contains(opening_tag_name.as_str()) {
                        p.error(
                            expected_matching_closing_tag(p, closing.range(p)).into_diagnostic(p),
                        );
                        closing.change_to_bogus(p);
                        continue;
                    }
                }
                break;
            }
        }
        p.set_at_vue_sfc_top_level(is_sfc_block);
        let previous = opening.precede(p);

        Present(previous.complete(p, HTML_ELEMENT))
    }
}

fn parse_closing_tag(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![<]) || !p.nth_at(1, T![/]) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![<], inside_tag_context(p));
    p.bump_with_context(T![/], inside_tag_context(p));
    // The closing tag name has been classified by the lexer; component closings
    // (`HTML_COMPONENT_LITERAL`) are never void, so this is `O(1)` and correct.
    let is_void_element = VOID_ELEMENTS.contains(p.cur());
    let _name = parse_any_tag_name(p);

    // There shouldn't be any attributes in a closing tag.
    while p.at(HTML_LITERAL) || p.at(T!["{{"]) || p.at(T!["}}"]) {
        p.error(closing_tag_should_not_have_attributes(p, p.cur_range()));
        p.bump_remap_with_context(HTML_BOGUS, inside_tag_context(p));
    }
    p.expect(T![>]);
    let closing = m.complete(p, HTML_CLOSING_ELEMENT);

    if is_void_element {
        p.error(void_element_should_not_have_closing_tag(p, closing.range(p)).into_diagnostic(p));
    }

    Present(closing)
}

fn is_void_closing_tag(p: &HtmlParser, closing: &CompletedMarker) -> bool {
    let text = closing.text(p);
    let Some(name) = text
        .strip_prefix("</")
        .and_then(|text| text.strip_suffix('>'))
        .map(|text| text.trim())
    else {
        return false;
    };

    // PascalCase names in framework files are components, never void elements.
    if !p.options().is_html() && name.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
        return false;
    }

    // Void elements are all HTML elements, matched case-insensitively.
    HtmlSyntaxKind::from_keyword(name.to_ascii_lowercase_cow().as_ref())
        .is_some_and(|kind| VOID_ELEMENTS.contains(kind))
}

#[inline]
pub(crate) fn parse_html_element(p: &mut HtmlParser) -> ParsedSyntax {
    match p.cur() {
        T!["<![CDATA["] => parse_cdata_section(p),
        T![<?] => parse_processing_instruction(p),
        T![<] => parse_element(p),
        T!["{{"] => HtmlSyntaxFeatures::DoubleTextExpressions.parse_exclusive_syntax(
            p,
            |p| parse_double_text_expression(p, HtmlLexContext::Regular),
            |p, m| disabled_interpolation(p, m.range(p)),
        ),
        T!["{@"] => parse_svelte_at_block(p),
        T!["{#"] => parse_svelte_hash_block(p),
        T!['{'] => parse_svelte_declaration_or_expression(p).or_else(|| {
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
        // At this position, keywords are plain text unless a more specific parser
        // handled them first.
        _ if is_at_keyword(p) => {
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
        p.at(T![>]) || p.at(T![?>]) || p.at(T![/]) || p.at(T!['}'])
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
                    |p| parse_double_text_expression(p, inside_tag_context(p)),
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
        T![#] if Angular.is_supported(p) => parse_angular_template_ref(p),
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
        T!['('] => Angular.parse_exclusive_syntax(
            p,
            parse_angular_event_binding,
            |p: &HtmlParser<'_>, m: &CompletedMarker| disabled_angular(p, m.range(p)),
        ),
        T!['['] => Angular.parse_exclusive_syntax(
            p,
            parse_angular_property_binding,
            |p: &HtmlParser<'_>, m: &CompletedMarker| disabled_angular(p, m.range(p)),
        ),
        T!["[("] => Angular.parse_exclusive_syntax(
            p,
            parse_angular_two_way_binding,
            |p: &HtmlParser<'_>, m: &CompletedMarker| disabled_angular(p, m.range(p)),
        ),
        T![*] => Angular.parse_exclusive_syntax(
            p,
            parse_angular_structural_directive,
            |p: &HtmlParser<'_>, m: &CompletedMarker| disabled_angular(p, m.range(p)),
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
                parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
            }
            Present(m.complete(p, HTML_ATTRIBUTE))
        }
    }
}

#[inline]
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
        || Angular.is_supported(p) && p.at_ts(token_set![T!['('], T!['['], T!["[("], T![*]])
}

#[inline]
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

#[inline]
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

struct SvelteTemplateElementList {
    chunk_context: HtmlLexContext,
    has_interpolation: bool,
}

impl SvelteTemplateElementList {
    fn new(chunk_context: HtmlLexContext) -> Self {
        Self {
            chunk_context,
            has_interpolation: false,
        }
    }
}

impl ParseNodeList for SvelteTemplateElementList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = SVELTE_TEMPLATE_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if p.at(T!['{']) {
            let result = parse_single_text_expression(p, self.chunk_context);
            if result.is_present() {
                self.has_interpolation = true;
            }
            result
        } else if p.at(HTML_TEMPLATE_CHUNK) {
            let chunk = p.start();
            p.bump_with_context(HTML_TEMPLATE_CHUNK, self.chunk_context);
            Present(chunk.complete(p, SVELTE_TEMPLATE_CHUNK_ELEMENT))
        } else {
            Absent
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !p.at(T!['{']) && !p.at(HTML_TEMPLATE_CHUNK)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, token_set![T!['"'], T!["'"]]),
            expected_attribute,
        )
    }
}

/// Parses a quoted Svelte attribute value as a template that mixes literal text
/// and `{expression}` interpolations, e.g. `style="top: {top}px"`. The opening
/// quote token must be the current token.
///
/// Returns `true` if any interpolation was encountered. When it returns `false`
/// the value is a plain string and the caller is expected to rewind and re-parse
/// it as an `HtmlString`.
fn parse_svelte_template_attribute_value(p: &mut HtmlParser) -> bool {
    let quote_kind = p.cur();
    let quote = if quote_kind == T!['"'] { b'"' } else { b'\'' };
    let chunk_context = HtmlLexContext::SvelteTemplateChunk { quote };

    let m = p.start();
    p.bump_with_context(quote_kind, chunk_context);

    let mut list = SvelteTemplateElementList::new(chunk_context);
    list.parse_list(p);
    let has_interpolation = list.has_interpolation;

    // r_quote — lex the next token in the inside-tag context so `>` / attributes
    // are correctly recognised after the closing quote.
    if p.at(quote_kind) {
        p.bump_with_context(quote_kind, inside_tag_context(p));
    } else {
        p.error(p.err_builder("Missing closing quote", p.cur_range()));
    }

    m.complete(p, SVELTE_TEMPLATE_ATTRIBUTE_VALUE);
    has_interpolation
}

fn parse_attribute_initializer(
    p: &mut HtmlParser,
    context: AttrInitializerContext,
) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }
    let m = p.start();

    // For v-for, we need to switch to VueVForValue context immediately
    // and parse the v-for value directly (not as an HtmlString first)
    if context == AttrInitializerContext::VueVFor {
        p.bump_with_context(T![=], HtmlLexContext::VueVForValue);
        parse_vue_v_for_value(p).or_add_diagnostic(p, expected_vue_v_for_value);
        return Present(m.complete(p, HTML_ATTRIBUTE_INITIALIZER_CLAUSE));
    }

    let attr_value_context = if Svelte.is_supported(p) {
        HtmlLexContext::SvelteAttributeValue
    } else {
        HtmlLexContext::AttributeValue
    };
    p.bump_with_context(T![=], attr_value_context);
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
    } else if Svelte.is_supported(p) && matches!(p.cur(), T!['"'] | T!["'"]) {
        // Speculatively parse as a template. If no interpolation is found the
        // value is a plain string, so rewind and re-lex it as a single
        // `HTML_STRING_LITERAL` to parse it as a normal `HtmlString`.
        let checkpoint = p.checkpoint();
        if !parse_svelte_template_attribute_value(p) {
            p.rewind(checkpoint);
            p.re_lex(HtmlReLexContext::SvelteAttributeString);
            parse_attribute_string_literal(p).or_add_diagnostic(p, expected_initializer);
        }
    } else {
        parse_attribute_string_literal(p).or_add_diagnostic(p, expected_initializer);
    }
    Present(m.complete(p, HTML_ATTRIBUTE_INITIALIZER_CLAUSE))
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AttrInitializerContext {
    #[default]
    Regular,
    VueVFor,
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
        if matches!(
            context,
            HtmlLexContext::InsideTag { .. } | HtmlLexContext::InsideTagWithDirectives { .. }
        ) {
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

#[inline]
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

    parse_single_text_expression_after_opening(p, context, checkpoint, m, opening_range)
}

pub(crate) fn parse_single_text_expression_after_opening(
    p: &mut HtmlParser,
    context: HtmlLexContext,
    checkpoint: HtmlParserCheckpoint,
    m: Marker,
    opening_range: TextRange,
) -> ParsedSyntax {
    TextExpression::new_single().parse_element(p).ok();

    if p.at(T!['}']) {
        p.bump_remap_with_context(T!['}'], context);
        if matches!(
            context,
            HtmlLexContext::InsideTag { .. }
                | HtmlLexContext::InsideTagWithDirectives { .. }
                | HtmlLexContext::SvelteTemplateChunk { .. }
        ) {
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
    if p.at(EOF) || p.at(T![<]) || p.at(T!['}']) {
        return Absent;
    }
    if p.cur_text().is_empty() {
        p.re_lex(HtmlReLexContext::Svelte);
        return Absent;
    }
    if p.cur_text().trim().is_empty() {
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
                } else if p.cur_text().is_empty() {
                    m.abandon(p);
                    p.re_lex(HtmlReLexContext::Svelte);
                    return Absent;
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

const ALL_POSSIBLE_KEYWORDS: TokenSet<HtmlSyntaxKind> =
    HTML_KEYWORDS.union(SVELTE_KEYWORDS).union(VUE_KEYWORDS);

const HTML_KEYWORDS: TokenSet<HtmlSyntaxKind> = token_set!(T![html], T![doctype]);

#[inline]
fn is_at_keyword(p: &mut HtmlParser) -> bool {
    p.at_ts(ALL_POSSIBLE_KEYWORDS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_attribute_value_reads_quoted_and_bare_values() {
        assert_eq!(find_attribute_value(r#"lang="pug""#, "lang"), Some("pug"));
        assert_eq!(find_attribute_value("lang='pug'", "lang"), Some("pug"));
        assert_eq!(find_attribute_value("lang=pug", "lang"), Some("pug"));
        assert_eq!(find_attribute_value("LANG=\"pug\"", "lang"), Some("pug"));
        assert_eq!(
            find_attribute_value(r#"  lang = "pug"  "#, "lang"),
            Some("pug")
        );
    }

    #[test]
    fn find_attribute_value_skips_other_attributes() {
        assert_eq!(
            find_attribute_value(r#"src="a.pug" lang="pug" scoped"#, "lang"),
            Some("pug")
        );
        // A `lang=` inside another attribute's value is not an attribute.
        assert_eq!(find_attribute_value(r#"title="lang=pug""#, "lang"), None);
        assert_eq!(find_attribute_value("scoped src=a", "lang"), None);
        assert_eq!(find_attribute_value("", "lang"), None);
    }

    #[test]
    fn find_attribute_value_reports_a_valueless_attribute_as_empty() {
        assert_eq!(find_attribute_value("lang", "lang"), Some(""));
        assert_eq!(find_attribute_value("scoped lang", "lang"), Some(""));
        assert_eq!(find_attribute_value(r#"lang="""#, "lang"), Some(""));
    }

    #[test]
    fn has_closing_tag_matches_only_the_whole_name() {
        let from = TextSize::from(0);
        assert!(has_closing_tag("x </docs>", from, "docs"));
        assert!(has_closing_tag("x </DOCS>", from, "docs"));
        assert!(has_closing_tag("x </docs   >", from, "docs"));
        // A longer name that merely starts with `docs` is a different tag.
        assert!(!has_closing_tag("x </docsy>", from, "docs"));
        assert!(!has_closing_tag("x </doc>", from, "docs"));
        assert!(!has_closing_tag("x <docs>", from, "docs"));
        assert!(!has_closing_tag("nothing here", from, "docs"));
    }

    #[test]
    fn has_closing_tag_starts_looking_at_the_offset() {
        // The closing tag sits before the offset, so it does not count.
        assert!(!has_closing_tag("</docs> rest", TextSize::from(7), "docs"));
        assert!(has_closing_tag(
            "</docs> </docs>",
            TextSize::from(7),
            "docs"
        ));
    }
}
