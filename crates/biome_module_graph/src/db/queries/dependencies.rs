//! Queries over the resolved dependencies of modules.

use crate::css_module_info::CssModuleInfo;
use crate::html_module_info::HtmlModuleInfo;
use crate::{
    JsModuleInfo, ModuleDb, ModuleDependencies, ModuleInfo, ModuleInfoKind, ResolutionMode,
    resolve_specifier,
};
use biome_resolver::ResolverDb;
use camino::Utf8Path;

/// Returns the resolved dependencies of `module`.
#[salsa::tracked(returns(ref))]
pub fn module_dependencies(db: &dyn ModuleDb, module: ModuleInfo) -> ModuleDependencies {
    dependencies_of(db, module.path(db), &module.kind(db))
}

fn dependencies_of(
    db: &dyn ResolverDb,
    path: &Utf8Path,
    kind: &ModuleInfoKind,
) -> ModuleDependencies {
    match kind {
        ModuleInfoKind::Js(info) => js_dependencies(db, path, info),
        ModuleInfoKind::Css(info) => css_dependencies(db, path, info),
        ModuleInfoKind::Html(info) => html_dependencies(db, path, info),
    }
}

pub(crate) fn js_dependencies(
    db: &dyn ResolverDb,
    path: &Utf8Path,
    info: &JsModuleInfo,
) -> ModuleDependencies {
    let mut dependencies = DependencyCollector::new(db, path);
    for import in info.all_import_paths() {
        dependencies.resolve(&import.specifier, ResolutionMode::JavaScript);
    }
    dependencies.finish()
}

pub(crate) fn css_dependencies(
    db: &dyn ResolverDb,
    path: &Utf8Path,
    info: &CssModuleInfo,
) -> ModuleDependencies {
    let mut dependencies = DependencyCollector::new(db, path);
    for import in info.imports.iter() {
        dependencies.resolve(&import.specifier, ResolutionMode::Css);
    }
    dependencies.finish()
}

pub(crate) fn html_dependencies(
    db: &dyn ResolverDb,
    path: &Utf8Path,
    info: &HtmlModuleInfo,
) -> ModuleDependencies {
    let mut dependencies = DependencyCollector::new(db, path);
    for import in &info.imported_stylesheets {
        dependencies.resolve(&import.specifier, ResolutionMode::Css);
    }
    for import in info.import_paths.iter() {
        dependencies.resolve(&import.specifier, ResolutionMode::HtmlScript);
    }
    dependencies.finish()
}

/// Collects the resolved paths of the imports of one module.
struct DependencyCollector<'a> {
    db: &'a dyn ResolverDb,
    directory: &'a Utf8Path,
    dependencies: ModuleDependencies,
}

impl<'a> DependencyCollector<'a> {
    fn new(db: &'a dyn ResolverDb, module_path: &'a Utf8Path) -> Self {
        Self {
            db,
            directory: module_path.parent().unwrap_or(module_path),
            dependencies: ModuleDependencies::default(),
        }
    }

    fn resolve(&mut self, specifier: &str, mode: ResolutionMode) {
        let resolved = resolve_specifier(self.db, self.directory, specifier, mode);
        if let Some(path) = resolved.path().as_path() {
            self.dependencies.insert(path.to_path_buf());
        }
    }

    fn finish(self) -> ModuleDependencies {
        self.dependencies
    }
}
