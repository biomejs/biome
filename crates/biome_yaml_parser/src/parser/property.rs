use biome_parser::{
    CompletedMarker, Parser,
    parse_lists::ParseNodeList,
    parse_recovery::ParseRecoveryTokenSet,
    prelude::ParsedSyntax::{self, *},
    token_set,
};
use biome_yaml_syntax::YamlSyntaxKind::{self, *};

use super::YamlParser;

#[derive(Default)]
pub(crate) struct PropertyList;

impl ParseNodeList for PropertyList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_PROPERTY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if p.at(ANCHOR_PROPERTY_LITERAL) {
            Present(parse_anchor_property(p))
        } else if p.at(TAG_PROPERTY_LITERAL) {
            Present(parse_tag_property(p))
        } else {
            Absent
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        !is_at_property(p)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(YAML_BOGUS, token_set![]),
            |p, range| p.err_builder("expected property", range),
        )
    }
}

fn parse_anchor_property(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(ANCHOR_PROPERTY_LITERAL);
    m.complete(p, YAML_ANCHOR_PROPERTY)
}

fn parse_tag_property(p: &mut YamlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(TAG_PROPERTY_LITERAL);
    m.complete(p, YAML_TAG_PROPERTY)
}

pub(super) fn is_at_property(p: &YamlParser) -> bool {
    p.at(ANCHOR_PROPERTY_LITERAL) || p.at(TAG_PROPERTY_LITERAL)
}
