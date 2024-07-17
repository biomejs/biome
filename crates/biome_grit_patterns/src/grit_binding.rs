use crate::{
    grit_context::GritQueryContext, grit_target_language::GritTargetLanguage,
    grit_target_node::GritTargetNode, source_location_ext::SourceFileExt, util::TextRangeGritExt,
};
use biome_diagnostics::{display::SourceFile, SourceCode};
use biome_rowan::TextRange;
use grit_pattern_matcher::{
    binding::Binding, constant::Constant, effects::Effect, pattern::FileRegistry,
};
use grit_util::{AnalysisLogs, AstNode, ByteRange, CodeRange, Range};
use std::{borrow::Cow, collections::HashMap, path::Path};

#[derive(Clone, Debug, PartialEq)]
pub enum GritBinding<'a> {
    /// Bindings to the file with the given path.
    File(&'a Path),

    /// Binds to a specific node.
    Node(GritTargetNode<'a>),

    /// Binds to a range in the tree's source text.
    Range(TextRange, &'a str),

    /// Binds to an empty slot inside a node.
    Empty(GritTargetNode<'a>, u32),

    /// Binds to an individual constant.
    Constant(&'a Constant),
}

impl<'a> Binding<'a, GritQueryContext> for GritBinding<'a> {
    fn from_constant(constant: &'a Constant) -> Self {
        Self::Constant(constant)
    }

    fn from_node(node: GritTargetNode<'a>) -> Self {
        Self::Node(node)
    }

    fn from_path(path: &'a Path) -> Self {
        Self::File(path)
    }

    fn from_range(range: ByteRange, source: &'a str) -> Self {
        Self::Range(
            TextRange::new((range.start as u32).into(), (range.end as u32).into()),
            source,
        )
    }

    /// Returns the only node bound by this binding.
    ///
    /// This includes list bindings that only match a single child.
    ///
    /// Returns `None` if the binding has no associated node, or if there is
    /// more than one associated node.
    fn singleton(&self) -> Option<GritTargetNode<'a>> {
        match self {
            Self::Node(node) => Some(node.clone()),
            Self::File(..) | Self::Range(..) | Self::Empty(..) | Self::Constant(..) => None,
        }
    }

    fn get_sexp(&self) -> Option<String> {
        None
    }

    fn position(&self, _language: &GritTargetLanguage) -> Option<Range> {
        match self {
            Self::Node(node) => {
                let source = SourceFile::new(SourceCode {
                    text: node.source(),
                    line_starts: None,
                });
                source.to_grit_range(node.text_trimmed_range())
            }
            Self::Range(range, source) => {
                let source = SourceFile::new(SourceCode {
                    text: source,
                    line_starts: None,
                });
                source.to_grit_range(*range)
            }
            Self::File(..) | Self::Empty(..) | Self::Constant(_) => None,
        }
    }

    fn range(&self, _language: &GritTargetLanguage) -> Option<ByteRange> {
        match self {
            Self::Node(node) => Some(node.byte_range()),
            Self::Range(range, _) => Some(range.to_byte_range()),
            Self::File(..) | Self::Empty(..) | Self::Constant(_) => None,
        }
    }

    fn code_range(&self, _language: &GritTargetLanguage) -> Option<CodeRange> {
        match self {
            Self::Node(node) => Some(node.code_range()),
            Self::Range(range, source) => Some(range.to_code_range(source)),
            Self::File(..) | Self::Empty(..) | Self::Constant(_) => None,
        }
    }

    fn is_equivalent_to(&self, _other: &Self, _language: &GritTargetLanguage) -> bool {
        todo!()
    }

    fn is_suppressed(&self, _language: &GritTargetLanguage, _current_name: Option<&str>) -> bool {
        false // TODO: Implement suppression
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
        _effects: &[Effect<'a, GritQueryContext>],
        _files: &FileRegistry<'a, GritQueryContext>,
        _memo: &mut HashMap<grit_util::CodeRange, Option<String>>,
        _distributed_indent: Option<usize>,
        _logs: &mut AnalysisLogs,
    ) -> anyhow::Result<Cow<'a, str>> {
        todo!()
    }

    fn text(&self, _language: &GritTargetLanguage) -> anyhow::Result<Cow<'a, str>> {
        match self {
            Self::File(path) => Ok(path.to_string_lossy()),
            Self::Node(node) => Ok(node.text().into()),
            Self::Range(range, source) => {
                Ok((&source[range.start().into()..range.end().into()]).into())
            }
            Self::Empty(_, _) => Ok("".into()),
            Self::Constant(constant) => Ok(constant.to_string().into()),
        }
    }

    fn source(&self) -> Option<&'a str> {
        match self {
            Self::Node(node) => Some(node.source()),
            Self::Range(_, source) => Some(*source),
            Self::Empty(node, _) => Some(node.source()),
            Self::File(..) | Self::Constant(..) => None,
        }
    }

    fn as_constant(&self) -> Option<&Constant> {
        match self {
            Self::Constant(constant) => Some(constant),
            _ => None,
        }
    }

    fn as_filename(&self) -> Option<&Path> {
        match self {
            Self::File(path) => Some(path),
            _ => None,
        }
    }

    fn as_node(&self) -> Option<GritTargetNode<'a>> {
        match self {
            Self::Node(node) => Some(node.clone()),
            _ => None,
        }
    }

    fn is_list(&self) -> bool {
        self.as_node().map_or(false, |node| node.is_list())
    }

    fn list_items(&self) -> Option<impl Iterator<Item = GritTargetNode<'a>> + Clone> {
        match self {
            Self::Node(node) if node.is_list() => Some(node.named_children()),
            _ => None,
        }
    }

    fn parent_node(&self) -> Option<GritTargetNode<'a>> {
        todo!()
    }

    fn is_truthy(&self) -> bool {
        match self {
            Self::File(_) => true,
            Self::Node(node) => {
                if node.is_list() {
                    node.has_children()
                } else {
                    true
                }
            }
            Self::Range(..) => true,
            Self::Empty(..) => false,
            Self::Constant(c) => c.is_truthy(),
        }
    }

    fn log_empty_field_rewrite_error(
        &self,
        _language: &GritTargetLanguage,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
