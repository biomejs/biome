use biome_fs::BiomePath;
use biome_module_graph::{
    ModuleDb, SymbolFromModuleInfo, is_class_referenced_by_importers, module_dependencies,
};
use biome_service::db::WorkspaceDb;
use camino::{Utf8Path, Utf8PathBuf};

use super::support::{add_js_modules, build_css_db};

fn dependencies(db: &WorkspaceDb, path: &str) -> Vec<Utf8PathBuf> {
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    let mut dependencies: Vec<_> = module_dependencies(db, module).iter().cloned().collect();
    dependencies.sort();
    dependencies
}

#[test]
fn test_scss_loads_resolve_to_partials() {
    let (_, db) = build_css_db(&[
        (
            "/src/main.scss",
            r#"@use "sass:math";
@use "tokens" as t;
@forward "mixins";
@import "frames", "theme.css", "components", url("print.css"), "screen" screen;
.a { width: math.div(1px, 2); }
"#,
        ),
        ("/src/_tokens.scss", "$x: 1px;\n"),
        ("/src/_mixins.scss", "@mixin m {}\n"),
        ("/src/_frames.scss", ".frames {}\n"),
        ("/src/frames.css", ".unused {}\n"),
        ("/src/theme.css", ".theme {}\n"),
        ("/src/components/_index.scss", ".card {}\n"),
        ("/src/print.css", ".print {}\n"),
        ("/src/screen.css", ".screen {}\n"),
        ("/src/_screen.scss", ".unused {}\n"),
    ]);
    assert_eq!(
        dependencies(&db, "/src/main.scss"),
        [
            "/src/_frames.scss",
            "/src/_mixins.scss",
            "/src/_tokens.scss",
            "/src/components/_index.scss",
            "/src/print.css",
            "/src/screen.css",
            "/src/theme.css",
        ]
        .map(Utf8PathBuf::from)
    );
}

#[test]
fn test_scss_import_prefers_import_only_files() {
    let (_, db) = build_css_db(&[
        ("/src/legacy.scss", "@import \"theme\";\n"),
        ("/src/modern.scss", "@use \"theme\";\n"),
        ("/src/_theme.scss", ".theme {}\n"),
        ("/src/_theme.import.scss", "@forward \"theme\";\n"),
    ]);
    assert_eq!(
        dependencies(&db, "/src/legacy.scss"),
        [Utf8PathBuf::from("/src/_theme.import.scss")]
    );
    assert_eq!(
        dependencies(&db, "/src/modern.scss"),
        [Utf8PathBuf::from("/src/_theme.scss")]
    );
}

#[test]
fn test_ambiguous_scss_load_has_no_dependency() {
    let (_, db) = build_css_db(&[
        ("/src/main.scss", "@use \"theme\";\n"),
        ("/src/theme.scss", ".a {}\n"),
        ("/src/_theme.scss", ".b {}\n"),
    ]);
    assert!(dependencies(&db, "/src/main.scss").is_empty());
}

#[test]
fn test_classes_in_scss_partials_are_referenced_through_loads() {
    let (fs, mut db) = build_css_db(&[
        ("/src/styles.scss", "@use \"card\";\n.page {}\n"),
        ("/src/_card.scss", ".card {} .orphan {}\n"),
    ]);
    fs.insert(
        "/src/App.jsx".into(),
        "import './styles.scss'; export const App = () => <div className=\"page card\" />;",
    );
    add_js_modules(&mut db, &fs, &[BiomePath::new("/src/App.jsx")], false);
    let partial = db
        .module_for_path(Utf8Path::new("/src/_card.scss"))
        .unwrap();
    assert!(is_class_referenced_by_importers(
        &db,
        SymbolFromModuleInfo::new(&db, "card", partial)
    ));
    assert!(!is_class_referenced_by_importers(
        &db,
        SymbolFromModuleInfo::new(&db, "orphan", partial)
    ));
}

#[test]
fn test_bare_scss_load_falls_back_to_packages() {
    let (_, db) = build_css_db(&[
        ("/src/main.scss", "@import \"normalize.css/normalize\";\n"),
        ("/package.json", "{}"),
        (
            "/node_modules/normalize.css/package.json",
            "{\"name\":\"normalize.css\"}",
        ),
        ("/node_modules/normalize.css/normalize.css", "html {}\n"),
    ]);
    assert_eq!(
        dependencies(&db, "/src/main.scss"),
        [Utf8PathBuf::from(
            "/node_modules/normalize.css/normalize.css"
        )]
    );
}
