mod parse_error;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use biome_css_syntax::{CssDeclarationList, CssSyntaxKind::*};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, CompletedMarker, Parser, TokenSet};
use biome_rowan::SyntaxKind;

use self::parse_error::{expected_block, expected_component_value, expected_declaration_item};

pub(crate) fn parse_style_sheet(p: &mut CssParser) {
    let m = p.start();
    p.eat(UNICODE_BOM);

    parse_style_sheet_content(p);

    m.complete(p, CSS_STYLE_SHEET);
}

pub(crate) fn parse_style_sheet_content(p: &mut CssParser) {
    AnyStyleSheetContent::default().parse_list(p);
}

#[derive(Default)]
struct AnyStyleSheetContent {}
impl ParseNodeList for AnyStyleSheetContent {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_STYLE_SHEET_CONTENT;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_at_identifier(p) {
            return parse_at_rule(p);
        } else {
            return parse_qualified_rule(p);
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, token_set!(EOF)),
            expected_component_value,
        )
    }
}

pub(crate) fn parse_any_style_sheet_content(p: &mut CssParser) {
    if is_at_at_identifier(p) {
        parse_at_rule(p);
    } else {
        parse_qualified_rule(p);
    }
}

#[derive(Default)]
struct CssAtRuleComponentValue {}
impl ParseNodeList for CssAtRuleComponentValue {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_AT_RULE_COMPONENT_VALUE;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set![T!['}'], T![;], T!['{']])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, token_set!(T!['}'], T![;], T!['{'])),
            expected_component_value,
        )
    }
}

pub(crate) fn parse_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_at_identifier(p) {
        return Absent;
    }
    let m = p.start();
    parse_at_at_identifier(p);

    CssAtRuleComponentValue::default().parse_list(p);

    if p.at(T!['}']) {
        if p.nested {
            return Present(m.complete(p, CSS_AT_RULE));
        }
    }
    parse_at_rule_content(p).or_add_diagnostic(p, expected_block);

    Present(m.complete(p, CSS_AT_RULE))
}

pub(crate) fn is_at_rule_content(p: &mut CssParser) -> bool {
    p.at(T![;]) || is_at_curly_brackets_block(p)
}

pub(crate) fn parse_at_rule_content(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_rule_content(p) {
        return Absent;
    }
    if p.at(T![;]) {
        let content = p.start();
        p.eat(T![;]);
        return Present(content.complete(p, CSS_AT_RULE_SEMICOLON));
    }
    return parse_curly_brackets_block(p);
}

#[derive(Default)]
struct CssQualifiedRulePrelude {}
impl ParseNodeList for CssQualifiedRulePrelude {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_QUALIFIED_RULE_PRELUDE;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set![T!['}'], T!['{']])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, token_set!(T!['}'])),
            expected_component_value,
        )
    }
}

pub(crate) fn parse_qualified_rule(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T!['}']) {
        return Absent;
    }

    let m = p.start();

    CssQualifiedRulePrelude::default().parse_list(p);

    parse_curly_brackets_block(p).or_add_diagnostic(p, expected_block);

    return Present(m.complete(p, CSS_QUALIFIED_RULE));
    // Absent
}

#[inline]
pub(crate) fn is_at_curly_brackets_block(p: &mut CssParser) -> bool {
    p.at(T!['{'])
}
pub(crate) fn parse_curly_brackets_block(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_curly_brackets_block(p) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);
    CssCurlyBracketsBlockContent::default().parse_list(p);
    p.expect(T!['}']);
    Present(m.complete(p, CSS_CURLY_BRACKETS_BLOCK))
}

#[derive(Default)]
pub(crate) struct CssBlockDeclarationList {}

impl ParseSeparatedList for CssBlockDeclarationList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_DECLARATION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_declaration(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, token_set!(T![;], T!['}'])),
            expected_declaration_item,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![;]
    }
}

#[derive(Default)]
pub(crate) struct CssCurlyBracketsBlockContent {}

impl ParseNodeList for CssCurlyBracketsBlockContent {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_CURLY_BRACKETS_BLOCK_CONTENT;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_at_identifier(p) {
            return parse_at_rule(p);
        } else if is_parse_declaration(p) {
            let m = p.start();
            CssBlockDeclarationList::default().parse_list(p);
            return Present(m.complete(p, CSS_BLOCK_DECLARATION_LIST));
        } else {
            return parse_qualified_rule(p);
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set![T!['}']])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, token_set!(T!['}'])),
            expected_component_value,
        )
    }
}

#[derive(Default)]
struct CssComponentValueList {}
impl ParseNodeList for CssComponentValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        // dbg!("parse_element", p.cur());
        parse_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // dbg!("is_at_list_end", p.cur());
        p.at_ts(token_set![T!['}'], T![;], T![')']])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, token_set!(T!['}'], T![;], T![')'])),
            expected_component_value,
        )
    }
}

pub(crate) fn is_parse_declaration(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at(1, T![:])
}

pub(crate) fn parse_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }
    let m = p.start();
    parse_regular_identifier(p);
    p.eat(T![:]);

    CssComponentValueList::default().parse_list(p);

    parse_declaration_important(p).ok();
    Present(m.complete(p, CSS_DECLARATION))
}

pub(crate) fn parse_qualified_rule_block(p: &mut CssParser) {}

pub(crate) fn parse_component_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_simple_block(p) {
        return parse_simple_block(p);
    } else if is_at_func_identifier(p) {
        return parse_function_block(p);
    } else {
        return parse_preserved_token(p);
    }
}

pub(crate) fn parse_simple_block(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_simple_block(p) {
        return Absent;
    }
    if p.at(T!['{']) {
        return parse_simple_curly_brackets_block(p);
    } else if p.at(T!['(']) {
        return parse_simple_parentheses_block(p);
    } else if p.at(T!['[']) {
        return parse_simple_square_brackets_block(p);
    }
    Absent
}

#[derive(Default)]
struct CssSimpleComponentValueList {}
impl ParseNodeList for CssSimpleComponentValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_SIMPLE_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_component_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set![T!['}'], T![')'], T![']']])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ParseRecovery::new(CSS_BOGUS, token_set![T!['}'], T![')'], T![']']]),
            expected_component_value,
        )
    }
}

pub(crate) fn parse_simple_curly_brackets_block(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T!['{']);
    CssSimpleComponentValueList::default().parse_list(p);
    p.expect(T!['}']);
    Present(m.complete(p, CSS_SIMPLE_CURLY_BRACKETS_BLOCK))
}

pub(crate) fn parse_simple_parentheses_block(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T!['(']);
    CssSimpleComponentValueList::default().parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, CSS_SIMPLE_PARENTHESES_BLOCK))
}

pub(crate) fn parse_simple_square_brackets_block(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T!['[']);
    CssSimpleComponentValueList::default().parse_list(p);
    p.expect(T![']']);
    Present(m.complete(p, CSS_SIMPLE_SQUARE_BRACKETS_BLOCK))
}

pub(crate) fn parse_function_block(p: &mut CssParser) -> ParsedSyntax {
    if is_at_func_identifier(p) {
        dbg!("iis");
        let m = p.start();
        let tm = p.start();
        p.eat(FUNCTION_TOKEN);
        tm.complete(p, CSS_FUNCTION_TOKEN);
        CssComponentValueList::default().parse_list(p);
        p.expect(T![')']);
        return Present(m.complete(p, CSS_FUNCTION_BLOCK));
    }
    Absent
}
pub(crate) fn parse_preserved_token(p: &mut CssParser) -> ParsedSyntax {
    dbg!(p.at(DELIM));

    if is_at_string(p) {
        return parse_string(p);
    }
    if is_at_percentage(p) {
        return parse_percentage(p);
    }
    if is_at_dimension(p) {
        return parse_dimension(p);
    }
    if is_at_number(p) {
        return parse_regular_number(p);
    }
    if is_at_hash(p) {
        return parse_hash(p);
    }
    if is_at_at_identifier(p){
        return  parse_at_at_identifier(p);
    }
    dbg!(p.cur_text(), p.cur().is_punct());

    if p.at(DELIM) {
        let m = p.start();
        p.bump(DELIM);
        return Present(m.complete(p, CSS_DELIM));
    }
    if is_at_identifier(p) {
        return parse_regular_identifier(p);
    }
    if p.cur().is_keyword() || p.cur().is_punct() {
        let m = p.start();
        p.bump_any();
        return Present(m.complete(p, CSS_PRESERVED_TOKEN_KEY));
    }

    return Absent;
}

#[inline]
pub(crate) fn is_at_func_identifier(p: &mut CssParser) -> bool {
    p.at(FUNCTION_TOKEN)
}

#[inline]
pub(crate) fn is_at_simple_block(p: &mut CssParser) -> bool {
    matches!(p.cur(), T!['['] | T!['{'] | T!['('])
}

#[inline]
pub(crate) fn is_at_at_identifier(p: &mut CssParser) -> bool {
    p.at(AT_IDENT)
}
pub(crate) fn parse_at_at_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_at_identifier(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(AT_IDENT);
    Present(m.complete(p, CSS_AT_KEYWORD))
}

#[inline]
pub(crate) fn is_at_identifier(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![ident])
}

#[inline]
pub(crate) fn parse_regular_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_identifier(p, CssLexContext::Regular)
}

#[inline]
pub(crate) fn parse_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_IDENTIFIER);

    Present(identifier)
}

#[inline]
fn is_at_declaration_important(p: &mut CssParser) -> bool {
    p.at(T![!]) && p.nth_at(1, T![important])
}

#[inline]
fn parse_declaration_important(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_declaration_important(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![!]);
    p.bump(T![important]);
    Present(m.complete(p, CSS_DECLARATION_IMPORTANT))
}

pub(crate) fn is_at_number(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL)
}
#[inline]
pub(crate) fn parse_regular_number(p: &mut CssParser) -> ParsedSyntax {
    parse_number(p, CssLexContext::Regular)
}
#[inline]
pub(crate) fn parse_number(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !p.at(CSS_NUMBER_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(CSS_NUMBER_LITERAL, context);

    Present(m.complete(p, CSS_NUMBER))
}

pub(crate) fn is_at_string(p: &mut CssParser) -> bool {
    p.at(CSS_STRING_LITERAL)
}
#[inline]
pub(crate) fn parse_string(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(CSS_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump(CSS_STRING_LITERAL);

    Present(m.complete(p, CSS_STRING))
}

pub(crate) fn is_at_hash(p: &mut CssParser) -> bool {
    p.at(HASH_TOKEN)
}
#[inline]
pub(crate) fn parse_hash(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_hash(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(HASH_TOKEN);

    Present(m.complete(p, CSS_HASH))
}

pub(crate) fn is_at_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER) && p.nth_at(1, CSS_IDENTIFIER)
}
#[inline]
pub(crate) fn parse_dimension(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_dimension(p) {
        return Absent;
    }

    let m = p.start();
    parse_regular_number(p);
    parse_regular_identifier(p);

    Present(m.complete(p, CSS_DIMENSION))
}

pub(crate) fn is_at_percentage(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER) && p.nth_at(1, T![%])
}
#[inline]
pub(crate) fn parse_percentage(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_percentage(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_number(p);
    p.eat(T![%]);

    Present(m.complete(p, CSS_PERCENTAGE))
}
