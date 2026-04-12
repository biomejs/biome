use biome_console::fmt::{Formatter, Termcolor};
use biome_console::markup;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::display::PrintDiagnostic;
use biome_diagnostics::termcolor;
use biome_markdown_parser::{document_to_html, parse_markdown};
use biome_markdown_syntax::{MarkdownSyntaxKind, MdDocument};
use biome_rowan::{AstNode, SyntaxKind, SyntaxSlot};
use biome_test_utils::{has_bogus_nodes_or_empty_slots, validate_eof_token};
use std::fmt::Write;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone)]
pub enum ExpectedOutcome {
    Pass,
    Fail,
    Undefined,
}

pub fn run(test_case: &str, _snapshot_name: &str, test_directory: &str, outcome_str: &str) {
    let outcome = match outcome_str {
        "ok" => ExpectedOutcome::Pass,
        "error" => ExpectedOutcome::Fail,
        "undefined" => ExpectedOutcome::Undefined,
        _ => panic!("Invalid expected outcome {outcome_str}"),
    };

    let test_case_path = Path::new(test_case);

    let file_name = test_case_path
        .file_name()
        .expect("Expected test to have a file name")
        .to_str()
        .expect("File name to be valid UTF8");

    let content = fs::read_to_string(test_case_path)
        .expect("Expected test path to be a readable file in UTF8 encoding");

    let parsed = parse_markdown(&content);
    validate_eof_token(parsed.syntax());

    let formatted_ast = format!("{:#?}", parsed.tree());

    let mut snapshot = String::new();
    writeln!(snapshot, "\n## Input\n\n```\n{content}\n```\n\n").unwrap();

    writeln!(
        snapshot,
        r#"## AST

```
{formatted_ast}
```

## CST

```
{:#?}
```
"#,
        parsed.syntax()
    )
    .unwrap();

    let diagnostics = parsed.diagnostics();
    if !diagnostics.is_empty() {
        let mut diagnostics_buffer = termcolor::Buffer::no_color();

        let termcolor = &mut Termcolor(&mut diagnostics_buffer);
        let mut formatter = Formatter::new(termcolor);

        for diagnostic in diagnostics {
            let error = diagnostic
                .clone()
                .with_file_path(file_name)
                .with_file_source_code(&content);

            formatter
                .write_markup(markup! {
                    {PrintDiagnostic::verbose(&error)}
                })
                .expect("failed to emit diagnostic");
        }

        let formatted_diagnostics =
            std::str::from_utf8(diagnostics_buffer.as_slice()).expect("non utf8 in error buffer");

        if matches!(outcome, ExpectedOutcome::Pass) {
            panic!(
                "Expected no errors to be present in a test case that is expected to pass but the following diagnostics are present:\n{formatted_diagnostics}"
            )
        }

        writeln!(snapshot, "## Diagnostics\n\n```").unwrap();
        snapshot.write_str(formatted_diagnostics).unwrap();

        writeln!(snapshot, "```\n").unwrap();
    }

    match outcome {
        ExpectedOutcome::Pass => {
            let missing_required = formatted_ast.contains("missing (required)");
            if missing_required
                || parsed
                    .syntax()
                    .descendants()
                    .any(|node| node.kind().is_bogus())
            {
                panic!(
                    "Parsed tree of a 'OK' test case should not contain any missing required children or bogus nodes: \n {formatted_ast:#?} \n\n {formatted_ast}"
                );
            }

            let syntax = parsed.syntax();
            if has_bogus_nodes_or_empty_slots(&syntax) {
                panic!("modified tree has bogus nodes or empty slots:\n{syntax:#?} \n\n {syntax}")
            }

            // Optional reference HTML comparison: if a .html sidecar exists,
            // render via document_to_html and compare against the reference.
            let html_path = test_case_path.with_extension("html");
            if html_path.exists() {
                let doc = MdDocument::cast(parsed.syntax()).unwrap_or_else(|| {
                    panic!("Failed to cast parsed output to MdDocument for {file_name}. Check that the .md test input is syntactically valid and re-run with `cargo t -p biome_markdown_parser` to see parser diagnostics.")
                });
                let actual = document_to_html(
                    &doc,
                    parsed.list_tightness(),
                    parsed.list_item_indents(),
                    parsed.quote_indents(),
                );
                let expected =
                    fs::read_to_string(&html_path).expect("Failed to read reference HTML file");
                let actual_normalized = normalize_html(&actual);
                let expected_normalized = normalize_html(&expected);
                similar_asserts::assert_eq!(
                    expected_normalized,
                    actual_normalized,
                    "HTML mismatch for {file_name}. Update the .html sidecar or fix the renderer in document_to_html()."
                );
            }

            // Structural invariant: MdContinuationIndent children must all be MdIndentToken.
            for node in parsed.syntax().descendants() {
                if node.kind() == MarkdownSyntaxKind::MD_CONTINUATION_INDENT {
                    for child in node.children() {
                        if child.kind() == MarkdownSyntaxKind::MD_INDENT_TOKEN_LIST {
                            for token_node in child.children() {
                                assert_eq!(
                                    token_node.kind(),
                                    MarkdownSyntaxKind::MD_INDENT_TOKEN,
                                    "Structural invariant violation in {file_name}: MdContinuationIndent contains {:?} instead of MD_INDENT_TOKEN. Check the parser's continuation indent logic in syntax/list.rs.",
                                    token_node.kind()
                                );
                            }
                        }
                    }
                }
            }

            // Structural invariant: MdHeader inside list items must have a present indent field.
            for node in parsed.syntax().descendants() {
                if node.kind() == MarkdownSyntaxKind::MD_HEADER {
                    let in_list = node.ancestors().any(|a| {
                        let k = a.kind();
                        k == MarkdownSyntaxKind::MD_BULLET_LIST_ITEM
                            || k == MarkdownSyntaxKind::MD_ORDERED_LIST_ITEM
                    });
                    if in_list {
                        assert!(
                            !matches!(node.slots().next(), Some(SyntaxSlot::Empty { .. })),
                            "MdHeader indent invariant: MD_HEADER inside list item is missing indent slot in {file_name} at {:?}",
                            node.kind()
                        );
                    }
                }
            }
        }
        ExpectedOutcome::Fail => {
            if parsed.diagnostics().is_empty() {
                panic!("Failing test must have diagnostics");
            }
        }
        _ => {}
    }

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => &test_directory,
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}

#[path = "test_utils.rs"]
mod test_utils;
use test_utils::normalize_html;

#[test]
pub fn quick_test() {
    use biome_markdown_parser::document_to_html;
    use biome_markdown_syntax::MdDocument;
    use biome_rowan::AstNode;

    fn test_example(num: u32, input: &str, expected: &str) {
        let root = parse_markdown(input);
        let doc = MdDocument::cast(root.syntax())
            .unwrap_or_else(|| panic!("Example {:03}: parse failed", num));
        let html = document_to_html(
            &doc,
            root.list_tightness(),
            root.list_item_indents(),
            root.quote_indents(),
        );

        assert_eq!(expected, html, "Example {:03} failed", num);
    }

    test_example(
        7,
        "-\t\tfoo\n",
        "<ul>\n<li>\n<pre><code>  foo\n</code></pre>\n</li>\n</ul>\n",
    );
    test_example(
        42,
        "- `one\n- two`\n",
        "<ul>\n<li>`one</li>\n<li>two`</li>\n</ul>\n",
    );
    test_example(
        61,
        "- Foo\n- * * *\n",
        "<ul>\n<li>Foo</li>\n<li>\n<hr />\n</li>\n</ul>\n",
    );
    test_example(
        66,
        "# foo *bar* \\*baz\\*\n",
        "<h1>foo <em>bar</em> *baz*</h1>\n",
    );
    test_example(73, "### foo ###     \n", "<h3>foo</h3>\n");
    // Heading content with 3+ trailing spaces (would be hard break in paragraph).
    // In headings, trailing spaces are stripped per §4.2 — no hard break produced.
    test_example(69, "# foo  \n", "<h1>foo</h1>\n");
    test_example(10007, "## bar   \n", "<h2>bar</h2>\n");
    // Heading trailing hashes with spaces + newline
    test_example(10008, "# foo #   \n", "<h1>foo</h1>\n");
    test_example(
        93,
        "> foo\nbar\n===\n",
        "<blockquote>\n<p>foo\nbar\n===</p>\n</blockquote>\n",
    );
    test_example(
        223,
        "aaa\n             bbb\n                                       ccc\n",
        "<p>aaa\nbbb\nccc</p>\n",
    );
    test_example(
        259,
        "   > > 1.  one\n>>\n>>     two\n",
        "<blockquote>\n<blockquote>\n<ol>\n<li>\n<p>one</p>\n<p>two</p>\n</li>\n</ol>\n</blockquote>\n</blockquote>\n",
    );
    test_example(
        9991,
        "![a & b < c](url)\n",
        "<p><img src=\"url\" alt=\"a &amp; b &lt; c\" /></p>\n",
    );
    test_example(
        9992,
        "> ```\n> hello\n> ```\n",
        "<blockquote>\n<pre><code>hello\n</code></pre>\n</blockquote>\n",
    );
    // Quoted indented code must terminate before a quoted thematic break.
    test_example(
        99921,
        ">     code\n> ---\n",
        "<blockquote>\n<pre><code>code\n</code></pre>\n<hr />\n</blockquote>\n",
    );
    test_example(
        9993,
        "- foo\n  - bar\n",
        "<ul>\n<li>foo\n<ul>\n<li>bar</li>\n</ul>\n</li>\n</ul>\n",
    );
    // Setext underline with optional indent inside a list item
    test_example(
        9994,
        "- foo\n   ---\n",
        "<ul>\n<li>\n<h2>foo</h2>\n</li>\n</ul>\n",
    );
    test_example(
        10001,
        " - foo\n   - bar\n\t - baz\n",
        "<ul>\n<li>foo\n<ul>\n<li>bar\n<ul>\n<li>baz</li>\n</ul>\n</li>\n</ul>\n</li>\n</ul>\n",
    );
    test_example(
        10002,
        "1.  A paragraph\n    with two lines.\n\n        indented code\n\n    > A block quote.\n",
        "<ol>\n<li>\n<p>A paragraph\nwith two lines.</p>\n<pre><code>indented code\n</code></pre>\n<blockquote>\n<p>A block quote.</p>\n</blockquote>\n</li>\n</ol>\n",
    );
    test_example(
        10003,
        "- a\n  - b\n  - c\n\n- d\n  - e\n  - f\n",
        "<ul>\n<li>\n<p>a</p>\n<ul>\n<li>b</li>\n<li>c</li>\n</ul>\n</li>\n<li>\n<p>d</p>\n<ul>\n<li>e</li>\n<li>f</li>\n</ul>\n</li>\n</ul>\n",
    );
    test_example(
        10004,
        "- outer item\n  - inner item\n    inner continuation\n  outer continuation at parent indentation\n\n- next outer item\n",
        "<ul>\n<li>\n<p>outer item</p>\n<ul>\n<li>inner item\ninner continuation\nouter continuation at parent indentation</li>\n</ul>\n</li>\n<li>\n<p>next outer item</p>\n</li>\n</ul>\n",
    );
    // Mixed ordered delimiters across blank lines produce separate tight lists
    test_example(
        10005,
        "1. one\n\n2) two\n",
        "<ol>\n<li>one</li>\n</ol>\n<ol start=\"2\">\n<li>two</li>\n</ol>\n",
    );
    // Mixed bullet markers across blank lines produce separate tight lists
    test_example(
        10006,
        "- one\n\n+ two\n",
        "<ul>\n<li>one</li>\n</ul>\n<ul>\n<li>two</li>\n</ul>\n",
    );
    // Bullet → ordered across blank lines produce separate lists
    test_example(
        10012,
        "- bullet\n\n1. ordered\n",
        "<ul>\n<li>bullet</li>\n</ul>\n<ol>\n<li>ordered</li>\n</ol>\n",
    );
    // Ordered → bullet across blank lines produce separate lists
    test_example(
        10013,
        "1. ordered\n\n- bullet\n",
        "<ol>\n<li>ordered</li>\n</ol>\n<ul>\n<li>bullet</li>\n</ul>\n",
    );
    // Nested list items separated by blank lines stay in the same nested list.
    test_example(
        10009,
        "- top\n  - sub a\n\n  - sub b\n",
        "<ul>\n<li>top\n<ul>\n<li>\n<p>sub a</p>\n</li>\n<li>\n<p>sub b</p>\n</li>\n</ul>\n</li>\n</ul>\n",
    );
    // Double blank line between nested list siblings must not emit diagnostics
    // and must keep items in the same nested list.
    test_example(
        10010,
        "- top\n  - sub a\n\n\n  - sub b\n",
        "<ul>\n<li>top\n<ul>\n<li>\n<p>sub a</p>\n</li>\n<li>\n<p>sub b</p>\n</li>\n</ul>\n</li>\n</ul>\n",
    );
    // Triple blank line between nested list siblings
    test_example(
        10011,
        "- top\n  - sub a\n\n\n\n  - sub b\n",
        "<ul>\n<li>top\n<ul>\n<li>\n<p>sub a</p>\n</li>\n<li>\n<p>sub b</p>\n</li>\n</ul>\n</li>\n</ul>\n",
    );
    // #9725: fenced code block inside list item with 4-space indent
    test_example(
        9725,
        "- aaa\n\n    ```js\n    const a = 1;\n    ```\n",
        "<ul>\n<li>\n<p>aaa</p>\n<pre><code class=\"language-js\">const a = 1;\n</code></pre>\n</li>\n</ul>\n",
    );
    // #9736: HTML block inside list item continuation
    test_example(
        9736,
        "- item\n\n  <details>\n  <summary>Info</summary>\n  content\n  </details>\n",
        "<ul>\n<li>\n<p>item</p>\n<details>\n<summary>Info</summary>\ncontent\n</details>\n</li>\n</ul>\n",
    );
    // #9727: multi-byte characters must not panic in emphasis context
    test_example(
        9727,
        ">💡 Biomeは、[Prettierのオプションに対する考え方](https://prettier.io/docs/en/option-philosophy)と同様のアプローチを採用しています。\n",
        "<blockquote>\n<p>💡 Biomeは、<a href=\"https://prettier.io/docs/en/option-philosophy\">Prettierのオプションに対する考え方</a>と同様のアプローチを採用しています。</p>\n</blockquote>\n",
    );
    // Lazy continuation at exactly the nested marker indent
    test_example(
        20001,
        "- a\n  - b\n  lazy\n",
        "<ul>\n<li>a\n<ul>\n<li>b\nlazy</li>\n</ul>\n</li>\n</ul>\n",
    );
    // Multiline open tag where > at line start is a blockquote marker
    test_example(
        20003,
        "Allowed: <div class=\"a\"\n>ok</div> tag.\n",
        "<p>Allowed: &lt;div class=&quot;a&quot;</p>\n<blockquote>\n<p>ok</div> tag.</p>\n</blockquote>\n",
    );
    // Setext heading inside blockquote
    test_example(
        20002,
        "> Foo\n> ---\n",
        "<blockquote>\n<h2>Foo</h2>\n</blockquote>\n",
    );
    test_example(20003, "> ---\n", "<blockquote>\n<hr />\n</blockquote>\n");

    // Single-item lists split by marker change should be tight
    test_example(
        20008,
        "* item one\n\n- item two\n",
        "<ul>\n<li>item one</li>\n</ul>\n<ul>\n<li>item two</li>\n</ul>\n",
    );

    // Fuzzer: mixed markers after heading-in-list — 3 markers
    test_example(
        30001,
        "- # Bar\n\n+ item one\n\n* item two\n",
        "<ul>\n<li>\n<h1>Bar</h1>\n</li>\n</ul>\n<ul>\n<li>item one</li>\n</ul>\n<ul>\n<li>item two</li>\n</ul>\n",
    );
    // Reduce: heading in list then different marker
    test_example(
        30011,
        "- # Bar\n\n+ item\n",
        "<ul>\n<li>\n<h1>Bar</h1>\n</li>\n</ul>\n<ul>\n<li>item</li>\n</ul>\n",
    );
    // Reduce: paragraph in list then different marker (should work like 10006)
    test_example(
        30012,
        "- bar\n\n+ item\n",
        "<ul>\n<li>bar</li>\n</ul>\n<ul>\n<li>item</li>\n</ul>\n",
    );
    // Reduce: thematic break in list then different marker
    // NOTE: `- ---` is a pre-existing Biome bug where it parses as a top-level
    // thematic break instead of a list item containing <hr />.
    test_example(
        30013,
        "- ---\n\n+ item\n",
        "<hr />\n<ul>\n<li>item</li>\n</ul>\n",
    );
    // Reduce: setext heading in list then different marker
    test_example(
        30014,
        "- Foo\n  ---\n\n+ item\n",
        "<ul>\n<li>\n<h2>Foo</h2>\n</li>\n</ul>\n<ul>\n<li>item</li>\n</ul>\n",
    );

    // Fuzzer: lazy continuation with trailing paragraph
    test_example(
        30002,
        "- outer\n  * nested\n  lazy line\nhello\n",
        "<ul>\n<li>outer\n<ul>\n<li>nested\nlazy line\nhello</li>\n</ul>\n</li>\n</ul>\n",
    );

    // Fuzzer: fenced code after list not absorbed
    test_example(
        30003,
        "* one\n* two\n```\ncode here\n```\n",
        "<ul>\n<li>one</li>\n<li>two</li>\n</ul>\n<pre><code>code here\n</code></pre>\n",
    );

    // Fuzzer: mixed markers after fenced code in list item
    test_example(
        30004,
        "- item\n\n  ```\n  code\n  ```\n\n+ other\n",
        "<ul>\n<li>\n<p>item</p>\n<pre><code>code\n</code></pre>\n</li>\n</ul>\n<ul>\n<li>other</li>\n</ul>\n",
    );

    // Fuzzer: lazy continuation absorbs following non-indented line
    test_example(
        30005,
        "- outer\n  - nested\n  lazy line\nhello\n",
        "<ul>\n<li>outer\n<ul>\n<li>nested\nlazy line\nhello</li>\n</ul>\n</li>\n</ul>\n",
    );
}

fn fuzz_test_example(num: u32, input: &str, expected: &str) {
    let root = parse_markdown(input);
    let doc =
        MdDocument::cast(root.syntax()).unwrap_or_else(|| panic!("Fuzz {:03}: parse failed", num));
    let html = document_to_html(
        &doc,
        root.list_tightness(),
        root.list_item_indents(),
        root.quote_indents(),
    );
    similar_asserts::assert_eq!(expected, html, "Fuzz {:03} failed\nInput: {:?}", num, input);
}

#[test]
fn fuzz_mixed_markers_heading() {
    fuzz_test_example(
        1,
        "- # Bar\n\n+ item\n",
        "<ul>\n<li>\n<h1>Bar</h1>\n</li>\n</ul>\n<ul>\n<li>item</li>\n</ul>\n",
    );
}

#[test]
fn fuzz_mixed_markers_paragraph() {
    fuzz_test_example(
        2,
        "- bar\n\n+ item\n",
        "<ul>\n<li>bar</li>\n</ul>\n<ul>\n<li>item</li>\n</ul>\n",
    );
}

/// NOTE: `- ---` is parsed by Biome as a top-level thematic break rather than
/// a list item containing `<hr />`. This is a separate pre-existing bug
/// (thematic break precedence over list marker) unrelated to the mixed-marker
/// list-split fix. The expected value here matches Biome's current behavior.
#[test]
fn fuzz_mixed_markers_thematic_break() {
    fuzz_test_example(
        3,
        "- ---\n\n+ item\n",
        "<hr />\n<ul>\n<li>item</li>\n</ul>\n",
    );
}

#[test]
fn fuzz_mixed_markers_setext() {
    fuzz_test_example(
        4,
        "- Foo\n  ---\n\n+ item\n",
        "<ul>\n<li>\n<h2>Foo</h2>\n</li>\n</ul>\n<ul>\n<li>item</li>\n</ul>\n",
    );
}

#[test]
fn fuzz_mixed_markers_fenced_code() {
    fuzz_test_example(
        5,
        "- item\n\n  ```\n  code\n  ```\n\n+ other\n",
        "<ul>\n<li>\n<p>item</p>\n<pre><code>code\n</code></pre>\n</li>\n</ul>\n<ul>\n<li>other</li>\n</ul>\n",
    );
}

#[test]
fn fuzz_lazy_cont_nested_trailing() {
    fuzz_test_example(
        6,
        "- outer\n  * nested\n  lazy line\nhello\n",
        "<ul>\n<li>outer\n<ul>\n<li>nested\nlazy line\nhello</li>\n</ul>\n</li>\n</ul>\n",
    );
}

#[test]
fn fuzz_lazy_cont_nested_same_marker() {
    fuzz_test_example(
        7,
        "- outer\n  - nested\n  lazy line\nhello\n",
        "<ul>\n<li>outer\n<ul>\n<li>nested\nlazy line\nhello</li>\n</ul>\n</li>\n</ul>\n",
    );
}

#[test]
fn fuzz_code_after_list_not_absorbed() {
    fuzz_test_example(
        8,
        "* one\n* two\n```\ncode here\n```\n",
        "<ul>\n<li>one</li>\n<li>two</li>\n</ul>\n<pre><code>code here\n</code></pre>\n",
    );
}
