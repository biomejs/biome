use biome_db::Db;
use biome_fs::BiomePath;
use biome_module_graph::{ModuleDb, SymbolFromModuleInfo, css_property_definitions};
use biome_project_layout::ProjectLayout;
use camino::Utf8Path;

use super::support::{add_js_modules, build_css_db, build_module_db_via_workspace};

const PROPERTY: &str =
    "@property --value { syntax: '<color>'; inherits: true; initial-value: red; }";

fn definitions(files: &[(&str, &str)], path: &str) -> Vec<String> {
    let (_, db) = build_css_db(files);
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    css_property_definitions(&db, SymbolFromModuleInfo::new(&db, "--value", module))
        .iter()
        .map(|definition| definition.module_path.as_str().replace('\\', "/"))
        .collect()
}

fn workspace_definitions(files: &[(&str, &str)], path: &str) -> Vec<String> {
    let db = build_module_db_via_workspace(files);
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    css_property_definitions(&db, SymbolFromModuleInfo::new(&db, "--value", module))
        .iter()
        .map(|definition| definition.module_path.as_str().replace('\\', "/"))
        .collect()
}

#[test]
fn css_property_query_uses_last_local_definition() {
    let (_, db) = build_css_db(&[(
        "/value.css",
        &format!(
            "{PROPERTY}\n@property --value {{ syntax: '<length>'; inherits: true; initial-value: 0px; }}"
        ),
    )]);
    let module = db.module_for_path(Utf8Path::new("/value.css")).unwrap();
    assert_eq!(
        css_property_definitions(&db, SymbolFromModuleInfo::new(&db, "--value", module)).len(),
        1
    );
}

#[test]
fn css_property_query_only_sees_preceding_siblings() {
    assert_eq!(
        definitions(
            &[
                ("/theme.css", PROPERTY),
                ("/leaf.css", ".leaf{}"),
                ("/parent.css", "@import 'theme.css'; @import 'leaf.css';")
            ],
            "/leaf.css"
        ),
        ["/theme.css"]
    );
    assert!(
        definitions(
            &[
                ("/theme.css", PROPERTY),
                ("/leaf.css", ".leaf{}"),
                ("/parent.css", "@import 'leaf.css'; @import 'theme.css';")
            ],
            "/leaf.css"
        )
        .is_empty()
    );
}

#[test]
fn css_property_query_preserves_parent_branches() {
    let mut found = definitions(
        &[
            ("/leaf.css", ".leaf{}"),
            ("/first.css", &format!("@import 'leaf.css';{PROPERTY}")),
            ("/second.css", &format!("@import 'leaf.css';{PROPERTY}")),
        ],
        "/leaf.css",
    );
    found.sort();
    assert_eq!(found, ["/first.css", "/second.css"]);
}

#[test]
fn css_property_query_stops_each_branch_at_nearest_parent() {
    assert_eq!(
        definitions(
            &[
                ("/leaf.css", ".leaf{}"),
                ("/theme.css", &format!("@import 'leaf.css';{PROPERTY}")),
                ("/app.css", &format!("@import 'theme.css';{PROPERTY}"))
            ],
            "/leaf.css"
        ),
        ["/theme.css"]
    );
}

#[test]
fn css_property_query_continues_through_empty_parents() {
    assert_eq!(
        definitions(
            &[
                ("/leaf.css", ".leaf{}"),
                ("/theme.css", "@import 'leaf.css';"),
                ("/app.css", &format!("@import 'theme.css';{PROPERTY}"))
            ],
            "/leaf.css"
        ),
        ["/app.css"]
    );
}

#[test]
fn css_property_query_deduplicates_diamonds_and_stops_cycles() {
    assert_eq!(
        definitions(
            &[
                ("/leaf.css", ".leaf{}"),
                ("/left.css", "@import 'leaf.css';"),
                ("/right.css", "@import 'leaf.css';"),
                (
                    "/root.css",
                    &format!("@import 'left.css';@import 'right.css';{PROPERTY}")
                )
            ],
            "/leaf.css"
        ),
        ["/root.css"]
    );
    assert!(
        definitions(
            &[
                ("/leaf.css", ".leaf{}"),
                ("/left.css", "@import 'leaf.css';@import 'right.css';"),
                ("/right.css", "@import 'left.css';")
            ],
            "/leaf.css"
        )
        .is_empty()
    );
}

#[test]
fn css_property_query_does_not_cache_branch_specific_empty_results() {
    assert_eq!(
        definitions(
            &[
                ("/leaf.css", ".leaf{}"),
                ("/definition.css", PROPERTY),
                ("/right.css", "@import 'leaf.css';"),
                ("/left.css", "@import 'leaf.css';@import 'definition.css';"),
                ("/join.css", "@import 'right.css';@import 'left.css';"),
                ("/root.css", "@import 'left.css';@import 'join.css';")
            ],
            "/leaf.css"
        ),
        ["/definition.css"]
    );
}

#[test]
fn css_property_query_preserves_branch_specific_continuations() {
    assert_eq!(
        definitions(
            &[
                ("/leaf.css", ".leaf{}"),
                ("/definition.css", PROPERTY),
                ("/b.css", "@import 'leaf.css';"),
                (
                    "/a.css",
                    "@import 'leaf.css';@import 'definition.css';@import 'y.css';"
                ),
                ("/x.css", "@import 'b.css';@import 'a.css';"),
                ("/y.css", "@import 'x.css';")
            ],
            "/leaf.css"
        ),
        ["/definition.css"]
    );
}

#[test]
fn css_property_query_reads_the_current_semantic_model() {
    let (_, mut db) = build_css_db(&[("/value.css", PROPERTY)]);
    let parsed = db
        .parsed_source_for_path(Utf8Path::new("/value.css"))
        .unwrap();
    let replacement =
        biome_css_parser::parse_css(".value{}", Default::default(), Default::default());
    salsa::Setter::to(parsed.set_parsed(&mut db), replacement.into());
    let module = db.module_for_path(Utf8Path::new("/value.css")).unwrap();
    assert!(
        css_property_definitions(&db, SymbolFromModuleInfo::new(&db, "--value", module)).is_empty()
    );
}

fn js_definitions(js: &str, target: &str) -> Vec<String> {
    let (fs, mut db) = build_css_db(&[("/theme.css", PROPERTY)]);
    let app_path = if js.contains("import type") {
        "/app.ts"
    } else {
        "/app.js"
    };
    fs.insert(app_path.into(), js);
    fs.insert("/component.js".into(), "export const component=true;");
    add_js_modules(
        &mut db,
        &fs,
        &ProjectLayout::default(),
        &[BiomePath::new(app_path), BiomePath::new("/component.js")],
        false,
    );
    let target = if target == "/app.js" {
        app_path
    } else {
        target
    };
    let module = db.module_for_path(Utf8Path::new(target)).unwrap();
    css_property_definitions(&db, SymbolFromModuleInfo::new(&db, "--value", module))
        .iter()
        .map(|definition| definition.module_path.as_str().replace('\\', "/"))
        .collect()
}

#[test]
fn css_property_query_skips_non_css_sibling_imports() {
    assert!(
        js_definitions(
            "import './component.js'; import './theme.css';",
            "/component.js"
        )
        .is_empty()
    );
}
#[test]
fn css_property_query_reads_stylesheets_imported_by_js() {
    assert_eq!(
        js_definitions("import './theme.css';", "/app.js"),
        ["/theme.css"]
    );
    assert_eq!(
        js_definitions("import('./theme.css');", "/app.js"),
        ["/theme.css"]
    );
}
#[test]
fn css_property_query_preserves_js_import_order() {
    assert_eq!(
        js_definitions(
            "import './theme.css'; import './component.js';",
            "/component.js"
        ),
        ["/theme.css"]
    );
}
#[test]
fn css_property_query_ignores_type_only_js_imports() {
    assert!(js_definitions("import type { Theme } from './theme.css';", "/app.js").is_empty());
}
#[test]
fn css_property_query_traverses_js_importers() {
    assert_eq!(
        js_definitions(
            "import './theme.css'; import './component.js';",
            "/component.js"
        ),
        ["/theme.css"]
    );
    assert!(
        js_definitions(
            "import './component.js'; import './theme.css';",
            "/component.js"
        )
        .is_empty()
    );
}

#[test]
fn css_property_query_reads_html_like_embedded_styles() {
    for path in [
        "/index.html",
        "/Component.vue",
        "/Component.svelte",
        "/Component.astro",
    ] {
        assert_eq!(
            workspace_definitions(&[(path, &format!("<style>{PROPERTY}</style>"))], path),
            [path]
        );
    }
}
#[test]
fn css_property_query_tracks_html_style_offsets() {
    assert_eq!(
        workspace_definitions(
            &[("/index.html", &format!("<style>{PROPERTY}</style>"))],
            "/index.html"
        ),
        ["/index.html"]
    );
}
#[test]
fn css_property_query_ignores_html_script_snippets() {
    assert_eq!(
        workspace_definitions(
            &[(
                "/index.html",
                &format!("<style>{PROPERTY}</style><script>const x=1;</script>")
            )],
            "/index.html"
        ),
        ["/index.html"]
    );
}
#[test]
fn css_property_query_tracks_html_style_parse_changes() {
    assert!(
        workspace_definitions(&[("/index.html", "<style>.value{}</style>")], "/index.html")
            .is_empty()
    );
}
#[test]
fn css_property_query_uses_last_html_style_definition() {
    assert_eq!(
        workspace_definitions(
            &[(
                "/index.html",
                &format!("<style>{PROPERTY}</style><style>{PROPERTY}</style>")
            )],
            "/index.html"
        ),
        ["/index.html"]
    );
}
#[test]
fn css_property_query_only_exposes_global_html_importer_styles() {
    assert_eq!(
        workspace_definitions(
            &[
                (
                    "/Global.vue",
                    &format!("<style>{PROPERTY}</style><link rel='stylesheet' href='./leaf.css'>")
                ),
                ("/leaf.css", ".leaf{}")
            ],
            "/leaf.css"
        ),
        ["/Global.vue"]
    );
    assert!(
        workspace_definitions(
            &[
                (
                    "/Local.vue",
                    &format!(
                        "<style scoped>{PROPERTY}</style><link rel='stylesheet' href='./leaf.css'>"
                    )
                ),
                ("/leaf.css", ".leaf{}")
            ],
            "/leaf.css"
        )
        .is_empty()
    );
}
#[test]
fn css_property_query_follows_stylesheets_linked_from_html() {
    assert_eq!(
        workspace_definitions(
            &[
                ("/index.html", "<link rel='stylesheet' href='./theme.css'>"),
                ("/theme.css", PROPERTY)
            ],
            "/index.html"
        ),
        ["/theme.css"]
    );
}
#[test]
fn css_property_query_follows_stylesheets_imported_from_html_scripts() {
    assert_eq!(
        workspace_definitions(
            &[
                ("/index.html", "<script>import './theme.css';</script>"),
                ("/theme.css", PROPERTY)
            ],
            "/index.html"
        ),
        ["/theme.css"]
    );
}
#[test]
fn css_property_query_preserves_html_script_import_order() {
    assert_eq!(
        workspace_definitions(
            &[
                (
                    "/index.html",
                    "<script>import './first.css'; import './second.css';</script>"
                ),
                ("/first.css", PROPERTY),
                ("/second.css", PROPERTY)
            ],
            "/index.html"
        ),
        ["/second.css"]
    );
}

#[test]
fn css_imports_preserve_duplicate_occurrences() {
    let (_, db) = build_css_db(&[
        ("/theme.css", ""),
        ("/leaf.css", ""),
        (
            "/parent.css",
            "@import 'theme.css';@import 'leaf.css';@import 'theme.css';",
        ),
    ]);
    assert_eq!(
        db.css_module_info_for_path(Utf8Path::new("/parent.css"))
            .unwrap()
            .imports
            .len(),
        3
    );
}
