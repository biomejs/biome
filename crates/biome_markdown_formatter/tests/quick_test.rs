use biome_markdown_formatter::{MdFormatLanguage, context::MdFormatOptions};
use biome_markdown_parser::{MarkdownParserOptions, parse_markdown, parse_markdown_with_cache};
use biome_rowan::NodeCache;

#[ignore]
#[test]
fn quick_test() {
    let source = r#"1. 123
2. 123
   1.   123
"#;
    let parse = parse_markdown(source, MarkdownParserOptions::default());

    // Print CST
    eprintln!("{:#?}", parse.syntax());
    // print red tree
    // eprintln!("{:#?}", parse.tree());

    let options = MdFormatOptions::default();
    let result = biome_formatter::format_node(
        &parse.syntax(),
        MdFormatLanguage::new(options.clone()),
        false,
    );

    let formatted = result.unwrap();
    let first_ir = formatted.document();
    let output = formatted.print().unwrap();
    eprintln!("Formatted:\n{}", output.as_code());
    eprintln!("IR:\n{}", first_ir);

    // Idempotency
    // Now re-parse the formatted output and show its CST
    let reparse = parse_markdown(output.as_code(), MarkdownParserOptions::default());
    eprintln!("\n--- Re-parsed CST ---");
    eprintln!("{:#?}", reparse.syntax());

    let result2 =
        biome_formatter::format_node(&reparse.syntax(), MdFormatLanguage::new(options), false);
    let output2 = result2.unwrap();
    let second_ir = output2.document();
    eprintln!("Re-IR:\n{}", second_ir);

    similar_asserts::assert_eq!(
        output2.print().unwrap().as_code(),
        output.as_code(),
        "left is the re-formatted"
    );
    similar_asserts::assert_eq!(
        second_ir.to_string(),
        first_ir.to_string(),
        "left is the re-formatted"
    );
}

#[test]
fn formats_crlf_frontmatter() {
    let source = "---\r\n# ---\r\n---\r\n\r\n#   Heading\r\n";
    let parse = parse_markdown_with_cache(
        source,
        &mut NodeCache::default(),
        MarkdownParserOptions::default().with_frontmatter(true),
    );
    let options = MdFormatOptions::default();
    let formatted =
        biome_formatter::format_node(&parse.syntax(), MdFormatLanguage::new(options), false)
            .expect("frontmatter should format");

    assert_eq!(
        formatted
            .print()
            .expect("frontmatter should print")
            .as_code(),
        "---\n# ---\n---\n\n# Heading\n"
    );
}

#[test]
fn formats_gfm_task_list_items() {
    let source = "- [ ] todo\n  - [x] nested\n1. [X] done\n";
    let expected = "- [ ] todo\n  - [x] nested\n\n1. [X] done\n";
    let options = MarkdownParserOptions::default().with_gfm(true);
    let parse = parse_markdown_with_cache(source, &mut NodeCache::default(), options.clone());
    let formatted = biome_formatter::format_node(
        &parse.syntax(),
        MdFormatLanguage::new(MdFormatOptions::default()),
        false,
    )
    .expect("GFM task list items should format");
    let output = formatted
        .print()
        .expect("GFM task list items should print");

    assert_eq!(output.as_code(), expected);

    let reparse = parse_markdown_with_cache(
        output.as_code(),
        &mut NodeCache::default(),
        options,
    );
    let reformatted = biome_formatter::format_node(
        &reparse.syntax(),
        MdFormatLanguage::new(MdFormatOptions::default()),
        false,
    )
    .expect("reparsed GFM task list items should format");

    assert_eq!(
        reformatted
            .print()
            .expect("reparsed GFM task list items should print")
            .as_code(),
        output.as_code()
    );
}
