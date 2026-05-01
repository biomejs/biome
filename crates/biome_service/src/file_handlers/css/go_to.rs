use crate::file_handlers::ResolveDefinitionParams;
use crate::workspace::{DefinitionReference, GoToDefinitionResult};
use biome_fs::BiomePath;
use std::ops::Add;

/// Destination-side capability for CSS: resolves a binding reference to a
/// CSS class definition location.
pub(crate) fn resolve_definition(params: ResolveDefinitionParams) -> Option<GoToDefinitionResult> {
    let mut result = GoToDefinitionResult::default();
    if let DefinitionReference::CssClass { class_name } = params.definition_ref {
        let path = params.path.as_path();
        for (css_path, mut range, content_offset) in params
            .module_graph
            .find_css_class_definition(path, class_name)
        {
            // For inline `<style>` blocks, the range is snippet-local.
            // Apply the content_offset to get parent document coordinates.
            if let Some(offset) = content_offset {
                range += offset;
            }
            result.store(BiomePath::new(css_path), range);
        }

        if let Some(model) = params
            .services
            .as_css_services()
            .and_then(|s| s.semantic_model.as_ref())
        {
            for rule in model.rules() {
                for selector in rule.selectors() {
                    if selector.resolved().normalize().contains(class_name) {
                        let mut range = selector.range(&model.root());
                        if let Some(offset) = params.offset {
                            range = range.add(offset);
                        }
                        result.store(BiomePath::new(path), range);
                    }
                }
            }
        };
    };

    Some(result)
}
