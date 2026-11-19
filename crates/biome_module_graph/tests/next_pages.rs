use biome_fs::{BiomePath, MemoryFileSystem};
use biome_module_graph::{ModuleInfoKind, NextPage, next_pages, resolve_js_module};
use biome_resolver::ResolverDb;
use biome_service::db::WorkspaceDb;
use biome_test_utils::get_added_js_paths;
use camino::Utf8Path;
use std::sync::Arc;

const PAGE: &str = "export default function Page() {}";

fn update_module(db: &mut WorkspaceDb, fs: &MemoryFileSystem, path: &str) {
    let paths = [BiomePath::new(path)];
    let mut added_paths = get_added_js_paths(fs, &paths);
    let (path, root, semantic_model) = added_paths.pop().expect("module must parse");
    let (module_info, _, _) = resolve_js_module(&*db, root, path, semantic_model, false);
    db.update_or_insert_module(
        path.as_path().to_path_buf(),
        ModuleInfoKind::Js(module_info),
    );
}

fn module_db(paths: &[&str]) -> (MemoryFileSystem, WorkspaceDb) {
    let fs = MemoryFileSystem::default();
    for &path in paths {
        fs.insert(path.into(), PAGE);
    }

    let mut db = WorkspaceDb::new(Arc::new(MemoryFileSystem::from_files(fs.files.0.clone())));
    for &path in paths {
        update_module(&mut db, &fs, path);
    }
    (fs, db)
}

fn pages<'db>(db: &'db WorkspaceDb, project: &str) -> &'db [NextPage] {
    let project = db
        .resolver_paths()
        .get_or_create(db, Utf8Path::new(project));
    next_pages(db, project)
}

fn links_to_page(db: &WorkspaceDb, project: &str, path: &str) -> bool {
    pages(db, project).iter().any(|page| page.matches(path))
}

#[test]
fn reads_pages_of_both_routers() {
    let (_, db) = module_db(&[
        "/web/app/page.jsx",
        "/web/app/about/page.tsx",
        "/web/app/(marketing)/pricing/page.jsx",
        "/web/app/@modal/settings/page.jsx",
        "/web/pages/contact.jsx",
        "/web/pages/stores/index.js",
    ]);

    assert_eq!(pages(&db, "/web").len(), 6);
    for path in [
        "/",
        "/about",
        "/pricing",
        "/settings",
        "/contact",
        "/stores",
    ] {
        assert!(links_to_page(&db, "/web", path), "{path}");
    }
}

#[test]
fn ignores_files_that_are_not_pages() {
    let (_, db) = module_db(&[
        "/web/app/layout.jsx",
        "/web/app/status/route.js",
        "/web/app/_components/page.jsx",
        "/web/app/feed/(..)photo/page.jsx",
        "/web/pages/_app.jsx",
        "/web/pages/_document.jsx",
        "/web/pages/_error/index.jsx",
        "/web/pages/api.js",
        "/web/pages/api/users.js",
        "/web/components/button.jsx",
    ]);

    assert!(pages(&db, "/web").is_empty());
}

#[test]
fn project_path_skips_src() {
    let (_, db) = module_db(&["/web/src/app/team/page.jsx", "/web/src/pages/careers.jsx"]);

    assert!(links_to_page(&db, "/web", "/team"));
    assert!(links_to_page(&db, "/web", "/careers"));
}

#[test]
fn pages_are_scoped_to_their_project() {
    let (_, db) = module_db(&[
        "/repo/apps/web/app/about/page.jsx",
        "/repo/apps/docs/pages/guide.jsx",
    ]);

    assert!(links_to_page(&db, "/repo/apps/web", "/about"));
    assert!(!links_to_page(&db, "/repo/apps/web", "/guide"));
    assert!(links_to_page(&db, "/repo/apps/docs", "/guide"));
    assert!(!links_to_page(&db, "/repo/apps/docs", "/about"));
}

#[test]
fn pages_of_nested_projects_are_scoped_to_them() {
    let (_, db) = module_db(&[
        "/web/app/page.jsx",
        "/web/packages/inner/app/about/page.jsx",
    ]);

    assert!(!links_to_page(&db, "/web", "/about"));
    assert!(links_to_page(&db, "/web/packages/inner", "/about"));
    assert!(!links_to_page(&db, "/web/packages/inner", "/"));
}

#[test]
fn recomputes_after_pages_are_added_and_removed() {
    let (fs, mut db) = module_db(&["/web/app/page.jsx"]);
    assert!(!links_to_page(&db, "/web", "/about"));

    fs.insert("/web/app/about/page.jsx".into(), PAGE);
    update_module(&mut db, &fs, "/web/app/about/page.jsx");
    assert!(links_to_page(&db, "/web", "/about"));

    db.remove_module(Utf8Path::new("/web/app/about/page.jsx"));
    assert!(!links_to_page(&db, "/web", "/about"));
}
