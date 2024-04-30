use crate::{grit_context::GritQueryContext, grit_language::GritLanguage, grit_node::GritNode};
use grit_pattern_matcher::{binding::Binding, constant::Constant};
use grit_util::{ByteRange, CodeRange, Range};
use std::path::Path;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GritBinding;

impl<'a> Binding<'a, GritQueryContext> for GritBinding {
    fn from_constant(_constant: &'a Constant) -> Self {
        todo!()
    }

    fn from_node(_node: GritNode) -> Self {
        todo!()
    }

    fn from_path(_path: &'a Path) -> Self {
        todo!()
    }

    fn from_range(_range: ByteRange, _source: &'a str) -> Self {
        todo!()
    }

    fn singleton(&self) -> Option<GritNode> {
        todo!()
    }

    fn get_sexp(&self) -> Option<String> {
        todo!()
    }

    fn position(&self, _language: &GritLanguage) -> Option<Range> {
        todo!()
    }

    fn range(&self, _language: &GritLanguage) -> Option<ByteRange> {
        todo!()
    }

    fn code_range(&self, _language: &GritLanguage) -> Option<CodeRange> {
        todo!()
    }

    fn is_equivalent_to(&self, _other: &Self, _language: &GritLanguage) -> bool {
        todo!()
    }

    fn is_suppressed(&self, _language: &GritLanguage, _current_name: Option<&str>) -> bool {
        todo!()
    }

    fn get_insertion_padding(
        &self,
        _text: &str,
        _is_first: bool,
        _language: &GritLanguage,
    ) -> Option<String> {
        todo!()
    }

    fn linearized_text(
        &self,
        _language: &GritLanguage,
        _effects: &[grit_pattern_matcher::effects::Effect<'a, GritQueryContext>],
        _files: &grit_pattern_matcher::pattern::FileRegistry<'a, GritQueryContext>,
        _memo: &mut std::collections::HashMap<grit_util::CodeRange, Option<String>>,
        _distributed_indent: Option<usize>,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<std::borrow::Cow<'a, str>> {
        todo!()
    }

    fn text(&self, _language: &GritLanguage) -> anyhow::Result<std::borrow::Cow<str>> {
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

    fn as_node(&self) -> Option<GritNode> {
        todo!()
    }

    fn is_list(&self) -> bool {
        todo!()
    }

    fn list_items(&self) -> Option<impl Iterator<Item = GritNode> + Clone> {
        None::<TodoIterator>
    }

    fn parent_node(&self) -> Option<GritNode> {
        todo!()
    }

    fn is_truthy(&self) -> bool {
        todo!()
    }

    fn log_empty_field_rewrite_error(
        &self,
        _language: &GritLanguage,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoIterator;

impl Iterator for TodoIterator {
    type Item = GritNode;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
