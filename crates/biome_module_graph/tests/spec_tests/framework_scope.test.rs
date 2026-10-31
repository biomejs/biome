use biome_fs::{BiomePath, MemoryFileSystem};
use biome_module_graph::{
    CssPropertyDefinition, FrameworkScope, ModuleDb, SymbolFromModuleInfo,
    css_property_definitions, find_css_class_definition, framework_class_steps,
    has_unknown_framework_styles, is_class_referenced_by_importers,
    traverse_import_tree_for_classes,
};
use biome_rowan::{TextRange, TextSize};
use biome_service::db::WorkspaceDb;
use camino::{Utf8Path, Utf8PathBuf};

use super::support::{
    TestFs, add_css_modules, add_js_modules, build_css_db, build_module_db_via_workspace,
};

fn scope(package_path: &str, stylesheets: &[&str], components: &[&str]) -> FrameworkScope {
    FrameworkScope {
        package_path: package_path.into(),
        stylesheets: stylesheets.iter().copied().map(Utf8PathBuf::from).collect(),
        components: components.iter().copied().map(Utf8PathBuf::from).collect(),
        ..FrameworkScope::default()
    }
}

fn property_definitions(db: &WorkspaceDb, path: &str) -> Vec<String> {
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    let mut definitions: Vec<_> =
        css_property_definitions(db, SymbolFromModuleInfo::new(db, "--value", module))
            .iter()
            .map(|definition| definition.module_path.to_string())
            .collect();
    definitions.sort_unstable();
    definitions
}

fn graph(css: &[(&str, &str)], js: &[(&str, &str)]) -> WorkspaceDb {
    let fs = MemoryFileSystem::default();
    for (path, source) in css.iter().chain(js) {
        fs.insert((*path).into(), *source);
    }
    let mut db = WorkspaceDb::new(fs.share());
    let paths = |files: &[(&str, &str)]| {
        files
            .iter()
            .map(|(path, _)| BiomePath::new(*path))
            .collect::<Vec<_>>()
    };
    add_css_modules(&mut db, &fs, &paths(css));
    add_js_modules(&mut db, &fs, &paths(js), false);
    db
}

fn framework_classes(db: &WorkspaceDb, path: &str) -> Vec<String> {
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    let mut classes: Vec<_> = framework_class_steps(db, module)
        .iter()
        .flat_map(|step| step.css_classes.values())
        .map(|class| class.text().to_string())
        .collect();
    classes.sort_unstable();
    classes
}

fn is_used(db: &WorkspaceDb, stylesheet: &str, class_name: &str) -> bool {
    let module = db.module_for_path(Utf8Path::new(stylesheet)).unwrap();
    is_class_referenced_by_importers(db, SymbolFromModuleInfo::new(db, class_name, module))
}

#[test]
fn framework_stylesheets_define_custom_properties_for_every_stylesheet() {
    let (_, mut db) = build_css_db(&[
        (
            "/node_modules/starlight/props.css",
            ":root { --value: red; }",
        ),
        ("/src/custom.css", "@import './partial.css';"),
        ("/src/partial.css", "a { color: var(--value); }"),
    ]);
    assert!(property_definitions(&db, "/src/partial.css").is_empty());

    db.set_framework_scope_for_test(
        "/".into(),
        scope(
            "/",
            &["/node_modules/starlight/props.css", "/src/custom.css"],
            &[],
        ),
    );
    assert_eq!(
        property_definitions(&db, "/src/partial.css"),
        ["/node_modules/starlight/props.css"]
    );
}

#[test]
fn imported_custom_properties_win_over_framework_stylesheets() {
    let (_, mut db) = build_css_db(&[
        (
            "/node_modules/starlight/props.css",
            ":root { --value: red; }",
        ),
        ("/src/theme.css", ":root { --value: blue; }"),
        (
            "/src/partial.css",
            "@import './theme.css'; a { color: var(--value); }",
        ),
    ]);
    db.set_framework_scope_for_test(
        "/".into(),
        scope("/", &["/node_modules/starlight/props.css"], &[]),
    );
    assert_eq!(
        property_definitions(&db, "/src/partial.css"),
        ["/src/theme.css"]
    );
}

#[test]
fn framework_configuration_defines_custom_properties() {
    let (_, mut db) = build_css_db(&[("/src/page.css", "a { font-family: var(--value); }")]);
    let definition = CssPropertyDefinition {
        module_path: "/astro.config.mjs".into(),
        range: TextRange::new(TextSize::from(10), TextSize::from(19)),
    };
    db.set_framework_scope_for_test(
        "/".into(),
        FrameworkScope {
            package_path: "/".into(),
            custom_properties: vec![("--value".into(), definition)],
            ..FrameworkScope::default()
        },
    );
    assert_eq!(
        property_definitions(&db, "/src/page.css"),
        ["/astro.config.mjs"]
    );
}

#[test]
fn framework_scope_belongs_to_the_nearest_package() {
    let (_, mut db) = build_css_db(&[
        ("/props.css", ":root { --value: red; }"),
        ("/packages/site/page.css", "a { color: var(--value); }"),
    ]);
    db.set_framework_scope_for_test("/".into(), scope("/", &["/props.css"], &[]));
    db.set_framework_scope_for_test("/packages/site".into(), scope("/packages/site", &[], &[]));
    assert!(property_definitions(&db, "/packages/site/page.css").is_empty());
}

#[test]
fn framework_stylesheets_provide_classes_to_every_file() {
    let mut db = graph(
        &[
            ("/src/custom.css", "@import './partial.css'; .custom {}"),
            ("/src/partial.css", ".partial {}"),
        ],
        &[("/src/App.jsx", "export const App = () => <div />;")],
    );
    assert!(framework_classes(&db, "/src/App.jsx").is_empty());

    db.set_framework_scope_for_test("/".into(), scope("/", &["/src/custom.css"], &[]));
    assert_eq!(
        framework_classes(&db, "/src/App.jsx"),
        ["custom", "partial"]
    );

    // The file still imports no stylesheet.
    let app = db.module_for_path(Utf8Path::new("/src/App.jsx")).unwrap();
    assert!(traverse_import_tree_for_classes(&db, app).is_empty());
}

#[test]
fn classes_of_framework_stylesheets_are_used_by_any_file_of_the_package() {
    let mut db = graph(
        &[
            (
                "/site/global.css",
                "@import './partial.css'; .used {} .unused {}",
            ),
            ("/site/partial.css", ".partial {}"),
            ("/site/local.css", ".orphan {}"),
        ],
        &[
            (
                "/site/Page.jsx",
                "export const Page = () => <div className=\"used partial orphan\" />;",
            ),
            (
                "/other/Page.jsx",
                "export const Page = () => <div className=\"unused\" />;",
            ),
        ],
    );
    db.set_framework_scope_for_test("/site".into(), scope("/site", &["/site/global.css"], &[]));
    assert!(is_used(&db, "/site/global.css", "used"));
    assert!(is_used(&db, "/site/partial.css", "partial"));
    assert!(!is_used(&db, "/site/global.css", "unused"));
    assert!(!is_used(&db, "/site/local.css", "orphan"));
}

#[test]
fn classes_of_framework_stylesheets_are_used_when_some_pages_are_unknown() {
    let mut db = graph(
        &[
            ("/site/global.css", ".unused {}"),
            ("/site/local.css", ".orphan {}"),
        ],
        &[],
    );
    db.set_framework_scope_for_test(
        "/site".into(),
        FrameworkScope {
            has_unknown_class_usages: true,
            ..scope("/site", &["/site/global.css"], &[])
        },
    );
    assert!(is_used(&db, "/site/global.css", "unused"));
    assert!(!is_used(&db, "/site/local.css", "orphan"));
}

#[test]
fn classes_of_every_stylesheet_are_used_when_framework_stylesheets_are_unknown() {
    let mut db = graph(&[("/site/local.css", ".orphan {}")], &[]);
    assert!(!is_used(&db, "/site/local.css", "orphan"));

    db.set_framework_scope_for_test(
        "/site".into(),
        FrameworkScope {
            has_unknown_styles: true,
            ..scope("/site", &[], &[])
        },
    );
    assert!(is_used(&db, "/site/local.css", "orphan"));
}

#[test]
fn framework_components_provide_global_styles_and_use_classes() {
    let mut db = build_module_db_via_workspace(&[
        (
            "/src/vendor/Frame.astro",
            "---\n---\n<div class=\"attribute\"></div>\n<style>\n  .scoped {}\n</style>\n<style is:global>\n  .global {}\n</style>\n",
        ),
        (
            "/src/site/global.css",
            ".attribute {} .scoped {} .unused {}",
        ),
        (
            "/src/site/Page.jsx",
            "export const Page = () => <div className=\"global\" />;",
        ),
    ]);
    db.set_framework_scope_for_test(
        "/src/site".into(),
        scope(
            "/src/site",
            &["/src/site/global.css"],
            &["/src/vendor/Frame.astro"],
        ),
    );

    let page = db
        .module_for_path(Utf8Path::new("/src/site/Page.jsx"))
        .unwrap();
    let component_classes: Vec<_> = framework_class_steps(&db, page)
        .iter()
        .filter(|step| step.css_path == "/src/vendor/Frame.astro")
        .flat_map(|step| step.css_classes.values())
        .map(|class| class.text().to_string())
        .collect();
    assert_eq!(component_classes, ["global"]);

    let definitions =
        find_css_class_definition(&db, SymbolFromModuleInfo::new(&db, "global", page));
    let paths: Vec<_> = definitions
        .iter()
        .map(|(path, _, _)| path.as_str())
        .collect();
    assert_eq!(paths, ["/src/vendor/Frame.astro"]);

    assert!(is_used(&db, "/src/site/global.css", "attribute"));
    assert!(is_used(&db, "/src/site/global.css", "scoped"));
    assert!(!is_used(&db, "/src/site/global.css", "unused"));
}

#[test]
fn classes_used_by_a_nested_package_are_not_used_by_the_enclosing_package() {
    let mut db = graph(
        &[("/site/global.css", ".card {}")],
        &[(
            "/site/packages/ui/Button.jsx",
            "export const Button = () => <div className=\"card\" />;",
        )],
    );
    db.set_framework_scope_for_test("/site".into(), scope("/site", &["/site/global.css"], &[]));
    assert!(is_used(&db, "/site/global.css", "card"));

    db.set_framework_scope_for_test(
        "/site/packages/ui".into(),
        scope("/site/packages/ui", &[], &[]),
    );
    assert!(!is_used(&db, "/site/global.css", "card"));
}

#[test]
fn classes_of_stylesheets_from_other_packages_are_used_by_the_pages_that_apply_them() {
    let mut db = graph(
        &[("/tokens/theme.css", ".brand {} .orphan {}")],
        &[(
            "/docs/Page.jsx",
            "export const Page = () => <div className=\"brand\" />;",
        )],
    );
    db.set_framework_scope_for_test("/tokens".into(), scope("/tokens", &[], &[]));
    db.set_framework_scope_for_test("/docs".into(), scope("/docs", &["/tokens/theme.css"], &[]));
    assert!(is_used(&db, "/tokens/theme.css", "brand"));
    assert!(!is_used(&db, "/tokens/theme.css", "orphan"));

    db.set_framework_scope_for_test(
        "/docs".into(),
        FrameworkScope {
            has_unknown_class_usages: true,
            ..scope("/docs", &["/tokens/theme.css"], &[])
        },
    );
    assert!(is_used(&db, "/tokens/theme.css", "orphan"));
}

#[test]
fn stylesheets_from_other_packages_see_the_custom_properties_of_the_pages_that_apply_them() {
    let (_, mut db) = build_css_db(&[
        (
            "/docs/node_modules/starlight/props.css",
            ":root { --value: red; }",
        ),
        ("/tokens/theme.css", "a { color: var(--value); }"),
    ]);
    db.set_framework_scope_for_test("/tokens".into(), scope("/tokens", &[], &[]));
    assert!(property_definitions(&db, "/tokens/theme.css").is_empty());

    db.set_framework_scope_for_test(
        "/docs".into(),
        scope(
            "/docs",
            &[
                "/docs/node_modules/starlight/props.css",
                "/tokens/theme.css",
            ],
            &[],
        ),
    );
    assert_eq!(
        property_definitions(&db, "/tokens/theme.css"),
        ["/docs/node_modules/starlight/props.css"]
    );
}

#[test]
fn stylesheets_from_other_packages_have_the_unknown_styles_of_the_pages_that_apply_them() {
    let mut db = graph(&[("/tokens/theme.css", ".brand {}")], &[]);
    db.set_framework_scope_for_test("/tokens".into(), scope("/tokens", &[], &[]));
    let theme = db
        .module_for_path(Utf8Path::new("/tokens/theme.css"))
        .unwrap();
    assert!(!has_unknown_framework_styles(&db, theme));

    db.set_framework_scope_for_test(
        "/docs".into(),
        FrameworkScope {
            has_unknown_styles: true,
            ..scope("/docs", &["/tokens/theme.css"], &[])
        },
    );
    let theme = db
        .module_for_path(Utf8Path::new("/tokens/theme.css"))
        .unwrap();
    assert!(has_unknown_framework_styles(&db, theme));
    assert!(is_used(&db, "/tokens/theme.css", "brand"));
}

#[test]
fn stylesheets_that_framework_components_import_apply_to_every_page() {
    let mut db = build_module_db_via_workspace(&[
        (
            "/src/vendor/Frame.astro",
            "---\nimport \"./extra.css\";\n---\n<div></div>\n",
        ),
        ("/src/vendor/extra.css", ".extra {}"),
        ("/src/site/global.css", ".site {}"),
        (
            "/src/site/Page.jsx",
            "export const Page = () => <div className=\"extra\" />;",
        ),
    ]);
    db.set_framework_scope_for_test(
        "/src/site".into(),
        scope(
            "/src/site",
            &["/src/site/global.css"],
            &["/src/vendor/Frame.astro"],
        ),
    );
    assert_eq!(
        framework_classes(&db, "/src/site/Page.jsx"),
        ["extra", "site"]
    );
    assert!(is_used(&db, "/src/vendor/extra.css", "extra"));
}

#[test]
fn classes_of_framework_stylesheets_are_used_when_markup_is_unread() {
    let mut db = graph(
        &[("/site/global.css", ".unused {}")],
        &[("/site/Page.astro", "const title = 1;")],
    );
    db.set_framework_scope_for_test("/site".into(), scope("/site", &["/site/global.css"], &[]));
    assert!(is_used(&db, "/site/global.css", "unused"));
}

#[test]
fn classes_used_by_dependencies_are_not_used_by_the_package() {
    let mut db = graph(
        &[("/site/global.css", ".card {}")],
        &[(
            "/site/node_modules/library/Button.jsx",
            "export const Button = () => <div className=\"card\" />;",
        )],
    );
    db.set_framework_scope_for_test("/site".into(), scope("/site", &["/site/global.css"], &[]));
    assert!(!is_used(&db, "/site/global.css", "card"));
}
