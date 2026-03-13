use biome_parser::{
    Parser,
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

    fn parse_element(&mut self, _p: &mut Self::Parser<'_>) -> ParsedSyntax {
        Absent
    }

    fn is_at_list_end(&self, _p: &mut Self::Parser<'_>) -> bool {
        true
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
