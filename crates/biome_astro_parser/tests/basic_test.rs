use biome_astro_parser::{parse_astro, AstroParse};
use biome_astro_syntax::{AstroFileSource, AstroRoot};
use biome_rowan::AstNode;

#[test]
fn test_basic_astro_parsing() {
    let source = r#"---
const message = 'Hello, Astro!';
const count = 42;
---

<html>
<head>
    <title>Test</title>
</head>
<body>
    <h1>{message}</h1>
    <p>Count: {count}</p>
</body>
</html>"#;

    let file_source = AstroFileSource::astro();
    let parsed: AstroParse = parse_astro(source, file_source);

    // Should parse without critical errors
    assert!(!parsed.has_errors(), "Parser should not have critical errors");

    // Should have a root node
    let root: AstroRoot = parsed.root();
    assert!(root.syntax().children().count() > 0, "Root should have children");

    // Should detect frontmatter
    let frontmatter = root.frontmatter();
    assert!(frontmatter.is_ok(), "Should have frontmatter");

    // Should have body content
    let body = root.body();
    assert!(body.is_ok(), "Should have body");
}

#[test]
fn test_astro_without_frontmatter() {
    let source = r#"<html>
<head>
    <title>Simple Test</title>
</head>
<body>
    <h1>Hello World</h1>
</body>
</html>"#;

    let file_source = AstroFileSource::astro();
    let parsed: AstroParse = parse_astro(source, file_source);

    // Should parse without critical errors
    assert!(!parsed.has_errors(), "Parser should not have critical errors");

    let root: AstroRoot = parsed.root();
    
    // Should not have frontmatter
    let frontmatter = root.frontmatter();
    assert!(frontmatter.is_err(), "Should not have frontmatter");

    // Should have body content
    let body = root.body();
    assert!(body.is_ok(), "Should have body");
}

#[test]
fn test_astro_with_expressions() {
    let source = r#"---
const name = 'World';
---

<div>
    <h1>Hello {name}!</h1>
    <p>Expression: {2 + 2}</p>
</div>"#;

    let file_source = AstroFileSource::astro();
    let parsed: AstroParse = parse_astro(source, file_source);

    // Should parse without critical errors
    assert!(!parsed.has_errors(), "Parser should not have critical errors");

    let root: AstroRoot = parsed.root();
    
    // Should have frontmatter
    assert!(root.frontmatter().is_ok(), "Should have frontmatter");

    // Should have body content with expressions
    assert!(root.body().is_ok(), "Should have body");
}