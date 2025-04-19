use biome_parser::{
    CompletedMarker, Parser,
    parse_lists::ParseNodeList,
    prelude::ParsedSyntax::{self, *},
};
use biome_yaml_syntax::YamlSyntaxKind::{self, *};

use super::{YamlParser, block::parse_any_block};

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
    ) -> biome_parser::parse_recovery::RecoveryResult {
        Ok(parsed_element.expect("A document can not be absent"))
    }
}

fn parse_document(p: &mut YamlParser) -> CompletedMarker {
    p.eat(UNICODE_BOM);
    parse_any_node(p)
}

fn parse_any_node(p: &mut YamlParser) -> CompletedMarker {
    parse_any_block(p)
}
