use crate::grit_code_snippet::GritCodeSnippet;
use crate::grit_context::GritExecContext;
use crate::grit_file::GritFile;
use crate::grit_target_node::GritTargetNode;
use crate::grit_tree::GritTargetTree;
use crate::GritTargetLanguage;
use crate::{grit_binding::GritBinding, grit_context::GritQueryContext};
use grit_pattern_matcher::binding::Binding;
use grit_pattern_matcher::constant::Constant;
use grit_pattern_matcher::context::{ExecContext, QueryContext};
use grit_pattern_matcher::effects::Effect;
use grit_pattern_matcher::pattern::{
    to_unsigned, Accessor, DynamicPattern, DynamicSnippet, DynamicSnippetPart, File, FilePtr,
    FileRegistry, GritCall, ListIndex, Pattern, PatternName, PatternOrResolved, ResolvedFile,
    ResolvedPattern, ResolvedSnippet, State,
};
use grit_util::error::{GritPatternError, GritResult};
use grit_util::{AnalysisLogs, Ast, CodeRange, Range};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, PartialEq)]
pub enum GritResolvedPattern<'a> {
    Binding(Vec<GritBinding<'a>>),
    Snippets(Vec<ResolvedSnippet<'a, GritQueryContext>>),
    List(Vec<GritResolvedPattern<'a>>),
    Map(BTreeMap<String, GritResolvedPattern<'a>>),
    File(GritFile<'a>),
    Files(Box<GritResolvedPattern<'a>>),
    Constant(Constant),
}

impl<'a> GritResolvedPattern<'a> {
    pub fn from_empty_binding(node: GritTargetNode<'a>, slot_index: u32) -> Self {
        Self::from_binding(GritBinding::Empty(node, slot_index))
    }

    pub fn from_tree(tree: &'a GritTargetTree) -> Self {
        Self::from_binding(GritBinding::from_node(tree.root_node()))
    }

    fn to_snippets(&self) -> GritResult<Vec<ResolvedSnippet<'a, GritQueryContext>>> {
        match self {
            Self::Snippets(snippets) => Ok(snippets.clone()),
            Self::Binding(bindings) => Ok(vec![ResolvedSnippet::from_binding(
                bindings
                    .last()
                    .ok_or_else(|| {
                        GritPatternError::new(
                            "cannot create resolved snippet from unresolved binding",
                        )
                    })?
                    .to_owned(),
            )]),
            Self::List(elements) => {
                // merge separated by space
                let mut snippets = Vec::new();
                for pattern in elements {
                    snippets.extend(pattern.to_snippets()?);
                    snippets.push(ResolvedSnippet::Text(" ".into()));
                }
                snippets.pop();
                Ok(snippets)
            }
            Self::Map(map) => {
                let mut snippets = Vec::new();
                snippets.push(ResolvedSnippet::Text("{".into()));
                for (key, value) in map {
                    snippets.push(ResolvedSnippet::Text(format!("\"{key}\": ").into()));
                    snippets.extend(value.to_snippets()?);
                    snippets.push(ResolvedSnippet::Text(", ".into()));
                }
                snippets.pop();
                snippets.push(ResolvedSnippet::Text("}".into()));
                Ok(snippets)
            }
            Self::File(_) => Err(GritPatternError::new(
                "cannot convert ResolvedPattern::File to ResolvedSnippet",
            )),
            Self::Files(_) => Err(GritPatternError::new(
                "cannot convert ResolvedPattern::Files to ResolvedSnippet",
            )),
            Self::Constant(constant) => {
                Ok(vec![ResolvedSnippet::Text(constant.to_string().into())])
            }
        }
    }
}

impl<'a> ResolvedPattern<'a, GritQueryContext> for GritResolvedPattern<'a> {
    fn from_binding(binding: GritBinding<'a>) -> Self {
        Self::Binding(vec![binding])
    }

    fn from_constant(constant: Constant) -> Self {
        Self::Constant(constant)
    }

    fn from_file_pointer(file: FilePtr) -> Self {
        Self::File(GritFile::Ptr(file))
    }

    fn from_files(files: Self) -> Self {
        Self::Files(Box::new(files))
    }

    fn from_list_parts(parts: impl Iterator<Item = Self>) -> Self {
        Self::List(parts.collect())
    }

    fn from_string(string: String) -> Self {
        Self::Snippets(vec![ResolvedSnippet::Text(string.into())])
    }

    fn from_resolved_snippet(snippet: ResolvedSnippet<'a, GritQueryContext>) -> Self {
        Self::Snippets(vec![snippet])
    }

    fn from_dynamic_snippet(
        snippet: &'a DynamicSnippet,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut grit_util::AnalysisLogs,
    ) -> GritResult<Self> {
        let mut parts = Vec::new();
        for part in &snippet.parts {
            match part {
                DynamicSnippetPart::String(string) => {
                    parts.push(ResolvedSnippet::Text(string.into()));
                }
                DynamicSnippetPart::Variable(var) => {
                    let content = &state.bindings[var.try_scope().unwrap() as usize]
                        .last()
                        .unwrap()[var.try_index().unwrap() as usize];
                    let name = &content.name;
                    // feels weird not sure if clone is correct
                    let value = if let Some(value) = &content.value {
                        value.clone()
                    } else if let Some(pattern) = content.pattern {
                        Self::from_pattern(pattern, state, context, logs)?
                    } else {
                        return Err(GritPatternError::new(format!(
                            "cannot create resolved snippet from unresolved variable {name}"
                        )));
                    };
                    let value = value.to_snippets()?;
                    parts.extend(value);
                }
            }
        }
        Ok(Self::Snippets(parts))
    }

    fn from_dynamic_pattern(
        pattern: &'a DynamicPattern<GritQueryContext>,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Self> {
        match pattern {
            DynamicPattern::Variable(var) => {
                let content = &state.bindings[var.try_scope().unwrap() as usize]
                    .last()
                    .unwrap()[var.try_index().unwrap() as usize];
                let name = &content.name;
                // feels weird not sure if clone is correct
                if let Some(value) = &content.value {
                    Ok(value.clone())
                } else if let Some(pattern) = content.pattern {
                    Self::from_pattern(pattern, state, context, logs)
                } else {
                    return Err(GritPatternError::new(format!(
                        "cannot create resolved snippet from unresolved variable {name}"
                    )));
                }
            }
            DynamicPattern::Accessor(accessor) => {
                Self::from_accessor(accessor, state, context, logs)
            }
            DynamicPattern::ListIndex(index) => Self::from_list_index(index, state, context, logs),
            DynamicPattern::List(list) => {
                let mut elements = Vec::new();
                for element in &list.elements {
                    elements.push(Self::from_dynamic_pattern(element, state, context, logs)?);
                }
                Ok(Self::List(elements))
            }
            DynamicPattern::Snippet(snippet) => {
                Self::from_dynamic_snippet(snippet, state, context, logs)
            }
            DynamicPattern::CallBuiltIn(built_in) => built_in.call(state, context, logs),
            DynamicPattern::CallFunction(func) => func.call(state, context, logs),
            DynamicPattern::CallForeignFunction(_) => unimplemented!(),
        }
    }

    fn from_accessor(
        accessor: &'a Accessor<GritQueryContext>,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Self> {
        match accessor.get(state, context.language())? {
            Some(PatternOrResolved::Pattern(pattern)) => {
                Self::from_pattern(pattern, state, context, logs)
            }
            Some(PatternOrResolved::ResolvedBinding(resolved)) => Ok(resolved),
            Some(PatternOrResolved::Resolved(resolved)) => Ok(resolved.clone()),
            None => Ok(Self::from_constant_binding(&Constant::Undefined)),
        }
    }

    fn from_list_index(
        index: &'a ListIndex<GritQueryContext>,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Self> {
        match index.get(state, context.language())? {
            Some(PatternOrResolved::Pattern(pattern)) => {
                Self::from_pattern(pattern, state, context, logs)
            }
            Some(PatternOrResolved::ResolvedBinding(resolved)) => Ok(resolved),
            Some(PatternOrResolved::Resolved(resolved)) => Ok(resolved.clone()),
            None => Ok(Self::from_constant_binding(&Constant::Undefined)),
        }
    }

    fn from_pattern(
        pattern: &'a Pattern<GritQueryContext>,
        state: &mut State<'a, GritQueryContext>,
        context: &'a GritExecContext,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Self> {
        match pattern {
            Pattern::Dynamic(pattern) => Self::from_dynamic_pattern(pattern, state, context, logs),
            Pattern::CodeSnippet(GritCodeSnippet {
                dynamic_snippet: Some(pattern),
                ..
            }) => Self::from_dynamic_pattern(pattern, state, context, logs),
            Pattern::CallBuiltIn(built_in) => built_in.call(state, context, logs),
            Pattern::CallFunction(func) => func.call(state, context, logs),
            Pattern::CallForeignFunction(_) => unimplemented!(),
            Pattern::CallbackPattern(callback) => Err(GritPatternError::new(format!(
                "cannot make resolved pattern from callback pattern {}",
                callback.name()
            ))),
            Pattern::StringConstant(string) => Ok(Self::Snippets(vec![ResolvedSnippet::Text(
                (&string.text).into(),
            )])),
            Pattern::IntConstant(int) => Ok(Self::Constant(Constant::Integer(int.value))),
            Pattern::FloatConstant(double) => Ok(Self::Constant(Constant::Float(double.value))),
            Pattern::BooleanConstant(bool) => Ok(Self::Constant(Constant::Boolean(bool.value))),
            Pattern::Variable(var) => {
                let content = &state.bindings[var.try_scope().unwrap() as usize]
                    .last()
                    .unwrap()[var.try_index().unwrap() as usize];
                let name = &content.name;
                // feels weird not sure if clone is correct
                if let Some(value) = &content.value {
                    Ok(value.clone())
                } else if let Some(pattern) = content.pattern {
                    Self::from_pattern(pattern, state, context, logs)
                } else {
                    return Err(GritPatternError::new(format!(
                        "cannot create resolved snippet from unresolved variable {name}"
                    )));
                }
            }
            Pattern::List(list) => list
                .patterns
                .iter()
                .map(|pattern| Self::from_pattern(pattern, state, context, logs))
                .collect::<GritResult<Vec<_>>>()
                .map(Self::List),
            Pattern::ListIndex(index) => Self::from_list_index(index, state, context, logs),
            Pattern::Map(map) => map
                .elements
                .iter()
                .map(|(key, value)| {
                    Ok((
                        key.clone(),
                        Self::from_pattern(value, state, context, logs)?,
                    ))
                })
                .collect::<GritResult<BTreeMap<_, _>>>()
                .map(Self::Map),
            Pattern::Accessor(accessor) => Self::from_accessor(accessor, state, context, logs),
            Pattern::File(file_pattern) => {
                let name = &file_pattern.name;
                let body = &file_pattern.body;
                let name = Self::from_pattern(name, state, context, logs)?;
                let name = name.text(&state.files, context.language())?;
                let name = Self::Constant(Constant::String(name.to_string()));
                let body = Self::from_pattern(body, state, context, logs)?;
                Ok(Self::File(GritFile::Resolved(Box::new(ResolvedFile {
                    name,
                    body,
                }))))
            }
            Pattern::Add(add_pattern) => add_pattern.call(state, context, logs),
            Pattern::Subtract(subtract_pattern) => subtract_pattern.call(state, context, logs),
            Pattern::Multiply(multiply_pattern) => multiply_pattern.call(state, context, logs),
            Pattern::Divide(divide_pattern) => divide_pattern.call(state, context, logs),
            Pattern::Modulo(modulo_pattern) => modulo_pattern.call(state, context, logs),
            Pattern::Before(before) => before.prev_pattern(state, context, logs),
            Pattern::After(after) => after.next_pattern(state, context, logs),
            Pattern::AstNode(_)
            | Pattern::CodeSnippet(_)
            | Pattern::Call(_)
            | Pattern::Regex(_)
            | Pattern::Files(_)
            | Pattern::Bubble(_)
            | Pattern::Limit(_)
            | Pattern::Assignment(_)
            | Pattern::Accumulate(_)
            | Pattern::And(_)
            | Pattern::Or(_)
            | Pattern::Maybe(_)
            | Pattern::Any(_)
            | Pattern::Not(_)
            | Pattern::If(_)
            | Pattern::Undefined
            | Pattern::Top
            | Pattern::Bottom
            | Pattern::Underscore
            | Pattern::AstLeafNode(_)
            | Pattern::Rewrite(_)
            | Pattern::Range(_)
            | Pattern::Contains(_)
            | Pattern::Includes(_)
            | Pattern::Within(_)
            | Pattern::Where(_)
            | Pattern::Some(_)
            | Pattern::Every(_)
            | Pattern::Dots
            | Pattern::Like(_)
            | Pattern::Sequential(_) => Err(GritPatternError::new(format!(
                "cannot make resolved pattern from arbitrary pattern {}",
                pattern.name()
            ))),
        }
    }

    fn extend(
        &mut self,
        _with: Self,
        _effects: &mut Vec<Effect<'a, GritQueryContext>>,
        _language: &<GritQueryContext as QueryContext>::Language<'a>,
    ) -> GritResult<()> {
        Err(GritPatternError::new("Not implemented")) // TODO: Implement rewriting
    }

    fn float(
        &self,
        state: &FileRegistry<'a, GritQueryContext>,
        language: &GritTargetLanguage,
    ) -> GritResult<f64> {
        match self {
            Self::Constant(c) => match c {
                Constant::Float(d) => Ok(*d),
                Constant::Integer(i) => Ok(*i as f64),
                Constant::String(s) => Ok(s.parse::<f64>()?),
                Constant::Boolean(_) | Constant::Undefined => Err(GritPatternError::new("Cannot convert constant to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")),
            },
            Self::Snippets(s) => {
                let text = s
                    .iter()
                    .map(|snippet| snippet.text(state, language))
                    .collect::<GritResult<Vec<_>>>()?
                    .join("");
                text.parse::<f64>().map_err(|_| {
                    GritPatternError::new("Failed to convert snippet to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")
                })
            }
            Self::Binding(binding) => {
                let text = binding
                    .last()
                    .ok_or_else(|| GritPatternError::new("cannot grab text of resolved_pattern with no binding"))?
                    .text(language)?;
                text.parse::<f64>().map_err(|_| {
                    GritPatternError::new("Failed to convert binding to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")
                })
            }
            Self::List(_) | Self::Map(_) | Self::File(_) | Self::Files(_) => Err(GritPatternError::new("Cannot convert type to double. Ensure that you are only attempting arithmetic operations on numeric-parsable types.")),
        }
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
        match self {
            Self::Binding(_) => None,
            Self::Snippets(_) => None,
            Self::List(_) => handle_files(self),
            Self::Map(_) => None,
            Self::File(file) => extract_file_pointer(file).map(|f| vec![f]),
            Self::Files(files) => handle_files(files),
            Self::Constant(_) => None,
        }
    }

    fn get_files(&self) -> Option<&Self> {
        if let Self::Files(files) = self {
            Some(files)
        } else {
            None
        }
    }

    fn get_last_binding(&self) -> Option<&GritBinding<'a>> {
        if let Self::Binding(bindings) = self {
            bindings.last()
        } else {
            None
        }
    }

    fn get_list_item_at(&self, index: isize) -> Option<&Self> {
        if let Self::List(items) = self {
            to_unsigned(index, items.len()).and_then(|index| items.get(index))
        } else {
            None
        }
    }

    fn get_list_item_at_mut(&mut self, index: isize) -> Option<&mut Self> {
        if let Self::List(items) = self {
            to_unsigned(index, items.len()).and_then(|index| items.get_mut(index))
        } else {
            None
        }
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
        state: &mut State<'a, GritQueryContext>,
        language: &GritTargetLanguage,
    ) -> GritResult<bool> {
        let truthiness = match self {
            Self::Binding(bindings) => bindings.last().is_some_and(Binding::is_truthy),
            Self::List(elements) => !elements.is_empty(),
            Self::Map(map) => !map.is_empty(),
            Self::Constant(c) => c.is_truthy(),
            Self::Snippets(s) => {
                if let Some(s) = s.last() {
                    s.is_truthy(state, language)?
                } else {
                    false
                }
            }
            Self::File(..) => true,
            Self::Files(..) => true,
        };
        Ok(truthiness)
    }

    fn linearized_text(
        &self,
        _language: &GritTargetLanguage,
        _effects: &[Effect<'a, GritQueryContext>],
        _files: &FileRegistry<'a, GritQueryContext>,
        _memo: &mut HashMap<CodeRange, Option<String>>,
        _should_pad_snippet: bool,
        _logs: &mut AnalysisLogs,
    ) -> GritResult<Cow<'a, str>> {
        Err(GritPatternError::new("Not implemented")) // TODO: Implement rewriting
    }

    fn matches_undefined(&self) -> bool {
        match self {
            Self::Binding(b) => b
                .last()
                .and_then(Binding::as_constant)
                .is_some_and(Constant::is_undefined),
            Self::Constant(Constant::Undefined) => true,
            Self::Constant(_)
            | Self::Snippets(_)
            | Self::List(_)
            | Self::Map(_)
            | Self::File(_)
            | Self::Files(_) => false,
        }
    }

    fn matches_false_or_undefined(&self) -> bool {
        // should this match a binding to the constant `false` as well?
        matches!(self, Self::Constant(Constant::Boolean(false))) || self.matches_undefined()
    }

    fn normalize_insert(
        &mut self,
        _binding: &GritBinding,
        _is_first: bool,
        _language: &GritTargetLanguage,
    ) -> GritResult<()> {
        Err(GritPatternError::new("Not implemented")) // TODO: Implement insertion padding
    }

    fn position(&self, language: &GritTargetLanguage) -> Option<Range> {
        if let Self::Binding(binding) = self {
            if let Some(binding) = binding.last() {
                return binding.position(language);
            }
        }

        None
    }

    fn push_binding(&mut self, binding: GritBinding<'a>) -> GritResult<()> {
        let Self::Binding(bindings) = self else {
            return Err(GritPatternError::new("can only push to bindings"));
        };

        bindings.push(binding);
        Ok(())
    }

    fn set_list_item_at_mut(&mut self, index: isize, value: Self) -> GritResult<bool> {
        let Self::List(items) = self else {
            return Err(GritPatternError::new("can only set items on a list"));
        };

        let Some(index) = to_unsigned(index, items.len()) else {
            return Ok(false);
        };

        items.insert(index, value);
        Ok(true)
    }

    fn text(
        &self,
        state: &FileRegistry<'a, GritQueryContext>,
        language: &GritTargetLanguage,
    ) -> GritResult<Cow<'a, str>> {
        match self {
            Self::Binding(bindings) => Ok(bindings
                .last()
                .ok_or_else(|| {
                    GritPatternError::new("cannot grab text of resolved_pattern with no binding")
                })?
                .text(language)?
                .into_owned()
                .into()),
            Self::Snippets(snippets) => Ok(snippets
                .iter()
                .try_fold(String::new(), |mut text, snippet| {
                    text.push_str(&snippet.text(state, language)?);
                    Ok::<String, GritPatternError>(text)
                })?
                .into()),
            Self::List(list) => Ok(list
                .iter()
                .map(|pattern| pattern.text(state, language))
                .collect::<GritResult<Vec<_>>>()?
                .join(",")
                .into()),
            Self::Map(map) => Ok(format!(
                "{{{}}}",
                map.iter()
                    .map(|(key, value)| {
                        let value = value
                            .text(state, language)
                            .expect("failed to get text of map value");
                        format!("\"{key}\": {value}")
                    })
                    .reduce(|mut acc, entry| {
                        acc.push_str(", ");
                        acc.push_str(&entry);
                        acc
                    })
                    .unwrap_or_default()
            )
            .into()),
            Self::File(file) => Ok(format!(
                "{}:\n{}",
                file.name(state).text(state, language)?,
                file.body(state).text(state, language)?
            )
            .into()),
            Self::Files(files) => files.text(state, language),
            Self::Constant(constant) => Ok(constant.to_string().into()),
        }
    }
}

fn extract_file_pointer(file: &GritFile) -> Option<FilePtr> {
    match file {
        GritFile::Resolved(_) => None,
        GritFile::Ptr(ptr) => Some(*ptr),
    }
}

fn handle_files(files_list: &GritResolvedPattern) -> Option<Vec<FilePtr>> {
    let GritResolvedPattern::List(files) = files_list else {
        return None;
    };

    files
        .iter()
        .map(|resolved| {
            if let GritResolvedPattern::File(GritFile::Ptr(ptr)) = resolved {
                Some(*ptr)
            } else {
                None
            }
        })
        .collect()
}
