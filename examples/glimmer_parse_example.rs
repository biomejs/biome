use biome_html_parser::{parse_html, HtmlParseOptions};
use biome_html_syntax::HtmlFileSource;
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::JsFileSource;
use biome_rowan::NodeCache;
use regex::Regex;
use std::fs;
use std::sync::LazyLock;

static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid Glimmer template regex")
});

fn main() {
    println!("=== Glimmer .gjs Parsing Example ===\n");

    // Read the example .gjs file
    let content = fs::read_to_string("example_component.gjs")
        .expect("Failed to read example_component.gjs");

    println!("📄 Original file content:");
    println!("{}\n", content);

    // Step 1: Extract JavaScript with templates replaced by markers
    println!("🔧 Step 1: Extract JavaScript code\n");
    let js_content = extract_js_content(&content);
    println!("JavaScript (with template markers):");
    println!("{}\n", js_content);

    // Step 2: Parse JavaScript
    println!("🔍 Step 2: Parse JavaScript code\n");
    let mut cache = NodeCache::default();
    let js_parse = parse_js_with_cache(
        &js_content,
        JsFileSource::gjs(),
        JsParserOptions::default(),
        &mut cache,
    );

    if js_parse.has_errors() {
        println!("❌ JavaScript parsing errors:");
        for diagnostic in js_parse.diagnostics() {
            println!("  - {:?}", diagnostic);
        }
    } else {
        println!("✅ JavaScript parsed successfully!");
        println!("   Root node: {:?}", js_parse.syntax().kind());
    }
    println!();

    // Step 3: Parse Glimmer templates with the new HTML parser
    println!("🎨 Step 3: Parse Glimmer templates with HTML parser\n");
    let templates = parse_templates(&content);

    println!("Found {} template(s):", templates.len());
    for (idx, parse) in templates.iter().enumerate() {
        println!("\nTemplate {}:", idx + 1);

        if parse.has_errors() {
            println!("  ❌ Parsing errors:");
            for diagnostic in parse.diagnostics() {
                println!("    - {:?}", diagnostic);
            }
        } else {
            println!("  ✅ Parsed successfully!");
            println!("  Root node: {:?}", parse.syntax().kind());
            println!("  Tree structure:");
            print_tree(parse.syntax(), "    ", 3);
        }
    }

    println!("\n🎉 Demo complete!");
    println!("\n💡 Key benefits of the new architecture:");
    println!("  • JavaScript and templates are parsed separately");
    println!("  • Unified AST enables cross-linting");
    println!("  • Glimmer syntax ({{#if}}, ...attrs, etc.) fully supported");
    println!("  • Both .gjs and .gts files work seamlessly");
}

fn extract_js_content(text: &str) -> String {
    let mut result = String::new();
    let mut last_end = 0;
    let mut template_index = 0;

    for template_match in GLIMMER_TEMPLATE.find_iter(text) {
        result.push_str(&text[last_end..template_match.start()]);
        result.push_str(&format!("__BIOME_GLIMMER_TEMPLATE_{template_index}__"));
        last_end = template_match.end();
        template_index += 1;
    }

    result.push_str(&text[last_end..]);
    result
}

fn parse_templates(text: &str) -> Vec<biome_parser::AnyParse> {
    let mut results = Vec::new();

    for template_match in GLIMMER_TEMPLATE.find_iter(text) {
        let template_content = template_match.as_str();

        // Parse with Glimmer-enabled HTML parser
        let file_source = HtmlFileSource::glimmer();
        let options = HtmlParseOptions::from(&file_source);
        let parse = parse_html(template_content, options);

        results.push(parse.into());
    }

    results
}

fn print_tree(node: &biome_html_syntax::HtmlSyntaxNode, prefix: &str, max_depth: usize) {
    if max_depth == 0 {
        println!("{}...", prefix);
        return;
    }

    for child in node.children() {
        match child {
            biome_html_syntax::HtmlSyntaxElement::Node(node) => {
                println!("{}{:?}", prefix, node.kind());
                print_tree(&node, &format!("{}  ", prefix), max_depth - 1);
            }
            biome_html_syntax::HtmlSyntaxElement::Token(token) => {
                let text = token.text_trimmed();
                if !text.trim().is_empty() {
                    println!("{}Token({:?}): \"{}\"", prefix, token.kind(), text);
                }
            }
        }
    }
}
