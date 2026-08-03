use biome_fs::{BiomePath, MemoryFileSystem};
use biome_module_graph::{
    ModuleDb, SymbolFromModuleInfo, is_class_referenced_by_importers, transitive_importers_of,
    traverse_import_tree_for_classes,
};
use biome_project_layout::ProjectLayout;
use biome_workspace_db::WorkspaceDb;
use camino::Utf8Path;
use rustc_hash::FxHashSet;

use super::support::{add_css_modules, add_js_modules};

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
