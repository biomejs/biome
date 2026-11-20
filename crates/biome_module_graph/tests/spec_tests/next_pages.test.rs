use biome_db::testing::{assert_function_query_was_not_run, assert_function_query_was_run};
use biome_fs::MemoryFileSystem;
use biome_module_graph::{ModuleDb, ModuleGraphGeneration, ModuleInfo, next_pages};
use biome_resolver::ResolverDb;
use camino::{Utf8Path, Utf8PathBuf};
use salsa::Setter;

use super::{build_js_test_module_db, resolve_js_module_kind_for_test};

#[test]
fn next_pages_reuses_edits_and_recomputes_after_pages_are_added() {
    let fs = MemoryFileSystem::default();
    fs.insert(
        "/web/app/page.jsx".into(),
        "export default function Page() {}",
    );
    fs.insert(
        "/web/app/about/page.jsx".into(),
        "export default function About() {}",
    );
    let mut db = build_js_test_module_db(&fs, &["/web/app/page.jsx"], false);
    let project = db
        .resolver_paths()
        .get_or_create(&db, Utf8Path::new("/web"));
    assert_eq!(next_pages(&db, project).len(), 1);

    let page = db
        .module_for_path(Utf8Path::new("/web/app/page.jsx"))
        .unwrap();
    fs.insert(
        "/web/app/page.jsx".into(),
        "export default function Home() {}",
    );
    let kind = resolve_js_module_kind_for_test(&fs, "/web/app/page.jsx", false);
    page.set_kind(&mut db).to(kind);
    db.clear_salsa_events();
    assert_eq!(next_pages(&db, project).len(), 1);
    let events = db.take_salsa_events();
    assert_function_query_was_not_run(&db, next_pages, project, &events);

    let kind = resolve_js_module_kind_for_test(&fs, "/web/app/about/page.jsx", false);
    let about = ModuleInfo::new(&db, Utf8PathBuf::from("/web/app/about/page.jsx"), kind);
    db.modules
        .insert(Utf8PathBuf::from("/web/app/about/page.jsx"), about);
    ModuleGraphGeneration::get(&db).set_value(&mut db).to(1);
    db.clear_salsa_events();
    assert_eq!(next_pages(&db, project).len(), 2);
    let events = db.take_salsa_events();
    assert_function_query_was_run(&db, next_pages, project, &events);
}
