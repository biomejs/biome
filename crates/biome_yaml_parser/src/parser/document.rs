use biome_parser::{
    CompletedMarker, Parser,
    parse_lists::ParseNodeList,
    parse_recovery::{ParseRecoveryTokenSet, RecoveryResult},
    prelude::ParsedSyntax::{self, *},
    token_set,
};
use biome_yaml_syntax::{
    T,
    YamlSyntaxKind::{self, *},
};

use crate::lexer::YamlLexContext;

use super::{YamlParser, flow::parse_any_flow_node, parse_error::expected_directive};

#[derive(Default)]
pub(crate) struct DocumentList;

impl ParseNodeList for DocumentList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_DOCUMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        Present(parse_document(p))
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
    }

    fn recover(
        &mut self,
        _p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        Ok(parsed_element.expect("A document can not be absent"))
    }
}

fn parse_document(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.eat(UNICODE_BOM);
    DirectiveList.parse_list(p);
    parse_any_flow_node(p, YamlLexContext::FlowOut);
    m.complete(p, YAML_DOCUMENT)
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
