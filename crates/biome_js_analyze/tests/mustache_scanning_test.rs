/// Test mustache expression scanning for property/method references
///
/// This tests that we correctly detect {{this.property}} and {{this.#private}}
/// patterns in Glimmer templates

#[test]
fn test_mustache_property_detection() {
    use regex::Regex;

    let template = r#"<template>{{this.count}} and {{this.#privateField}}</template>"#;

    // Test regex for this.member patterns
    let this_member_regex = Regex::new(r"this\.([#]?[a-zA-Z_$][a-zA-Z0-9_$]*)").unwrap();

    let mut found = Vec::new();
    for cap in this_member_regex.captures_iter(template) {
        if let Some(member) = cap.get(1) {
            found.push(member.as_str().to_string());
        }
    }

    println!("Found members: {:?}", found);

    assert!(found.contains(&"count".to_string()), "Should find 'count'");
    assert!(
        found.contains(&"#privateField".to_string()),
        "Should find '#privateField'"
    );
    assert_eq!(found.len(), 2, "Should find exactly 2 members");
}

#[test]
fn test_mustache_method_detection() {
    use regex::Regex;

    let template = r#"<template>{{this.formatDate()}} {{this.getValue(arg)}}</template>"#;

    let this_member_regex = Regex::new(r"this\.([#]?[a-zA-Z_$][a-zA-Z0-9_$]*)").unwrap();

    let mut found = Vec::new();
    for cap in this_member_regex.captures_iter(template) {
        if let Some(member) = cap.get(1) {
            found.push(member.as_str().to_string());
        }
    }

    println!("Found methods: {:?}", found);

    assert!(found.contains(&"formatDate".to_string()));
    assert!(found.contains(&"getValue".to_string()));
}

#[test]
fn test_mustache_extraction() {
    use regex::Regex;

    let template = r#"<template>
        {{this.count}}
        {{helper arg}}
        {{@arg}}
        <div>{{this.#private}}</div>
    </template>"#;

    // Extract all mustache expressions
    let mustache_regex = Regex::new(r"\{\{([^}]+)\}\}").unwrap();

    let mut expressions = Vec::new();
    for cap in mustache_regex.captures_iter(template) {
        if let Some(expr) = cap.get(1) {
            expressions.push(expr.as_str().trim().to_string());
        }
    }

    println!("Found expressions: {:?}", expressions);

    assert!(expressions.contains(&"this.count".to_string()));
    assert!(expressions.contains(&"helper arg".to_string()));
    assert!(expressions.contains(&"@arg".to_string()));
    assert!(expressions.contains(&"this.#private".to_string()));
}

#[test]
fn test_variable_reference_extraction() {
    use regex::Regex;

    let expressions = vec![
        "myVariable",
        "formatDate date",
        "if condition",
        "this.property", // Should not match simple variable pattern
        "@arg",          // Should not match simple variable pattern
    ];

    for expr in expressions {
        let is_this = expr.contains("this.");
        let is_arg = expr.starts_with('@');

        if !is_this && !is_arg {
            if let Some(first_word) = expr.split_whitespace().next() {
                let identifier = first_word
                    .trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '$');
                println!("Expression '{}' -> identifier '{}'", expr, identifier);
            }
        }
    }
}

#[test]
fn test_complex_template_patterns() {
    use regex::Regex;

    let template = r#"<template>
        <div class={{if this.isActive "active" "inactive"}}>
            {{this.title}}
        </div>
        <button {{on "click" this.handleClick}}>
            {{@label}}
        </button>
        {{#each this.items as |item|}}
            {{item.name}}
        {{/each}}
        {{this.#privateCount}}
    </template>"#;

    let this_member_regex = Regex::new(r"this\.([#]?[a-zA-Z_$][a-zA-Z0-9_$]*)").unwrap();

    let mut found = Vec::new();
    for cap in this_member_regex.captures_iter(template) {
        if let Some(member) = cap.get(1) {
            let name = member.as_str();
            if !found.contains(&name.to_string()) {
                found.push(name.to_string());
            }
        }
    }

    println!("Found members in complex template: {:?}", found);

    // Should find all this.X references
    assert!(found.contains(&"isActive".to_string()));
    assert!(found.contains(&"title".to_string()));
    assert!(found.contains(&"handleClick".to_string()));
    assert!(found.contains(&"items".to_string()));
    assert!(found.contains(&"#privateCount".to_string()));
}
