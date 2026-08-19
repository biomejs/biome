use biome_db::testing::{assert_function_query_was_not_run, assert_function_query_was_run};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_module_graph::{
    ModuleDb, ModuleInfo, ModuleInfoKind, PathInfoCache, SymbolFromModuleInfo,
    build_import_tree_for_js, css_classes_for_module, is_class_referenced_by_importers,
    resolve_css_module, transitive_importers_of, traverse_import_tree_for_classes,
};
use biome_project_layout::ProjectLayout;
use biome_service::db::WorkspaceDb;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;

use super::support::{add_css_modules, add_js_modules};
use super::{TestModuleDb, resolve_js_module_kind_for_test};

fn graph(css: &[(&str, &str)], js: &[(&str, &str)]) -> (MemoryFileSystem, WorkspaceDb) {
    let fs = MemoryFileSystem::default();
    for (path, source) in css.iter().chain(js) {
        fs.insert((*path).into(), *source);
    }
    let mut db = WorkspaceDb::default();
    add_css_modules(
        &mut db,
        &fs,
        &ProjectLayout::default(),
        &css.iter()
            .map(|(p, _)| BiomePath::new(*p))
            .collect::<Vec<_>>(),
    );
    add_js_modules(
        &mut db,
        &fs,
        &ProjectLayout::default(),
        &js.iter()
            .map(|(p, _)| BiomePath::new(*p))
            .collect::<Vec<_>>(),
        false,
    );
    (fs, db)
}

#[test]
fn test_jsx_imports_css_file() {
    let (_, db) = graph(
        &[(
            "/src/styles.css",
            ".button { color: red; }\n.header { font-size: 24px; }",
        )],
        &[(
            "/src/App.jsx",
            "import \"./styles.css\";\n\nexport function App() {\n    return <div className=\"button header\">Hello</div>;\n}",
        )],
    );
    let info = db
        .js_module_info_for_path(Utf8Path::new("/src/App.jsx"))
        .unwrap();
    assert!(
        info.import_paths
            .iter()
            .any(|path| path.as_path() == Some(Utf8Path::new("/src/styles.css")))
    );
    assert!(["button", "header"].into_iter().all(|name| {
        info.referenced_classes
            .iter()
            .any(|class| class.matches(name))
    }));
    assert!(
        db.css_module_info_for_path(Utf8Path::new("/src/styles.css"))
            .is_some()
    );
}

#[test]
fn test_css_classes_referenced_by_jsx() {
    let (_, db) = graph(
        &[("/src/styles.css", ".used{} .unused{}")],
        &[(
            "/src/App.jsx",
            "import './styles.css'; export const App=()=> <div className=\"used\"/>;",
        )],
    );
    let module = db
        .module_for_path(Utf8Path::new("/src/styles.css"))
        .unwrap();
    assert!(is_class_referenced_by_importers(
        &db,
        SymbolFromModuleInfo::new(&db, "used", module)
    ));
    assert!(!is_class_referenced_by_importers(
        &db,
        SymbolFromModuleInfo::new(&db, "unused", module)
    ));
}

#[test]
fn test_transitive_css_import_chain() {
    let (_, db) = graph(
        &[
            ("/src/base.css", ".base{} .orphan{}"),
            ("/src/theme.css", "@import './base.css'; .theme{}"),
        ],
        &[(
            "/src/App.jsx",
            "import './theme.css'; export const App=()=> <div className=\"base theme\"/>;",
        )],
    );
    let module = db.module_for_path(Utf8Path::new("/src/base.css")).unwrap();
    assert!(
        transitive_importers_of(&db, module)
            .iter()
            .any(|path| path.as_path() == Utf8Path::new("/src/App.jsx"))
    );
    assert!(is_class_referenced_by_importers(
        &db,
        SymbolFromModuleInfo::new(&db, "base", module)
    ));
    assert!(!is_class_referenced_by_importers(
        &db,
        SymbolFromModuleInfo::new(&db, "orphan", module)
    ));
}

#[test]
fn test_single_entry_point_with_nested_css_imports() {
    let (_, db) = graph(
        &[
            ("/src/components.css", ".button{} .card{} .unused{}"),
            ("/src/utils.css", ".flex{} .grid{}"),
            (
                "/src/app.css",
                "@import './components.css'; @import './utils.css';",
            ),
        ],
        &[(
            "/src/App.tsx",
            "import './app.css'; export const App=()=> <div className=\"button card flex\"/>;",
        )],
    );
    for (path, used, unused) in [
        (
            "/src/components.css",
            ["button", "card"].as_slice(),
            "unused",
        ),
        ("/src/utils.css", ["flex"].as_slice(), "grid"),
    ] {
        let module = db.module_for_path(Utf8Path::new(path)).unwrap();
        assert!(used.iter().all(|name| is_class_referenced_by_importers(
            &db,
            SymbolFromModuleInfo::new(&db, *name, module)
        )));
        assert!(!is_class_referenced_by_importers(
            &db,
            SymbolFromModuleInfo::new(&db, unused, module)
        ));
    }
}

#[test]
fn test_multiple_entry_points_sharing_css() {
    let (_, db) = graph(
        &[
            ("/src/components.css", ".button{} .card{} .modal{}"),
            ("/src/app.css", "@import './components.css';"),
        ],
        &[
            (
                "/src/App.tsx",
                "import './app.css'; export const App=()=> <div className=\"button\"/>;",
            ),
            (
                "/src/Dashboard.tsx",
                "import './app.css'; export const Dashboard=()=> <div className=\"card\"/>;",
            ),
        ],
    );
    let module = db
        .module_for_path(Utf8Path::new("/src/components.css"))
        .unwrap();
    assert_eq!(transitive_importers_of(&db, module).len(), 2);
    assert!(
        ["button", "card"]
            .into_iter()
            .all(|name| is_class_referenced_by_importers(
                &db,
                SymbolFromModuleInfo::new(&db, name, module)
            ))
    );
    assert!(!is_class_referenced_by_importers(
        &db,
        SymbolFromModuleInfo::new(&db, "modal", module)
    ));
}

fn available(css: &[(&str, &str)]) -> (usize, FxHashSet<String>) {
    let mut js = String::new();
    for (path, _) in css {
        js.push_str(&format!("import '{path}';"));
    }
    let (_, db) = graph(css, &[("/src/App.jsx", &js)]);
    let traversal = traverse_import_tree_for_classes(
        &db,
        db.module_for_path(Utf8Path::new("/src/App.jsx")).unwrap(),
    );
    let classes = traversal
        .iter()
        .flat_map(|step| step.css_classes.values())
        .map(|class| class.text().to_string())
        .collect();
    (traversal.len(), classes)
}

#[test]
fn test_collect_available_classes_for_js_file() {
    let (len, classes) = available(&[("/src/styles.css", ".button{} .card{}")]);
    assert_eq!(len, 1);
    assert!(classes.contains("button") && classes.contains("card"));
}

#[test]
fn test_collect_available_classes_for_js_file_multiple_css() {
    let (len, classes) = available(&[
        ("/src/buttons.css", ".btn{} .btn-primary{}"),
        ("/src/layout.css", ".container{} .flex{}"),
    ]);
    assert_eq!(len, 2);
    assert!(
        ["btn", "btn-primary", "container", "flex"]
            .into_iter()
            .all(|name| classes.contains(name))
    );
}

#[test]
fn css_class_upward_traversal_does_not_revisit_the_start() {
    let (_, db) = graph(
        &[("/style.css", ".style{}")],
        &[
            ("/a.js", "import './b.js'; import './style.css';"),
            ("/b.js", "import './a.js';"),
        ],
    );
    let steps =
        traverse_import_tree_for_classes(&db, db.module_for_path(Utf8Path::new("/a.js")).unwrap());
    assert_eq!(
        steps
            .iter()
            .filter(|step| step.css_path == Utf8Path::new("/style.css"))
            .count(),
        1
    );
}

#[test]
fn css_importers_and_parent_nodes_are_sorted_and_deduplicated() {
    let (_, db) = graph(
        &[
            ("/shared.css", ".shared{}"),
            ("/left.css", "@import './shared.css';"),
            ("/right.css", "@import './shared.css';"),
        ],
        &[
            ("/z.js", "import './left.css'; import './right.css';"),
            ("/a.js", "import './left.css'; import './right.css';"),
            ("/leaf.js", "export const leaf = true;"),
            ("/z-parent.js", "import './leaf.js'; import './leaf.js';"),
            ("/a-parent.js", "import './leaf.js';"),
        ],
    );

    let shared = db.module_for_path(Utf8Path::new("/shared.css")).unwrap();
    assert_eq!(
        transitive_importers_of(&db, shared),
        [Utf8PathBuf::from("/a.js"), Utf8PathBuf::from("/z.js")]
    );

    let leaf = db.module_for_path(Utf8Path::new("/leaf.js")).unwrap();
    let tree = build_import_tree_for_js(&db, leaf).unwrap();
    assert_eq!(
        tree.parent_components
            .iter()
            .map(|parent| parent.file_path.as_path())
            .collect::<Vec<_>>(),
        [Utf8Path::new("/a-parent.js"), Utf8Path::new("/z-parent.js")]
    );
}

fn resolve_css_module_kind_for_test(fs: &MemoryFileSystem, path: &str) -> ModuleInfoKind {
    let paths = [BiomePath::new(path)];
    let (_, root) = biome_test_utils::get_css_added_paths(fs, &paths)
        .pop()
        .unwrap();
    let (info, _, _) = resolve_css_module(
        root,
        &paths[0],
        fs,
        &ProjectLayout::default(),
        &PathInfoCache::default(),
    );
    ModuleInfoKind::Css(info)
}

#[test]
fn css_classes_query_reuses_unrelated_edits_and_invalidates_dependency_edits() {
    let fs = MemoryFileSystem::default();
    fs.insert("/app.js".into(), "import './theme.css';");
    fs.insert("/theme.css".into(), ".theme{}");
    fs.insert("/unrelated.css".into(), ".unrelated{}");

    let mut db = TestModuleDb::new();
    for path in ["/theme.css", "/unrelated.css"] {
        let module = ModuleInfo::new(
            &db,
            Utf8PathBuf::from(path),
            resolve_css_module_kind_for_test(&fs, path),
        );
        db.modules.insert(Utf8PathBuf::from(path), module);
    }
    let app = ModuleInfo::new(
        &db,
        Utf8PathBuf::from("/app.js"),
        resolve_js_module_kind_for_test(&fs, "/app.js", false),
    );
    db.modules.insert(Utf8PathBuf::from("/app.js"), app);

    assert_eq!(css_classes_for_module(&db, app).len(), 1);

    let unrelated = db.module_for_path(Utf8Path::new("/unrelated.css")).unwrap();
    fs.insert("/unrelated.css".into(), ".changed{}");
    let kind = resolve_css_module_kind_for_test(&fs, "/unrelated.css");
    salsa::Setter::to(unrelated.set_kind(&mut db), kind);
    db.clear_salsa_events();
    assert_eq!(css_classes_for_module(&db, app).len(), 1);
    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, css_classes_for_module, app, &events);

    let theme = db.module_for_path(Utf8Path::new("/theme.css")).unwrap();
    fs.insert("/theme.css".into(), ".changed{}");
    let kind = resolve_css_module_kind_for_test(&fs, "/theme.css");
    salsa::Setter::to(theme.set_kind(&mut db), kind);
    db.clear_salsa_events();
    assert_eq!(
        css_classes_for_module(&db, app)[0]
            .css_classes
            .values()
            .next()
            .unwrap()
            .text(),
        "changed"
    );
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, css_classes_for_module, app, &events);
}
