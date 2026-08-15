use biome_fs::{BiomePath, MemoryFileSystem};
use biome_module_graph::{
    ModuleDb, ModuleGraphGeneration, ModuleInfoKind, PathInfoCache, js_module_sccs,
    resolve_js_module,
};
use biome_project_layout::ProjectLayout;
use biome_service::db::WorkspaceDb;
use biome_test_utils::get_added_js_paths;
use camino::{Utf8Path, Utf8PathBuf};

fn resolve_module(fs: &MemoryFileSystem, path: &str) -> ModuleInfoKind {
    let paths = [BiomePath::new(path)];
    let mut added_paths = get_added_js_paths(fs, &paths);
    let (path, root, semantic_model) = added_paths.pop().expect("module must parse");
    let (module_info, _, _) = resolve_js_module(
        root,
        path,
        fs,
        &ProjectLayout::default(),
        semantic_model,
        &PathInfoCache::default(),
        false,
    );
    ModuleInfoKind::Js(module_info)
}

fn module_db(files: &[(&str, &str)]) -> (MemoryFileSystem, WorkspaceDb) {
    let fs = MemoryFileSystem::default();
    for &(path, source) in files {
        fs.insert(path.into(), source);
    }

    let mut db = WorkspaceDb::default();
    for &(path, _) in files {
        db.update_or_insert_module(Utf8PathBuf::from(path), resolve_module(&fs, path));
    }
    (fs, db)
}

#[test]
fn scc_query_is_scoped_to_its_database() {
    let (_, cyclic_db) = module_db(&[
        ("/cyclic/a.js", "import './b.js';"),
        ("/cyclic/b.js", "import './a.js';"),
    ]);
    let (_, acyclic_db) = module_db(&[
        ("/acyclic/a.js", "import './b.js';"),
        ("/acyclic/b.js", "export {};"),
    ]);

    assert_eq!(
        cyclic_db.module_graph_generation(),
        acyclic_db.module_graph_generation()
    );
    assert!(
        js_module_sccs(&cyclic_db, ModuleGraphGeneration::get(&cyclic_db))
            .contains_cycle_between(Utf8Path::new("/cyclic/a.js"), Utf8Path::new("/cyclic/b.js"))
    );
    assert!(
        !js_module_sccs(&acyclic_db, ModuleGraphGeneration::get(&acyclic_db))
            .contains_cycle_between(
                Utf8Path::new("/acyclic/a.js"),
                Utf8Path::new("/acyclic/b.js")
            )
    );
}

#[test]
fn scc_query_recomputes_after_module_graph_changes() {
    let (fs, mut db) = module_db(&[
        ("/src/a.js", "import './b.js';"),
        ("/src/b.js", "export {};"),
    ]);

    assert!(
        !js_module_sccs(&db, ModuleGraphGeneration::get(&db))
            .contains_cycle_between(Utf8Path::new("/src/a.js"), Utf8Path::new("/src/b.js"))
    );

    let generation = db.module_graph_generation();
    fs.insert("/src/b.js".into(), "import './a.js';");
    db.update_or_insert_module(
        Utf8PathBuf::from("/src/b.js"),
        resolve_module(&fs, "/src/b.js"),
    );

    assert_eq!(db.module_graph_generation(), generation);
    assert!(
        js_module_sccs(&db, ModuleGraphGeneration::get(&db))
            .contains_cycle_between(Utf8Path::new("/src/a.js"), Utf8Path::new("/src/b.js"))
    );
}

#[test]
fn scc_query_recomputes_after_modules_are_added_and_removed() {
    let fs = MemoryFileSystem::default();
    fs.insert("/src/a.js".into(), "import './b.js';");
    fs.insert("/src/b.js".into(), "import './a.js';");

    let mut db = WorkspaceDb::default();
    let a_path = Utf8Path::new("/src/a.js");
    let b_path = Utf8Path::new("/src/b.js");
    db.update_or_insert_module(a_path.to_path_buf(), resolve_module(&fs, "/src/a.js"));

    assert!(
        !js_module_sccs(&db, ModuleGraphGeneration::get(&db))
            .contains_cycle_between(a_path, b_path)
    );

    let generation = db.module_graph_generation();
    db.update_or_insert_module(b_path.to_path_buf(), resolve_module(&fs, "/src/b.js"));

    assert_eq!(db.module_graph_generation(), generation.wrapping_add(1));
    assert!(
        js_module_sccs(&db, ModuleGraphGeneration::get(&db)).contains_cycle_between(a_path, b_path)
    );

    let generation = db.module_graph_generation();
    db.remove_module(b_path);

    assert_eq!(db.module_graph_generation(), generation.wrapping_add(1));
    assert!(
        !js_module_sccs(&db, ModuleGraphGeneration::get(&db))
            .contains_cycle_between(a_path, b_path)
    );
}

#[test]
fn scc_query_ignores_paths_in_node_modules() {
    let (_, db) = module_db(&[
        ("/src/a.js", "import '../node_modules/dependency/index.js';"),
        ("/src/b.js", "import './a.js';"),
        (
            "/node_modules/dependency/index.js",
            "import '../../src/b.js';",
        ),
    ]);

    assert!(
        !js_module_sccs(&db, ModuleGraphGeneration::get(&db))
            .contains_cycle_between(Utf8Path::new("/src/a.js"), Utf8Path::new("/src/b.js"))
    );
}
