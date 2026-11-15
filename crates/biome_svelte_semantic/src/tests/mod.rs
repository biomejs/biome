mod component;
mod db;
mod module;

use crate::{SvelteSemanticModel, SvelteSnippet, component_model, module_model};
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::{
    AnyHtmlAttribute, AnySvelteBlock, AnySvelteBlockItem, AnySvelteDirective,
    AnySvelteDirectiveInitializerClause, HtmlAttributeSingleTextExpression, HtmlElement,
    HtmlEmbeddedContent, HtmlRoot, HtmlSingleTextExpression, HtmlSpreadAttribute,
    HtmlTextExpression, SvelteAttachAttribute,
};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::javascript::{JsEmbeddingKind, SvelteEmbeddingKind, SvelteFileKind};
use biome_languages::{HtmlFileSource, JsFileSource};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TextSize};
use camino::Utf8Path;

fn svelte_kind(embedding_kind: SvelteEmbeddingKind, is_module_script: bool) -> JsEmbeddingKind {
    JsEmbeddingKind::Svelte {
        file_kind: SvelteFileKind::Component,
        embedding_kind,
        is_class_attribute: false,
        is_module_script,
    }
}

/// A snippet found in a test document: its text, content range, content
/// offset and embedding kind.
type Found = (String, TextRange, TextSize, JsEmbeddingKind);

fn expression(expression: &HtmlTextExpression, kind: SvelteEmbeddingKind) -> Option<Found> {
    let token = expression.html_literal_token().ok()?;
    Some((
        token.text().to_string(),
        token.text_range(),
        token.text_range().start(),
        svelte_kind(kind, false),
    ))
}

/// Builds the model of a component at `path`.
///
/// The workspace finds and parses the embedded snippets of a `.svelte` file
/// in production. This helper does the same for the constructs the tests use.
pub(crate) fn component_at(code: &str, path: &str) -> SvelteSemanticModel {
    let options = HtmlParserOptions::from(&HtmlFileSource::svelte());
    let host: HtmlRoot = parse_html(code, options).tree();
    let mut found: Vec<Found> = Vec::new();
    let mut is_ts = false;
    use SvelteEmbeddingKind::*;

    for node in host.syntax().descendants() {
        if let Some(element) = HtmlElement::cast_ref(&node) {
            if !element.is_script_tag() {
                continue;
            }
            is_ts |= element.is_typescript_lang();
            let is_module = element.find_attribute_by_name("module").is_some()
                || element.has_attribute_with_value("context", "module");
            if let Some(content) = element
                .children()
                .syntax()
                .descendants()
                .find_map(HtmlEmbeddedContent::cast)
                .and_then(|content| content.value_token().ok())
            {
                found.push((
                    content.text().to_string(),
                    content.text_trimmed_range(),
                    content.text_range().start(),
                    svelte_kind(Source, is_module),
                ));
            }
        } else if let Some(text) = HtmlSingleTextExpression::cast_ref(&node) {
            let kind = if text
                .expression()
                .is_some_and(|expression| expression.is_svelte_text_interpolation())
            {
                TextInterpolation
            } else {
                Expression
            };
            found.extend(text.expression().and_then(|e| expression(&e, kind)));
        } else if let Some(attribute) = HtmlAttributeSingleTextExpression::cast_ref(&node) {
            found.extend(
                attribute
                    .expression()
                    .ok()
                    .and_then(|e| expression(&e, Expression)),
            );
        } else if let Some(spread) = HtmlSpreadAttribute::cast_ref(&node) {
            found.extend(
                spread
                    .argument()
                    .ok()
                    .and_then(|e| expression(&e, Expression)),
            );
        } else if let Some(attach) = SvelteAttachAttribute::cast_ref(&node) {
            found.extend(
                attach
                    .expression()
                    .ok()
                    .and_then(|e| expression(&e, Expression)),
            );
        } else if let Some(directive) = AnySvelteDirective::cast_ref(&node) {
            let value = AnyHtmlAttribute::AnySvelteDirective(directive.clone());
            let initializer = match &value {
                AnyHtmlAttribute::AnySvelteDirective(directive) => match directive {
                    AnySvelteDirective::SvelteBindDirective(d) => d.value().ok(),
                    AnySvelteDirective::SvelteClassDirective(d) => d.value().ok(),
                    AnySvelteDirective::SvelteStyleDirective(d) => d.value().ok(),
                    AnySvelteDirective::SvelteUseDirective(d) => d.value().ok(),
                    AnySvelteDirective::SvelteTransitionDirective(d) => d.value().ok(),
                    AnySvelteDirective::SvelteInDirective(d) => d.value().ok(),
                    AnySvelteDirective::SvelteOutDirective(d) => d.value().ok(),
                    AnySvelteDirective::SvelteAnimateDirective(d) => d.value().ok(),
                },
                _ => None,
            }
            .and_then(|value| value.initializer());
            if let Some(
                AnySvelteDirectiveInitializerClause::SvelteBindFunctionBindingInitializerClause(
                    initializer,
                ),
            ) = initializer
                && let Ok(binding) = initializer.value()
            {
                found.extend(binding.get().ok().and_then(|e| expression(&e, Expression)));
                found.extend(binding.set().ok().and_then(|e| expression(&e, Expression)));
            }
        } else if let Some(block) = AnySvelteBlock::cast_ref(&node) {
            match block {
                AnySvelteBlock::SvelteAwaitBlock(block) => found.extend(
                    block
                        .opening_block()
                        .ok()
                        .and_then(|opening| opening.expression().ok())
                        .and_then(|e| expression(&e, Expression)),
                ),
                AnySvelteBlock::SvelteConstBlock(block) => found.extend(
                    block
                        .expression()
                        .ok()
                        .and_then(|e| expression(&e, LegacyConst)),
                ),
                AnySvelteBlock::SvelteDeclarationBlock(block) => found.extend(
                    block
                        .declaration()
                        .ok()
                        .and_then(|e| expression(&e, Declaration)),
                ),
                AnySvelteBlock::SvelteDebugBlock(block) => {
                    for name in block.bindings().iter().flatten() {
                        if let Ok(token) = name.ident_token() {
                            found.push((
                                token.text().to_string(),
                                token.text_range(),
                                token.text_range().start(),
                                svelte_kind(Expression, false),
                            ));
                        }
                    }
                }
                AnySvelteBlock::SvelteEachBlock(block) => {
                    if let Ok(opening) = block.opening_block() {
                        found.extend(opening.list().ok().and_then(|e| expression(&e, Expression)));
                        if let Some(AnySvelteBlockItem::SvelteEachAsKeyedItem(item)) =
                            opening.item()
                            && let Some(key) = item.key()
                        {
                            found.extend(
                                key.expression()
                                    .ok()
                                    .and_then(|e| expression(&e, Expression)),
                            );
                        }
                    }
                }
                AnySvelteBlock::SvelteHtmlBlock(block) => found.extend(
                    block
                        .expression()
                        .ok()
                        .and_then(|e| expression(&e, Expression)),
                ),
                AnySvelteBlock::SvelteIfBlock(block) => {
                    found.extend(
                        block
                            .opening_block()
                            .ok()
                            .and_then(|opening| opening.expression().ok())
                            .and_then(|e| expression(&e, Expression)),
                    );
                    for clause in block.else_if_clauses() {
                        found.extend(
                            clause
                                .expression()
                                .ok()
                                .and_then(|e| expression(&e, Expression)),
                        );
                    }
                }
                AnySvelteBlock::SvelteKeyBlock(block) => found.extend(
                    block
                        .opening_block()
                        .ok()
                        .and_then(|opening| opening.expression().ok())
                        .and_then(|e| expression(&e, Expression)),
                ),
                AnySvelteBlock::SvelteRenderBlock(block) => found.extend(
                    block
                        .expression()
                        .ok()
                        .and_then(|e| expression(&e, Expression)),
                ),
                AnySvelteBlock::SvelteSnippetBlock(block) => found.extend(
                    block
                        .opening_block()
                        .ok()
                        .and_then(|opening| opening.expression().ok())
                        .and_then(|e| expression(&e, SnippetSignature)),
                ),
                AnySvelteBlock::SvelteBogusBlock(_) => {}
            }
        }
    }

    let base = if is_ts {
        JsFileSource::ts()
    } else {
        JsFileSource::js_module()
    };
    let parsed: Vec<_> = found
        .iter()
        .map(|(text, _, _, kind)| {
            let source = base.with_embedding_kind(*kind);
            let root: AnyJsRoot =
                biome_js_parser::parse(text, source, JsParserOptions::default()).tree();
            let js = semantic_model(&root, SemanticModelOptions::from(&source));
            (root, js, source)
        })
        .collect();
    let snippets: Vec<_> = parsed
        .iter()
        .zip(&found)
        .map(
            |((root, js, source), (_, range, offset, _))| SvelteSnippet {
                root,
                js,
                source: *source,
                content_range: *range,
                content_offset: *offset,
            },
        )
        .collect();
    component_model(&host, &snippets, Utf8Path::new(path))
}

/// Builds the model of a component.
pub(crate) fn component(code: &str) -> SvelteSemanticModel {
    component_at(code, "src/lib/Component.svelte")
}

/// Builds the model of a `.svelte.ts` module.
pub(crate) fn module(code: &str) -> SvelteSemanticModel {
    let path = Utf8Path::new("src/lib/state.svelte.ts");
    let source = JsFileSource::try_from(path).expect("a file source");
    let root = biome_js_parser::parse(code, source, JsParserOptions::default()).tree();
    let js = semantic_model(&root, SemanticModelOptions::from(&source));
    module_model(&root, &js, path)
}
