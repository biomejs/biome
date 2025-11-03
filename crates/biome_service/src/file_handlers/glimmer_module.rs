/// Unified AST for Glimmer files
///
/// This module provides a unified view of Glimmer files that contain both
/// JavaScript/TypeScript and HTML template code. Instead of treating them
/// as completely separate ASTs, we create a virtual unified tree that can
/// be traversed seamlessly.
///
/// ## Architecture
///
/// 1. Parse JavaScript with template markers (existing approach)
/// 2. Parse each template separately as HTML (existing approach)
/// 3. Create a unified view that transparently switches between trees
///
/// ## Benefits
///
/// - Enables cross-language linting (e.g., unused imports used in templates)
/// - Simplifies semantic analysis across both languages
/// - Foundation for making existing rules work with templates

use super::glimmer::GLIMMER_TEMPLATE;
use biome_html_parser::{parse_html, HtmlParseOptions};
use biome_html_syntax::{HtmlFileSource, HtmlLanguage, HtmlRoot};
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::{AnyJsRoot, JsFileSource, JsLanguage, JsModule};
use biome_rowan::{AstNode, NodeCache, SyntaxNode, TextRange};

/// A node in the unified Glimmer AST - can be either JS or HTML
#[derive(Debug, Clone)]
pub enum GlimmerSyntaxNode {
    /// A JavaScript/TypeScript node
    Js(SyntaxNode<JsLanguage>),
    /// An HTML/Glimmer template node
    Html(SyntaxNode<HtmlLanguage>),
}

impl GlimmerSyntaxNode {
    /// Get the text range of this node in the extracted JS or template text
    pub fn text_range(&self) -> TextRange {
        match self {
            GlimmerSyntaxNode::Js(node) => node.text_trimmed_range(),
            GlimmerSyntaxNode::Html(node) => node.text_trimmed_range(),
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
pub struct TemplateMapping {
    /// The range of the marker in the JS code (e.g., "__BIOME_GLIMMER_TEMPLATE_0__")
    pub marker_range: TextRange,
    /// The actual template text from the original file
    pub template_text: String,
    /// The parsed HTML tree for this template
    pub html_root: HtmlRoot,
    /// The range in the ORIGINAL file (not the extracted JS)
    pub original_range: TextRange,
}

/// A unified Glimmer module containing both JS and HTML trees
pub struct GlimmerModule {
    /// The original source text
    source: String,
    /// The JavaScript AST (with template markers)
    js_root: AnyJsRoot,
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
        let js_root = js_parse.tree();

        // Step 3: Parse each template
        let html_file_source = HtmlFileSource::glimmer();

        let templates = template_infos
            .into_iter()
            .enumerate()
            .map(|(idx, info)| {
                let html_options = HtmlParseOptions::from(&html_file_source);
                let html_parse = parse_html(&info.template_text, html_options);
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
            js_root,
            js_text,
            templates,
        }
    }

    /// Get the JavaScript root AST node
    pub fn js_root(&self) -> &AnyJsRoot {
        &self.js_root
    }

    /// Get the JavaScript module if the root is a module
    pub fn js_module(&self) -> Option<JsModule> {
        self.js_root.as_js_module().cloned()
    }

    /// Get all templates
    pub fn templates(&self) -> &[TemplateMapping] {
        &self.templates
    }

    /// Get the original source text
    pub fn source(&self) -> &str {
        &self.source
    }

    /// Get the extracted JS text (with template markers)
    pub fn js_text(&self) -> &str {
        &self.js_text
    }

    /// Iterate over all nodes in the unified tree
    /// This transparently includes both JS and HTML nodes
    pub fn unified_descendants(&self) -> UnifiedDescendantsIter<'_> {
        UnifiedDescendantsIter {
            js_nodes: Some(Box::new(self.js_root.syntax().descendants())),
            current_html_iter: None,
            templates: &self.templates,
            current_template_idx: 0,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_ast_basic() {
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
        assert!(js_count > 0, "Should have JS nodes");
        assert!(html_count > 0, "Should have HTML nodes");
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

        assert!(template.template_text.contains("<div>Hello</div>"));
    }

    #[test]
    fn test_multiple_templates() {
        let source = r#"
const Foo = <template>{{@foo}}</template>;
const Bar = <template>{{@bar}}</template>;
"#;

        let file_source = JsFileSource::gjs();
        let glimmer_module = GlimmerModule::parse(source, file_source);

        assert_eq!(glimmer_module.templates().len(), 2);

        // Should be able to access both template trees
        for (idx, template) in glimmer_module.templates().iter().enumerate() {
            println!("Template {}: {}", idx, template.template_text);
            assert!(template.template_text.starts_with("<template>"));
        }
    }

    #[test]
    fn test_unified_iteration_order() {
        let source = r#"
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  <template>
    <div>Test</div>
  </template>
}
"#;

        let file_source = JsFileSource::gjs();
        let glimmer_module = GlimmerModule::parse(source, file_source);

        let mut node_types = Vec::new();
        for node in glimmer_module.unified_descendants() {
            node_types.push(if node.is_js() { "JS" } else { "HTML" });
        }

        // Should see JS nodes, then HTML nodes, then more JS nodes
        // The exact sequence depends on the AST structure
        println!("Node sequence: {:?}", node_types);

        // Verify we have both types
        assert!(node_types.contains(&"JS"));
        assert!(node_types.contains(&"HTML"));
    }
}
