use biome_fs::MemoryFileSystem;
use biome_languages::HtmlFileSource;
use biome_languages::css::EmbeddingStyleApplicability;
use biome_module_graph::{ModuleDb, traverse_import_tree_for_html_classes};
use biome_service::db::WorkspaceDb;
use camino::Utf8Path;

use super::support::{
    astro_css_source, build_html_db, html_css_source, parse_embedded_css, svelte_css_source,
    vue_css_source,
};

fn classes(
    path: &str,
    source: HtmlFileSource,
    styles: &[(&str, biome_languages::CssFileSource)],
) -> (WorkspaceDb, Vec<String>) {
    let fs = MemoryFileSystem::default();
    fs.insert(path.into(), "<div class=\"card\"></div>");
    let embedded = styles
        .iter()
        .map(|(css, source)| parse_embedded_css(css, *source))
        .collect();
    let db = build_html_db(
        &fs,
        &[(path, "<div class=\"card\"></div>", source, embedded)],
    );
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    let classes = traverse_import_tree_for_html_classes(&db, module)
        .iter()
        .flat_map(|step| step.css_classes.values())
        .map(|class| class.text().to_string())
        .collect();
    (db, classes)
}

#[test]
fn test_html_inline_style_classes_are_global() {
    let (db, classes) = classes(
        "/src/index.html",
        HtmlFileSource::html(),
        &[(".card{}", html_css_source())],
    );
    let info = db
        .html_module_info_for_path(Utf8Path::new("/src/index.html"))
        .unwrap();
    assert_eq!(
        info.style_classes[0].applicability,
        EmbeddingStyleApplicability::Global
    );
    assert!(classes.contains(&"card".into()));
}

#[test]
fn test_html_self_closing_element_class_references_are_collected() {
    let source = "<img class=\"hero\"/><input class=\"field\"/><br class=\"spacer\"/>";
    let fs = MemoryFileSystem::default();
    fs.insert("/src/index.html".into(), source);
    let db = build_html_db(
        &fs,
        &[("/src/index.html", source, HtmlFileSource::html(), vec![])],
    );
    let info = db
        .html_module_info_for_path(Utf8Path::new("/src/index.html"))
        .unwrap();
    assert!(["hero", "field", "spacer"].into_iter().all(|name| {
        info.referenced_classes
            .iter()
            .any(|class| class.matches(name))
    }));
}

#[test]
fn test_vue_unscoped_style_classes_are_global() {
    let (db, classes) = classes(
        "/src/Comp.vue",
        HtmlFileSource::vue(),
        &[(
            ".card{}",
            vue_css_source(EmbeddingStyleApplicability::Global),
        )],
    );
    assert_eq!(
        db.html_module_info_for_path(Utf8Path::new("/src/Comp.vue"))
            .unwrap()
            .style_classes[0]
            .applicability,
        EmbeddingStyleApplicability::Global
    );
    assert!(classes.contains(&"card".into()));
}

#[test]
fn test_vue_scoped_style_classes_are_local_and_hidden() {
    let (db, classes) = classes(
        "/src/Scoped.vue",
        HtmlFileSource::vue(),
        &[(
            ".alpha{}",
            vue_css_source(EmbeddingStyleApplicability::Local),
        )],
    );
    assert_eq!(
        db.html_module_info_for_path(Utf8Path::new("/src/Scoped.vue"))
            .unwrap()
            .style_classes[0]
            .applicability,
        EmbeddingStyleApplicability::Local
    );
    assert!(classes.contains(&"alpha".into()));
}

#[test]
fn test_vue_mixed_scoped_and_unscoped() {
    let (_, classes) = classes(
        "/src/Mixed.vue",
        HtmlFileSource::vue(),
        &[
            (
                ".global-btn{}",
                vue_css_source(EmbeddingStyleApplicability::Global),
            ),
            (
                ".scoped-card{}",
                vue_css_source(EmbeddingStyleApplicability::Local),
            ),
        ],
    );
    assert!(
        ["global-btn", "scoped-card"]
            .into_iter()
            .all(|name| classes.contains(&name.into()))
    );
}

#[test]
fn test_astro_local_style_classes_are_hidden() {
    let (db, classes) = classes(
        "/src/Page.astro",
        HtmlFileSource::astro(),
        &[(
            ".hero{}",
            astro_css_source(EmbeddingStyleApplicability::Local),
        )],
    );
    assert_eq!(
        db.html_module_info_for_path(Utf8Path::new("/src/Page.astro"))
            .unwrap()
            .style_classes[0]
            .applicability,
        EmbeddingStyleApplicability::Local
    );
    assert!(classes.contains(&"hero".into()));
}

#[test]
fn test_astro_global_style_classes_are_visible() {
    let (_, classes) = classes(
        "/src/Layout.astro",
        HtmlFileSource::astro(),
        &[(
            ".wrapper{}",
            astro_css_source(EmbeddingStyleApplicability::Global),
        )],
    );
    assert!(classes.contains(&"wrapper".into()));
}

#[test]
fn test_svelte_local_style_classes_are_hidden() {
    let (db, classes) = classes(
        "/src/Button.svelte",
        HtmlFileSource::svelte(),
        &[(".btn{}", svelte_css_source())],
    );
    assert_eq!(
        db.html_module_info_for_path(Utf8Path::new("/src/Button.svelte"))
            .unwrap()
            .style_classes[0]
            .applicability,
        EmbeddingStyleApplicability::Local
    );
    assert!(classes.contains(&"btn".into()));
}

#[test]
fn test_svelte_global_pseudo_class_is_visible() {
    let (db, classes) = classes(
        "/src/Global.svelte",
        HtmlFileSource::svelte(),
        &[(":global(.prose){}", svelte_css_source())],
    );
    assert_eq!(
        db.html_module_info_for_path(Utf8Path::new("/src/Global.svelte"))
            .unwrap()
            .style_classes
            .iter()
            .find(|class| class.name.text() == "prose")
            .unwrap()
            .applicability,
        EmbeddingStyleApplicability::Global
    );
    assert!(classes.contains(&"prose".into()));
}
