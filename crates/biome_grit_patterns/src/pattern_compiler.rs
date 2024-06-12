pub(crate) mod compilation_context;

/// Creates a new scope within the given `context`.
///
/// This is implemented as a macro instead of method to avoid capturing the
/// entire `context` instance, which would run afoul of the borrow-checking due
/// to its mutable references.
macro_rules! create_scope {
    ($context: expr, $local_vars: expr) => {{
        let scope_index = $context.vars_array.len();
        $context.vars_array.push(Vec::new());
        let context = crate::pattern_compiler::NodeCompilationContext {
            compilation: $context.compilation,
            vars: &mut $local_vars,
            vars_array: $context.vars_array,
            scope_index,
            global_vars: $context.global_vars,
            diagnostics: $context.diagnostics,
        };
        (scope_index, context)
    }};
}

mod accumulate_compiler;
mod add_compiler;
mod after_compiler;
mod and_compiler;
mod any_compiler;
mod as_compiler;
mod assignment_compiler;
mod auto_wrap;
mod before_compiler;
mod bubble_compiler;
mod container_compiler;
mod contains_compiler;
mod divide_compiler;
mod equal_compiler;
mod every_compiler;
mod if_compiler;
mod includes_compiler;
mod like_compiler;
mod limit_compiler;
mod list_compiler;
mod list_index_compiler;
mod literal_compiler;
mod map_accessor_compiler;
mod map_compiler;
mod match_compiler;
mod maybe_compiler;
mod modulo_compiler;
mod multiply_compiler;
mod not_compiler;
mod or_compiler;
mod predicate_compiler;
mod predicate_return_compiler;
mod rewrite_compiler;
mod sequential_compiler;
mod snippet_compiler;
mod some_compiler;
mod step_compiler;
mod subtract_compiler;
mod variable_compiler;
mod where_compiler;
mod within_compiler;

use std::collections::BTreeMap;

use self::{
    accumulate_compiler::AccumulateCompiler, add_compiler::AddCompiler,
    after_compiler::AfterCompiler, and_compiler::AndCompiler, any_compiler::AnyCompiler,
    assignment_compiler::AssignmentCompiler, before_compiler::BeforeCompiler,
    bubble_compiler::BubbleCompiler, compilation_context::NodeCompilationContext,
    contains_compiler::ContainsCompiler, divide_compiler::DivideCompiler,
    every_compiler::EveryCompiler, if_compiler::IfCompiler, includes_compiler::IncludesCompiler,
    like_compiler::LikeCompiler, limit_compiler::LimitCompiler,
    list_index_compiler::ListIndexCompiler, literal_compiler::LiteralCompiler,
    map_accessor_compiler::MapAccessorCompiler, maybe_compiler::MaybeCompiler,
    modulo_compiler::ModuloCompiler, multiply_compiler::MultiplyCompiler,
    not_compiler::NotCompiler, or_compiler::OrCompiler, rewrite_compiler::RewriteCompiler,
    sequential_compiler::SequentialCompiler, some_compiler::SomeCompiler,
    subtract_compiler::SubtractCompiler, variable_compiler::VariableCompiler,
    where_compiler::WhereCompiler, within_compiler::WithinCompiler,
};
use crate::{
    grit_context::GritQueryContext,
    grit_node_patterns::{GritLeafNodePattern, GritNodeArg, GritNodePattern},
    grit_target_node::{GritSyntaxSlot, GritTargetNode, GritTargetToken},
    CompileError, GritTargetLanguage,
};
use biome_grit_syntax::{AnyGritMaybeCurlyPattern, AnyGritPattern, GritSyntaxKind};
use biome_rowan::AstNode as _;
use grit_pattern_matcher::pattern::{
    is_reserved_metavariable, DynamicPattern, DynamicSnippet, DynamicSnippetPart, List, Pattern,
    RegexLike, RegexPattern, Variable,
};
use grit_util::{traverse, AstNode, ByteRange, GritMetaValue, Language, Order};

pub(crate) struct PatternCompiler;

impl PatternCompiler {
    pub(crate) fn from_node(
        node: &AnyGritPattern,
        context: &mut NodeCompilationContext,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        Self::from_node_with_rhs(node, context, false)
    }

    pub(crate) fn from_maybe_curly_node(
        node: &AnyGritMaybeCurlyPattern,
        context: &mut NodeCompilationContext,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        match node {
            AnyGritMaybeCurlyPattern::AnyGritPattern(pattern) => Self::from_node(pattern, context),
            AnyGritMaybeCurlyPattern::GritCurlyPattern(pattern) => {
                Self::from_node(&pattern.pattern()?, context)
            }
        }
    }

    fn from_node_with_rhs(
        node: &AnyGritPattern,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        match node {
            AnyGritPattern::AnyGritLiteral(node) => {
                LiteralCompiler::from_node_with_rhs(node, context, is_rhs)
            }
            AnyGritPattern::GritAddOperation(node) => Ok(Pattern::Add(Box::new(
                AddCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritAssignmentAsPattern(node) => Ok(Pattern::Assignment(Box::new(
                AssignmentCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritBracketedPattern(node) => {
                Self::from_node_with_rhs(&node.pattern()?, context, is_rhs)
            }
            AnyGritPattern::GritBubble(node) => Ok(Pattern::Bubble(Box::new(
                BubbleCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritDivOperation(node) => Ok(Pattern::Divide(Box::new(
                DivideCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritDot(_) => {
                Ok(Pattern::Dynamic(DynamicPattern::Snippet(DynamicSnippet {
                    parts: vec![DynamicSnippetPart::String(String::new())],
                })))
            }
            AnyGritPattern::GritEvery(node) => Ok(Pattern::Every(Box::new(
                EveryCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritFiles(node) => Ok(Pattern::Sequential(
                SequentialCompiler::from_files_node(node, context)?,
            )),
            AnyGritPattern::GritLike(node) => Ok(Pattern::Like(Box::new(LikeCompiler::from_node(
                node, context,
            )?))),
            AnyGritPattern::GritListAccessor(node) => Ok(Pattern::ListIndex(Box::new(
                ListIndexCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritMapAccessor(node) => Ok(Pattern::Accessor(Box::new(
                MapAccessorCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritModOperation(node) => Ok(Pattern::Modulo(Box::new(
                ModuloCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritMulOperation(node) => Ok(Pattern::Multiply(Box::new(
                MultiplyCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritNodeLike(_) => todo!(),
            AnyGritPattern::GritPatternAccumulate(node) => Ok(Pattern::Accumulate(Box::new(
                AccumulateCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAfter(node) => Ok(Pattern::After(Box::new(
                AfterCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAnd(node) => Ok(Pattern::And(Box::new(
                AndCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAny(node) => Ok(Pattern::Any(Box::new(
                AnyCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternAs(_) => todo!(),
            AnyGritPattern::GritPatternBefore(node) => Ok(Pattern::Before(Box::new(
                BeforeCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternContains(node) => Ok(Pattern::Contains(Box::new(
                ContainsCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternIfElse(node) => {
                Ok(Pattern::If(Box::new(IfCompiler::from_node(node, context)?)))
            }
            AnyGritPattern::GritPatternIncludes(node) => Ok(Pattern::Includes(Box::new(
                IncludesCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternLimit(node) => Ok(Pattern::Limit(Box::new(
                LimitCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternMaybe(node) => Ok(Pattern::Maybe(Box::new(
                MaybeCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternNot(node) => Ok(Pattern::Not(Box::new(
                NotCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritPatternOr(node) => {
                Ok(Pattern::Or(Box::new(OrCompiler::from_node(node, context)?)))
            }
            AnyGritPattern::GritPatternOrElse(node) => {
                Err(CompileError::UnsupportedKind(node.syntax().kind().into()))
            }
            AnyGritPattern::GritPatternWhere(node) => Ok(Pattern::Where(Box::new(
                WhereCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritRegexPattern(_) => todo!(),
            AnyGritPattern::GritRewrite(node) => Ok(Pattern::Rewrite(Box::new(
                RewriteCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritSequential(node) => Ok(Pattern::Sequential(
                SequentialCompiler::from_node(node, context)?,
            )),
            AnyGritPattern::GritSome(node) => Ok(Pattern::Some(Box::new(SomeCompiler::from_node(
                node, context,
            )?))),
            AnyGritPattern::GritSubOperation(node) => Ok(Pattern::Subtract(Box::new(
                SubtractCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritUnderscore(_) => Ok(Pattern::Underscore),
            AnyGritPattern::GritVariable(node) => Ok(Pattern::Variable(
                VariableCompiler::from_node(node, context)?,
            )),
            AnyGritPattern::GritWithin(node) => Ok(Pattern::Within(Box::new(
                WithinCompiler::from_node(node, context)?,
            ))),
            AnyGritPattern::GritBogusPattern(_) => Err(CompileError::UnexpectedKind(
                GritSyntaxKind::GRIT_BOGUS_PATTERN.into(),
            )),
        }
    }
}

impl PatternCompiler {
    pub(crate) fn from_snippet_node(
        node: GritTargetNode,
        context_range: ByteRange,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        let snippet_start = node.text().char_at(0.into()).unwrap_or_default() as usize;
        let ranges = metavariable_ranges(&node, &context.compilation.lang);
        let range_map = metavariable_range_mapping(ranges, snippet_start);

        fn node_to_pattern(
            node: GritTargetNode,
            context_range: ByteRange,
            range_map: &BTreeMap<ByteRange, ByteRange>,
            context: &mut NodeCompilationContext,
            is_rhs: bool,
        ) -> anyhow::Result<Pattern<GritQueryContext>, CompileError> {
            let metavariable =
                metavariable_descendent(&node, context_range, range_map, context, is_rhs)?;
            if let Some(metavariable) = metavariable {
                return Ok(metavariable);
            }

            let kind = node.kind();
            if !node.has_children() {
                if let Some(token) = node.first_token() {
                    let content = token.text();
                    if context
                        .compilation
                        .lang
                        .replaced_metavariable_regex()
                        .is_match(content)
                    {
                        let regex =
                            implicit_metavariable_regex(&token, context_range, range_map, context)?;
                        if let Some(regex) = regex {
                            return Ok(Pattern::Regex(Box::new(regex)));
                        }
                    }

                    return Ok(Pattern::AstLeafNode(GritLeafNodePattern::new(
                        kind, content,
                    )));
                }
            }

            let args: Vec<GritNodeArg> = node
                .slots()
                // TODO: Implement filtering for disregarded snippet fields.
                // Implementing this will make it more convenient to match
                // CST nodes without needing to match all the trivia in the
                // snippet.
                .map(|slot| {
                    let mut nodes_list: Vec<Pattern<GritQueryContext>> = match &slot {
                        GritSyntaxSlot::Node(node) => node
                            .children()
                            .map(|n| node_to_pattern(n, context_range, range_map, context, is_rhs))
                            .collect::<Result<_, CompileError>>()?,
                        _ => Vec::new(),
                    };
                    if !slot.contains_list() {
                        Ok(GritNodeArg::new(
                            slot.index(),
                            nodes_list
                                .pop()
                                .unwrap_or(Pattern::Dynamic(DynamicPattern::Snippet(
                                    DynamicSnippet {
                                        parts: vec![DynamicSnippetPart::String(String::new())],
                                    },
                                ))),
                        ))
                    } else if nodes_list.len() == 1
                        && matches!(
                            nodes_list.first(),
                            Some(Pattern::Variable(_) | Pattern::Underscore)
                        )
                    {
                        Ok(GritNodeArg::new(slot.index(), nodes_list.pop().unwrap()))
                    } else {
                        Ok(GritNodeArg::new(
                            slot.index(),
                            Pattern::List(Box::new(List::new(nodes_list))),
                        ))
                    }
                })
                .collect::<Result<_, CompileError>>()?;
            Ok(Pattern::AstNode(Box::new(GritNodePattern { kind, args })))
        }
        node_to_pattern(node, context_range, &range_map, context, is_rhs)
    }
}

fn implicit_metavariable_regex(
    token: &GritTargetToken,
    context_range: ByteRange,
    range_map: &BTreeMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
) -> Result<Option<RegexPattern<GritQueryContext>>, CompileError> {
    let source = token.text();
    let capture_string = "(.*)";
    let uncapture_string = ".*";
    let variable_regex = context.compilation.lang.replaced_metavariable_regex();
    let mut last = 0;
    let mut regex_string = String::new();
    let mut variables: Vec<Variable> = vec![];
    for m in variable_regex.find_iter(source) {
        regex_string.push_str(&regex::escape(&source[last..m.start()]));
        let range = ByteRange::new(m.start(), m.end());
        last = range.end;
        let name = m.as_str();
        let variable = text_to_var(name, range, context_range, range_map, context)?;
        match variable {
            SnippetValues::Dots => return Ok(None),
            SnippetValues::Underscore => regex_string.push_str(uncapture_string),
            SnippetValues::Variable(var) => {
                regex_string.push_str(capture_string);
                variables.push(var);
            }
        }
    }

    if last < source.len() {
        regex_string.push_str(&regex::escape(&source[last..]));
    }
    let regex = regex_string.to_string();
    let regex = RegexLike::Regex(regex);
    Ok(Some(RegexPattern::new(regex, variables)))
}

fn metavariable_descendent(
    node: &GritTargetNode,
    context_range: ByteRange,
    range_map: &BTreeMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Option<Pattern<GritQueryContext>>, CompileError> {
    let Some(token) = node.first_token() else {
        return Ok(None);
    };
    if !context.compilation.lang.is_metavariable(node) {
        return Ok(None);
    }

    let name = token.text();
    if is_reserved_metavariable(name, Some(&context.compilation.lang)) && !is_rhs {
        return Err(CompileError::ReservedMetavariable(
            name.trim_start_matches(context.compilation.lang.metavariable_prefix_substitute())
                .to_string(),
        ));
    }

    let range = node.byte_range();
    text_to_var(name, range, context_range, range_map, context).map(|s| Some(s.into()))
}

fn metavariable_ranges(node: &GritTargetNode, lang: &GritTargetLanguage) -> Vec<ByteRange> {
    let cursor = node.walk();
    traverse(cursor, Order::Pre)
        .flat_map(|child| {
            if lang.is_metavariable(&child) {
                vec![child.byte_range()]
            } else {
                node_sub_variables(&child, lang)
            }
        })
        .collect()
}

// assumes that metavariable substitute is 1 byte larger than the original. eg.
// len(µ) = 2 bytes, len($) = 1 byte
fn metavariable_range_mapping(
    mut ranges: Vec<ByteRange>,
    snippet_offset: usize,
) -> BTreeMap<ByteRange, ByteRange> {
    // assumes metavariable ranges do not enclose one another
    ranges.sort_by_key(|r| r.start);

    let mut byte_offset = snippet_offset;
    let mut map = BTreeMap::new();
    for range in ranges {
        let start_byte = range.start - byte_offset;
        if !cfg!(target_arch = "wasm32") {
            byte_offset += 1;
        }

        let end_byte = range.end - byte_offset;
        let new_range = ByteRange::new(start_byte, end_byte);
        map.insert(range, new_range);
    }

    map
}

fn node_sub_variables(node: &GritTargetNode, lang: &impl Language) -> Vec<ByteRange> {
    let mut ranges = vec![];
    if node.has_children() {
        return ranges;
    }

    let Some(token) = node.first_token() else {
        return ranges;
    };

    let source = token.text();
    let variable_regex = lang.replaced_metavariable_regex();
    for m in variable_regex.find_iter(source) {
        let var_range = ByteRange::new(m.start(), m.end());
        let start_byte = node.start_byte() as usize;
        let end_byte = node.end_byte() as usize;
        if var_range.start >= start_byte && var_range.end <= end_byte {
            ranges.push(var_range);
        }
    }

    ranges
}

enum SnippetValues {
    Dots,
    Underscore,
    Variable(Variable),
}

impl From<SnippetValues> for Pattern<GritQueryContext> {
    fn from(value: SnippetValues) -> Self {
        match value {
            SnippetValues::Dots => Pattern::Dots,
            SnippetValues::Underscore => Pattern::Underscore,
            SnippetValues::Variable(v) => Pattern::Variable(v),
        }
    }
}

fn text_to_var(
    name: &str,
    range: ByteRange,
    context_range: ByteRange,
    range_map: &BTreeMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
) -> Result<SnippetValues, CompileError> {
    let name = context
        .compilation
        .lang
        .snippet_metavariable_to_grit_metavariable(name)
        .ok_or_else(|| CompileError::MetavariableNotFound(name.to_string()))?;
    match name {
        GritMetaValue::Dots => Ok(SnippetValues::Dots),
        GritMetaValue::Underscore => Ok(SnippetValues::Underscore),
        GritMetaValue::Variable(name) => {
            let range = *range_map
                .get(&range)
                .ok_or_else(|| CompileError::InvalidMetavariableRange(range))?;
            let var = context.register_variable(name, range + context_range.start)?;
            Ok(SnippetValues::Variable(var))
        }
    }
}
