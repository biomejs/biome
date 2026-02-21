use crate::snap_test::{SnapshotPayload, assert_cli_snapshot};
use crate::{run_cli, run_cli_with_dyn_fs};
use biome_console::BufferConsole;
use biome_fs::{MemoryFileSystem, TemporaryFs};
use bpaf::Args;
use camino::Utf8Path;

#[test]
fn should_not_format_files_by_default() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"html {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", css_file.as_str()].as_slice()),
    );

    // no files processed error
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_not_format_files_by_default",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_format_files_by_when_opt_in() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"html {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(["format", "--css-formatter-enabled=true", css_file.as_str()].as_slice()),
    );

    // not formatted error
    assert!(result.is_err(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_files_by_when_opt_in",
        fs,
        console,
        result,
    ));
}

#[test]
fn should_format_write_files_by_when_opt_in() {
    let fs = MemoryFileSystem::default();
    let mut console = BufferConsole::default();

    let css_file_content = r#"html {}"#;
    let css_file = Utf8Path::new("input.css");
    fs.insert(css_file.into(), css_file_content.as_bytes());

    let (fs, result) = run_cli(
        fs,
        &mut console,
        Args::from(
            [
                "format",
                "--write",
                "--css-formatter-enabled=true",
                css_file.as_str(),
            ]
            .as_slice(),
        ),
    );

    assert!(result.is_ok(), "run_cli returned {result:?}");

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "should_format_write_files_by_when_opt_in",
        fs,
        console,
        result,
    ));
}

// ── noUnusedStyles ────────────────────────────────────────────────────────────

/// Referenced classes produce no diagnostics, regardless of attribute name
/// (`className` vs `class=`) or file extension (`.jsx` / `.tsx`).
#[test]
fn no_unused_styles_referenced_class_not_flagged() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_referenced_class_not_flagged");

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
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_referenced_class_not_flagged",
        fs.create_mem(),
        console,
        result,
    ));
}

/// Unreferenced classes are flagged; referenced classes in the same file are
/// not. Tests single-class files, mixed used/unused, and multi-class strings.
#[test]
fn no_unused_styles_unreferenced_class_flagged() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_unreferenced_class_flagged");

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
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_unreferenced_class_flagged",
        fs.create_mem(),
        console,
        result,
    ));
}

/// Dynamic `className={"foo"}` (JSX expression, not a string literal) is not
/// statically collected — the class is treated as unreferenced and flagged.
#[test]
fn no_unused_styles_dynamic_classname_not_collected() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_dynamic_classname_not_collected");

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
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_dynamic_classname_not_collected",
        fs.create_mem(),
        console,
        result,
    ));
}

/// Classes inside `:global(.foo)` are never flagged regardless of references,
/// because they are globally scoped (CSS Modules convention).
#[test]
fn no_unused_styles_global_pseudo_class_is_exempt() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_global_pseudo_class_is_exempt");

    fs.create_file(
        "styles.css",
        ":global(.reset) { margin: 0; } :global(.base) { padding: 0; }",
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_global_pseudo_class_is_exempt",
        fs.create_mem(),
        console,
        result,
    ));
}

/// CSS `@import` chain: `App.jsx → theme.css → base.css`. Classes in
/// `theme.css` are used (App.jsx directly imports it). Classes in `base.css`
/// are NOT resolved through the transitive `@import` — this is a known
/// limitation documented by the snapshot.
#[test]
fn no_unused_styles_transitive_css_import() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_transitive_css_import");

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
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_transitive_css_import",
        fs.create_mem(),
        console,
        result,
    ));
}

/// All selector pattern variants where every class is referenced → no
/// diagnostics. Covers: compound (`.btn.active`), child combinator
/// (`.list > .item`), element-qualified (`div.container`), `:is()` pseudo,
/// and pseudo-element/state variants (`.btn:hover`, `.btn::before`).
#[test]
fn no_unused_styles_selector_patterns_all_referenced() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_selector_patterns_all_referenced");

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
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_selector_patterns_all_referenced",
        fs.create_mem(),
        console,
        result,
    ));
}

/// Selector pattern variants where only some classes are referenced — the
/// unreferenced ones are flagged. Covers: descendant combinator (`.parent`
/// unused), selector list (`.bar` unused), and `@media` block (`.hidden`
/// unused).
#[test]
fn no_unused_styles_selector_patterns_partial_reference() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_selector_patterns_partial_reference");

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
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_selector_patterns_partial_reference",
        fs.create_mem(),
        console,
        result,
    ));
}

/// A class referenced in an HTML file's `class=` attribute via a linked
/// stylesheet. Requires `experimentalFullSupportEnabled` so HTML files are
/// parsed as HTML and their `class=` attributes are walked by the module-graph
/// visitor.
///
/// Note: whether the CSS class is considered "used" depends on scan order.
/// This snapshot documents the current behavior.
#[test]
fn no_unused_styles_html_consumer() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_html_consumer");

    fs.create_file(
        "biome.json",
        r#"{ "html": { "experimentalFullSupportEnabled": true } }"#,
    );
    // Single HTML consumer
    fs.create_file("styles.css", ".hero { font-size: 2rem; }");
    fs.create_file(
        "index.html",
        r#"<!DOCTYPE html>
<html>
<head><link rel="stylesheet" href="./styles.css" /></head>
<body><h1 class="hero">Hello</h1></body>
</html>
"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_html_consumer",
        fs.create_mem(),
        console,
        result,
    ));
}

/// A CSS file imported by both a JSX file and an HTML file. The JSX consumer
/// covers `.header`; the HTML consumer covers `.footer`.
///
/// Note: whether HTML-referenced classes suppress CSS diagnostics depends on
/// scan order. This snapshot documents the current behavior.
#[test]
fn no_unused_styles_mixed_html_jsx_consumers() {
    let mut console = BufferConsole::default();
    let mut fs = TemporaryFs::new("no_unused_styles_mixed_html_jsx_consumers");

    fs.create_file(
        "biome.json",
        r#"{ "html": { "experimentalFullSupportEnabled": true } }"#,
    );
    fs.create_file(
        "styles.css",
        ".header { font-weight: bold; } .footer { font-size: 0.8rem; }",
    );
    fs.create_file(
        "App.jsx",
        r#"import "./styles.css";
export default () => <div className="header" />;
"#,
    );
    fs.create_file(
        "index.html",
        r#"<link rel="stylesheet" href="./styles.css" /><footer class="footer">end</footer>"#,
    );

    let result = run_cli_with_dyn_fs(
        Box::new(fs.create_os()),
        &mut console,
        Args::from(["lint", "--only=nursery/noUnusedStyles", fs.cli_path()].as_slice()),
    );

    assert_cli_snapshot(SnapshotPayload::new(
        module_path!(),
        "no_unused_styles_mixed_html_jsx_consumers",
        fs.create_mem(),
        console,
        result,
    ));
}
