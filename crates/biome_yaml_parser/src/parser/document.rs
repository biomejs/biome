use biome_parser::{
    Parser,
    parse_lists::ParseNodeList,
    parse_recovery::{ParseRecoveryTokenSet, RecoveryResult},
    prelude::ParsedSyntax::{self, *},
    token_set,
};
use biome_rowan::{TextRange, TextSize};
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
    p.clear_tag_handles();
    p.eat(UNICODE_BOM);
    let directives = DirectiveList::default().parse_list(p);
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
pub(crate) struct DirectiveList {
    yaml_directive_range: Option<TextRange>,
}

impl ParseNodeList for DirectiveList {
    type Kind = YamlSyntaxKind;
    type Parser<'source> = YamlParser<'source>;

    const LIST_KIND: Self::Kind = YAML_DIRECTIVE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if p.at(DIRECTIVE_LITERAL) {
            let tag_handle_range = {
                let text = p.cur_text();
                let mut fields = text.split_ascii_whitespace();
                if fields.next() == Some("%TAG") {
                    fields.next().and_then(|handle| {
                        let start = TextSize::try_from(text.find(handle)?).ok()?;
                        let length = TextSize::try_from(handle.len()).ok()?;
                        Some(TextRange::at(p.cur_range().start() + start, length))
                    })
                } else {
                    None
                }
            };
            if let Some(range) = tag_handle_range {
                p.declare_tag_handle(range);
            }

            let (is_yaml, has_extra_arguments) = {
                let text = p.cur_text();
                let mut fields = text.split_ascii_whitespace();
                let is_yaml = fields.next() == Some("%YAML");
                (
                    is_yaml,
                    is_yaml && !text.contains('#') && fields.count() > 1,
                )
            };

            if is_yaml {
                if let Some(first_range) = self.yaml_directive_range {
                    p.error(
                        p.err_builder(
                            "A document can contain only one `%YAML` directive.",
                            p.cur_range(),
                        )
                        .with_detail(first_range, "The first `%YAML` directive is here.")
                        .with_hint("Remove this duplicate directive."),
                    );
                } else {
                    self.yaml_directive_range = Some(p.cur_range());
                }
            }
            if has_extra_arguments {
                p.error(
                    p.err_builder(
                        "The `%YAML` directive accepts only a version number.",
                        p.cur_range(),
                    )
                    .with_hint("Remove everything after the version number."),
                );
            }
        }
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
