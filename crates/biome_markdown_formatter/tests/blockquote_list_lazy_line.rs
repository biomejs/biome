//! Checks that formatting keeps the meaning of lazy lines in list items
//! opened inside a block quote, in every prose wrap mode.
//!
//! Such a line lacks the quote marker, so it either ends the paragraph or
//! continues it lazily. Printing it with the list item's prefix or
//! indentation can turn paragraph text into a new block. Spec snapshots run in
//! one mode and don't compare meaning, so this test renders the source and the
//! output with the parser's HTML renderer and compares them, then checks that
//! formatting the output again changes nothing. The parser fixtures
//! `quote_list_unprefixed_line_*` check that renderer against commonmark.js.

use biome_markdown_formatter::{
    MdFormatLanguage,
    context::{MdFormatOptions, ProseWrap},
};
use biome_markdown_parser::{MarkdownParserOptions, document_to_html, parse_markdown};
use biome_markdown_syntax::MdQuote;
use biome_rowan::AstNode;

/// Renders `source` to HTML, collapsing ordinary text whitespace so a soft
/// break compares equal to the space that replaces it when a line is joined.
fn render(source: &str) -> String {
    let parsed = parse_markdown(source, MarkdownParserOptions::default());
    normalize_html(&document_to_html(
        &parsed.tree(),
        parsed.list_tightness(),
        parsed.list_item_indents(),
        parsed.quote_indents(),
    ))
}

/// Collapses ordinary text whitespace before the first whitespace-sensitive
/// element. Markup stays verbatim; a protected element or incomplete tag keeps
/// the remaining HTML unchanged rather than guessing where normal text resumes.
fn normalize_html(mut html: &str) -> String {
    let mut normalized = String::new();
    while !html.is_empty() {
        if html.starts_with('<') {
            let end = if html.starts_with("<!--") {
                html.find("-->").map(|end| end + 3)
            } else {
                let mut quote = None;
                html.char_indices().find_map(|(index, character)| {
                    if quote == Some(character) {
                        quote = None;
                    } else if quote.is_none() {
                        match character {
                            '\'' | '"' => quote = Some(character),
                            '>' => return Some(index + 1),
                            _ => {}
                        }
                    }
                    None
                })
            };
            let Some(end) = end else {
                normalized.push_str(html);
                break;
            };
            let (tag, remaining) = html.split_at(end);
            normalized.push_str(tag);
            html = remaining;

            let name = tag
                .strip_prefix('<')
                .unwrap()
                .split(|character: char| !character.is_ascii_alphanumeric())
                .next()
                .unwrap();
            if ["pre", "code", "script", "style", "textarea"]
                .iter()
                .any(|protected| name.eq_ignore_ascii_case(protected))
            {
                normalized.push_str(html);
                break;
            }
        } else {
            let end = html.find('<').unwrap_or(html.len());
            let mut was_whitespace = false;
            for character in html[..end].chars() {
                let is_whitespace = matches!(character, ' ' | '\t' | '\n' | '\r' | '\x0c');
                if !is_whitespace || !was_whitespace {
                    normalized.push(if is_whitespace { ' ' } else { character });
                }
                was_whitespace = is_whitespace;
            }
            html = &html[end..];
        }
    }
    normalized
}

#[test]
fn html_comparison_preserves_significant_whitespace() {
    assert_eq!(render("a\nb\n"), render("a b\n"));
    for (left, right) in [
        ("a\u{a0}b\n", "a b\n"),
        ("`a  b`\n", "`a b`\n"),
        ("```\na  b\n```\n", "```\na b\n```\n"),
        ("[a](url \"a  > b\")\n", "[a](url \"a > b\")\n"),
    ] {
        assert_ne!(render(left), render(right), "{left:?}: {right:?}");
    }
    for html in [
        "<!-- a  > b -->",
        "<pre>a  b\n</pre>",
        "<pre><a title=\"</pre>\">a  b</a></pre>",
        "<pre><!-- </pre> -->a  b</pre>",
        "<code>a  b</code><p>c\nd</p>",
        "<code>a  b</code>",
        "<SCRIPT>a  b</SCRIPT>",
        "<style>a  b</style>",
        "<textarea>a  b</textarea>",
        "<a title=\"a >  b",
    ] {
        assert_eq!(normalize_html(html), html);
    }
    assert_eq!(
        normalize_html("<a title=\"a >  b\">a\nb</a>"),
        "<a title=\"a >  b\">a b</a>"
    );
    assert_eq!(normalize_html("a <em>b</em> c"), "a <em>b</em> c");
}

fn format(source: &str, mode: ProseWrap) -> String {
    let parsed = parse_markdown(source, MarkdownParserOptions::default());
    biome_formatter::format_node(
        &parsed.syntax(),
        MdFormatLanguage::new(MdFormatOptions::default().with_prose_wrap(mode)),
        false,
    )
    .unwrap()
    .print()
    .unwrap()
    .as_code()
    .to_owned()
}

#[test]
fn lazy_lines_keep_meaning() {
    for source in [
        "> - a\n  - b\n",
        "> - a\n    - b\n",
        "> - a\n  ==\n",
        "> - a\n    <div>\n",
        "> - *a\n    - b*\n",
        "> - a\nb\n",
        "> 1. a\n    >\ntail\n",
        "> - a\n    *b\\\n    - c*\n",
        "> - a\n1.\n",
        "- > - a\n1.\n",
        "- > - > - a\n    2. b\n",
        "- > - > - a\n    - b\n",
        "-\t> - > - a\n\t  2. b\n",
        "- - > - > - a\n      2. b\n",
        "- > - > - a\n      2. b\n",
        "> - a\n    2. b\n",
        "> - a  \n    - b\n",
        "> - a\\\n    - b\n> c\n",
        "> - [a\n    b\n](x)c\n",
        "* > - a\\\n      b\n* c\n",
        "> - - - a\\\nb\n",
        "> -\t-\t- a\\\n    b\n",
        "* > - a\\\n      b\n- c\n",
        "* a\n- b\n* > - c\\\n      d\n",
        "- a\n* b\n\nc\n\n- > - d\\\n      e\n* f\n",
        "1) > - a\\\n       b\n2) c\n",
        "1) > - a\\\n       b\n2. c\n",
        "9. a\n9. > - a\\\n       - b\n",
        "009) > - a\\\n         b\n010) c\n",
        "> - a\\\n    [b\\\n    c](x)\\\n    [b\\\n    c](x)\\\n    d\n",
    ] {
        for mode in [ProseWrap::Preserve, ProseWrap::Always, ProseWrap::Never] {
            let output = format(source, mode);
            assert_eq!(render(source), render(&output), "{mode}: {source:?}");
            assert_eq!(output, format(&output, mode), "{mode}: {source:?}");
        }
    }
}

#[test]
fn lazy_line_after_hard_break_keeps_meaning_in_subtree() {
    for padding in ["x".repeat(80), "é".repeat(80)] {
        let source = format!("{padding}\n\n> - a\\\n    - b\n");
        let parsed = parse_markdown(&source, MarkdownParserOptions::default());
        let quote = parsed
            .syntax()
            .descendants()
            .find_map(MdQuote::cast)
            .unwrap();
        for mode in [ProseWrap::Preserve, ProseWrap::Always, ProseWrap::Never] {
            let printed = biome_formatter::format_sub_tree(
                quote.syntax(),
                MdFormatLanguage::new(MdFormatOptions::default().with_prose_wrap(mode)),
            )
            .unwrap();
            assert_eq!(printed.as_code(), "> - a\\\n    - b\n");
        }
    }
}

#[test]
fn indented_partial_quote_prefixes_keep_source_columns() {
    // Partial-prefix parsing can split inline markup, so source preservation
    // is checked independently of the parser's HTML renderer.
    for (source, expected) in [
        ("> > - a\n  >\n\n", "> > - a\n  >\n"),
        ("> > - a\n   >\n\n", "> > - a\n   >\n"),
        ("> > - *a\n  > b*\n", "> > - *a\n  > b*\n"),
        ("> > - *a\n   > b*\n", "> > - *a\n   > b*\n"),
        ("> - > - *a\n  > b*\n", "> - > - *a\n  > b*\n"),
        ("> > - `a\n  > b`\n", "> > - `a\n  > b`\n"),
        ("> > - *a  \n  > b*\n", "> > - *a  \n  > b*\n"),
        ("> - > - a\n>   >\n    c*\n", "> - > - a\n>   >\n    c*\n"),
        (
            "> - > - a\n>     >\n    c*\n",
            "> - > - a\n>     >\n    c*\n",
        ),
        ("1. > - a\n  >\n> c\n", "1. > - a\n  >\n> c\n"),
        ("1. > - a\n  >\n> tail\n", "1. > - a\n  >\n> tail\n"),
        ("009) > - a\n   >\n> c\n", "009) > - a\n   >\n> c\n"),
        ("009) > + a\n   > b\n", "009) > + a\n   > b\n"),
        (
            "* > > - *a\n    > b*\n- c\n",
            "* > > - *a\n    > b*\n\n- c\n",
        ),
    ] {
        for mode in [ProseWrap::Preserve, ProseWrap::Always, ProseWrap::Never] {
            let output = format(source, mode);
            assert_eq!(output, expected, "{mode}: {source:?}");
            assert_eq!(output, format(&output, mode), "{mode}: {source:?}");
        }
    }
}

#[test]
fn fully_prefixed_quoted_lists_still_format() {
    for (source, expected) in [
        ("> > + a\n> >   b\n", "> > - a\n> >   b\n"),
        ("> + a\n>   b\n", "> - a\n>   b\n"),
        ("+ > > + a\n  > >   b\n", "- > > - a\n  > >   b\n"),
        ("- > - a\n  >   \n- b\n", "- > - a\n- b\n"),
        ("1. > 1. a\n   >      \n2. b\n", "1. > 1. a\n2. b\n"),
    ] {
        for mode in [ProseWrap::Preserve, ProseWrap::Always, ProseWrap::Never] {
            let output = format(source, mode);
            assert_eq!(output, expected, "{mode}: {source:?}");
            assert_eq!(render(source), render(&output), "{mode}: {source:?}");
            assert_eq!(output, format(&output, mode), "{mode}: {source:?}");
        }
    }
}
