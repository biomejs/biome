use crate::grit_context::GritExecContext;
use crate::{grit_binding::GritBinding, grit_context::GritQueryContext};
use anyhow::Result;
use grit_pattern_matcher::constant::Constant;
use grit_pattern_matcher::effects::Effect;
use grit_pattern_matcher::pattern::{
    Accessor, DynamicPattern, DynamicSnippet, FilePtr, FileRegistry, ListIndex, Pattern,
    ResolvedPattern, ResolvedSnippet, State,
};
use grit_util::{AnalysisLogs, CodeRange, Range};
use im::Vector;
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct GritResolvedPattern;

impl<'a> ResolvedPattern<'a, GritQueryContext> for GritResolvedPattern {
    fn from_binding(_binding: GritBinding) -> Self {
        todo!()
    }

    fn from_constant(_constant: Constant) -> Self {
        todo!()
    }

    fn from_file_pointer(_file: FilePtr) -> Self {
        todo!()
    }

    fn from_files(_files: Self) -> Self {
        todo!()
    }

    fn from_list_parts(_parts: impl Iterator<Item = Self>) -> Self {
        todo!()
    }

    fn from_string(_string: String) -> Self {
        todo!()
    }

    fn from_resolved_snippet(_snippet: ResolvedSnippet<'a, GritQueryContext>) -> Self {
        todo!()
    }

    fn from_dynamic_snippet(
        _snippet: &'a DynamicSnippet,
        _state: &mut State<'a, GritQueryContext>,
        _context: &'a GritExecContext,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    fn from_dynamic_pattern(
        _pattern: &'a DynamicPattern<GritQueryContext>,
        _state: &mut State<'a, GritQueryContext>,
        _context: &'a GritExecContext,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    fn from_accessor(
        _accessor: &'a Accessor<GritQueryContext>,
        _state: &mut State<'a, GritQueryContext>,
        _context: &'a GritExecContext,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    fn from_list_index(
        _index: &'a ListIndex<GritQueryContext>,
        _state: &mut State<'a, GritQueryContext>,
        _context: &'a GritExecContext,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    fn from_pattern(
        _pattern: &'a Pattern<GritQueryContext>,
        _state: &mut State<'a, GritQueryContext>,
        _context: &'a GritExecContext,
        _logs: &mut grit_util::AnalysisLogs,
    ) -> anyhow::Result<Self> {
        todo!()
    }

    fn extend(
        &mut self,
        _with: Self,
        _effects: &mut Vector<Effect<'a, GritQueryContext>>,
        _language: &<GritQueryContext as grit_pattern_matcher::context::QueryContext>::Language<'a>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn float(
        &self,
        _state: &FileRegistry<'a, GritQueryContext>,
        _language: &<GritQueryContext as grit_pattern_matcher::context::QueryContext>::Language<'a>,
    ) -> anyhow::Result<f64> {
        todo!()
    }

    fn get_bindings(&self) -> Option<impl Iterator<Item = GritBinding>> {
        None::<TodoBindingIterator>
    }

    fn get_file(
        &self,
    ) -> Option<&<GritQueryContext as grit_pattern_matcher::context::QueryContext>::File<'a>> {
        todo!()
    }

    fn get_file_pointers(&self) -> Option<Vec<FilePtr>> {
        todo!()
    }

    fn get_files(&self) -> Option<&Self> {
        todo!()
    }

    fn get_last_binding(&self) -> Option<&GritBinding> {
        todo!()
    }

    fn get_list_item_at(&self, _index: isize) -> Option<&Self> {
        todo!()
    }

    fn get_list_item_at_mut(&mut self, _index: isize) -> Option<&mut Self> {
        todo!()
    }

    fn get_list_items(&self) -> Option<impl Iterator<Item = &Self>> {
        None::<TodoSelfRefIterator>
    }

    fn get_list_binding_items(&self) -> Option<impl Iterator<Item = Self> + Clone> {
        None::<TodoSelfIterator>
    }

    fn get_map(&self) -> Option<&std::collections::BTreeMap<String, Self>> {
        todo!()
    }

    fn get_map_mut(&mut self) -> Option<&mut std::collections::BTreeMap<String, Self>> {
        todo!()
    }

    fn get_snippets(&self) -> Option<impl Iterator<Item = ResolvedSnippet<'a, GritQueryContext>>> {
        None::<TodoSnippetIterator>
    }

    fn is_binding(&self) -> bool {
        todo!()
    }

    fn is_list(&self) -> bool {
        todo!()
    }

    fn is_truthy(
        &self,
        _state: &mut State<'a, GritQueryContext>,
        _language: &<GritQueryContext as grit_pattern_matcher::context::QueryContext>::Language<'a>,
    ) -> Result<bool> {
        todo!()
    }

    fn linearized_text(
        &self,
        _language: &<GritQueryContext as grit_pattern_matcher::context::QueryContext>::Language<'a>,
        _effects: &[Effect<'a, GritQueryContext>],
        _files: &FileRegistry<'a, GritQueryContext>,
        _memo: &mut HashMap<CodeRange, Option<String>>,
        _should_pad_snippet: bool,
        _logs: &mut AnalysisLogs,
    ) -> Result<std::borrow::Cow<'a, str>> {
        todo!()
    }

    fn matches_undefined(&self) -> bool {
        todo!()
    }

    fn matches_false_or_undefined(&self) -> bool {
        todo!()
    }

    fn normalize_insert(
        &mut self,
        _binding: &GritBinding,
        _is_first: bool,
        _language: &<GritQueryContext as grit_pattern_matcher::context::QueryContext>::Language<'a>,
    ) -> Result<()> {
        todo!()
    }

    fn position(
        &self,
        _language: &<GritQueryContext as grit_pattern_matcher::context::QueryContext>::Language<'a>,
    ) -> Option<Range> {
        todo!()
    }

    fn push_binding(&mut self, _binding: GritBinding) -> Result<()> {
        todo!()
    }

    fn set_list_item_at_mut(&mut self, _index: isize, _value: Self) -> anyhow::Result<bool> {
        todo!()
    }

    fn text(
        &self,
        _state: &grit_pattern_matcher::pattern::FileRegistry<'a, GritQueryContext>,
        _language: &<GritQueryContext as grit_pattern_matcher::context::QueryContext>::Language<'a>,
    ) -> Result<Cow<'a, str>> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoBindingIterator;

impl Iterator for TodoBindingIterator {
    type Item = GritBinding;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoSelfIterator;

impl Iterator for TodoSelfIterator {
    type Item = GritResolvedPattern;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

struct TodoSelfRefIterator<'a> {
    _pattern: &'a GritResolvedPattern,
}

impl<'a> Iterator for TodoSelfRefIterator<'a> {
    type Item = &'a GritResolvedPattern;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoSnippetIterator<'a> {
    _pattern: &'a GritResolvedPattern,
}

impl<'a> Iterator for TodoSnippetIterator<'a> {
    type Item = ResolvedSnippet<'a, GritQueryContext>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
