use super::support::{build_module_db_via_workspace, snapshot_files};
use crate::snap::ModuleGraphSnapshot;
use biome_module_graph::{ModuleDb, traverse_import_tree_for_html_classes};
use camino::Utf8Path;

fn assert_component_snapshot(name: &str, files: &[(&str, &str)]) {
    let db = build_module_db_via_workspace(files);
    ModuleGraphSnapshot::from_files(&db, snapshot_files(files)).assert_snapshot(name);
}

#[test]
fn test_vue_component_imports_snapshot() {
    assert_component_snapshot(
        "test_vue_component_imports_snapshot",
        &[
            (
                "/src/app.css",
                ".app { color: red; }\n.btn { padding: 8px; }\n",
            ),
            (
                "/src/Button.vue",
                "<template>\n  <button class=\"btn\">Click me</button>\n</template>\n\n<style scoped>\n  .btn { font-weight: bold; }\n</style>\n",
            ),
            (
                "/src/App.vue",
                "<template>\n  <div class=\"app\"><Button /></div>\n</template>\n\n<script>\nimport \"./app.css\";\nimport Button from \"./Button.vue\";\n</script>\n",
            ),
        ],
    );
}

#[test]
fn test_astro_component_imports_snapshot() {
    assert_component_snapshot(
        "test_astro_component_imports_snapshot",
        &[
            (
                "/src/global.css",
                ".layout { display: flex; }\n.hero { font-size: 2rem; }\n",
            ),
            (
                "/src/Hero.astro",
                "---\n---\n<section class=\"hero\">Welcome</section>\n\n<style>\n  .hero { color: navy; }\n</style>\n\n<style is:global>\n  .hero { color: navy; }\n</style>\n",
            ),
            (
                "/src/Layout.astro",
                "---\nimport \"./global.css\";\nimport Hero from \"./Hero.astro\";\n---\n<div class=\"layout\"><Hero /></div>\n",
            ),
        ],
    );
}

#[test]
fn test_svelte_component_imports_snapshot() {
    assert_component_snapshot(
        "test_svelte_component_imports_snapshot",
        &[
            (
                "/src/theme.css",
                ".wrapper { max-width: 1200px; }\n.title { font-weight: bold; }\n",
            ),
            (
                "/src/Card.svelte",
                "<script>\n</script>\n\n<div class=\"card\">Content</div>\n\n<style>\n  .card { border: 1px solid; }\n</style>\n",
            ),
            (
                "/src/App.svelte",
                "<script>\nimport \"./theme.css\";\nimport Card from \"./Card.svelte\";\n</script>\n\n<div class=\"wrapper title\"><Card /></div>\n",
            ),
        ],
    );
}

#[test]
fn test_vue_upward_traversal() {
    let files = [
        ("/src/app.css", ".app{} .page{} .btn{}"),
        (
            "/src/App.vue",
            "<script>import './app.css'; import Page from './Page.vue';</script><template><div class='app'/></template>",
        ),
        (
            "/src/Page.vue",
            "<script>import Button from './Button.vue';</script><template><div class='page'/></template>",
        ),
        (
            "/src/Button.vue",
            "<template><button class='invalid'/></template>",
        ),
    ];
    let db = build_module_db_via_workspace(&files);
    let module = db
        .module_for_path(Utf8Path::new("/src/Button.vue"))
        .unwrap();
    assert!(
        traverse_import_tree_for_html_classes(&db, module)
            .iter()
            .any(|step| step.css_classes.values().any(|class| class.text() == "btn"))
    );
}
