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

    fn from_path(_path: &'a Path) -> Self {
        todo!()
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
            Self::Range(..) | Self::Empty(..) | Self::Constant(..) => None,
        }
    }

    fn get_sexp(&self) -> Option<String> {
        None
    }

    fn position(&self, _language: &GritTargetLanguage) -> Option<Range> {
        match self {
            GritBinding::Node(node) => {
                let source = SourceFile::new(SourceCode {
                    text: node.text(),
                    line_starts: None,
                });
                source.to_grit_range(node.text_trimmed_range())
            }
            GritBinding::Range(range, source) => {
                let source = SourceFile::new(SourceCode {
                    text: source,
                    line_starts: None,
                });
                source.to_grit_range(*range)
            }
            GritBinding::Empty(..) | GritBinding::Constant(_) => None,
        }
    }

    fn range(&self, _language: &GritTargetLanguage) -> Option<ByteRange> {
        match self {
            GritBinding::Node(node) => Some(node.byte_range()),
            GritBinding::Range(range, _) => Some(range.to_byte_range()),
            GritBinding::Empty(..) | GritBinding::Constant(_) => None,
        }
    }

    fn code_range(&self, _language: &GritTargetLanguage) -> Option<CodeRange> {
        match self {
            GritBinding::Node(node) => Some(node.code_range()),
            GritBinding::Range(range, source) => Some(range.to_code_range(source)),
            GritBinding::Empty(..) | GritBinding::Constant(_) => None,
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
            GritBinding::Node(node) => Ok(node.text().into()),
            GritBinding::Range(range, source) => {
                Ok((&source[range.start().into()..range.end().into()]).into())
            }
            GritBinding::Empty(_, _) => Ok("".into()),
            GritBinding::Constant(constant) => Ok(constant.to_string().into()),
        }
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

    fn as_node(&self) -> Option<GritTargetNode<'a>> {
        match self {
            GritBinding::Node(node) => Some(node.clone()),
            GritBinding::Range(..) | GritBinding::Empty(..) | GritBinding::Constant(_) => None,
        }
    }

    fn is_list(&self) -> bool {
        self.as_node().map_or(false, |node| node.is_list())
    }

    fn list_items(&self) -> Option<impl Iterator<Item = GritTargetNode<'a>> + Clone> {
        match self {
            GritBinding::Node(node) if node.is_list() => Some(node.children()),
            _ => None,
        }
    }

    fn parent_node(&self) -> Option<GritTargetNode<'a>> {
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
