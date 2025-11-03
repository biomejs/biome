/// Debug test to verify template scanning is working

#[test]
fn test_template_regex_matches() {
    use regex::Regex;

    let template_regex = Regex::new(r"<template>[\s\S]*?</template>").unwrap();
    let source = r#"
import Button from './Button';

export default class MyComponent {
  <template>
    <Button />
  </template>
}
"#;

    let matches: Vec<_> = template_regex.find_iter(source).collect();
    println!("Found {} template matches", matches.len());
    for m in &matches {
        println!("Match: {}", m.as_str());
    }

    assert!(matches.len() > 0, "Should find at least one template");
}

#[test]
fn test_component_detection_in_template() {
    use regex::Regex;
    use biome_html_parser::{parse_html, HtmlParseOptions};
    use biome_html_syntax::{AnyHtmlElement, HtmlFileSource};
    use biome_rowan::AstNode;

    let template_content = r#"<template>
        <Button />
        <Card>Content</Card>
    </template>"#;

    let file_source = HtmlFileSource::glimmer();
    let options = HtmlParseOptions::from(&file_source);
    let parse = parse_html(template_content, options);

    let root = parse.tree();
    let root_node = root.syntax();

    let mut components = Vec::new();
    for node in root_node.descendants() {
        if AnyHtmlElement::can_cast(node.kind()) {
            let element = AnyHtmlElement::unwrap_cast(node.clone());
            if let Some(name_token) = element.name() {
                let tag_name = name_token.to_string();
                // Check if PascalCase
                if tag_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    components.push(tag_name);
                }
            }
        }
    }

    println!("Found components: {:?}", components);
    assert!(components.contains(&"Button".to_string()), "Should find Button");
    assert!(components.contains(&"Card".to_string()), "Should find Card");
}
