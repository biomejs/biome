// Prototype: Unified AST for Glimmer
// This shows how we can parse separately and combine into a single tree

use biome_html_parser::{parse_html, HtmlParseOptions};
use biome_html_syntax::{HtmlFileSource, HtmlLanguage, HtmlRoot};
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::{JsFileSource, JsLanguage, JsModule};
use biome_rowan::{AstNode, NodeCache, SyntaxNode, TextRange};
use regex::Regex;
use std::sync::LazyLock;

static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid regex")
});

/// A node in the unified Glimmer AST - can be either JS or HTML
#[derive(Debug, Clone)]
pub enum GlimmerSyntaxNode {
    /// A JavaScript/TypeScript node
    Js(SyntaxNode<JsLanguage>),
    /// An HTML/Glimmer template node
    Html(SyntaxNode<HtmlLanguage>),
}

impl GlimmerSyntaxNode {
    /// Get the text range of this node in the original file
    pub fn text_range(&self) -> TextRange {
        match self {
            GlimmerSyntaxNode::Js(node) => node.text_range(),
            GlimmerSyntaxNode::Html(node) => node.text_range(),
        }
    }

    /// Check if this is a JS node
    pub fn is_js(&self) -> bool {
        matches!(self, GlimmerSyntaxNode::Js(_))
    }

    /// Check if this is an HTML node
    pub fn is_html(&self) -> bool {
        matches!(self, GlimmerSyntaxNode::Html(_))
    }

    /// Get as JS node if possible
    pub fn as_js(&self) -> Option<&SyntaxNode<JsLanguage>> {
        match self {
            GlimmerSyntaxNode::Js(node) => Some(node),
            _ => None,
        }
    }

    /// Get as HTML node if possible
    pub fn as_html(&self) -> Option<&SyntaxNode<HtmlLanguage>> {
        match self {
            GlimmerSyntaxNode::Html(node) => Some(node),
            _ => None,
        }
    }
}

/// Mapping from a template marker in JS to its HTML tree
#[derive(Debug)]
struct TemplateMapping {
    /// The range of the marker in the JS code (e.g., "__BIOME_GLIMMER_TEMPLATE_0__")
    marker_range: TextRange,
    /// The actual template text from the original file
    template_text: String,
    /// The parsed HTML tree for this template
    html_root: HtmlRoot,
    /// The range in the ORIGINAL file (not the extracted JS)
    original_range: TextRange,
}

/// A unified Glimmer module containing both JS and HTML trees
pub struct GlimmerModule {
    /// The original source text
    source: String,
    /// The JavaScript AST (with template markers)
    js_module: JsModule,
    /// The extracted JS text (what was parsed)
    js_text: String,
    /// Mappings from template markers to HTML trees
    templates: Vec<TemplateMapping>,
}

impl GlimmerModule {
    /// Parse a .gjs or .gts file into a unified AST
    pub fn parse(source: &str, file_source: JsFileSource) -> Self {
        // Step 1: Extract JavaScript with template markers
        let (js_text, template_infos) = extract_js_with_templates(source);

        // Step 2: Parse JavaScript
        let mut cache = NodeCache::default();
        let js_parse = parse_js_with_cache(
            &js_text,
            file_source,
            JsParserOptions::default(),
            &mut cache,
        );
        let js_module = js_parse.tree();

        // Step 3: Parse each template
        let html_file_source = HtmlFileSource::glimmer();
        let html_options = HtmlParseOptions::from(&html_file_source);

        let templates = template_infos
            .into_iter()
            .enumerate()
            .map(|(idx, info)| {
                let html_parse = parse_html(&info.template_text, html_options.clone());
                let html_root = html_parse.tree();

                // Find the marker in the JS text
                let marker = format!("__BIOME_GLIMMER_TEMPLATE_{idx}__");
                let marker_start = js_text.find(&marker).expect("Marker not found");
                let marker_range = TextRange::new(
                    marker_start.try_into().unwrap(),
                    (marker_start + marker.len()).try_into().unwrap(),
                );

                TemplateMapping {
                    marker_range,
                    template_text: info.template_text,
                    html_root,
                    original_range: info.original_range,
                }
            })
            .collect();

        GlimmerModule {
            source: source.to_string(),
            js_module,
            js_text,
            templates,
        }
    }

    /// Get the JavaScript module
    pub fn js_module(&self) -> &JsModule {
        &self.js_module
    }

    /// Get all templates
    pub fn templates(&self) -> &[TemplateMapping] {
        &self.templates
    }

    /// Iterate over all nodes in the unified tree
    /// This transparently includes both JS and HTML nodes
    pub fn unified_descendants(&self) -> UnifiedDescendantsIter {
        UnifiedDescendantsIter {
            js_nodes: Some(Box::new(self.js_module.syntax().descendants())),
            current_html_iter: None,
            templates: &self.templates,
            current_template_idx: 0,
        }
    }

    /// Check if a JS node is a template marker
    fn is_template_marker(&self, node: &SyntaxNode<JsLanguage>) -> Option<usize> {
        let range = node.text_range();
        self.templates
            .iter()
            .position(|t| t.marker_range == range)
    }
}

/// Iterator that traverses both JS and HTML nodes in a unified way
pub struct UnifiedDescendantsIter<'a> {
    js_nodes: Option<Box<dyn Iterator<Item = SyntaxNode<JsLanguage>> + 'a>>,
    current_html_iter: Option<Box<dyn Iterator<Item = SyntaxNode<HtmlLanguage>> + 'a>>,
    templates: &'a [TemplateMapping],
    current_template_idx: usize,
}

impl<'a> Iterator for UnifiedDescendantsIter<'a> {
    type Item = GlimmerSyntaxNode;

    fn next(&mut self) -> Option<Self::Item> {
        // First, drain any HTML nodes from current template
        if let Some(ref mut html_iter) = self.current_html_iter {
            if let Some(html_node) = html_iter.next() {
                return Some(GlimmerSyntaxNode::Html(html_node));
            } else {
                // Done with this template
                self.current_html_iter = None;
                self.current_template_idx += 1;
            }
        }

        // Then get next JS node
        if let Some(ref mut js_nodes) = self.js_nodes {
            if let Some(js_node) = js_nodes.next() {
                // Check if this is a template marker
                let node_text = js_node.text_trimmed().to_string();
                if node_text.starts_with("__BIOME_GLIMMER_TEMPLATE_") {
                    // Extract template index from marker
                    if let Some(idx_str) = node_text
                        .strip_prefix("__BIOME_GLIMMER_TEMPLATE_")
                        .and_then(|s| s.strip_suffix("__"))
                    {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            if idx < self.templates.len() {
                                // Start iterating the HTML tree
                                let template = &self.templates[idx];
                                self.current_html_iter = Some(Box::new(
                                    template.html_root.syntax().descendants(),
                                ));
                                self.current_template_idx = idx;
                                // Return the first HTML node
                                return self.next();
                            }
                        }
                    }
                }

                // Regular JS node
                return Some(GlimmerSyntaxNode::Js(js_node));
            }
        }

        None
    }
}

// Helper struct for template extraction
struct TemplateInfo {
    template_text: String,
    original_range: TextRange,
}

/// Extract JavaScript with template markers and template info
fn extract_js_with_templates(source: &str) -> (String, Vec<TemplateInfo>) {
    let mut js_text = String::new();
    let mut templates = Vec::new();
    let mut last_end = 0;
    let mut template_idx = 0;

    for template_match in GLIMMER_TEMPLATE.find_iter(source) {
        // Add JS before this template
        js_text.push_str(&source[last_end..template_match.start()]);

        // Store template info
        templates.push(TemplateInfo {
            template_text: template_match.as_str().to_string(),
            original_range: TextRange::new(
                template_match.start().try_into().unwrap(),
                template_match.end().try_into().unwrap(),
            ),
        });

        // Add marker
        js_text.push_str(&format!("__BIOME_GLIMMER_TEMPLATE_{template_idx}__"));

        last_end = template_match.end();
        template_idx += 1;
    }

    // Add remaining JS
    js_text.push_str(&source[last_end..]);

    (js_text, templates)
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_ast() {
        let source = r#"
import Button from './Button';
import Card from './Card';

export default class MyComponent {
  <template>
    <div>
      <Button>Click me</Button>
      <Card>Content</Card>
    </div>
  </template>
}
"#;

        let file_source = JsFileSource::gjs();
        let glimmer_module = GlimmerModule::parse(source, file_source);

        // Count all nodes in unified tree
        let mut js_count = 0;
        let mut html_count = 0;

        for node in glimmer_module.unified_descendants() {
            match node {
                GlimmerSyntaxNode::Js(_) => js_count += 1,
                GlimmerSyntaxNode::Html(_) => html_count += 1,
            }
        }

        println!("JS nodes: {}", js_count);
        println!("HTML nodes: {}", html_count);

        // We should have both JS and HTML nodes
        assert!(js_count > 0);
        assert!(html_count > 0);
    }

    #[test]
    fn test_template_access() {
        let source = r#"
export default class MyComponent {
  <template>
    <div>Hello</div>
  </template>
}
"#;

        let file_source = JsFileSource::gjs();
        let glimmer_module = GlimmerModule::parse(source, file_source);

        // Access templates directly
        assert_eq!(glimmer_module.templates().len(), 1);

        let template = &glimmer_module.templates()[0];
        println!("Template text: {}", template.template_text);
        println!("Template range: {:?}", template.original_range);
    }
}
