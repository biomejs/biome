use crate::grit_context::GritExecContext;
use crate::grit_file::GritFile;
use crate::grit_tree::GritTree;
use crate::{grit_binding::GritBinding, grit_context::GritQueryContext};
use anyhow::Result;
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::constant::Constant;
use grit_pattern_matcher::context::QueryContext;
use grit_pattern_matcher::effects::Effect;
use grit_pattern_matcher::pattern::{
    Accessor, DynamicPattern, DynamicSnippet, FilePtr, FileRegistry, ListIndex, Pattern,
    ResolvedPattern, ResolvedSnippet, State,
};
use grit_util::{AnalysisLogs, CodeRange, Range};
use im::{vector, Vector};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum GritResolvedPattern<'a> {
    Binding(Vector<GritBinding<'a>>),
    Snippets(Vector<ResolvedSnippet<'a, GritQueryContext>>),
    List(Vector<GritResolvedPattern<'a>>),
    Map(BTreeMap<String, GritResolvedPattern<'a>>),
    File(GritFile<'a>),
    Files(Box<GritResolvedPattern<'a>>),
    Constant(Constant),
    Tree(GritTree),
}

impl<'a> GritResolvedPattern<'a> {
    pub fn from_tree(tree: &'a GritTree) -> Self {
        Self::from_binding(GritBinding::from_tree(tree))
    }
}

impl<'a> ResolvedPattern<'a, GritQueryContext> for GritResolvedPattern<'a> {
    fn from_binding(binding: GritBinding<'a>) -> Self {
        Self::Binding(vector![binding])
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
        _language: &<GritQueryContext as QueryContext>::Language<'a>,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn float(
        &self,
        _state: &FileRegistry<'a, GritQueryContext>,
        _language: &<GritQueryContext as QueryContext>::Language<'a>,
    ) -> anyhow::Result<f64> {
        todo!()
    }

    fn get_bindings(&self) -> Option<impl Iterator<Item = GritBinding<'a>>> {
        if let Self::Binding(bindings) = self {
            Some(bindings.iter().cloned())
        } else {
            None
        }
    }

    fn get_file(&self) -> Option<&GritFile<'a>> {
        if let Self::File(file) = self {
            Some(file)
        } else {
            None
        }
    }

    fn get_file_pointers(&self) -> Option<Vec<FilePtr>> {
        todo!()
    }

    fn get_files(&self) -> Option<&Self> {
        todo!()
    }

    fn get_last_binding(&self) -> Option<&GritBinding<'a>> {
        if let Self::Binding(bindings) = self {
            bindings.last()
        } else {
            None
        }
    }

    fn get_list_item_at(&self, _index: isize) -> Option<&Self> {
        todo!()
    }

    fn get_list_item_at_mut(&mut self, _index: isize) -> Option<&mut Self> {
        todo!()
    }

    fn get_list_items(&self) -> Option<impl Iterator<Item = &Self>> {
        if let Self::List(items) = self {
            Some(items.iter())
        } else {
            None
        }
    }

    fn get_list_binding_items(&self) -> Option<impl Iterator<Item = Self> + Clone> {
        self.get_last_binding()
            .and_then(Binding::list_items)
            .map(|items| items.map(GritResolvedPattern::from_node_binding))
    }

    fn get_map(&self) -> Option<&std::collections::BTreeMap<String, Self>> {
        if let Self::Map(map) = self {
            Some(map)
        } else {
            None
        }
    }

    fn get_map_mut(&mut self) -> Option<&mut std::collections::BTreeMap<String, Self>> {
        if let Self::Map(map) = self {
            Some(map)
        } else {
            None
        }
    }

    fn get_snippets(&self) -> Option<impl Iterator<Item = ResolvedSnippet<'a, GritQueryContext>>> {
        if let Self::Snippets(snippets) = self {
            Some(snippets.iter().cloned())
        } else {
            None
        }
    }

    fn is_binding(&self) -> bool {
        matches!(self, Self::Binding(_))
    }

    fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }

    fn is_truthy(
        &self,
        _state: &mut State<'a, GritQueryContext>,
        _language: &<GritQueryContext as QueryContext>::Language<'a>,
    ) -> Result<bool> {
        todo!()
    }

    fn linearized_text(
        &self,
        _language: &<GritQueryContext as QueryContext>::Language<'a>,
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
        _language: &<GritQueryContext as QueryContext>::Language<'a>,
    ) -> Result<()> {
        todo!()
    }

    fn position(
        &self,
        _language: &<GritQueryContext as QueryContext>::Language<'a>,
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
        _language: &<GritQueryContext as QueryContext>::Language<'a>,
    ) -> Result<Cow<'a, str>> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoBindingIterator<'a> {
    _pattern: &'a GritResolvedPattern<'a>,
}

impl<'a> Iterator for TodoBindingIterator<'a> {
    type Item = GritBinding<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoSelfIterator<'a> {
    _pattern: &'a GritResolvedPattern<'a>,
}

impl<'a> Iterator for TodoSelfIterator<'a> {
    type Item = GritResolvedPattern<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

struct TodoSelfRefIterator<'a> {
    _pattern: &'a GritResolvedPattern<'a>,
}

impl<'a> Iterator for TodoSelfRefIterator<'a> {
    type Item = &'a GritResolvedPattern<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[derive(Clone)]
struct TodoSnippetIterator<'a> {
    _pattern: &'a GritResolvedPattern<'a>,
}

impl<'a> Iterator for TodoSnippetIterator<'a> {
    type Item = ResolvedSnippet<'a, GritQueryContext>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
