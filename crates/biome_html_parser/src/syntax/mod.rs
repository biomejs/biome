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

    /// Determines if the syntax feature is enabled for the given parser based on its options.
    ///
    /// Returns `true` if the feature is supported according to the parser's configuration; otherwise, returns `false`.
    fn is_supported(&self, p: &HtmlParser) -> bool {
        match self {
            Self::Astro => p.options().frontmatter,
            Self::DoubleTextExpressions => {
                p.options().text_expression == Some(crate::parser::TextExpressionKind::Double)
            }
            Self::SingleTextExpressions => {
                p.options().text_expression == Some(crate::parser::TextExpressionKind::Single)
            }
        }
    }
}

const RECOVER_ATTRIBUTE_LIST: TokenSet<HtmlSyntaxKind> = token_set!(T![>], T![<], T![/]);
const RECOVER_TEXT_EXPRESSION_LIST: TokenSet<HtmlSyntaxKind> = token_set!(T![<], T!['}'], T!["}}"]);

/// These elements are effectively always self-closing. They should not have a closing tag (if they do, it should be a parsing error). They might not contain a `/` like in `<img />`.
static VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

/// For these elements, the content is treated as raw text and no parsing is done inside them. This is so that the contents of these tags can be parsed by a different parser.
pub(crate) static EMBEDDED_LANGUAGE_ELEMENTS: &[&str] = &["script", "style", "pre"];

/// Parses the root of an HTML document, including optional Astro frontmatter, doctype, and the main element list.
///
/// If the document starts with a Unicode BOM, it is consumed. If frontmatter (`---`) is present and the Astro feature is enabled, it is parsed; otherwise, an error is emitted. The function then parses an optional doctype and the main list of HTML elements, completing the root node as `HTML_ROOT`.
///
/// # Examples
///
/// ```
/// let mut parser = HtmlParser::new("<!doctype html><div>Hello</div>");
/// parse_root(&mut parser);
/// // The parser tree now has an HTML_ROOT node with a doctype and a div element.
/// ```
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
        loop {
            ElementList.parse_list(p);
            if let Some(mut closing) =
                parse_closing_tag(p).or_add_diagnostic(p, expected_closing_tag)
            {
                if !closing.text(p).contains(opening_tag_name.as_str()) {
                    p.error(expected_matching_closing_tag(p, closing.range(p)).into_diagnostic(p));
                    closing.change_to_bogus(p);
                    continue;
                }
            }
            break;
        }
        let previous = opening.precede(p);

        Present(previous.complete(p, HTML_ELEMENT))
    }
}

/// Parses an HTML closing tag (e.g., `</div>`).
///
/// Emits an error if the tag is a void element or if attributes are present within the closing tag.
/// Returns `Present` if a closing tag is successfully parsed, otherwise returns `Absent`.
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

    /// Parses a single HTML content node within an element list.
    ///
    /// Handles CDATA sections, nested elements, double and single text expressions (if enabled), and literal content. Remaps unmatched closing expression tokens and literals to `HTML_CONTENT`.
    ///
    /// # Returns
    /// * `Present` if a valid content node is parsed.
    /// * `Absent` if no valid content node is found.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = HtmlParser::new("<div>{{ expr }}</div>");
    /// let mut element_list = ElementList;
    /// let node = element_list.parse_element(&mut parser);
    /// assert!(node.is_present());
    /// ```
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

    /// Parses a single HTML attribute within an attribute list.
    ///
    /// Returns the parsed attribute node if present; otherwise, returns `Absent`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut parser = HtmlParser::new(r#"class="foo""#);
    /// let mut list = AttributeList;
    /// let attr = list.parse_element(&mut parser);
    /// assert!(attr.is_present());
    /// ```
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

/// Parses an HTML attribute, supporting both standard attributes and double text expression attributes (`{{ ... }}`).
///
/// If the attribute starts with `{{`, parses it as a double text expression attribute if the feature is enabled; otherwise, emits an error. For standard attributes, parses the attribute name and, if present, its initializer.
///
/// # Examples
///
/// ```
/// let mut parser = HtmlParser::new(r#"foo="bar""#);
/// let attr = parse_attribute(&mut parser);
/// assert!(attr.is_present());
/// ```
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

/// Returns `true` if the parser is positioned at the start of an HTML attribute.
///
/// Recognizes attribute starts as a literal, a double text expression (`{{`), or a single text expression (`{`).
///
/// # Examples
///
/// ```
/// assert!(is_at_attribute_start(&mut parser)); // when at a literal, `{{`, or `{`
/// ```
fn is_at_attribute_start(p: &mut HtmlParser) -> bool {
    p.at(HTML_LITERAL) || p.at(T!["{{"]) || p.at(T!['{'])
}

/// Parses a literal token or a double text expression as the specified HTML syntax kind.
///
/// If the current token is a double text expression (`{{ ... }}`) and the feature is enabled, parses it as a text expression. Otherwise, remaps the token as a literal with the appropriate lexing context based on the kind.
///
/// Returns `Present` if a literal or double text expression is parsed, or `Absent` if not at a literal start.
///
/// # Examples
///
/// ```
/// let mut parser = HtmlParser::new("foo");
/// let result = parse_literal(&mut parser, HTML_LITERAL);
/// assert!(result.is_present());
/// ```
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

/// Parses an attribute initializer following an equals sign in an HTML tag.
///
/// Supports parsing string literals, single text expressions (`{}`) if enabled, and double text expressions (`{{}}`) if enabled. Emits diagnostics and attempts recovery if expressions are not supported or malformed.
///
/// # Returns
/// `Present` if an attribute initializer is successfully parsed, otherwise `Absent`.
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

/// Parses a double text expression delimited by `{{` and `}}`.
///
/// This function recognizes and parses double-brace text expressions, such as `{{ expression }}`, within HTML content. It handles error recovery for missing or malformed closing delimiters and marks invalid expressions as bogus nodes when necessary.
///
/// # Examples
///
/// ```
/// let mut parser = HtmlParser::new("{{ foo + bar }}");
/// let syntax = parse_double_text_expression(&mut parser, HtmlLexContext::default());
/// assert!(syntax.is_present());
/// ```
fn parse_double_text_expression(p: &mut HtmlParser, context: HtmlLexContext) -> ParsedSyntax {
    if !is_at_opening_double_expression(p) {
        return Absent;
    }
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
        let recovery =
            ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_TEXT_EXPRESSION_LIST);
        if let Ok(m) = recovery.enable_recovery_on_line_break().recover(p) {
            let diagnostic = expected_text_expression(p, m.range(p), opening_range);
            p.error(diagnostic);
        }

        p.expect(T![<]);
        Present(m.complete(p, HTML_DOUBLE_TEXT_EXPRESSION))
    }
}

/// Returns `true` if the current token is the start of a double text expression (`{{`).
///
/// # Examples
///
/// ```
/// let mut parser = HtmlParser::new("{{ expression }}");
/// assert!(is_at_opening_double_expression(&mut parser));
/// ```
pub(crate) fn is_at_opening_double_expression(p: &mut HtmlParser) -> bool {
    p.at(T!["{{"])
}

// Parsers a single tag expression. `context` is applied after lexing the last token `}`
/// Parses a single text expression delimited by `{` and `}` if the feature is enabled.
///
/// Returns a completed `HTML_SINGLE_TEXT_EXPRESSION` node on success, or a `HTML_BOGUS_TEXT_EXPRESSION` node if the closing delimiter is missing or malformed. If the feature is not supported or the current token is not `{`, returns `Absent`.
///
/// # Examples
///
/// ```
/// // Assuming the parser is configured to support single text expressions:
/// let syntax = parse_single_text_expression(&mut parser, HtmlLexContext::default());
/// assert!(syntax.is_present());
/// ```
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
        let recovery =
            ParseRecoveryTokenSet::new(HTML_BOGUS_TEXT_EXPRESSION, RECOVER_TEXT_EXPRESSION_LIST);
        if let Ok(m) = recovery.enable_recovery_on_line_break().recover(p) {
            let diagnostic = expected_text_expression(p, m.range(p), opening_range);
            p.error(diagnostic);
        }

        p.expect(T![<]);
        Present(m.complete(p, HTML_SINGLE_TEXT_EXPRESSION))
    }
}

struct TextExpression {
    kind: TextExpressionKind,
}

impl TextExpression {
    /// Creates a `TextExpression` representing a single-brace text expression (`{ }`).
    ///
    /// # Examples
    ///
    /// ```
    /// let expr = TextExpression::new_single();
    /// assert_eq!(expr.kind, TextExpressionKind::Single);
    /// ```
    pub fn new_single() -> Self {
        Self {
            kind: TextExpressionKind::Single,
        }
    }

    /// Creates a new `TextExpression` representing a double-brace (`{{ }}`) text expression.
    ///
    /// # Examples
    ///
    /// ```
    /// let expr = TextExpression::new_double();
    /// assert_eq!(expr.kind, TextExpressionKind::Double);
    /// ```
    pub fn new_double() -> Self {
        Self {
            kind: TextExpressionKind::Double,
        }
    }
}

impl TextExpression {
    /// Parses a single token inside a text expression, remapping it as a literal within the expression context.
    ///
    /// Returns `Absent` if at the end of file or at the start of a new HTML element.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut expr = TextExpression::new_single();
    /// let syntax = expr.parse_element(&mut parser);
    /// assert!(syntax.is_present() || syntax.is_absent());
    /// ```
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
