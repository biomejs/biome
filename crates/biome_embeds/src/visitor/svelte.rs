use super::{EmbeddedBindingsBuilder, EmbeddedSnippet, generic_parameter_names};
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnySvelteAwaitClauses, AnySvelteBlock, AnySvelteBlockItem,
    AnySvelteDirectiveInitializerClause, AnySvelteTemplateElement, HtmlElement, HtmlRoot,
    HtmlRootItemList, HtmlSyntaxKind, HtmlTextExpression, ScriptType, SvelteBindDirective,
    element_ext::AnyHtmlTagElement,
};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{SemanticModelOptions, identifier_name, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::{JsFileSource, javascript::JsEmbeddingKind};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TextSize, WalkEvent};
use biome_svelte_semantic::{
    SvelteEachKeyStatus, SvelteRootSnippet, SvelteTemplateNames, UnknownContent,
};
use std::collections::{HashMap, HashSet};

pub(super) fn collect_template(
    root: &HtmlRoot,
    snippets: &[EmbeddedSnippet],
    bindings: &mut EmbeddedBindingsBuilder,
) -> SvelteTemplateNames {
    let mut template = SvelteTemplateNames::default();
    let mut guests = HashMap::new();
    let mut readable_scripts = HashSet::new();
    for snippet in snippets {
        let Some(source) = snippet.file_source.to_js_file_source() else {
            continue;
        };
        let unreadable = snippet.parse.has_errors();
        if unreadable {
            template
                .unknown
                .unreadable_snippets
                .insert(snippet.content_range.start());
        }
        if !source.as_embedding_kind().is_svelte() {
            continue;
        }
        if source.is_embedded_source() {
            if !unreadable {
                readable_scripts.insert(snippet.content_range);
            }
        } else if source.is_generics_list() {
            if unreadable {
                template.unknown.generics = true;
                template
                    .unknown
                    .template_reads
                    .push(snippet.content_range.start());
                continue;
            }
            for token in generic_parameter_names(std::slice::from_ref(snippet)) {
                let range = token.text_trimmed_range();
                let name = token.token_text_trimmed();
                bindings.register_type_only_binding(range, name.clone());
                let range = range + snippet.content_range.start();
                template.generics.insert(identifier_name(name), range);
            }
        } else {
            if unreadable {
                template
                    .unknown
                    .template_reads
                    .push(snippet.content_range.start());
            }
            guests.insert(snippet.content_range, snippet);
        }
    }

    let kinds = AnySvelteBlock::KIND_SET
        .union(SvelteBindDirective::KIND_SET)
        .union(AnyHtmlTagElement::KIND_SET)
        .union(HtmlTextExpression::KIND_SET);
    for event in root.syntax().preorder_of_kinds(kinds) {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(expr) = HtmlTextExpression::cast_ref(&node) {
            // Each and await patterns have their own parameter parser, not a JS guest.
            if !node.parent().is_some_and(|p| {
                matches!(
                    p.kind(),
                    HtmlSyntaxKind::SVELTE_EACH_BINDING_CLAUSE
                        | HtmlSyntaxKind::SVELTE_AWAIT_THEN_BLOCK
                        | HtmlSyntaxKind::SVELTE_AWAIT_CATCH_BLOCK
                        | HtmlSyntaxKind::SVELTE_AWAIT_THEN_CLAUSE
                        | HtmlSyntaxKind::SVELTE_AWAIT_CATCH_CLAUSE
                )
            }) && let Ok(token) = expr.html_literal_token()
                && !guests.contains_key(&token.text_range())
            {
                template
                    .unknown
                    .template_reads
                    .push(token.text_range().start());
            }
            continue;
        }
        if let Some(tag) = AnyHtmlTagElement::cast_ref(&node) {
            if tag.is_root_script() {
                collect_script_uncertainty(&tag, &readable_scripts, &mut template.unknown);
            } else if tag.tag_name_kind() == Some(HtmlSyntaxKind::TEMPLATE_KW)
                && tag.requires_verbatim_svelte_content()
            {
                template.unknown.template_reads.push(tag.range().start());
            }
            continue;
        }
        let guest = |expr: HtmlTextExpression| {
            let range = expr.html_literal_token().ok()?.text_range();
            let parse = guests.get(&range)?.parse;
            (!parse.has_errors()).then(|| (parse.tree::<AnyJsRoot>(), range.start()))
        };
        if let Some(directive) = SvelteBindDirective::cast_ref(&node) {
            if let Some((range, write_only)) = bind_target(directive, guest) {
                template.register_bind_target(range, write_only);
            }
            continue;
        }
        let block = AnySvelteBlock::unwrap_cast(node);
        let parent_scope = || block.syntax().parent().map(|p| p.text_range_with_trivia());
        match &block {
            AnySvelteBlock::SvelteBogusBlock(bogus) => {
                template.unknown.template_reads.push(bogus.range().start());
            }
            AnySvelteBlock::SvelteEachBlock(each) => {
                let Ok(opening) = each.opening_block() else {
                    continue;
                };
                let Some(item) = opening.item() else {
                    continue;
                };
                let scope = item
                    .range()
                    .cover(each.children().syntax().text_range_with_trivia());
                match item {
                    AnySvelteBlockItem::SvelteEachBindingClause(item) => {
                        let parsed = item.clause().ok().and_then(guest);
                        let Some((AnyJsRoot::JsSvelteEachBindingRoot(root), offset)) = parsed
                        else {
                            template
                                .each_keys
                                .insert(opening.range(), SvelteEachKeyStatus::Unknown);
                            template.unknown.template_declarations.push(scope);
                            template.unknown.template_reads.push(item.range().start());
                            continue;
                        };
                        let status = if root.key().is_some() {
                            SvelteEachKeyStatus::Keyed
                        } else {
                            SvelteEachKeyStatus::Unkeyed
                        };
                        let start = bindings.js_bindings.len();
                        if bindings.visit_svelte_each_binding(&root).is_none() {
                            template
                                .each_keys
                                .insert(opening.range(), SvelteEachKeyStatus::Unknown);
                            template.unknown.template_declarations.push(scope);
                            template.unknown.template_reads.push(item.range().start());
                            continue;
                        }
                        for (range, _, _) in &mut bindings.js_bindings[start..] {
                            *range += offset;
                        }
                        register(
                            &mut template,
                            bindings,
                            start,
                            TextSize::default(),
                            scope,
                            false,
                        );
                        template.each_keys.insert(opening.range(), status);
                    }
                    AnySvelteBlockItem::SvelteEachKeyedItem(item) => {
                        let start = bindings.js_bindings.len();
                        if let Some(token) = item
                            .index()
                            .and_then(|index| index.value().ok()?.ident_token().ok())
                        {
                            bindings.register_binding(
                                token.text_trimmed_range(),
                                token.token_text_trimmed(),
                            );
                        }
                        register(
                            &mut template,
                            bindings,
                            start,
                            TextSize::default(),
                            scope,
                            false,
                        );
                    }
                }
            }
            AnySvelteBlock::SvelteAwaitBlock(block) => {
                if let Ok(open) = block.opening_block()
                    && let Some(pattern) = open
                        .then_clause()
                        .and_then(|c| c.name())
                        .or_else(|| open.catch_clause().and_then(|c| c.name()))
                {
                    let scope = open.children().syntax().text_range_with_trivia();
                    let start = bindings.js_bindings.len();
                    collect_pattern(pattern, bindings, &mut template, scope);
                    register(
                        &mut template,
                        bindings,
                        start,
                        TextSize::default(),
                        scope,
                        false,
                    );
                }
                for clause in block.clauses() {
                    let (pattern, children) = match clause {
                        AnySvelteAwaitClauses::SvelteAwaitThenBlock(c) => (c.name(), c.children()),
                        AnySvelteAwaitClauses::SvelteAwaitCatchBlock(c) => (c.name(), c.children()),
                        AnySvelteAwaitClauses::SvelteBogusBlock(_) => continue,
                    };
                    if let Some(pattern) = pattern {
                        let scope = children.syntax().text_range_with_trivia();
                        let start = bindings.js_bindings.len();
                        collect_pattern(pattern, bindings, &mut template, scope);
                        register(
                            &mut template,
                            bindings,
                            start,
                            TextSize::default(),
                            scope,
                            false,
                        );
                    }
                }
            }
            AnySvelteBlock::SvelteSnippetBlock(snippet) => {
                let Ok(open) = snippet.opening_block() else {
                    continue;
                };
                let Some((AnyJsRoot::JsSvelteSnippetRoot(signature), offset)) =
                    open.expression().ok().and_then(guest)
                else {
                    if let Some(scope) = parent_scope() {
                        template.unknown.root_snippets |=
                            snippet.parent::<HtmlRootItemList>().is_some();
                        template.unknown.template_declarations.push(scope);
                    }
                    continue;
                };
                if let Some(name) = signature
                    .name()
                    .ok()
                    .and_then(|name| name.as_js_identifier_binding()?.name_token().ok())
                    && let Some(scope) = parent_scope()
                {
                    let range = name.text_trimmed_range();
                    let token = name.token_text_trimmed();
                    bindings.register_binding(range, token.clone());
                    let name = identifier_name(token);
                    let declaration = range + offset;
                    template
                        .bindings
                        .insert(name.clone(), declaration, scope, false, false);
                    if snippet
                        .syntax()
                        .parent()
                        .is_some_and(|p| HtmlRootItemList::can_cast(p.kind()))
                    {
                        template.root_snippets.push(SvelteRootSnippet {
                            range: snippet.range(),
                            name,
                            declaration,
                        });
                    }
                }
                let scope = open.range();
                if let Some(types) = signature.type_parameters() {
                    for token in types
                        .items()
                        .iter()
                        .flatten()
                        .filter_map(|p| p.name().ok()?.ident_token().ok())
                    {
                        let range = token.text_trimmed_range() + offset;
                        template.bindings.insert(
                            identifier_name(token.token_text_trimmed()),
                            range,
                            scope,
                            true,
                            false,
                        );
                    }
                }
                if let Some(params) = signature
                    .parameters()
                    .ok()
                    .and_then(|p| p.as_js_parameters().cloned())
                {
                    let start = bindings.js_bindings.len();
                    bindings.visit_parameter_list_bindings(&params.items());
                    register(&mut template, bindings, start, offset, scope, false);
                }
            }
            AnySvelteBlock::SvelteConstBlock(_) | AnySvelteBlock::SvelteDeclarationBlock(_) => {
                let expr = match &block {
                    AnySvelteBlock::SvelteConstBlock(b) => b.expression(),
                    AnySvelteBlock::SvelteDeclarationBlock(b) => b.declaration(),
                    _ => unreachable!(),
                };
                let Some(scope) = parent_scope() else {
                    continue;
                };
                let Some((root, offset)) = expr.ok().and_then(guest) else {
                    template.unknown.template_declarations.push(scope);
                    continue;
                };
                let constant = match &root {
                    AnyJsRoot::JsSvelteDeclarationRoot(root) => {
                        root.declaration().is_ok_and(|declaration| {
                            declaration
                                .as_js_variable_declaration()
                                .is_some_and(|declaration| declaration.is_const())
                        })
                    }
                    AnyJsRoot::JsSvelteLegacyConstRoot(_) => true,
                    _ => continue,
                };
                let start = bindings.js_bindings.len();
                match root {
                    AnyJsRoot::JsSvelteDeclarationRoot(root) => {
                        bindings.visit_svelte_declaration(&root)
                    }
                    AnyJsRoot::JsSvelteLegacyConstRoot(root) => {
                        bindings.visit_svelte_legacy_const_declaration(&root)
                    }
                    _ => unreachable!(),
                };
                register(&mut template, bindings, start, offset, scope, constant);
            }
            _ => {}
        }
    }
    template
}

fn collect_script_uncertainty(
    tag: &AnyHtmlTagElement,
    readable_scripts: &HashSet<TextRange>,
    unknown: &mut UnknownContent,
) {
    let [external, module, context, ty, generics] =
        tag.find_multiple_attributes_by_name(&["src", "module", "context", "type", "generics"]);
    if ty
        .as_ref()
        .and_then(|a| a.as_static_value())
        .is_some_and(|ty| ScriptType::from_type_value(ty.text()).is_json())
    {
        return;
    }
    unknown.generics |= generics
        .and_then(|a| a.as_html_attribute().cloned())
        .is_some_and(|a| a.has_opaque_svelte_generics());
    let unreadable_body = tag.parent::<HtmlElement>().is_some_and(|element| {
        let content = element.children().syntax().text_range_with_trivia();
        !content.is_empty() && !readable_scripts.contains(&content)
    });
    if external.is_some() || unreadable_body {
        if module.is_some()
            || context
                .as_ref()
                .and_then(|a| a.as_static_value())
                .is_some_and(|v| v.text().eq_ignore_ascii_case("module"))
        {
            unknown.module_script = true;
        } else {
            unknown.instance_script = true;
        }
    }
}

fn bind_target(
    directive: SvelteBindDirective,
    guest: impl FnOnce(HtmlTextExpression) -> Option<(AnyJsRoot, TextSize)>,
) -> Option<(TextRange, bool)> {
    let value = directive.value().ok()?;
    let property = value
        .property()
        .ok()?
        .as_svelte_name()?
        .ident_token()
        .ok()?;
    let write_only = property.text_trimmed() == "this";
    let Some(initializer) = value.initializer() else {
        return Some((property.text_trimmed_range(), write_only));
    };
    let AnySvelteDirectiveInitializerClause::HtmlAttributeInitializerClause(initializer) =
        initializer
    else {
        return None;
    };
    let single = match initializer.value().ok()? {
        AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(single) => single,
        AnyHtmlAttributeInitializer::SvelteTemplateAttributeValue(value) => {
            let mut elements = value.elements().into_iter();
            let AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(single) =
                elements.next()?
            else {
                return None;
            };
            if elements.next().is_some() {
                return None;
            }
            single
        }
        _ => return None,
    };
    let (root, offset) = guest(single.expression().ok()?)?;
    let expr = root
        .as_js_expression_template_root()?
        .expression()?
        .omit_type_wrappers()?;
    let name = expr.as_js_identifier_expression()?.name().ok()?;
    Some((name.range() + offset, write_only))
}

fn register(
    template: &mut SvelteTemplateNames,
    bindings: &EmbeddedBindingsBuilder,
    start: usize,
    offset: TextSize,
    scope: TextRange,
    constant: bool,
) {
    for (range, name, _) in &bindings.js_bindings[start..] {
        let range = *range + offset;
        template
            .bindings
            .insert(identifier_name(name.clone()), range, scope, false, constant);
    }
}

fn collect_pattern(
    pattern: HtmlTextExpression,
    bindings: &mut EmbeddedBindingsBuilder,
    template: &mut SvelteTemplateNames,
    scope: TextRange,
) {
    let Ok(token) = pattern.html_literal_token() else {
        return;
    };
    // Await bindings remain host-owned and use the existing parameter grammar.
    let source = JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Vue {
        setup: false,
        is_source: false,
        event_handler: false,
        allow_statements: false,
        slot_props: true,
        generics_list: false,
        is_class_attribute: false,
    });
    let parsed = parse(token.text(), source, JsParserOptions::default());
    let Ok(root) = parsed.ok() else {
        template.unknown.template_declarations.push(scope);
        template
            .unknown
            .template_reads
            .push(pattern.range().start());
        return;
    };
    let Some(params) = root.as_js_vue_slot_props_root() else {
        return;
    };
    if params.parameters().len() != 1 {
        template.unknown.template_declarations.push(scope);
        template
            .unknown
            .template_reads
            .push(pattern.range().start());
        return;
    }
    let offset = token.text_range().start();
    let start = bindings.js_bindings.len();
    bindings.visit_parameter_list_bindings(&params.parameters());
    for (range, _, _) in &mut bindings.js_bindings[start..] {
        *range += offset;
    }
    if root
        .syntax()
        .preorder_of_kinds(biome_js_syntax::AnyJsIdentifierReference::KIND_SET)
        .next()
        .is_some()
    {
        let js = semantic_model(&root, SemanticModelOptions::from(&source));
        template.collect_pattern_reads(&js, offset);
    }
}
