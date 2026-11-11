use crate::{RuleDocumentItem, RuleSummaryFragment, parse_rule_document, parse_rule_summary};

fn first_code_block(source: &str) -> crate::RuleDocCodeBlock {
    parse_rule_document(source)
        .into_document()
        .into_items()
        .find_map(|item| match item {
            RuleDocumentItem::FencedCodeBlock(code_block) => Some(code_block),
            _ => None,
        })
        .expect("expected a fenced code block")
}

fn rendered_summary(source: &str) -> String {
    let parsed = parse_rule_summary(source);
    let summary = parsed
        .summary()
        .expect("expected a supported summary")
        .expect("expected a summary paragraph");
    let mut output = String::new();
    for fragment in summary.fragments() {
        match fragment {
            RuleSummaryFragment::Text(text) => output.push_str(text),
            RuleSummaryFragment::InlineCode(code) => {
                output.push_str(&code.replace('[', "\\[").replace('<', "\\<"));
            }
            RuleSummaryFragment::SoftBreak => output.push(' '),
        }
    }
    output
}

#[test]
fn lowers_relevant_document_items_in_source_order() {
    let parsed = parse_rule_document(
        "# Section\n\nText <span>inside</span> <!-- comment -->.\n\n```js\n<div>\n```\n\n<div>\nraw\n</div>\n",
    );
    assert!(parsed.diagnostics().is_empty());

    let items = parsed.document().items();
    assert_eq!(items.len(), 6);
    assert!(matches!(items[0], RuleDocumentItem::Heading(ref heading) if heading.level == 1));
    assert!(
        matches!(items[1], RuleDocumentItem::RawHtml(ref html) if html.source_fragments == ["<span>"])
    );
    assert!(
        matches!(items[2], RuleDocumentItem::RawHtml(ref html) if html.source_fragments == ["</span>"])
    );
    assert!(
        matches!(items[3], RuleDocumentItem::RawHtml(ref html) if html.source_fragments == ["<!-- comment -->"])
    );
    assert!(
        matches!(items[4], RuleDocumentItem::FencedCodeBlock(ref block) if block.info == "js" && block.body == "<div>\n")
    );
    assert!(
        matches!(items[5], RuleDocumentItem::RawHtml(ref html) if html.source_fragments == ["<div>\n", "raw\n", "</div>"])
    );
}

#[test]
fn preserves_html_block_line_boundaries() {
    let parsed = parse_rule_document("<!--\n<div>\n-->\n");
    let [RuleDocumentItem::RawHtml(html)] = parsed.document().items() else {
        panic!("expected one raw HTML block");
    };

    assert_eq!(html.source_fragments, ["<!--\n", "<div>\n", "-->"]);
}

#[test]
fn lowers_atx_and_setext_heading_levels() {
    let parsed = parse_rule_document("Section\n---\n\n##### Details\n");
    let levels: Vec<_> = parsed
        .document()
        .items()
        .iter()
        .filter_map(|item| match item {
            RuleDocumentItem::Heading(heading) => Some(heading.level),
            _ => None,
        })
        .collect();

    assert_eq!(levels, [2, 5]);
}

#[test]
fn preserves_fenced_code_block_info() {
    let block = first_code_block(
        "```  js&#44;expect_diagnostic file=example.js  \nconst value = 1;\n```\n",
    );
    assert_eq!(block.info, "js,expect_diagnostic file=example.js");

    let block = first_code_block("```js\\,ignore\nvalue\n```\n");
    assert_eq!(block.info, "js,ignore");

    let block = first_code_block("```js\\&amp;name\nvalue\n```\n");
    assert_eq!(block.info, "js&amp;name");
}

#[test]
fn normalizes_fenced_code_block_containers() {
    let cases = [
        ("```js\nvalue\n```\n", "value\n"),
        ("  ```js\n  value\n  ```\n", "value\n"),
        ("  ```js\n    value\n  ```\n", "  value\n"),
        (" ```js\n\tvalue\n ```\n", "   value\n"),
        (
            "- item\n\n    ```js\n    const value = 1;\n    ```\n",
            "const value = 1;\n",
        ),
        ("> ```js\n> value\n> ```\n", "value\n"),
        ("> ```js\n> > literal\n> ```\n", "> literal\n"),
        (
            "> > ```js\n> > > nested literal\n> > ```\n",
            "> nested literal\n",
        ),
        ("```js\r\nvalue\r\n```\r\n", "value\n"),
        ("```js\nunterminated", "unterminated"),
        (
            "> - item\n>\n>     ```js\n>     value\n>     ```\n",
            "value\n",
        ),
    ];

    for (source, expected) in cases {
        let block = first_code_block(source);
        assert_eq!(block.info, "js", "source: {source:?}");
        assert_eq!(block.body, expected, "source: {source:?}");
    }
}

#[test]
fn lowers_first_paragraph_summary() {
    let summary = rendered_summary(
        " A **rule** uses `T[]`, `<th>`, and \\*escaped\\*.\n It decodes &amp;.\n\n# Details\n",
    );

    assert_eq!(
        summary,
        "A rule uses T\\[], \\<th>, and *escaped*. It decodes &."
    );
}

#[test]
fn skips_link_reference_definitions_before_summary() {
    let summary = rendered_summary(
        "[documentation]: https://example.com\n\n A rule summary follows the definition.\n",
    );

    assert_eq!(summary, "A rule summary follows the definition.");
}

#[test]
fn rejects_unsupported_summary_content() {
    let parsed = parse_rule_summary("Read [the documentation](https://example.com).\n");
    assert!(parsed.summary().is_err());
}
