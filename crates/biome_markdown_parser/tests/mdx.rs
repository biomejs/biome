use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};
use quickcheck_macros::quickcheck;

/// Fragments that exercise the MDX constructs and the Markdown constructs
/// around them. Random strings rarely contain `import ` or a balanced `{`.
const FRAGMENTS: &[&str] = &[
    "{", "}", "import ", "export ", "\n", "\n\n", "\r\n", "a", " ", "\t", "    ", "`", "\"", "'",
    "/*", "*/", "//", "<", ">", "#", "> ", "- ", "1. ", "[", "]", "(", ")", "*", "_", "\\", "=",
];

fn mdx_source(indices: &[u8]) -> String {
    indices
        .iter()
        .map(|index| FRAGMENTS[usize::from(*index) % FRAGMENTS.len()])
        .collect()
}

#[quickcheck]
fn mdx_parsing_preserves_every_source_byte(indices: Vec<u8>) -> bool {
    let source = mdx_source(&indices);
    let parsed = parse_markdown(&source, MarkdownParserOptions::default().with_mdx(true));
    parsed.syntax().to_string() == source
}

#[test]
fn an_expression_at_the_end_of_the_source_preserves_every_byte() {
    for source in ["{", "a {", "{a", "{\n", "import a\n\n{"] {
        let parsed = parse_markdown(source, MarkdownParserOptions::default().with_mdx(true));
        assert_eq!(parsed.syntax().to_string(), source);
    }
}
