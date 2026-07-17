use crate::file_handlers::ResolveDefinitionParams;
use crate::workspace::{DefinitionReference, GoToDefinitionResult};
use biome_css_semantic::db::css_semantic_model;
use biome_css_syntax::CssClassSelector;
use biome_fs::BiomePath;
use biome_languages::LanguageDb;
#[cfg(feature = "module_graph")]
use biome_module_graph::{ModuleDb, SymbolFromModuleInfo, find_css_class_definition};
use biome_rowan::AstNode;
use std::ops::Add;

/// Destination-side capability for CSS: resolves a binding reference to a
/// CSS class definition location.
pub(crate) fn resolve_definition(params: ResolveDefinitionParams) -> Option<GoToDefinitionResult> {
    let mut result = GoToDefinitionResult::default();
    if let DefinitionReference::CssClass { class_name } = params.definition_ref {
        let path = params.path.as_path();
        #[cfg(feature = "module_graph")]
        if let Some(module) = params.workspace_db.module_for_path(path) {
            for (css_path, mut range, content_offset) in find_css_class_definition(
                &params.workspace_db,
                SymbolFromModuleInfo::new(&params.workspace_db, class_name, module),
            ) {
                // For inline `<style>` blocks, the range is snippet-local.
                // Apply the content_offset to get parent document coordinates.
                if let Some(offset) = content_offset {
                    range += offset;
                }
                result.store(BiomePath::new(css_path), range);
            }
        }

        let Some(file_source) = params.workspace_db.source_from_index(
            params
                .parsed_source
                .document_file_index(&params.workspace_db),
        ) else {
            return Some(result);
        };
        if !file_source.is_css_like() {
            return Some(result);
        }

        let diagnostic_offset = params.parsed_source.diagnostic_offset(&params.workspace_db);
        let semantic_model = css_semantic_model(&params.workspace_db, &params.parsed_source);
        for rule in semantic_model.rules() {
            for selector in rule.selectors() {
                let node = selector.node(&semantic_model.root());
                for class_sel in node
                    .syntax()
                    .descendants()
                    .filter_map(CssClassSelector::cast)
                {
                    if let Ok(name) = class_sel.name()
                        && name.syntax().text_trimmed().to_string() == *class_name
                    {
                        let mut range = name.syntax().text_trimmed_range();
                        if let Some(offset) = diagnostic_offset {
                            range = range.add(offset);
                        }
                        result.store(BiomePath::new(path), range);
                    }
                }
            }
        }
    };

    Some(result)
}
