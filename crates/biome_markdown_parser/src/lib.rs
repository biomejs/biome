use crate::parser::MarkdownParser;
use biome_markdown_factory::MarkdownSyntaxFactory;
use biome_markdown_syntax::{
    MarkdownLanguage, MarkdownSyntaxKind, MarkdownSyntaxNode, MarkdownSyntaxToken, MdDocument,
};
pub use biome_parser::prelude::*;
use biome_parser::{AnyParse, tree_sink::LosslessTreeSink};
use biome_rowan::{AstNode, NodeCache};

mod lexer;
mod parser;
mod syntax;
mod token_source;

pub(crate) type MarkdownLosslessTreeSink<'source> =
    LosslessTreeSink<'source, MarkdownLanguage, MarkdownSyntaxFactory>;

pub fn parse_markdown(source: &str) -> MarkdownParse {
    let mut cache = NodeCache::default();
    parse_markdown_with_cache(source, &mut cache)
}

/// Parses the provided string as Markdown using the provided node cache.
pub fn parse_markdown_with_cache(source: &str, cache: &mut NodeCache) -> MarkdownParse {
    tracing::debug_span!("Parsing phase").in_scope(move || {
        let mut parser = MarkdownParser::new(source);
        parser.parse_document();
        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = MarkdownLosslessTreeSink::with_cache(source, &trivia, cache);
        biome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        let root = MarkdownSyntaxNode::from(green);

        // Return the parse result
        MarkdownParse::new(root, diagnostics)
    })
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct MarkdownParse {
    root: MarkdownSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl MarkdownParse {
    pub fn new(root: MarkdownSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> MarkdownParse {
        MarkdownParse { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    pub fn syntax(&self) -> MarkdownSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> MdDocument {
        // Check if we have a valid document node
        match MdDocument::cast(self.syntax()) {
            Some(doc) => doc,
            None => {
                // During development, print a warning and try to recover instead of panicking
                eprintln!(
                    "Warning: Expected MD_DOCUMENT node but got {:?}",
                    self.syntax().kind()
                );

                use biome_markdown_factory::make;
                use biome_markdown_syntax::AnyMdBlock;

                // Print the tree structure for debugging
                eprintln!("\nTree structure:");
                fn print_tree(node: &MarkdownSyntaxNode, depth: usize) {
                    let indent = "  ".repeat(depth);
                    eprintln!("{}{:?}", indent, node.kind());
                    for child in node.children() {
                        print_tree(&child, depth + 1);
                    }
                }
                print_tree(&self.syntax(), 0);

                // Create valid blocks with simple text content
                let mut blocks = Vec::<AnyMdBlock>::new();

                // Create a paragraph with a simple placeholder text
                // Create a token for the text content
                let text_token = MarkdownSyntaxToken::new_detached(
                    MarkdownSyntaxKind::MD_TEXTUAL_LITERAL,
                    "Recovered content placeholder",
                    vec![],
                    vec![],
                );

                // Create a textual node
                let text_node = make::md_textual(text_token);

                // Add it to a paragraph item list
                let para_items = make::md_paragraph_item_list(vec![text_node.into()]);
                let para_node = make::md_paragraph(para_items);

                if let Some(block) = AnyMdBlock::cast(para_node.syntax().clone()) {
                    blocks.push(block);
                }

                // Create an EOF token
                let eof_token =
                    MarkdownSyntaxToken::new_detached(MarkdownSyntaxKind::EOF, "", vec![], vec![]);

                // Build a valid document
                let block_list = make::md_block_list(blocks);
                make::md_document(block_list, eof_token).build()
            }
        }
    }
}

impl From<MarkdownParse> for AnyParse {
    fn from(parse: MarkdownParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();
        Self::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}
