use crate::parser::HtmlParser;
use crate::syntax::parse_error::expected_svelte_closing_block;
use crate::token_source::HtmlLexContext;
use biome_html_syntax::HtmlSyntaxKind::{
    EOF, SVELTE_BINDING_LIST, SVELTE_BOGUS_BLOCK, SVELTE_DEBUG_BLOCK, SVELTE_IDENT, SVELTE_NAME,
};
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Marker, Parser, TokenSet, token_set};

pub(crate) fn parse_svelte_at_block(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["{@"]) {
        return Absent;
    };
    let m = p.start();
    p.bump_with_context(T!["{@"], HtmlLexContext::Svelte);

    match p.cur() {
        T![debug] => parse_debug_block(p, m),
        _ => Absent,
    }
}
pub(crate) fn parse_debug_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![debug]) {
        return Absent;
    }
    p.bump_with_context(T![debug], HtmlLexContext::Svelte);

    BindingList.parse_list(p);

    p.expect_with_context(T!['}'], HtmlLexContext::InsideTag);

    Present(marker.complete(p, SVELTE_DEBUG_BLOCK))
}

const BLOCK_RECOVER: TokenSet<HtmlSyntaxKind> =
    token_set!(T!['{'], T![<], T!["{@"], T!["{/"], T!["{:"], T!["{#"]);

#[derive(Debug)]
struct BindingList;

impl ParseSeparatedList for BindingList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = SVELTE_BINDING_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T!['}']) || p.at(T!['{'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SVELTE_BOGUS_BLOCK, BLOCK_RECOVER),
            expected_svelte_closing_block,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect_with_context(self.separating_element_kind(), HtmlLexContext::Svelte)
    }
}

fn parse_name(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(SVELTE_IDENT) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(SVELTE_IDENT, HtmlLexContext::Svelte);

    Present(m.complete(p, SVELTE_NAME))
}
