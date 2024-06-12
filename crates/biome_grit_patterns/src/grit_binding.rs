use crate::{
    grit_context::GritQueryContext, grit_target_language::GritTargetLanguage,
    grit_target_node::GritTargetNode, grit_tree::GritTree, source_location_ext::SourceFileExt,
};
use biome_diagnostics::{display::SourceFile, SourceCode};
use grit_pattern_matcher::{binding::Binding, constant::Constant};
use grit_util::{Ast, AstNode, ByteRange, CodeRange, Range};
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GritBinding<'a> {
    Tree(&'a GritTree),
    Node(GritTargetNode),
    Constant(&'a Constant),
}

impl<'a> GritBinding<'a> {
    pub fn from_tree(tree: &'a GritTree) -> Self {
        Self::Tree(tree)
    }
}

impl<'a> Binding<'a, GritQueryContext> for GritBinding<'a> {
    fn from_constant(constant: &'a Constant) -> Self {
        Self::Constant(constant)
    }

    fn from_node(node: GritTargetNode) -> Self {
        Self::Node(node)
    }

    fn from_path(_path: &'a Path) -> Self {
        todo!()
    }

    fn from_range(_range: ByteRange, _source: &'a str) -> Self {
        todo!()
    }

    /// Returns the only node bound by this binding.
    ///
    /// This includes list bindings that only match a single child.
    ///
    /// Returns `None` if the binding has no associated node, or if there is
    /// more than one associated node.
    fn singleton(&self) -> Option<GritTargetNode> {
        match self {
            Self::Node(node) => Some(node.clone()),
            Self::Tree(..) | Self::Constant(..) => None,
        }
    }

    fn get_sexp(&self) -> Option<String> {
        None
    }

    fn position(&self, _language: &GritTargetLanguage) -> Option<Range> {
        match self {
            GritBinding::Tree(tree) => {
                let source = tree.source();
                let source = SourceFile::new(SourceCode {
                    text: &source,
                    line_starts: None,
                });
                source.to_grit_range(tree.root_node().text_range())
            }
            GritBinding::Node(node) => {
                // TODO: This is probably very inefficient.
                let root = node.ancestors().last()?;
                let source = root.text().to_string();
                let source = SourceFile::new(SourceCode {
                    text: &source,
                    line_starts: None,
                });
                source.to_grit_range(root.text_trimmed_range())
            }
            GritBinding::Constant(_) => None,
        }
    }

    fn range(&self, _language: &GritTargetLanguage) -> Option<ByteRange> {
        match self {
            GritBinding::Tree(tree) => Some(tree.root_node().byte_range()),
            GritBinding::Node(node) => Some(node.byte_range()),
            GritBinding::Constant(_) => None,
        }
    }

    fn code_range(&self, _language: &GritTargetLanguage) -> Option<CodeRange> {
        match self {
            GritBinding::Tree(tree) => Some(tree.root_node().code_range()),
            GritBinding::Node(node) => Some(node.code_range()),
            GritBinding::Constant(_) => None,
        }
    }

    fn is_equivalent_to(&self, _other: &Self, _language: &GritTargetLanguage) -> bool {
        todo!()
    }

    fn is_suppressed(&self, _language: &GritTargetLanguage, _current_name: Option<&str>) -> bool {
        todo!()
    }

    fn get_insertion_padding(
        &self,
        _text: &str,
        _is_first: bool,
        _language: &GritTargetLanguage,
    ) -> Option<String> {
        todo!()
    }

    fn linearized_text(
        &self,
        _language: &GritTargetLanguage,
        _effects: &[grit_pattern_matcher::effects::Effect<'a, GritQueryContext>],
        _files: &grit_pattern_matcher::pattern::FileRegistry<'a, GritQueryContext>,
        _memo: &mut std::collections::HashMap<grit_util::CodeRange, Option<String>>,
        _distributed_indent: Option<usize>,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<std::borrow::Cow<'a, str>> {
        todo!()
    }

    fn text(&self, _language: &GritTargetLanguage) -> anyhow::Result<std::borrow::Cow<str>> {
        todo!()
    }

    fn source(&self) -> Option<&'a str> {
        todo!()
    }

    fn as_constant(&self) -> Option<&grit_pattern_matcher::constant::Constant> {
        todo!()
    }

    fn as_filename(&self) -> Option<&std::path::Path> {
        todo!()
    }

    fn as_node(&self) -> Option<GritTargetNode> {
        todo!()
    }

    fn is_list(&self) -> bool {
        todo!()
    }

    fn list_items(&self) -> Option<impl Iterator<Item = GritTargetNode> + Clone> {
        None::<TodoIterator>
    }

    fn parent_node(&self) -> Option<GritTargetNode> {
        todo!()
    }

    fn is_truthy(&self) -> bool {
        todo!()
    }

    fn log_empty_field_rewrite_error(
        &self,
        _language: &GritTargetLanguage,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoIterator;

impl Iterator for TodoIterator {
    type Item = GritTargetNode;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
