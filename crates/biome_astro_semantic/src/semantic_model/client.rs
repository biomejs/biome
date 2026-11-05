use super::builder::{ModelBuilder, NewSymbol, to_host};
use super::define_vars::analyze_define_vars;
use super::javascript::{collect_free_references, collect_module_bindings};
use super::model::*;
use super::snippets::AstroSnippet;
use biome_html_syntax::{AstroDefineDirective, HtmlElement, HtmlRoot, HtmlSelfClosingElement};
use biome_rowan::{AstNode, WalkEvent};

pub(crate) fn collect_client_semantics(
    builder: &mut ModelBuilder,
    host: &HtmlRoot,
    snippets: &[AstroSnippet],
    ids: &[SnippetId],
    client_globals: &[Box<str>],
) {
    mark_unreadable_define_vars(builder, host, ids);

    let scripts: Vec<_> = ids
        .iter()
        .enumerate()
        .filter_map(|(index, id)| match builder.data.snippets[*id].host {
            SnippetHost::Script(block) if !builder.data.blocks[block].external => {
                Some((index, *id, block))
            }
            _ => None,
        })
        .collect();
    let define_vars: Vec<_> = ids
        .iter()
        .enumerate()
        .filter_map(|(index, id)| match builder.data.snippets[*id].host {
            SnippetHost::ScriptDefineVars(block) if !builder.data.blocks[block].external => {
                Some((index, *id, block))
            }
            _ => None,
        })
        .collect();
    let mut declarations: Vec<_> = scripts
        .iter()
        .map(|(index, id, block)| (*index, *id, *block, true))
        .chain(
            define_vars
                .iter()
                .map(|(index, id, block)| (*index, *id, *block, false)),
        )
        .collect();
    declarations.sort_by_key(|(_, id, _, _)| builder.data.snippets[*id].range.start());
    for (index, id, block, is_script) in declarations {
        let Some(scope) = builder.data.blocks[block].scope else {
            continue;
        };
        if is_script {
            collect_module_bindings(builder, &snippets[index], id, scope);
        } else {
            collect_define_vars(builder, &snippets[index], id, scope);
        }
    }

    let external_scopes: Vec<_> = builder
        .data
        .blocks
        .iter()
        .filter(|block| block.kind == BlockKind::Script && block.external)
        .filter_map(|block| block.scope)
        .collect();
    for scope in external_scopes {
        for namespace in [Namespace::Value, Namespace::Type, Namespace::Namespace] {
            builder.open(scope, namespace, OpenReason::ExternalSource);
        }
    }

    for (index, id, block) in scripts {
        let Some(scope) = builder.data.blocks[block].scope else {
            continue;
        };
        collect_free_references(builder, &snippets[index], id, scope, client_globals);
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
                    block.kind == BlockKind::Script
                        && block.scope.is_some()
                        && block.range == element_range
                })
                .map(BlockId::new)
            && !builder.data.blocks[block].external
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
                        SnippetHost::ScriptDefineVars(candidate) if candidate == block
                    )
            })
        }) {
            continue;
        }
        let Some(scope) = builder.data.blocks[block].scope else {
            continue;
        };
        let reason = match initializer {
            None => OpenReason::ParseError,
            Some(value) if value.as_html_attribute_single_text_expression().is_some() => {
                OpenReason::ParseError
            }
            Some(_) => OpenReason::NonLiteral,
        };
        builder.open(scope, Namespace::Value, reason);
    }
}

fn collect_define_vars(
    builder: &mut ModelBuilder,
    snippet: &AstroSnippet,
    snippet_id: SnippetId,
    scope: ScopeId,
) {
    let result = analyze_define_vars(snippet);
    for reason in result.open.iter() {
        builder.open(scope, Namespace::Value, reason);
    }
    for variable in result.variables {
        let range = to_host(variable.range, snippet.range_offset());
        let declaration = to_host(variable.declaration, snippet.range_offset());
        let existing = builder.data.scopes[scope]
            .symbols
            .iter()
            .copied()
            .find(|id| {
                let symbol = &builder.data.symbols[*id];
                symbol.origin == Origin::DefineVars
                    && symbol.semantic_name == variable.semantic_name
            });
        if let Some(existing) = existing {
            let symbol = &mut builder.data.symbols[existing];
            symbol.name = variable.name;
            symbol.semantic_name = variable.semantic_name;
            symbol.range = range;
            symbol.declaration = declaration;
            symbol.snippet = Some(snippet_id);
        } else {
            builder.add_symbol(NewSymbol {
                name: variable.name,
                semantic_name: variable.semantic_name,
                range,
                declaration,
                snippet: Some(snippet_id),
                scope,
                namespace: Namespace::Value,
                kind: SymbolKind::Synthetic,
                origin: Origin::DefineVars,
                declares_value: true,
                declares_type: false,
                declares_namespace: false,
            });
        }
    }
}
