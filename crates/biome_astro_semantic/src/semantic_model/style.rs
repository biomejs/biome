use super::builder::{ModelBuilder, to_host};
use super::define_vars::analyze_define_vars;
use super::model::*;
use super::snippets::AstroSnippet;
use biome_html_syntax::{AstroDefineDirective, HtmlElement, HtmlRoot, HtmlSelfClosingElement};
use biome_rowan::{AstNode, WalkEvent};

pub(crate) fn collect_style_semantics(
    builder: &mut ModelBuilder,
    host: &HtmlRoot,
    snippets: &[AstroSnippet],
    ids: &[SnippetId],
) {
    mark_unreadable_define_vars(builder, host, ids);
    let mut define_vars: Vec<_> = ids
        .iter()
        .enumerate()
        .filter_map(|(index, id)| match builder.data.snippets[*id].host {
            SnippetHost::StyleDefineVars(block) => Some((index, *id, block)),
            _ => None,
        })
        .collect();
    define_vars.sort_by_key(|(_, id, _)| builder.data.snippets[*id].range.start());
    for (index, _, block) in define_vars {
        let snippet = &snippets[index];
        let result = analyze_define_vars(snippet);
        let data = &mut builder.data.blocks[block];
        data.define_vars_open |= result.open;
        for variable in result.variables {
            let variable = DefineVarData {
                name: variable.name,
                semantic_name: variable.semantic_name,
                range: to_host(variable.range, snippet.content_offset),
                declaration: to_host(variable.declaration, snippet.content_offset),
            };
            if let Some(existing) = data
                .define_vars
                .iter_mut()
                .find(|existing| existing.semantic_name == variable.semantic_name)
            {
                *existing = variable;
            } else {
                data.define_vars.push(variable);
            }
        }
        data.define_vars
            .sort_by_key(|variable| variable.range.start());
    }
}

fn mark_unreadable_define_vars(builder: &mut ModelBuilder, host: &HtmlRoot, ids: &[SnippetId]) {
    for event in host
        .syntax()
        .preorder_of_kinds(AstroDefineDirective::KIND_SET)
    {
        let (value, block) = if let WalkEvent::Enter(node) = event
            && let Some(directive) = AstroDefineDirective::cast_ref(&node)
            && let Ok(value) = directive.value()
            && let Some(name) = value.name().ok().and_then(|name| name.value_token().ok())
            && name.text_trimmed() == "vars"
            && let Some(element_range) = directive.syntax().ancestors().find_map(|node| {
                if let Some(element) = HtmlElement::cast_ref(&node) {
                    Some(element.range())
                } else {
                    HtmlSelfClosingElement::cast_ref(&node).map(|element| element.range())
                }
            })
            && let Some(block) = builder
                .data
                .blocks
                .iter()
                .position(|block| {
                    block.kind == BlockKind::Style
                        && block.supported
                        && !block.external
                        && block.range == element_range
                })
                .map(BlockId::new)
        {
            (value, block)
        } else {
            continue;
        };
        let initializer = value
            .initializer()
            .and_then(|initializer| initializer.value().ok());
        let expression_range = initializer
            .as_ref()
            .and_then(|value| value.as_html_attribute_single_text_expression())
            .and_then(|expression| expression.expression().ok())
            .and_then(|expression| expression.html_literal_token().ok())
            .map(|token| token.text_range());
        if expression_range.is_some_and(|range| {
            ids.iter().any(|id| {
                builder.data.snippets[*id].range == range
                    && matches!(
                        builder.data.snippets[*id].host,
                        SnippetHost::StyleDefineVars(candidate) if candidate == block
                    )
            })
        }) {
            continue;
        }
        let reason = match initializer {
            None => OpenReason::ParseError,
            Some(value) if value.as_html_attribute_single_text_expression().is_some() => {
                OpenReason::ParseError
            }
            Some(_) => OpenReason::NonLiteral,
        };
        builder.data.blocks[block].define_vars_open.insert(reason);
    }
}
