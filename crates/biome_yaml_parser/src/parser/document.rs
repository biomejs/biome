use biome_parser::{
    Parser,
    parse_lists::ParseNodeList,
    parse_recovery::{ParseRecoveryTokenSet, RecoveryResult},
    prelude::ParsedSyntax::{self, *},
    token_set,
};
use biome_yaml_syntax::{
    T,
    YamlSyntaxKind::{self, *},
};

use super::{
    YamlParser,
    block::{is_at_any_block_node, parse_any_block_node},
    parse_error::{expected_directive, malformed_document},
};

#[derive(Default)]
pub(crate) struct DocumentList;

impl ParseNodeList for DocumentList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_DOCUMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_document(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(YamlSyntaxKind::YAML_BOGUS, token_set![EOF]),
            malformed_document,
        )
    }
}

fn parse_document(p: &mut YamlParser) -> ParsedSyntax {
    if !is_at_document(p) {
        return Absent;
    }
    let m = p.start();
    p.eat(UNICODE_BOM);
    DirectiveList.parse_list(p);
    p.eat(T![---]);
    parse_any_block_node(p).ok();
    p.eat(T![...]);
    Present(m.complete(p, YAML_DOCUMENT))
}

fn is_at_document(p: &YamlParser) -> bool {
    p.at(UNICODE_BOM)
        || p.at(T![---])
        || p.at(T![...])
        || p.at(DIRECTIVE_LITERAL)
        || is_at_any_block_node(p)
}

#[derive(Default)]
pub(crate) struct DirectiveList;

impl ParseNodeList for DirectiveList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_DIRECTIVE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_directive(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![...])
            // Consecutive directives are rare, so it's better to just fail fast here
            || !p.at(DIRECTIVE_LITERAL)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(YAML_BOGUS, token_set![DIRECTIVE_LITERAL]),
            expected_directive,
        )
    }
}

fn parse_directive(p: &mut YamlParser) -> ParsedSyntax {
    if !p.at(DIRECTIVE_LITERAL) {
        return Absent;
    }
    let m = p.start();
    p.bump(DIRECTIVE_LITERAL);
    Present(m.complete(p, YAML_DIRECTIVE))
}
