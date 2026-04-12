use crate::embed::registry::{EmbedDetectorsRegistry, EmbedMatch};
use crate::embed::types::{
    EmbedBlockKind, EmbedCandidate, EmbedContent, GuestLanguage, HostLanguage,
};
use crate::file_handlers::html::{EmbedParseContext, ParsedEmbed};
use crate::file_handlers::{DocumentFileSource, ParseEmbedResult};
use crate::settings::SettingsWithEditor;
use crate::workspace::document::services::embedded_bindings::EmbeddedBuilder;
use crate::workspace::{
    AnyEmbeddedSnippet, CssDocumentServices, DocumentServices, EmbeddedSnippet, JsDocumentServices,
};
use biome_css_parser::{CssModulesKind, parse_css_with_offset_and_cache};
use biome_css_syntax::{
    CssFileSource, CssLanguage, EmbeddingHtmlKind, EmbeddingKind as CssEmbeddingKind,
    EmbeddingStyleApplicability, TextSize,
};
use biome_fs::BiomePath;
use biome_html_syntax::{
    AnyAstroDirective, AnySvelteBindingAssignmentBinding, AnySvelteBlock, AnySvelteBlockItem,
    AnySvelteDestructuredName, AnySvelteDirective, AnySvelteEachName, AstroEmbeddedContent,
    HtmlAttribute, HtmlAttributeInitializerClause, HtmlAttributeSingleTextExpression,
    HtmlDoubleTextExpression, HtmlElement, HtmlFileSource, HtmlRoot, HtmlSingleTextExpression,
    HtmlTextExpression, HtmlTextExpressions, HtmlVariant, SvelteName, VueDirective,
    VueVBindShorthandDirective, VueVOnShorthandDirective, VueVSlotShorthandDirective,
};
use biome_js_parser::parse_js_with_offset_and_cache;
use biome_js_syntax::{EmbeddingKind, JsFileSource, JsLanguage};
use biome_json_parser::parse_json_with_offset_and_cache;
use biome_json_syntax::{JsonFileSource, JsonLanguage};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, NodeCache};
use std::collections::VecDeque;

pub(crate) fn parse_embedded_nodes(
    root: &AnyParse,
    biome_path: &BiomePath,
    file_source: &DocumentFileSource,
    settings: &SettingsWithEditor,
    cache: &mut NodeCache,
    builder: &mut EmbeddedBuilder,
) -> ParseEmbedResult {
    let mut nodes = Vec::new();
    let html_root: HtmlRoot = root.tree();
    let Some(file_source) = file_source.to_html_file_source() else {
        return ParseEmbedResult::default();
    };

    let doc_file_source = DocumentFileSource::Html(file_source);

    let mut ctx = EmbedParseContext {
        cache,
        biome_path,
        host_file_source: &file_source,
        settings,
        builder,
    };

    match file_source.variant() {
        HtmlVariant::Standard(text_expression) => {
            for element in html_root.syntax().descendants() {
                // Element-level embeds via registry
                if let Some(html_element) = HtmlElement::cast_ref(&element)
                    && let Some(candidate) = build_html_candidate(&html_element)
                {
                    ctx.parse_and_push(&candidate, &doc_file_source, None, &mut nodes);
                }

                // Text expressions via registry
                match text_expression {
                    HtmlTextExpressions::Single => {
                        if let Some(text_expression) = HtmlSingleTextExpression::cast_ref(&element)
                            && let Ok(expression) = text_expression.expression()
                            && let Some(candidate) = build_text_expression_candidate(&expression)
                        {
                            ctx.parse_and_push(&candidate, &doc_file_source, None, &mut nodes);
                        }
                    }

                    HtmlTextExpressions::Double => {
                        if let Some(text_expression) = HtmlDoubleTextExpression::cast_ref(&element)
                            && let Ok(expression) = text_expression.expression()
                            && let Some(candidate) = build_text_expression_candidate(&expression)
                        {
                            ctx.parse_and_push(&candidate, &doc_file_source, None, &mut nodes);
                        }
                    }
                    HtmlTextExpressions::None => {}
                }
            }
        }

        HtmlVariant::Astro => {
            for element in html_root.syntax().descendants() {
                // Astro frontmatter → registry
                if let Some(astro_content) = AstroEmbeddedContent::cast_ref(&element)
                    && let Some(candidate) = build_astro_frontmatter_candidate(&astro_content)
                {
                    ctx.parse_and_push(&candidate, &doc_file_source, None, &mut nodes);
                }

                // Text expressions via registry
                if let Some(text_expression) = HtmlSingleTextExpression::cast_ref(&element)
                    && let Ok(expression) = text_expression.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                {
                    ctx.parse_and_push(&candidate, &doc_file_source, None, &mut nodes);
                }

                // HTML elements (script/style) → registry
                if let Some(html_element) = HtmlElement::cast_ref(&element)
                    && let Some(candidate) = build_html_candidate(&html_element)
                {
                    ctx.parse_and_push(&candidate, &doc_file_source, None, &mut nodes);
                }

                // Astro directives: class:list={...}, define:vars={...}, etc.
                if let Some(directive) = AnyAstroDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) =
                        build_attribute_expression_candidate(&initializer, true)
                {
                    ctx.parse_and_push(&candidate, &doc_file_source, None, &mut nodes);
                }

                // Plain HTML attributes with expression values: class={expr}, id={expr}, etc.
                if let Some(attr) = HtmlAttribute::cast_ref(&element)
                    && let Some(initializer) = attr.initializer()
                    && let Some(candidate) = build_attribute_expression_candidate(
                        &initializer,
                        attr.name()
                            .ok()
                            .and_then(|name| name.value_token().ok())
                            .is_some_and(|token| token.text_trimmed() == "class"),
                    )
                {
                    ctx.parse_and_push(&candidate, &doc_file_source, None, &mut nodes);
                }
            }
        }
        HtmlVariant::Vue => {
            // Two-pass: collect elements + expressions, then process
            let mut elements = vec![];
            let mut snippet_expressions = vec![];
            for element in html_root.syntax().descendants() {
                if let Some(text_expression) = HtmlDoubleTextExpression::cast_ref(&element) {
                    snippet_expressions.push(text_expression);
                }

                if let Some(element) = HtmlElement::cast_ref(&element) {
                    elements.push(element);
                }
            }

            // Pass 1: elements via registry, collecting JS file sources
            let mut embedded_file_source = JsFileSource::js_module();
            for element in elements {
                if let Some(candidate) = build_html_candidate(&element)
                    && let Some(parsed) = ctx.detect_and_parse(&candidate, &doc_file_source, None)
                {
                    if let Some(js_fs) = parsed.js_file_source {
                        embedded_file_source = merge_js_file_source(embedded_file_source, js_fs);
                    }
                    nodes.push(parsed.node);
                }
            }

            // Pass 2: text expressions via registry using merged embedded_file_source
            for snippet in snippet_expressions {
                if let Ok(expression) = snippet.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        &mut nodes,
                    );
                }
            }

            // Pass 3: directive attributes via registry using merged embedded_file_source
            for element in html_root.syntax().descendants() {
                // Handle @click shorthand (VueVOnShorthandDirective)
                if let Some(directive) = VueVOnShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_vue_directive_candidate(&initializer, true)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        &mut nodes,
                    );
                }

                // Handle :prop shorthand (VueVBindShorthandDirective)
                if let Some(directive) = VueVBindShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_vue_directive_candidate(&initializer, false)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        &mut nodes,
                    );
                }

                // Handle #slot shorthand (VueVSlotShorthandDirective)
                if let Some(directive) = VueVSlotShorthandDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_vue_directive_candidate(&initializer, false)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        &mut nodes,
                    );
                }

                // Handle full directives (v-on:, v-bind:, v-if, v-show, etc.)
                if let Some(directive) = VueDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                {
                    let is_v_on = directive
                        .name_token()
                        .map(|t| t.text_trimmed() == "v-on")
                        .unwrap_or(false);
                    if let Some(candidate) = build_vue_directive_candidate(&initializer, is_v_on) {
                        ctx.parse_and_push(
                            &candidate,
                            &doc_file_source,
                            Some(embedded_file_source),
                            &mut nodes,
                        );
                    }
                }
            }
        }
        HtmlVariant::Svelte => {
            // Two-pass: collect elements + expressions, then process
            let mut elements = vec![];
            let mut snippet_expressions = vec![];
            for element in html_root.syntax().descendants() {
                if let Some(text_expression) = HtmlSingleTextExpression::cast_ref(&element) {
                    snippet_expressions.push(text_expression);
                }

                if let Some(element) = HtmlElement::cast_ref(&element) {
                    elements.push(element);
                }
            }

            // Pass 1: elements via registry, collecting JS file sources
            let mut embedded_file_source = JsFileSource::js_module();
            for element in elements {
                if let Some(candidate) = build_html_candidate(&element)
                    && let Some(parsed) = ctx.detect_and_parse(&candidate, &doc_file_source, None)
                {
                    if let Some(js_fs) = parsed.js_file_source {
                        embedded_file_source = merge_js_file_source(embedded_file_source, js_fs);
                    }
                    nodes.push(parsed.node);
                }
            }

            // Pass 2: text expressions via registry using merged embedded_file_source
            for snippet in snippet_expressions {
                if let Ok(expression) = snippet.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        &mut nodes,
                    );
                }
            }

            // Pass 3: control flow blocks via registry
            parse_svelte_blocks(
                &mut nodes,
                &html_root,
                doc_file_source,
                &mut ctx,
                embedded_file_source,
            );

            // Pass 4: directive attributes and attributes which initializer is a text expression
            for element in html_root.syntax().descendants() {
                // Handle special Svelte directives (bind:, class:, etc.)
                if let Some(directive) = AnySvelteDirective::cast_ref(&element)
                    && let Some(initializer) = directive.initializer()
                    && let Some(candidate) = build_svelte_directive_candidate(&initializer)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        &mut nodes,
                    );
                }

                if let Some(attr) = HtmlAttribute::cast_ref(&element)
                    && let Some(initializer) = attr.initializer()
                    && let Some(candidate) = build_attribute_expression_candidate(
                        &initializer,
                        attr.name()
                            .ok()
                            .and_then(|name| name.value_token().ok())
                            .is_some_and(|token| token.text_trimmed() == "class"),
                    )
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        &mut nodes,
                    );
                }

                if let Some(attr) = HtmlAttributeSingleTextExpression::cast_ref(&element)
                    && !attr.syntax().parent().is_some_and(|parent| {
                        HtmlAttributeInitializerClause::can_cast(parent.kind())
                    })
                    && let Ok(expression) = attr.expression()
                    && let Some(candidate) = build_text_expression_candidate(&expression)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        &mut nodes,
                    );
                }
            }
        }
    }

    ParseEmbedResult { nodes }
}

// Pass 3: control flow blocks via registry
fn parse_svelte_blocks(
    nodes: &mut Vec<(AnyEmbeddedSnippet, DocumentFileSource)>,
    html_root: &HtmlRoot,
    doc_file_source: DocumentFileSource,
    ctx: &mut EmbedParseContext,
    embedded_file_source: JsFileSource,
) {
    for element in html_root.syntax().descendants() {
        let Some(svelte_block) = AnySvelteBlock::cast_ref(&element) else {
            continue;
        };

        match &svelte_block {
            AnySvelteBlock::SvelteAwaitBlock(await_block) => {
                if let Ok(opening_block) = await_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some(candidate) =
                        build_svelte_text_expression_candidate(&expression, &svelte_block)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        nodes,
                    );
                }
            }
            AnySvelteBlock::SvelteBogusBlock(_) => {}
            AnySvelteBlock::SvelteConstBlock(const_block) => {
                if let Ok(expression) = const_block.expression()
                    && let Some(candidate) =
                        build_svelte_text_expression_candidate(&expression, &svelte_block)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        nodes,
                    );
                }
            }

            AnySvelteBlock::SvelteDebugBlock(debug_block) => {
                for name in debug_block.bindings().iter().flatten() {
                    if let Some(candidate) = build_svelte_name_candidate(&name) {
                        ctx.parse_and_push(
                            &candidate,
                            &doc_file_source,
                            Some(embedded_file_source),
                            nodes,
                        );
                    }
                }
            }
            AnySvelteBlock::SvelteEachBlock(each_block) => {
                if let Ok(opening_block) = each_block.opening_block() {
                    if let Ok(expression) = opening_block.list()
                        && let Some(candidate) =
                            build_svelte_text_expression_candidate(&expression, &svelte_block)
                    {
                        ctx.parse_and_push(
                            &candidate,
                            &doc_file_source,
                            Some(embedded_file_source),
                            nodes,
                        );
                    }

                    if let Some(item) = opening_block.item() {
                        match item {
                            AnySvelteBlockItem::SvelteEachAsKeyedItem(as_keyed) => {
                                if let Ok(name) = as_keyed.name() {
                                    register_svelte_each_name_bindings(ctx.builder, name);
                                }
                                if let Some(index) = as_keyed.index()
                                    && let Ok(value) = index.value()
                                    && let Ok(token) = value.ident_token()
                                {
                                    ctx.builder.register_binding(
                                        token.text_trimmed_range(),
                                        token.token_text_trimmed(),
                                    );
                                }
                                if let Some(key) = as_keyed.key()
                                    && let Ok(key_expression) = key.expression()
                                    && let Some(candidate) = build_svelte_text_expression_candidate(
                                        &key_expression,
                                        &svelte_block,
                                    )
                                {
                                    ctx.parse_and_push(
                                        &candidate,
                                        &doc_file_source,
                                        Some(embedded_file_source),
                                        nodes,
                                    );
                                }
                            }
                            AnySvelteBlockItem::SvelteEachKeyedItem(keyed) => {
                                if let Some(index) = keyed.index()
                                    && let Ok(value) = index.value()
                                    && let Ok(token) = value.ident_token()
                                {
                                    ctx.builder.register_binding(
                                        token.text_trimmed_range(),
                                        token.token_text_trimmed(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
            AnySvelteBlock::SvelteHtmlBlock(_) => {}
            AnySvelteBlock::SvelteIfBlock(if_block) => {
                if let Ok(opening_block) = if_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some(candidate) =
                        build_svelte_text_expression_candidate(&expression, &svelte_block)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        nodes,
                    );
                }

                for else_if_clause in if_block.else_if_clauses() {
                    if let Ok(expression) = else_if_clause.expression()
                        && let Some(candidate) =
                            build_svelte_text_expression_candidate(&expression, &svelte_block)
                    {
                        ctx.parse_and_push(
                            &candidate,
                            &doc_file_source,
                            Some(embedded_file_source),
                            nodes,
                        );
                    }
                }
            }
            AnySvelteBlock::SvelteKeyBlock(key_block) => {
                if let Ok(opening_block) = key_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some(candidate) =
                        build_svelte_text_expression_candidate(&expression, &svelte_block)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        nodes,
                    );
                }
            }
            AnySvelteBlock::SvelteRenderBlock(render_block) => {
                if let Ok(expression) = render_block.expression()
                    && let Some(candidate) =
                        build_svelte_text_expression_candidate(&expression, &svelte_block)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        nodes,
                    );
                }
            }
            AnySvelteBlock::SvelteSnippetBlock(snippet_block) => {
                if let Ok(opening_block) = snippet_block.opening_block()
                    && let Ok(expression) = opening_block.expression()
                    && let Some(candidate) =
                        build_svelte_text_expression_candidate(&expression, &svelte_block)
                {
                    ctx.parse_and_push(
                        &candidate,
                        &doc_file_source,
                        Some(embedded_file_source),
                        nodes,
                    );
                }
            }
        }
    }
}

/// Build an `EmbedCandidate::Directive` from a Svelte directive initializer clause.
///
/// Svelte directives use curly brace text expressions (`on:click={handler}`).
/// The JS content is the literal token inside the expression node.
fn build_svelte_directive_candidate(
    initializer: &HtmlAttributeInitializerClause,
) -> Option<EmbedCandidate> {
    build_attribute_expression_candidate(initializer, false)
}

/// Build an `EmbedCandidate::Directive` from an initializer clause containing
/// a curly brace text expression (`attr={expr}`).
///
/// Used by both Astro and Svelte attribute expression extraction.
/// Returns `None` if the initializer does not contain a text expression.
fn build_attribute_expression_candidate(
    initializer: &HtmlAttributeInitializerClause,
    is_class_attribute: bool,
) -> Option<EmbedCandidate> {
    let value_node = initializer.value().ok()?;
    let text_expression = value_node.as_html_attribute_single_text_expression()?;
    let expression = text_expression.expression().ok()?;
    let content_token = expression.html_literal_token().ok()?;

    Some(EmbedCandidate::Directive {
        content: EmbedContent {
            element_range: expression.range(),
            content_range: content_token.text_range(),
            content_offset: content_token.text_range().start(),
            text: content_token.token_text(),
        },
        is_event_handler: false,
        is_class_attribute,
    })
}

/// Build an `EmbedCandidate::Frontmatter` from Astro's `---` block.
fn build_astro_frontmatter_candidate(element: &AstroEmbeddedContent) -> Option<EmbedCandidate> {
    let content_token = element.content_token()?;

    Some(EmbedCandidate::Frontmatter {
        content: EmbedContent {
            element_range: element.range(),
            content_range: content_token.text_trimmed_range(),
            content_offset: content_token.text_range().start(),
            // Use full token text (including trivia) to match the untrimmed content_offset.
            // The parser needs text and offset to be consistent.
            text: content_token.token_text(),
        },
    })
}

/// Builds a text-expression candidate whose content is the single identifier
/// wrapped by a `SvelteName`. Used for constructs like `{@debug a, b, c}`
/// where each listed name is a reference to an existing binding.
fn build_svelte_name_candidate(svelte_name: &SvelteName) -> Option<EmbedCandidate> {
    let token = svelte_name.ident_token().ok()?;
    Some(EmbedCandidate::TextExpression {
        content: EmbedContent {
            element_range: token.text_trimmed_range(),
            content_range: token.text_range(),
            content_offset: token.text_range().start(),
            text: token.token_text(),
        },
        block_kind: EmbedBlockKind::Neutral,
    })
}

fn build_svelte_text_expression_candidate(
    expression: &HtmlTextExpression,
    svelte_block: &AnySvelteBlock,
) -> Option<EmbedCandidate> {
    let content_token = expression.html_literal_token().ok()?;
    Some(EmbedCandidate::TextExpression {
        content: EmbedContent {
            element_range: expression.range(),
            content_range: content_token.text_range(),
            content_offset: content_token.text_range().start(),
            text: content_token.token_text(),
        },
        block_kind: EmbedBlockKind::from(svelte_block),
    })
}

/// Build an `EmbedCandidate::Directive` from a Vue directive initializer clause.
///
/// Vue directives use quoted string values (`@click="handler()"`).
/// The JS content is the inner text without quotes, offset by +1 for the opening quote.
fn build_vue_directive_candidate(
    initializer: &HtmlAttributeInitializerClause,
    is_event_handler: bool,
) -> Option<EmbedCandidate> {
    let value_node = initializer.value().ok()?;
    let html_string = value_node.as_html_string()?;
    let content_token = html_string.value_token().ok()?;
    let inner_text = html_string.inner_string_text().ok()?;
    let token_range = content_token.text_trimmed_range();
    let inner_offset = token_range.start() + TextSize::from(1);

    Some(EmbedCandidate::Directive {
        content: EmbedContent {
            element_range: initializer.range(),
            content_range: token_range,
            content_offset: inner_offset,
            text: inner_text,
        },
        is_event_handler,
        is_class_attribute: false,
    })
}

/// Build an `EmbedCandidate::TextExpression` from an `HtmlTextExpression`.
///
/// This is the inner expression node (the JS code inside `{ }` or `{{ }}`).
/// The caller extracts it from the outer wrapper (`HtmlSingleTextExpression`,
/// `HtmlDoubleTextExpression`, or control flow block).
fn build_text_expression_candidate(expression: &HtmlTextExpression) -> Option<EmbedCandidate> {
    let content_token = expression.html_literal_token().ok()?;
    Some(EmbedCandidate::TextExpression {
        content: EmbedContent {
            element_range: expression.range(),
            content_range: content_token.text_range(),
            content_offset: content_token.text_range().start(),
            text: content_token.token_text(),
        },
        block_kind: EmbedBlockKind::Neutral,
    })
}

/// Build an `EmbedCandidate::Element` from an `HtmlElement`.
/// Returns `None` if the element has no embedded content or has multiple children (error).
fn build_html_candidate(element: &HtmlElement) -> Option<EmbedCandidate> {
    // Multiple children is likely an error — skip
    if element.children().len() > 1 {
        return None;
    }

    let tag_name = element.tag_name()?;

    let attributes: Vec<_> = element
        .opening_element()
        .ok()
        .into_iter()
        .flat_map(|opening| opening.attributes())
        .filter_map(|attr| {
            let html_attr = attr.as_html_attribute()?;
            let name = html_attr
                .name()
                .ok()?
                .value_token()
                .ok()?
                .token_text_trimmed();
            let value = html_attr
                .initializer()
                .and_then(|init| init.value().ok())
                .and_then(|v| v.as_html_string().cloned())
                .and_then(|s| s.inner_string_text().ok());
            Some((name, value))
        })
        .collect();

    // Extract content from HtmlEmbeddedContent child
    let content_child = element.children().iter().next().and_then(|child| {
        let child = child.as_any_html_content()?;
        child.as_html_embedded_content().cloned()
    })?;
    let value_token = content_child.value_token().ok()?;

    Some(EmbedCandidate::Element {
        tag_name,
        attributes,
        content: EmbedContent {
            element_range: content_child.range(),
            content_range: value_token.text_range(),
            content_offset: value_token.text_range().start(),
            // Use full token text (including trivia) to match the untrimmed content_offset.
            // The parser needs text and offset to be consistent.
            text: value_token.token_text(),
        },
    })
}

/// Merge two `JsFileSource` values by picking the most permissive one.
///
/// Vue and Svelte files can have multiple `<script>` tags with different
/// `lang` attributes. The merged result is used as the base file source
/// for text expressions and directives, so it must be able to parse any
/// syntax that might appear in the template.
///
/// Hierarchy: Tsx > Ts > Jsx > JsModule > JsScript.
fn merge_js_file_source(a: JsFileSource, b: JsFileSource) -> JsFileSource {
    let ts = a.is_typescript() || b.is_typescript();
    let jsx = a.is_jsx() || b.is_jsx();
    match (ts, jsx) {
        (true, true) => JsFileSource::tsx(),
        (true, false) => JsFileSource::ts(),
        (false, true) => JsFileSource::jsx(),
        (false, false) => JsFileSource::js_module(),
    }
}

fn embedded_css_file_source(
    host_file_source: &HtmlFileSource,
    candidate: &EmbedCandidate,
) -> CssFileSource {
    let base = if host_file_source.is_html() {
        CssFileSource::css()
    } else {
        CssFileSource::new_css_modules()
    };

    let embedding_kind = match host_file_source.variant() {
        HtmlVariant::Standard(_) => CssEmbeddingKind::Html(EmbeddingHtmlKind::Html),
        HtmlVariant::Vue => {
            let applicability = if candidate.has_attribute("scoped") {
                EmbeddingStyleApplicability::Local
            } else {
                EmbeddingStyleApplicability::Global
            };
            CssEmbeddingKind::Html(EmbeddingHtmlKind::Vue { applicability })
        }
        HtmlVariant::Astro => {
            let applicability = if candidate.has_attribute("is:global") {
                EmbeddingStyleApplicability::Global
            } else {
                EmbeddingStyleApplicability::Local
            };
            CssEmbeddingKind::Html(EmbeddingHtmlKind::Astro { applicability })
        }
        HtmlVariant::Svelte => CssEmbeddingKind::Html(EmbeddingHtmlKind::Svelte {
            applicability: EmbeddingStyleApplicability::Local,
        }),
    };

    base.with_embedding_kind(embedding_kind)
}

impl EmbedParseContext<'_, '_> {
    /// Runs the detector on a candidate and, if matched, parses the embed.
    /// Returns the raw `ParsedEmbed` for callers that need to inspect the
    /// resolved JS file source before deciding what to do with the node
    /// (the Vue/Svelte element pass that seeds `embedded_file_source`).
    fn detect_and_parse(
        &mut self,
        candidate: &EmbedCandidate,
        doc_file_source: &DocumentFileSource,
        embedded_file_source: Option<JsFileSource>,
    ) -> Option<ParsedEmbed> {
        let embed_match =
            EmbedDetectorsRegistry::detect_match(HostLanguage::Html, candidate, doc_file_source)?;
        parse_matched_embed(candidate, &embed_match, self, embedded_file_source)
    }

    /// Detects, parses, and pushes the resulting snippet into `nodes`. This is
    /// the common path used by every embed site that does not need to inspect
    /// the parsed result.
    fn parse_and_push(
        &mut self,
        candidate: &EmbedCandidate,
        doc_file_source: &DocumentFileSource,
        embedded_file_source: Option<JsFileSource>,
        nodes: &mut Vec<(AnyEmbeddedSnippet, DocumentFileSource)>,
    ) -> Option<()> {
        let parsed = self.detect_and_parse(candidate, doc_file_source, embedded_file_source)?;
        nodes.push(parsed.node);
        Some(())
    }
}

/// Parse an embedded code fragment using the parser for the matched guest language.
fn parse_matched_embed(
    candidate: &EmbedCandidate,
    embed_match: &EmbedMatch,
    ctx: &mut EmbedParseContext,
    embedded_file_source: Option<JsFileSource>,
) -> Option<ParsedEmbed> {
    let host_file_source = ctx.host_file_source;
    let content = candidate.content();

    match embed_match.guest {
        GuestLanguage::JsModule
        | GuestLanguage::JsScript
        | GuestLanguage::Jsx
        | GuestLanguage::Ts
        | GuestLanguage::Tsx => {
            // Determine base JsFileSource from guest language
            let mut js_source = match embed_match.guest {
                GuestLanguage::JsModule => JsFileSource::js_module(),
                GuestLanguage::JsScript => JsFileSource::js_script(),
                GuestLanguage::Jsx => JsFileSource::jsx(),
                GuestLanguage::Ts => JsFileSource::ts(),
                GuestLanguage::Tsx => JsFileSource::tsx(),
                _ => unreachable!(),
            };

            // Configure EmbeddingKind based on framework + candidate type
            let is_source_level = match candidate {
                EmbedCandidate::Frontmatter { .. } => {
                    js_source = js_source.with_embedding_kind(EmbeddingKind::Astro {
                        frontmatter: true,
                        is_class_attribute: false,
                    });
                    true
                }
                EmbedCandidate::Element { .. } => {
                    if ctx.host_file_source.is_svelte() {
                        js_source = js_source
                            .with_embedding_kind(EmbeddingKind::Svelte { is_source: true });
                    } else if ctx.host_file_source.is_vue() {
                        js_source = js_source.with_embedding_kind(EmbeddingKind::Vue {
                            setup: candidate.has_attribute("setup"),
                            is_source: true,
                            event_handler: false,
                            allow_statements: true,
                        });
                    }
                    // Astro <script> tags and plain HTML: no EmbeddingKind
                    true
                }
                EmbedCandidate::TextExpression { .. } => {
                    // Use embedded_file_source as base if available (Vue/Svelte pass 2+)
                    if let Some(efs) = embedded_file_source {
                        js_source = efs;
                    }
                    if ctx.host_file_source.is_astro() {
                        js_source = js_source.with_embedding_kind(EmbeddingKind::Astro {
                            frontmatter: false,
                            is_class_attribute: false,
                        });
                    } else if ctx.host_file_source.is_svelte() {
                        js_source = js_source
                            .with_embedding_kind(EmbeddingKind::Svelte { is_source: false });
                    } else if ctx.host_file_source.is_vue() {
                        js_source = js_source.with_embedding_kind(EmbeddingKind::Vue {
                            setup: false,
                            is_source: false,
                            event_handler: false,
                            allow_statements: false,
                        });
                    }
                    false
                }
                EmbedCandidate::Directive {
                    is_event_handler,
                    is_class_attribute,
                    ..
                } => {
                    // Use embedded_file_source as base if available (Vue/Svelte pass 2+)
                    if let Some(efs) = embedded_file_source {
                        js_source = efs;
                    }
                    match ctx.host_file_source.variant() {
                        HtmlVariant::Standard(_) => {}
                        HtmlVariant::Astro => {
                            js_source = js_source.with_embedding_kind(EmbeddingKind::Astro {
                                frontmatter: false,
                                is_class_attribute: *is_class_attribute,
                            });
                        }
                        HtmlVariant::Vue => {
                            js_source = js_source.with_embedding_kind(EmbeddingKind::Vue {
                                setup: false,
                                is_source: false,
                                event_handler: *is_event_handler,
                                allow_statements: false,
                            });
                        }
                        HtmlVariant::Svelte => {
                            js_source = js_source
                                .with_embedding_kind(EmbeddingKind::Svelte { is_source: false });
                        }
                    }

                    false
                }
                _ => false,
            };

            let doc_source = DocumentFileSource::Js(js_source);
            let options = ctx
                .settings
                .parse_options::<JsLanguage>(ctx.biome_path, &doc_source);
            let parse = parse_js_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                js_source,
                options,
                ctx.cache,
            );

            // We track bindings in the following cases:
            // - Source snippets
            // - Snippets declared inside svelte files. Blocks such as #snippet and #render can define functions and bindings.
            if is_source_level || host_file_source.is_svelte() {
                ctx.builder.visit_js_source_snippet(
                    &parse.tree(),
                    host_file_source,
                    candidate.as_block_kind(),
                );
            }

            let snippet: EmbeddedSnippet<JsLanguage> = EmbeddedSnippet::new(
                parse.into(),
                content.element_range,
                content.content_range,
                content.content_offset,
            );

            // Source-level embeds get full services; expression-level don't
            let js_services = if is_source_level
                && (ctx.settings.as_ref().is_linter_enabled()
                    || ctx.settings.as_ref().is_assist_enabled())
            {
                JsDocumentServices::default()
                    .with_js_semantic_model(&snippet.tree())
                    .into()
            } else {
                DocumentServices::none()
            };

            Some(ParsedEmbed {
                node: ((snippet, js_services).into(), doc_source),
                // Only source-level embeds contribute to embedded_file_source capture
                js_file_source: if is_source_level {
                    Some(js_source)
                } else {
                    None
                },
            })
        }

        GuestLanguage::Css => {
            let css_source = embedded_css_file_source(ctx.host_file_source, candidate);
            let doc_source = DocumentFileSource::Css(css_source);
            let mut options = ctx
                .settings
                .parse_options::<CssLanguage>(ctx.biome_path, &doc_source);
            if ctx.host_file_source.is_vue() {
                options.css_modules = CssModulesKind::Vue;
            } else if !ctx.host_file_source.is_html() {
                options.css_modules = CssModulesKind::Classic;
            }
            let parse = parse_css_with_offset_and_cache(
                content.text.text(),
                css_source,
                content.content_offset,
                ctx.cache,
                options,
            );

            let mut services = CssDocumentServices::default();
            if ctx.settings.as_ref().is_linter_enabled()
                || ctx.settings.as_ref().is_assist_enabled()
            {
                services = services.with_css_semantic_model(&parse.tree());
            }

            let snippet: EmbeddedSnippet<CssLanguage> = EmbeddedSnippet::new(
                parse.into(),
                content.element_range,
                content.content_range,
                content.content_offset,
            );

            Some(ParsedEmbed {
                node: ((snippet, services.into()).into(), doc_source),
                js_file_source: None,
            })
        }

        GuestLanguage::Json => {
            let doc_source = DocumentFileSource::Json(JsonFileSource::json());
            let options = ctx
                .settings
                .parse_options::<JsonLanguage>(ctx.biome_path, &doc_source);
            let parse = parse_json_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                ctx.cache,
                options,
            );

            let snippet: EmbeddedSnippet<JsonLanguage> = EmbeddedSnippet::new(
                parse.into(),
                content.element_range,
                content.content_range,
                content.content_offset,
            );

            Some(ParsedEmbed {
                node: (snippet.into(), doc_source),
                js_file_source: None,
            })
        }

        GuestLanguage::GraphQL => {
            // GraphQL embeds are only used by the JS handler, not HTML
            None
        }
    }
}

/// Registers bindings declared by the `as` clause of a Svelte `{#each}` block.
///
/// Handles the three name shapes the grammar allows: a plain identifier,
/// an object or array destructure, and a text expression. Only the first two
/// introduce new bindings; the text expression form is left alone.
fn register_svelte_each_name_bindings(builder: &mut EmbeddedBuilder, name: AnySvelteEachName) {
    match name {
        AnySvelteEachName::SvelteName(ident) => {
            if let Ok(token) = ident.ident_token() {
                builder.register_binding(token.text_trimmed_range(), token.token_text_trimmed());
            }
        }
        AnySvelteEachName::AnySvelteDestructuredName(destructured) => {
            register_svelte_destructured_bindings(builder, destructured);
        }
        AnySvelteEachName::HtmlTextExpression(_) => {}
    }
}

/// Walks a Svelte curly or square destructure pattern iteratively and
/// registers every identifier introduced by it, including nested patterns
/// and rest bindings (`{ ...rest }`).
fn register_svelte_destructured_bindings(
    builder: &mut EmbeddedBuilder,
    destructured: AnySvelteDestructuredName,
) -> Option<()> {
    let mut queue: VecDeque<AnySvelteDestructuredName> = VecDeque::new();
    queue.push_back(destructured);

    while let Some(current) = queue.pop_front() {
        let list = match current {
            AnySvelteDestructuredName::SvelteCurlyDestructuredName(n) => n.names(),
            AnySvelteDestructuredName::SvelteSquareDestructuredName(n) => n.names(),
        };
        for binding in list.iter().flatten() {
            match binding {
                AnySvelteBindingAssignmentBinding::SvelteName(ident) => {
                    let token = ident.ident_token().ok()?;
                    builder
                        .register_binding(token.text_trimmed_range(), token.token_text_trimmed());
                }
                AnySvelteBindingAssignmentBinding::AnySvelteDestructuredName(nested) => {
                    queue.push_back(nested);
                }
                AnySvelteBindingAssignmentBinding::SvelteRestBinding(rest) => {
                    let name = rest.name().ok()?;
                    let token = name.ident_token().ok()?;
                    builder
                        .register_binding(token.text_trimmed_range(), token.token_text_trimmed());
                }
            }
        }
    }

    Some(())
}
