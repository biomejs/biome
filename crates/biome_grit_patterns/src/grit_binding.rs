use crate::{
    grit_context::GritQueryContext, grit_target_language::GritTargetLanguage,
    grit_target_node::GritTargetNode, source_location_ext::SourceFileExt, util::TextRangeGritExt,
};
use biome_diagnostics::{display::SourceFile, SourceCode};
use biome_rowan::TextRange;
use grit_pattern_matcher::{
    binding::Binding, constant::Constant, effects::Effect, pattern::FileRegistry,
};
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogBuilder, AnalysisLogs, AstNode, ByteRange, CodeRange, Range,
};
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
    /// Returns `None` if the binding has no associated node, or if there is
    /// more than one associated node.
    fn singleton(&self) -> Option<GritTargetNode<'a>> {
        match self {
            Self::Node(node) => {
                if node.is_list() {
                    let mut children = node.named_children();
                    match (children.next(), children.next()) {
                        (Some(only_child), None) => Some(only_child),
                        _ => None,
                    }
                } else {
                    Some(node.clone())
                }
            }
            Self::File(..) | Self::Range(..) | Self::Empty(..) | Self::Constant(..) => None,
        }
    }

    fn get_sexp(&self) -> Option<String> {
        Some(match self {
            Self::File(path) => format!("({})", path.display()),
            Self::Node(grit_target_node) => format!("({grit_target_node:?})"),
            Self::Range(text_range, source) => format!(
                "({})",
                &source[text_range.start().into()..text_range.end().into()]
            ),
            Self::Empty(_, _) => "(empty)".to_owned(),
            Self::Constant(constant) => format!("({constant})"),
        })
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

    fn is_equivalent_to(&self, other: &Self, language: &GritTargetLanguage) -> bool {
        match self {
            Self::Node(node1) => match other {
                Self::Node(node2) => are_equivalent(node1, node2),
                Self::Range(range, source) => self
                    .text(language)
                    .is_ok_and(|t| t == source[range.start().into()..range.end().into()]),
                Self::File(_) | Self::Empty(..) | Self::Constant(_) => false,
            },
            Self::Empty(node1, sort1) => match other {
                Self::Empty(node2, sort2) => node1.kind() == node2.kind() && sort1 == sort2,
                Self::Range(..) | Self::File(_) | Self::Node(..) | Self::Constant(_) => false,
            },
            Self::Constant(c1) => other.as_constant().is_some_and(|c2| *c1 == c2),
            Self::Range(range, source) => other
                .text(language)
                .is_ok_and(|t| t == source[range.start().into()..range.end().into()]),
            Self::File(path1) => other.as_filename().is_some_and(|path2| *path1 == path2),
        }
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
        None // TODO: Implement insertion padding
    }

    fn linearized_text(
        &self,
        _language: &GritTargetLanguage,
        _effects: &[Effect<'a, GritQueryContext>],
        _files: &FileRegistry<'a, GritQueryContext>,
        _memo: &mut HashMap<grit_util::CodeRange, Option<String>>,
        _distributed_indent: Option<usize>,
        _logs: &mut AnalysisLogs,
    ) -> GritResult<Cow<'a, str>> {
        Err(GritPatternError::new("Not implemented")) // TODO: Implement rewriting
    }

    fn text(&self, _language: &GritTargetLanguage) -> GritResult<Cow<'a, str>> {
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
        match self {
            Self::Node(node) => node.is_list(),
            _ => false,
        }
    }

    fn list_items(&self) -> Option<impl Iterator<Item = GritTargetNode<'a>> + Clone> {
        match self {
            Self::Node(node) if node.is_list() => Some(node.named_children()),
            _ => None,
        }
    }

    fn parent_node(&self) -> Option<GritTargetNode<'a>> {
        match self {
            GritBinding::Node(node) => node.parent(),
            GritBinding::Empty(node, _) => Some(node.clone()),
            GritBinding::File(_) | GritBinding::Range(_, _) | GritBinding::Constant(_) => None,
        }
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
        logs: &mut grit_util::AnalysisLogs,
    ) -> GritResult<()> {
        if let Self::Empty(node, slot) = self {
            let range = Range::from_byte_range(node.source(), &node.byte_range());
            let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .source(node.source())
                .position(range.start)
                .range(range)
                .message(format!(
                    "Error: failed to rewrite binding, cannot derive range of empty slot {slot} of node with kind {:?}",
                    node.kind()
                ))
                .build()
                .map_err(|error| GritPatternError::Builder(error.to_string()))?;
            logs.push(log);
        }

        Ok(())
    }
}

/// Checks whether two nodes are equivalent.
///
/// We define two nodes to be equivalent if they have the same sort (kind) and
/// equivalent named fields.
///
/// TODO: Improve performance. Equivalence checks happen often so we want them to
/// be fast. The current implementation requires a traversal of the tree on all
/// named fields, which can be slow for large nodes. It also creates a cursor
/// at each traversal step.
///
/// Potential improvements:
/// 1. Use cursors that are passed as arguments -- not clear if this would be faster.
/// 2. Precompute hashes on all nodes, which define the equivalence relation. The check then becomes O(1).
fn are_equivalent(node1: &GritTargetNode, node2: &GritTargetNode) -> bool {
    // If the source is identical, we consider the nodes equivalent.
    // This covers most cases of constant nodes.
    // We may want a more precise check here eventually, but this is a good start.
    if node1.text() == node2.text() {
        return true;
    }

    // If the node kinds are different, then the nodes are not equivalent,
    // except in the presence of lists:
    // - If both are a list, we don't care about the kind of the list, we just
    //   compare the nodes individually.
    // - If one of them is a list with a single node, we may still find a
    //   match against that node.
    if node1.kind() != node2.kind() {
        return if node1.is_list() {
            if node2.is_list() {
                let mut children1 = node1.named_children();
                let mut children2 = node2.named_children();
                loop {
                    match (children1.next(), children2.next()) {
                        (Some(child1), Some(child2)) => {
                            if !are_equivalent(&child1, &child2) {
                                break false;
                            }
                        }
                        (None, None) => break true,
                        _ => break false,
                    }
                }
            } else {
                let mut children1 = node1.named_children();
                match (children1.next(), children1.next()) {
                    (Some(only_child), None) => are_equivalent(&only_child, node2),
                    _ => false,
                }
            }
        } else if node2.is_list() {
            let mut children2 = node2.named_children();
            match (children2.next(), children2.next()) {
                (Some(only_child), None) => are_equivalent(node1, &only_child),
                _ => false,
            }
        } else {
            false
        };
    }

    // If the node kinds are the same, then we need to check the named fields.
    let named_fields1 = node1.named_children();
    let mut named_fields2 = node2.named_children();

    // If there are no children, this is effectively a leaf node. If two leaf
    // nodes have different sources (see above), then they are not equivalent.
    // If they do not have the same sources, we consider them different.
    let mut is_empty = true;

    // Recurse through the named fields to find the first mismatch.
    for child1 in named_fields1 {
        is_empty = false;

        match named_fields2.next() {
            Some(child2) => {
                if !are_equivalent(&child1, &child2) {
                    return false;
                }
            }
            None => return false,
        }
    }

    named_fields2.next().is_none() && !is_empty
}
