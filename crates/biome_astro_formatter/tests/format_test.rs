use biome_astro_formatter::{format_node, AstroFormatOptions};
use biome_astro_parser::parse_astro;
use biome_astro_syntax::AstroFileSource;

#[test]
fn test_basic_astro_formatting() {
    let source = r#"---
const message='Hello, Astro!';
const count=42;
---

<html><head><title>Test</title></head><body><h1>{message}</h1><p>Count: {count}</p></body></html>"#;

    let file_source = AstroFileSource::astro();
    let parsed = parse_astro(source, file_source);
    let options = AstroFormatOptions::default();

    let formatted = format_node(options, &parsed.syntax());
    
    // Should format without errors
    assert!(formatted.is_ok(), "Formatting should succeed");
    
    if let Ok(result) = formatted {
        let formatted_code = result.print().unwrap();
        
        // Should have proper spacing and indentation
        assert!(formatted_code.as_code().contains("const message = 'Hello, Astro!';"));
        assert!(formatted_code.as_code().contains("const count = 42;"));
        
        // Should format HTML properly
        assert!(formatted_code.as_code().contains("<html>"));
        assert!(formatted_code.as_code().contains("</html>"));
    }
}

#[test]
fn test_astro_frontmatter_formatting() {
    let source = r#"---
import Component from'./Component.astro';
let title='My Site';
const colors=['red','yellow','blue',];
---"#;

    let file_source = AstroFileSource::astro();
    let parsed = parse_astro(source, file_source);
    let options = AstroFormatOptions::default();

    let formatted = format_node(options, &parsed.syntax());
    
    // Should format without errors
    assert!(formatted.is_ok(), "Formatting should succeed");
    
    if let Ok(result) = formatted {
        let formatted_code = result.print().unwrap();
        
        // Should have proper JavaScript formatting in frontmatter
        let code = formatted_code.as_code();
        assert!(code.contains("---"));
        assert!(code.contains("import Component from './Component.astro';"));
        assert!(code.contains("let title = 'My Site';"));
    }
}

#[test]
fn test_astro_template_formatting() {
    let source = r#"<div class="container"><h1    class="title">Hello</h1><p      >World</p></div>"#;

    let file_source = AstroFileSource::astro();
    let parsed = parse_astro(source, file_source);
    let options = AstroFormatOptions::default();

    let formatted = format_node(options, &parsed.syntax());
    
    // Should format without errors
    assert!(formatted.is_ok(), "Formatting should succeed");
    
    if let Ok(result) = formatted {
        let formatted_code = result.print().unwrap();
        
        // Should clean up whitespace and formatting
        let code = formatted_code.as_code();
        assert!(code.contains("class=\"container\""));
        assert!(code.contains("class=\"title\""));
        assert!(!code.contains("    class")); // Extra spaces should be removed
        assert!(!code.contains("      >")); // Extra spaces should be removed
    }
}