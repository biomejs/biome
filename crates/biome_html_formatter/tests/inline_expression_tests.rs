use biome_formatter::FormatElement;
use biome_formatter::prelude::Document;
use biome_html_formatter::{
    HtmlFormatLanguage, HtmlFormatOptions, HtmlInlineEmbeddedContent, HtmlInlineEmbeddedExpression,
};
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::HtmlTextExpression;
use biome_languages::HtmlFileSource;
use biome_rowan::AstNode;

fn format_expression(source: &str, content: HtmlInlineEmbeddedContent, shorthand: bool) -> String {
    let file_source = HtmlFileSource::svelte();
    let parsed = parse_html(source, HtmlParserOptions::from(&file_source));
    assert!(parsed.diagnostics().is_empty());
    let tree = parsed.syntax();
    let expression = tree
        .descendants()
        .find_map(HtmlTextExpression::cast)
        .unwrap();
    let token = expression.html_literal_token().unwrap();
    let prepared = HtmlInlineEmbeddedExpression {
        range: token.text_range(),
        content,
        shorthand_identifier: shorthand.then(|| token.token_text_trimmed()),
    };
    biome_formatter::format_node(
        &tree,
        HtmlFormatLanguage::new(HtmlFormatOptions::new(file_source))
            .with_inline_embedded_expressions(vec![prepared]),
        false,
    )
    .unwrap()
    .print()
    .unwrap()
    .as_code()
    .to_string()
}

#[test]
fn prepared_expression_replaces_its_source_token() {
    let output = format_expression(
        "<button onclick={1   +   2}>Go</button>",
        HtmlInlineEmbeddedContent::Formatted(Document::new(vec![FormatElement::Token {
            text: "answer",
        }])),
        false,
    );
    assert_eq!(output, "<button onclick={answer}>Go</button>\n");
}

#[test]
fn failed_expression_preserves_source() {
    let output = format_expression(
        "<button onclick={1   +   2}>Go</button>",
        HtmlInlineEmbeddedContent::Verbatim,
        false,
    );
    assert_eq!(output, "<button onclick={1   +   2}>Go</button>\n");
}

#[test]
fn compact_initializer_does_not_reinsert_embedded_content() {
    let output = format_expression(
        "<Component value={value} />",
        HtmlInlineEmbeddedContent::Formatted(Document::new(vec![FormatElement::Token {
            text: "answer",
        }])),
        true,
    );
    assert_eq!(output, "<Component {value} />\n");
}

#[test]
fn failed_expression_keeps_trailing_line_comment_and_next_attribute() {
    let output = format_expression(
        "<button onclick={foo( // keep\n} title=\"next\">Go</button>",
        HtmlInlineEmbeddedContent::Verbatim,
        false,
    );
    assert!(output.contains("foo( // keep\n}"), "{output}");
    assert!(output.contains("title=\"next\""), "{output}");
}
