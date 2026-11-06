//! Queries over the resolved dependencies of modules.

use crate::css_module_info::CssModuleInfo;
use crate::html_module_info::HtmlModuleInfo;
use crate::{
    JsModuleInfo, ModuleDb, ModuleDependencies, ModuleGraphGeneration, ModuleInfo, ModuleInfoKind,
    ResolutionMode, resolve_specifier,
};
use biome_resolver::ResolverDb;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashMap;

/// Returns the resolved dependencies of `module`.
#[salsa::tracked]
pub fn module_dependencies(db: &dyn ModuleDb, module: ModuleInfo) -> ModuleDependencies {
    dependencies_of(db, module.path(db), module.kind(db))
}

#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct ModuleImporters(FxHashMap<Utf8PathBuf, Vec<ModuleInfo>>);

impl ModuleImporters {
    pub(crate) fn for_path(&self, path: &Utf8Path) -> &[ModuleInfo] {
        self.0.get(path).map_or(&[], Vec::as_slice)
    }
}

/// Returns an index from resolved dependency paths to their direct importers.
#[salsa::tracked]
pub(crate) fn module_importers(
    db: &dyn ModuleDb,
    generation: ModuleGraphGeneration,
) -> ModuleImporters {
    let _ = generation.value(db);
    let mut importers = ModuleImporters::default();
    db.for_each_module(&mut |module| {
        for dependency in module_dependencies(db, module).iter() {
            importers
                .0
                .entry(dependency.clone())
                .or_default()
                .push(module);
        }
    });
    for direct_importers in importers.0.values_mut() {
        direct_importers.sort_unstable_by(|left, right| left.path(db).cmp(right.path(db)));
    }
    importers
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
