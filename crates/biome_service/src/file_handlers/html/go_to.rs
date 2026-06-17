use crate::file_handlers::html::is_component_element;
use crate::file_handlers::{ResolveBindingParams, ResolveDefinitionParams};
use crate::workspace::{DefinitionReference, GoToDefinitionResult, LocalEmbeddedLanguage};
use biome_fs::BiomePath;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, HtmlAttribute, HtmlComponentName, HtmlRoot, HtmlTextExpression,
};
use biome_module_graph::ModuleDb;
use biome_rowan::{AstNode, TextRange, TokenAtOffset};
use biome_workspace_db::WorkspaceDb;
use biome_workspace_db::embedded::EmbeddedDb;
use biome_workspace_db::embedded::bindings::{InternedBinding, get_binding_with_source};
use camino::Utf8Path;

pub(crate) fn resolve_binding_html(params: ResolveBindingParams) -> Option<DefinitionReference> {
    let root: HtmlRoot = params.parsed_source.tree(&params.workspace_db);

    let token = match root.syntax().token_at_offset(params.cursor_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(_, right) => right,
        TokenAtOffset::None => return None,
    };

    for ancestor in token.ancestors() {
        // This branch is responsible for resolving class names
        if let Some(html_attribute) = HtmlAttribute::cast_ref(&ancestor) {
            let name_token = html_attribute.name().ok()?.value_token().ok()?;
            if !name_token.text_trimmed().eq_ignore_ascii_case("class") {
                return None;
            }

            // Skip component elements — class on a component is a prop, not a CSS class
            if is_component_element(&html_attribute) {
                return None;
            }

            let initializer = html_attribute.initializer()?;
            let value = initializer.value().ok()?;

            let html_string = match value {
                AnyHtmlAttributeInitializer::HtmlString(s) => s,
                _ => return None,
            };

            let value_token = html_string.value_token().ok()?;
            let inner_text = html_string.inner_string_text().ok()?;
            let inner_source_range = inner_text.source_range(value_token.text_trimmed_range());

            let relative_offset = params
                .cursor_offset
                .checked_sub(inner_source_range.start())?;
            let relative_offset: usize = relative_offset.into();

            let text = inner_text.text();
            if relative_offset > text.len() {
                return None;
            }

            // Find which whitespace-separated class name the cursor falls within
            let mut pos = 0usize;
            for class_name in text.split_ascii_whitespace() {
                let start = text[pos..].find(class_name).map(|i| i + pos)?;
                let end = start + class_name.len();

                if relative_offset >= start && relative_offset < end {
                    return Some(DefinitionReference::CssClass {
                        class_name: class_name.to_string(),
                    });
                }

                pos = end;
            }
        }

        // This branch is responsible for resolving component names.
        if let Some(element) = HtmlComponentName::cast_ref(&ancestor)
            && let Some(element_value) = element.value_token().ok()
            && let Some(binding) = get_binding_with_source(
                &params.workspace_db,
                InternedBinding::new(
                    &params.workspace_db,
                    params.path.clone(),
                    element_value.token_text_trimmed(),
                ),
            )
            && let Some(source) = binding.source(&params.workspace_db)
        {
            return Some(DefinitionReference::HtmlComponent {
                local_name: binding.text(&params.workspace_db).to_string(),
                source: source.to_string(),
            });
        }

        if let Some(element) = HtmlTextExpression::cast_ref(&ancestor)
            && let Some(element_value) = element.html_literal_token().ok()
            && let Some(binding) = params
                .workspace_db
                .binding_by_name(params.path.as_path(), element_value.text_trimmed())
        {
            return Some(DefinitionReference::LocalEmbedded {
                range: binding.range(&params.workspace_db),
                to_language: LocalEmbeddedLanguage::Js,
            });
        }
    }

    None
}

pub(crate) fn resolve_definition(params: ResolveDefinitionParams) -> Option<GoToDefinitionResult> {
    let mut result = GoToDefinitionResult::default();
    match params.definition_ref {
        DefinitionReference::HtmlComponent { local_name, source } => {
            resolve_import_definition(
                local_name,
                source,
                params.path.as_path(),
                &params.workspace_db,
                &mut result,
            );
        }
        DefinitionReference::Local { .. }
        | DefinitionReference::Import { .. }
        | DefinitionReference::CssClass { .. }
        | DefinitionReference::LocalEmbedded { .. } => {}
    }

    Some(result)
}

fn resolve_import_definition(
    _local_name: &str,
    source: &str,
    current_path: &Utf8Path,
    module_db: &WorkspaceDb,
    result: &mut GoToDefinitionResult,
) -> Option<()> {
    let module_info = module_db.html_module_info_for_path(current_path)?;
    let html_import = module_info
        .static_import_paths
        .get(source)
        .or_else(|| module_info.dynamic_import_paths.get(source))?;

    let target_path = html_import.as_path()?;

    // Skip files not in the module graph
    if !module_db.contains(target_path) {
        return None;
    }

    result.store(
        BiomePath::new(target_path),
        TextRange::new(0.into(), 0.into()),
    );

    Some(())
}
