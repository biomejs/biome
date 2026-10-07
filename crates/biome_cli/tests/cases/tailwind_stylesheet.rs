//! CLI integration tests for the `tailwind.stylesheet` option.

use crate::TestArgs as Args;
use crate::run_cli_with_dyn_fs;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::TemporaryFs;

#[test]
fn use_tailwind_sorted_classes_reads_tailwind_stylesheet() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("use_tailwind_sorted_classes_reads_tailwind_stylesheet");

    fs.create_file(
        "biome.json",
        r#"{
    "tailwind": { "stylesheet": "./src/app.css" },
    "linter": { "rules": { "nursery": { "useTailwindSortedClasses": "error" } } }
}"#,
    );
    fs.create_file(
        "src/app.css",
        r#"@import "tailwindcss";
@import "./theme.css";

@utility btn-primary {
    color: white;
    background-color: blue;
}

@custom-variant theme-midnight (&:where([data-theme=midnight] *));
"#,
    );
    fs.create_file(
        "src/theme.css",
        r#"@theme {
    --color-brand: #00f;
    --breakpoint-3xl: 120rem;
}
"#,
    );
    fs.create_file(
        "src/App.jsx",
        r#"export const sorted = <div className="flex bg-brand" />;
export const theme = <div className="bg-brand flex" />;
export const utility = <div className="btn-primary flex" />;
export const variant = <div className="theme-midnight:flex hover:flex flex" />;
export const breakpoint = <div className="3xl:flex 2xl:flex" />;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(
            [
                "lint",
                "--only=nursery/useTailwindSortedClasses",
                fs.cli_path(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_tailwind_sorted_classes_reads_tailwind_stylesheet",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn nested_configs_resolve_tailwind_stylesheet() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("nested_configs_resolve_tailwind_stylesheet");

    fs.create_file(
        "biome.json",
        r#"{
    "root": true,
    "tailwind": { "stylesheet": "./web/app.css" },
    "linter": { "rules": { "nursery": { "useTailwindSortedClasses": "error" } } }
}"#,
    );
    fs.create_file(
        "web/app.css",
        "@import \"tailwindcss\";\n@theme { --color-brand: #00f; }\n",
    );
    // Inherits the root stylesheet, which must still resolve from the root.
    fs.create_file("web/biome.json", r#"{ "root": false, "extends": "//" }"#);
    fs.create_file(
        "web/App.jsx",
        "export const a = <div className=\"bg-brand flex\" />;\n",
    );
    // Replaces the root stylesheet with its own.
    fs.create_file(
        "admin/biome.json",
        r#"{ "root": false, "extends": "//", "tailwind": { "stylesheet": "./admin.css" } }"#,
    );
    fs.create_file(
        "admin/admin.css",
        "@import \"tailwindcss\";\n@theme { --color-admin: #f00; }\n",
    );
    fs.create_file(
        "admin/App.jsx",
        "export const a = <div className=\"bg-admin flex\" />;\nexport const b = <div className=\"bg-brand flex\" />;\n",
    );
    // Has no nested configuration, so it uses the root stylesheet.
    fs.create_file(
        "plain/App.jsx",
        "export const a = <div className=\"bg-brand flex\" />;\n",
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(
            [
                "lint",
                "--only=nursery/useTailwindSortedClasses",
                fs.cli_path(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "nested_configs_resolve_tailwind_stylesheet",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn missing_tailwind_stylesheet_errors() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("missing_tailwind_stylesheet_errors");

    fs.create_file(
        "biome.json",
        r#"{
    "tailwind": { "stylesheet": "./src/missing.css" },
    "linter": { "rules": { "nursery": { "useTailwindSortedClasses": "error" } } }
}"#,
    );
    fs.create_file(
        "src/App.jsx",
        "export const a = <div className=\"flex p-4\" />;\n",
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(
            [
                "lint",
                "--only=nursery/useTailwindSortedClasses",
                fs.cli_path(),
            ]
            .as_slice(),
        ),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "missing_tailwind_stylesheet_errors",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn use_tailwind_sorted_classes_reads_package_imports() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("use_tailwind_sorted_classes_reads_package_imports");
    fs.create_file("package.json", r#"{ "name": "app" }"#);
    fs.create_file(
        "biome.json",
        r#"{
    "tailwind": { "stylesheet": "./src/app.css" },
    "linter": { "rules": { "nursery": { "useTailwindSortedClasses": "error" } } }
}"#,
    );
    fs.create_file(
        "src/app.css",
        r#"@import "tw-animate-css";
@import "tw-animate-css/prefix";
@import "@example/theme";
"#,
    );
    fs.create_file(
        "node_modules/tw-animate-css/package.json",
        r#"{
    "name": "tw-animate-css",
    "exports": {
        ".": { "style": "./dist/tw-animate.css" },
        "./prefix": { "style": "./dist/tw-animate-prefix.css" }
    }
}"#,
    );
    fs.create_file(
        "node_modules/tw-animate-css/dist/tw-animate.css",
        "@import './utilities.css';\n",
    );
    fs.create_file(
        "node_modules/tw-animate-css/dist/utilities.css",
        "@utility animate-in { animation-name: enter; }\n",
    );
    fs.create_file(
        "node_modules/tw-animate-css/dist/tw-animate-prefix.css",
        "@utility tw-animate-in { animation-name: enter; }\n",
    );
    fs.create_file(
        "node_modules/@example/theme/package.json",
        r#"{ "name": "@example/theme", "exports": { ".": { "style": "./theme.css" } } }"#,
    );
    fs.create_file(
        "node_modules/@example/theme/theme.css",
        "@theme { --color-brand: blue; }\n",
    );
    fs.create_file(
        "src/App.jsx",
        r#"export const utility = <div className="animate-in flex" />;
export const prefix = <div className="tw-animate-in flex" />;
export const theme = <div className="bg-brand flex" />;
"#,
    );
    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(
            [
                "lint",
                "--write",
                "--unsafe",
                "--only=nursery/useTailwindSortedClasses",
                fs.cli_path(),
            ]
            .as_slice(),
        ),
    );
    let output = std::fs::read_to_string(fs.working_directory.join("src/App.jsx")).unwrap();
    assert!(output.contains("className=\"flex animate-in\""));
    assert!(output.contains("className=\"flex tw-animate-in\""));
    assert!(output.contains("className=\"flex bg-brand\""));
    assert!(result.is_ok());
    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "use_tailwind_sorted_classes_reads_package_imports",
        fs.create_mem(),
        console,
        result,
    ));
}
