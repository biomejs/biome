use super::module_importers;
use crate::{JsImportPhase, ModuleDb, ModuleGraphGeneration, ModuleInfo, ModuleInfoKind};
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

// #region EXPORTED TRACKED QUERIES

/// Returns a Vue, Svelte, or Astro component that evaluates `module` during
/// server-side rendering.
///
/// The component must reach `module` through static value imports in its
/// server-rendered sources, possibly through other JavaScript modules. Dynamic
/// imports, type-only imports, and Astro `<script>` tags aren't followed.
/// The component with the fewest imports between it and `module` is returned.
#[salsa::tracked]
pub fn server_rendered_component_importer(
    db: &dyn ModuleDb,
    module: ModuleInfo,
) -> Option<Utf8PathBuf> {
    let importers = module_importers(db, ModuleGraphGeneration::get(db));
    let mut visited: FxHashSet<Utf8PathBuf> = FxHashSet::default();
    let mut queue = VecDeque::from([module.path(db).to_path_buf()]);

    while let Some(current) = queue.pop_front() {
        if !visited.insert(current.clone()) {
            continue;
        }
        let target = Some(current.as_path());
        for &owner in importers.for_path(&current) {
            let path = owner.path(db);
            match owner.kind(db) {
                ModuleInfoKind::Js(js_info) => {
                    if js_info.import_paths.iter().any(|import| {
                        import.kind.is_static()
                            && import.phase == JsImportPhase::Default
                            && import.resolve_js(db, owner).path().as_path() == target
                    }) {
                        queue.push_back(path.to_path_buf());
                    }
                }
                ModuleInfoKind::Html(html_info) => {
                    if is_component_path(path)
                        && html_info.import_paths.iter().any(|import| {
                            import.server_rendered
                                && import.resolve_html(db, owner).path().as_path() == target
                        })
                    {
                        return Some(path.to_path_buf());
                    }
                }
                ModuleInfoKind::Css(_) => {}
            }
        }
    }

    None
}

// #endregion

fn is_component_path(path: &Utf8Path) -> bool {
    matches!(path.extension(), Some("vue" | "svelte" | "astro"))
}
