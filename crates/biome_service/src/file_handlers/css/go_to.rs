use crate::file_handlers::ResolveDefinitionParams;
use crate::workspace::{DefinitionReference, GoToDefinitionResult};
use biome_css_syntax::CssClassSelector;
use biome_fs::BiomePath;
use biome_module_graph::{SymbolFromModuleInfo, find_css_class_definition};
use biome_rowan::AstNode;
use std::ops::Add;

/// Destination-side capability for CSS: resolves a binding reference to a
/// CSS class definition location.
pub(crate) fn resolve_definition(params: ResolveDefinitionParams) -> Option<GoToDefinitionResult> {
    let mut result = GoToDefinitionResult::default();
    if let DefinitionReference::CssClass { class_name } = params.definition_ref {
        let path = params.path.as_path();
        if let Some(module) = params.module_db.module_for_path(path) {
            for (css_path, mut range, content_offset) in find_css_class_definition(
                params.module_db,
                SymbolFromModuleInfo::new(params.module_db, class_name, module),
            ) {
                // For inline `<style>` blocks, the range is snippet-local.
                // Apply the content_offset to get parent document coordinates.
                if let Some(offset) = content_offset {
                    range += offset;
                }
                result.store(BiomePath::new(css_path), range);
            }
        }

        if let Some(model) = params
            .services
            .as_css_services()
            .and_then(|s| s.semantic_model.as_ref())
        {
            for rule in model.rules() {
                for selector in rule.selectors() {
                    let node = selector.node(&model.root());
                    for class_sel in node
                        .syntax()
                        .descendants()
                        .filter_map(CssClassSelector::cast)
                    {
                        if let Ok(name) = class_sel.name()
                            && name.syntax().text_trimmed().to_string() == *class_name
                        {
                            let mut range = name.syntax().text_trimmed_range();
                            if let Some(offset) = params.offset {
                                range = range.add(offset);
                            }
                            result.store(BiomePath::new(path), range);
                        }
                    }
                }
            }
        };
    };

    Some(result)
}
