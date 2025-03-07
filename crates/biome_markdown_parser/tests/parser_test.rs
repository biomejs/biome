use biome_markdown_parser::parse_markdown;

#[test]
fn test_parse_header() {
    let source = "# Header 1";
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}

#[test]
fn test_parse_paragraph() {
    let source = "This is a paragraph.";
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}

#[test]
fn test_parse_thematic_break() {
    let source = "---";
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}

#[test]
fn test_parse_code_block() {
    let source = "```rust\nfn main() {}\n```";
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}

#[test]
fn test_parse_unordered_list() {
    let source = "- Item 1\n- Item 2\n- Item 3";
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}

#[test]
fn test_parse_ordered_list() {
    let source = "1. Item 1\n2. Item 2\n3. Item 3";
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}

#[test]
fn test_parse_blockquote() {
    let source = "> This is a blockquote.";
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}

#[test]
fn test_parse_table() {
    let source = "| Header 1 | Header 2 |\n| --- | --- |\n| Cell 1 | Cell 2 |";
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}

#[test]
fn test_parse_complex_document() {
    let source = r#"# Markdown Test

This is a paragraph with **bold** and *italic* text.

## Lists

- Item 1
- Item 2
  - Nested item
- Item 3

1. First item
2. Second item
3. Third item

## Code

```rust
fn main() {
    println!("Hello, world!");
}
```

## Blockquote

> This is a blockquote.
> It can span multiple lines.

## Table

| Name | Age | Occupation |
| ---- | --- | ---------- |
| John | 30  | Developer  |
| Jane | 25  | Designer   |
"#;
    let parse = parse_markdown(source);
    assert!(!parse.has_errors());
}
