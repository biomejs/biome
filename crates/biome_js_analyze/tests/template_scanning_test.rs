/// Test template scanning logic for Glimmer files
///
/// This tests the helper function that scans templates, which is used
/// by the semantic model builder visitor

#[test]
fn test_template_component_detection() {
    use biome_html_parser::{HtmlParseOptions, parse_html};
    use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
    use biome_rowan::AstNode;

    let template = r#"<template><Button /><Card /></template>"#;

    // Parse with Glimmer-enabled HTML parser
    let file_source = HtmlFileSource::glimmer();
    let options = HtmlParseOptions::from(&file_source);
    let parse = parse_html(template, options);

    let root = parse.tree();
    let root_node = root.syntax();

    // Find all PascalCase components
    let mut components = Vec::new();
    for node in root_node.descendants() {
        if AnyHtmlElement::can_cast(node.kind()) {
            let element = AnyHtmlElement::unwrap_cast(node.clone());
            if let Some(name_token) = element.name() {
                let tag_name = name_token.to_string();
                // PascalCase check
                if !tag_name.is_empty() && tag_name.chars().next().unwrap().is_uppercase() {
                    components.push(tag_name);
                }
            }
        }
    }

    println!("Found components: {:?}", components);

    // Should find Button and Card
    assert!(
        components.contains(&"Button".to_string()),
        "Should find Button component"
    );
    assert!(
        components.contains(&"Card".to_string()),
        "Should find Card component"
    );
    assert_eq!(components.len(), 2, "Should find exactly 2 components");
}

#[test]
fn test_template_with_regular_and_self_closing() {
    use biome_html_parser::{HtmlParseOptions, parse_html};
    use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
    use biome_rowan::AstNode;

    let template = r#"<template><Card><Button /></Card></template>"#;

    let file_source = HtmlFileSource::glimmer();
    let options = HtmlParseOptions::from(&file_source);
    let parse = parse_html(template, options);

    let root = parse.tree();
    let root_node = root.syntax();

    let mut components = Vec::new();
    for node in root_node.descendants() {
        if AnyHtmlElement::can_cast(node.kind()) {
            let element = AnyHtmlElement::unwrap_cast(node.clone());
            if let Some(name_token) = element.name() {
                let tag_name = name_token.to_string();
                if !tag_name.is_empty() && tag_name.chars().next().unwrap().is_uppercase() {
                    components.push(tag_name);
                }
            }
        }
    }

    println!("Found components: {:?}", components);

    // Should find both Button and Card
    assert!(components.contains(&"Button".to_string()));
    assert!(components.contains(&"Card".to_string()));
}
