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
    parse_error::{expected_directive, unexpected_token},
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
            unexpected_token,
        )
    }
}

fn parse_document(p: &mut YamlParser) -> ParsedSyntax {
    if !is_at_document(p) {
        return Absent;
    }
    let m = p.start();
    p.eat(UNICODE_BOM);
    let directives = DirectiveList.parse_list(p);
    let document_start_range = p.cur_range();
    let has_document_start = p.eat(T![---]);
    if !directives.range(p).is_empty() && !has_document_start {
        p.error(p.err_builder("Expected `---` after YAML directives.", directives.range(p)));
    }
    if has_document_start && p.at(MAPPING_START) && !p.has_preceding_line_break() {
        p.error(
            p.err_builder(
                "A mapping cannot start on the same line as `---`.",
                document_start_range,
            )
            .with_hint("Move the mapping to the next line after `---`."),
        );
    }
    parse_any_block_node(p).ok();
    if is_at_any_block_node(p) && p.has_preceding_line_break() {
        p.error(
            p.err_builder(
                "Expected `---` or `...` before another YAML document.",
                p.cur_range(),
            )
            .with_hint(
                "Add `---` before the next document, or end the current document with `...`.",
            ),
        );
    }
    let has_document_end = p.eat(T![...]);
    if !has_document_end && p.at(DIRECTIVE_LITERAL) {
        p.error(p.err_builder("Expected `...` before YAML directives.", p.cur_range()));
    }
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
