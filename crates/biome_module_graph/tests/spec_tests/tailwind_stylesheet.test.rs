use biome_module_graph::{ModuleDb, TailwindThemeEntry, TailwindUtility, tailwind_stylesheet};
use camino::Utf8Path;

use super::support::build_tailwind_css_db;

/// Returns the theme of the stylesheet at `path`, with variables as their name
/// and resets as `<reference>-*`.
fn theme(files: &[(&str, &str)], path: &str) -> Vec<String> {
    let db = build_tailwind_css_db(files);
    let module = db.module_for_path(Utf8Path::new(path)).unwrap();
    tailwind_stylesheet(&db, module)
        .theme
        .iter()
        .map(|entry| match entry {
            TailwindThemeEntry::Variable { name, .. } => name.to_string(),
            TailwindThemeEntry::Reset { reference } => format!("{reference}-*"),
        })
        .collect()
}

#[test]
fn imports_are_inlined_in_source_order() {
    assert_eq!(
        theme(
            &[
                (
                    "/main.css",
                    "@theme { --color-*: initial; }\n@import './colors.css';\n@theme { --color-accent: red; }"
                ),
                ("/colors.css", "@theme { --color-brand: blue; }"),
            ],
            "/main.css"
        ),
        ["--color-*", "--color-brand", "--color-accent"]
    );
}

#[test]
fn repeated_imports_are_inlined_each_time() {
    assert_eq!(
        theme(
            &[
                (
                    "/main.css",
                    "@import './colors.css';\n@import './reset.css';"
                ),
                (
                    "/reset.css",
                    "@theme { --color-*: initial; }\n@import './colors.css';"
                ),
                ("/colors.css", "@theme { --color-brand: blue; }"),
            ],
            "/main.css"
        ),
        ["--color-brand", "--color-*", "--color-brand"]
    );
}

#[test]
fn import_cycles_are_inlined_once_per_branch() {
    assert_eq!(
        theme(
            &[
                ("/a.css", "@import './b.css';\n@theme { --color-a: red; }"),
                ("/b.css", "@import './a.css';\n@theme { --color-b: blue; }"),
            ],
            "/a.css"
        ),
        ["--color-b", "--color-a"]
    );
}

#[test]
fn url_imports_are_not_inlined() {
    assert_eq!(
        theme(
            &[
                (
                    "/main.css",
                    "@import url('./colors.css');\n@import url(./colors.css);\n@import './accent.css';"
                ),
                ("/colors.css", "@theme { --color-brand: blue; }"),
                ("/accent.css", "@theme { --color-accent: red; }"),
            ],
            "/main.css"
        ),
        ["--color-accent"]
    );
}

#[test]
fn strings_outside_imports_are_not_inlined() {
    assert_eq!(
        theme(
            &[
                (
                    "/main.css",
                    "@theme { --font-display: './colors.css'; }\n.logo { background: url(./colors.css); }"
                ),
                ("/colors.css", "@theme { --color-brand: blue; }"),
            ],
            "/main.css"
        ),
        ["--font-display"]
    );
}

#[test]
fn functional_utilities_are_recorded_by_root() {
    let db = build_tailwind_css_db(&[
        (
            "/main.css",
            "@utility tab-4 { tab-size: 4; }\n@import './animate.css';\n@utility tab-* { tab-size: --value(integer); }",
        ),
        (
            "/animate.css",
            "@utility slide-in-from-top-* { --tw-enter-translate-y: --value([*]); }\n@utility -zoom-in-* { --tw-enter-scale: --value([*]); }",
        ),
    ]);
    let module = db.module_for_path(Utf8Path::new("/main.css")).unwrap();
    let stylesheet = tailwind_stylesheet(&db, module);
    let names = |utilities: &[TailwindUtility]| -> Vec<String> {
        utilities
            .iter()
            .map(|utility| utility.name.to_string())
            .collect()
    };
    assert_eq!(names(&stylesheet.utilities), ["tab-4"]);
    assert_eq!(
        names(&stylesheet.functional_utilities),
        ["slide-in-from-top", "-zoom-in", "tab"]
    );
}
