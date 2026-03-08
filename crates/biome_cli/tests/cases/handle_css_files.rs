//! CLI integration tests for CSS lint rules.
//!
//! This file tests the `noUnusedClasses` rule behavior in real-world scenarios:
//! - Referenced vs unreferenced classes
//! - Dynamic className expressions
//! - Global pseudo-class exemptions
//! - Transitive CSS imports via @import
//! - Complex selector patterns
//! - Component hierarchies (parent imports CSS, child uses classes)
//! - Mixed HTML and JSX consumers

use crate::run_cli_with_dyn_fs;
use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use biome_console::BufferConsole;
use biome_fs::TemporaryFs;
use bpaf::Args;

#[test]
fn no_unused_classes_referenced_class_not_flagged() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_referenced_class_not_flagged");

    // .button referenced via className= (JSX)
    // .card referenced via className= (TSX)
    // .solid referenced via class= (SolidJS-style JSX)
    fs.create_file(
        "styles.css",
        ".button { color: blue; } .card { border-radius: 8px; } .solid { margin: 0; }",
    );
    fs.create_file(
        "App.jsx",
        r#"import "./styles.css";
export default () => <div className="button" />;
"#,
    );
    fs.create_file(
        "Card.tsx",
        r#"import "./styles.css";
export default function Card() {
    return <div className="card">content</div>;
}
"#,
    );
    fs.create_file(
        "Solid.jsx",
        r#"import "./styles.css";
export default () => <div class="solid" />;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_referenced_class_not_flagged",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_unreferenced_class_flagged() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_unreferenced_class_flagged");

    // .used and .primary are referenced; .orphan and .ghost are not
    fs.create_file(
        "styles.css",
        ".used { color: green; } .orphan { color: red; } .primary { color: blue; } .ghost { opacity: 0.5; }",
    );
    fs.create_file(
        "App.jsx",
        r#"import "./styles.css";
export default () => <div className="used primary" />;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_unreferenced_class_flagged",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_dynamic_classname_not_collected() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_dynamic_classname_not_collected");

    fs.create_file("styles.css", ".button { color: blue; }");
    fs.create_file(
        "App.jsx",
        // className={"button"} — JSX expression, not a plain string literal
        r#"import "./styles.css";
export default () => <div className={"button"} />;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_dynamic_classname_not_collected",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_global_pseudo_class_is_exempt() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_global_pseudo_class_is_exempt");

    fs.create_file(
        "styles.css",
        ":global(.reset) { margin: 0; } :global(.base) { padding: 0; }",
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_global_pseudo_class_is_exempt",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_transitive_css_import() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_transitive_css_import");

    fs.create_file("base.css", ".base { box-sizing: border-box; }");
    fs.create_file(
        "theme.css",
        r#"@import "./base.css"; .theme { background: white; }"#,
    );
    fs.create_file(
        "App.jsx",
        r#"import "./theme.css";
export default () => <div className="base theme" />;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_transitive_css_import",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_selector_patterns_all_referenced() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_selector_patterns_all_referenced");

    fs.create_file(
        "styles.css",
        concat!(
            ".btn.active { background: blue; } ",
            ".list > .item { padding: 4px; } ",
            "div.container { max-width: 1200px; } ",
            ":is(.alert, .warning) { border: 1px solid; } ",
            ".btn:hover { color: darkblue; } .btn::before { content: ''; }",
        ),
    );
    fs.create_file(
        "App.jsx",
        r#"import "./styles.css";
export default () => (
  <div className="btn active container alert warning">
    <ul className="list"><li className="item" /></ul>
  </div>
);
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_selector_patterns_all_referenced",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_selector_patterns_partial_reference() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_selector_patterns_partial_reference");

    fs.create_file(
        "styles.css",
        concat!(
            ".parent .child { color: red; } ",
            ".foo, .bar { margin: 0; } ",
            "@media (max-width: 768px) { .mobile { display: block; } .hidden { display: none; } }",
        ),
    );
    fs.create_file(
        "App.jsx",
        r#"import "./styles.css";
export default () => <div className="child foo mobile" />;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_selector_patterns_partial_reference",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_component_hierarchy_parent_imports_css() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_component_hierarchy_parent_imports_css");

    // Real-world pattern: Parent imports CSS and child component
    // Child component uses classes from that CSS
    fs.create_file(
        "button.css",
        ".btn { padding: 8px; } .btn-primary { color: blue; }",
    );
    fs.create_file(
        "Button.jsx",
        r#"// Button component uses classes but doesn't import CSS
export default () => <button className="btn btn-primary">Click</button>;
"#,
    );
    fs.create_file(
        "App.jsx",
        r#"import "./button.css";
import Button from "./Button.jsx";
export default () => <div><Button /></div>;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_component_hierarchy_parent_imports_css",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_component_hierarchy_child_imports_css() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_component_hierarchy_child_imports_css");

    // Child component imports its own CSS
    fs.create_file(
        "button.css",
        ".btn { padding: 8px; } .unused { margin: 0; }",
    );
    fs.create_file(
        "Button.jsx",
        r#"import "./button.css";
export default () => <button className="btn">Click</button>;
"#,
    );
    fs.create_file(
        "App.jsx",
        r#"import Button from "./Button.jsx";
export default () => <div><Button /></div>;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_component_hierarchy_child_imports_css",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_unused_classes_component_hierarchy_nested_three_levels() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_classes_component_hierarchy_nested_three_levels");

    // Deep nesting: App → Page → Button (App imports all CSS)
    fs.create_file(
        "styles.css",
        ".container { width: 100%; } .btn { padding: 8px; }",
    );
    fs.create_file(
        "Button.jsx",
        r#"export default () => <button className="btn">Click</button>;"#,
    );
    fs.create_file(
        "Page.jsx",
        r#"import Button from "./Button.jsx";
export default () => <div className="container"><Button /></div>;
"#,
    );
    fs.create_file(
        "App.jsx",
        r#"import "./styles.css";
import Page from "./Page.jsx";
export default () => <Page />;
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_classes_component_hierarchy_nested_three_levels",
        fs.create_mem(),
        console,
        result,
    ));
}

#[test]
fn no_undeclared_classes_shows_checked_import_chain() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_undeclared_classes_shows_checked_import_chain");

    fs.create_file(
        "button.css",
        ".btn { background: blue; } .btn-primary { color: white; }",
    );
    fs.create_file(
        "Button.jsx",
        r#"import "./button.css";
export default () => <button className="btn btn-undefined">Click</button>;"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUndeclaredClasses", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_undeclared_classes_shows_checked_import_chain",
        fs.create_mem(),
        console,
        result,
    ));
}
