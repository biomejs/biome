use crate::visitor::EmbeddedSnippet;
use biome_db::ParsedSource;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnyHtmlTagName, AnySvelteBindingAssignmentBinding,
    AnySvelteBindingProperty, AnySvelteBlockItem, AnySvelteDestructuredName,
    AnySvelteDirectiveInitializerClause, AnySvelteEachName, HtmlRoot, SvelteBindDirective,
    SvelteEachBlock, SvelteName,
};
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstSeparatedList, TokenText};

#[salsa::tracked(returns(ref))]
pub(crate) fn svelte_element_references_from_source(
    db: &dyn LanguageDb,
    file: ParsedSource,
) -> Vec<TokenText> {
    let Some(host_source) = db.source_from_index(file.document_source_index(db)) else {
        return Vec::new();
    };

    let snippets = file
        .snippets(db)
        .iter()
        .filter_map(|snippet| {
            let file_source = db.source_from_index(snippet.document_source_index(db))?;
            Some(EmbeddedSnippet::new(
                snippet.parsed(db),
                snippet.content_range(db),
                file_source,
            ))
        })
        .collect::<Vec<_>>();

    collect_svelte_element_references(host_source, file.parsed(db), &snippets)
}

/// Collects the names of the variables that Svelte `bind:this={name}`
/// directives bind to DOM elements.
///
/// Only HTML elements, custom elements, and `<svelte:element>` are considered. Binding
/// `this` on a component gives the component instance instead of a DOM node,
/// and the other `svelte:` elements don't support `bind:this`.
///
/// A name that is declared by an enclosing `{#each}` block refers to the block
/// variable rather than to a top-level variable, so it is skipped.
pub(crate) fn collect_svelte_element_references(
    host_source: DocumentFileSource,
    host_parse: &AnyParse,
    snippets: &[EmbeddedSnippet],
) -> Vec<TokenText> {
    if !host_source
        .to_html_file_source()
        .is_some_and(|source| source.is_svelte())
    {
        return Vec::new();
    }

    let root: HtmlRoot = host_parse.tree();
    root.syntax()
        .descendants()
        .filter_map(SvelteBindDirective::cast)
        .filter_map(|directive| svelte_element_reference(&directive, snippets))
        .collect()
}

fn svelte_element_reference(
    directive: &SvelteBindDirective,
    snippets: &[EmbeddedSnippet],
) -> Option<TokenText> {
    let value = directive.value().ok()?;
    let AnySvelteBindingProperty::SvelteName(property) = value.property().ok()? else {
        return None;
    };
    if property.ident_token().ok()?.text_trimmed() != "this" {
        return None;
    }

    let AnySvelteDirectiveInitializerClause::HtmlAttributeInitializerClause(initializer) =
        value.initializer()?
    else {
        return None;
    };
    let AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(expression) =
        initializer.value().ok()?
    else {
        return None;
    };
    let content_range = expression
        .expression()
        .ok()?
        .html_literal_token()
        .ok()?
        .text_range();
    let snippet = snippets
        .iter()
        .find(|snippet| snippet.content_range == content_range)?;
    let name = snippet_identifier(&snippet.parse.tree())?;

    let element = directive
        .syntax()
        .ancestors()
        .find_map(AnyHtmlTagElement::cast)?;
    if !is_dom_element(&element) || is_each_block_name(directive, name.text()) {
        return None;
    }

    Some(name)
}

/// Returns the name of the variable when the snippet is a lone identifier.
fn snippet_identifier(root: &AnyJsRoot) -> Option<TokenText> {
    let expression = match root {
        AnyJsRoot::JsExpressionSnippet(root) => root.expression().ok()?,
        AnyJsRoot::JsExpressionTemplateRoot(root) => root.expression()?,
        _ => return None,
    };
    let token = expression
        .as_js_identifier_expression()?
        .name()
        .ok()?
        .value_token()
        .ok()?;
    Some(token.token_text_trimmed())
}

/// Returns whether Svelte renders `element` as a DOM element.
///
/// Svelte treats a capitalized name or a dotted name as a component. Custom
/// elements such as `<my-element>` are DOM elements, even though their names
/// are lexed as component names.
fn is_dom_element(element: &AnyHtmlTagElement) -> bool {
    match element.name() {
        Ok(AnyHtmlTagName::HtmlTagName(_)) => element.tag_name().is_some_and(|name| {
            let name = name.text();
            !name.starts_with("svelte:") || name == "svelte:element"
        }),
        Ok(AnyHtmlTagName::HtmlComponentName(name)) => name.value_token().is_ok_and(|token| {
            token
                .text_trimmed()
                .starts_with(|c: char| c.is_ascii_lowercase())
        }),
        Ok(AnyHtmlTagName::HtmlMemberName(_)) | Err(_) => false,
    }
}

fn is_each_block_name(directive: &SvelteBindDirective, name: &str) -> bool {
    directive
        .syntax()
        .ancestors()
        .filter_map(SvelteEachBlock::cast)
        .any(|block| {
            let Some(item) = block
                .opening_block()
                .ok()
                .and_then(|opening| opening.item())
            else {
                return false;
            };
            match item {
                AnySvelteBlockItem::SvelteEachAsKeyedItem(item) => {
                    item.name()
                        .is_ok_and(|each_name| each_name_declares(&each_name, name))
                        || item
                            .index()
                            .and_then(|index| index.value().ok())
                            .is_some_and(|index| svelte_name_is(&index, name))
                }
                AnySvelteBlockItem::SvelteEachKeyedItem(item) => item
                    .index()
                    .and_then(|index| index.value().ok())
                    .is_some_and(|index| svelte_name_is(&index, name)),
            }
        })
}

fn each_name_declares(each_name: &AnySvelteEachName, name: &str) -> bool {
    match each_name {
        AnySvelteEachName::SvelteName(each_name) => svelte_name_is(each_name, name),
        AnySvelteEachName::AnySvelteDestructuredName(destructured) => {
            destructured_name_declares(destructured, name)
        }
        AnySvelteEachName::HtmlTextExpression(_) => false,
    }
}

fn destructured_name_declares(destructured: &AnySvelteDestructuredName, name: &str) -> bool {
    let names = match destructured {
        AnySvelteDestructuredName::SvelteCurlyDestructuredName(names) => names.names(),
        AnySvelteDestructuredName::SvelteSquareDestructuredName(names) => names.names(),
    };
    names
        .iter()
        .flatten()
        .any(|binding| binding_declares(&binding, name))
}

fn binding_declares(binding: &AnySvelteBindingAssignmentBinding, name: &str) -> bool {
    match binding {
        AnySvelteBindingAssignmentBinding::SvelteName(binding) => svelte_name_is(binding, name),
        AnySvelteBindingAssignmentBinding::AnySvelteDestructuredName(nested) => {
            destructured_name_declares(nested, name)
        }
        AnySvelteBindingAssignmentBinding::SvelteRestBinding(rest) => rest
            .name()
            .is_ok_and(|rest_name| svelte_name_is(&rest_name, name)),
        AnySvelteBindingAssignmentBinding::SvelteRenameBinding(rename) => rename
            .name()
            .is_ok_and(|renamed| binding_declares(&renamed, name)),
    }
}

fn svelte_name_is(svelte_name: &SvelteName, name: &str) -> bool {
    svelte_name
        .ident_token()
        .is_ok_and(|token| token.text_trimmed() == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_js_parser::JsParserOptions;
    use biome_languages::javascript::{JsEmbeddingKind, SvelteEmbeddingKind, SvelteFileKind};
    use biome_languages::{HtmlFileSource, JsFileSource};
    use biome_rowan::TextRange;

    /// Parses `source` as a Svelte component, together with the expression of
    /// every `bind:` directive, the way the workspace extracts them.
    fn collect(source: &str) -> Vec<String> {
        let host_parse: AnyParse =
            parse_html(source, HtmlParserOptions::from(&HtmlFileSource::svelte())).into();
        let js_source = JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Svelte {
            is_module_script: false,
            file_kind: SvelteFileKind::Component,
            is_class_attribute: false,
            embedding_kind: SvelteEmbeddingKind::Expression,
        });

        let root: HtmlRoot = host_parse.tree();
        let expressions: Vec<(TextRange, AnyParse)> = root
            .syntax()
            .descendants()
            .filter_map(SvelteBindDirective::cast)
            .filter_map(|directive| {
                let AnySvelteDirectiveInitializerClause::HtmlAttributeInitializerClause(
                    initializer,
                ) = directive.value().ok()?.initializer()?
                else {
                    return None;
                };
                let token = initializer
                    .value()
                    .ok()?
                    .as_html_attribute_single_text_expression()?
                    .expression()
                    .ok()?
                    .html_literal_token()
                    .ok()?;
                let parse =
                    biome_js_parser::parse(token.text(), js_source, JsParserOptions::default());
                Some((token.text_range(), parse.into()))
            })
            .collect();
        let snippets: Vec<EmbeddedSnippet> = expressions
            .iter()
            .map(|(range, parse)| {
                EmbeddedSnippet::new(parse, *range, DocumentFileSource::Js(js_source))
            })
            .collect();

        collect_svelte_element_references(
            DocumentFileSource::Html(HtmlFileSource::svelte()),
            &host_parse,
            &snippets,
        )
        .iter()
        .map(|name| name.text().to_string())
        .collect()
    }

    #[test]
    fn collects_dom_elements() {
        assert_eq!(
            collect(
                r#"<div bind:this={a}></div><input bind:this={ b } /><my-element bind:this={c} /><svelte:element this="p" bind:this={d} />"#
            ),
            ["a", "b", "c", "d"]
        );
    }

    #[test]
    fn ignores_components_and_special_elements() {
        assert!(
            collect(
                r#"<Component bind:this={a} /><ui.Button bind:this={b} /><svelte:window bind:this={c} /><svelte:self bind:this={d} />"#
            )
            .is_empty()
        );
    }

    #[test]
    fn ignores_other_bindings_and_non_identifiers() {
        assert!(
            collect(
                r#"<input bind:value={a} /><div bind:this={b.c}></div><div bind:this={d[0]}></div><div bind:this></div>"#
            )
            .is_empty()
        );
    }

    #[test]
    fn ignores_each_block_names() {
        assert_eq!(
            collect(
                r#"{#each items as item, index}<div bind:this={item}></div><div bind:this={index}></div><div bind:this={other}></div>{/each}{#each items as { a, b: [c], ...d }}<div bind:this={a}></div><div bind:this={c}></div><div bind:this={d}></div>{/each}"#
            ),
            ["other"]
        );
    }
}
