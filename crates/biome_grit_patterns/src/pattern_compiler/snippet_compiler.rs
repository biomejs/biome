use super::compilation_context::NodeCompilationContext;
use crate::{
    grit_code_snippet::GritCodeSnippet, grit_context::GritQueryContext,
    grit_target_node::GritTargetNode, grit_tree::GritTree, CompileError,
};
use grit_pattern_matcher::{
    constants::GLOBAL_VARS_SCOPE_INDEX,
    pattern::{DynamicPattern, DynamicSnippet, DynamicSnippetPart, Pattern, Variable},
};
use grit_util::{Ast, AstNode, ByteRange, Language, SnippetTree};
use std::borrow::Cow;

pub(crate) fn parse_snippet_content(
    source: &str,
    range: ByteRange,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Pattern<GritQueryContext>, CompileError> {
    // we check for BRACKET_VAR_REGEX in the content, and if found
    // compile into a DynamicPattern, rather than a CodeSnippet.
    // This is because the syntax should only ever be necessary
    // when treating a metavariable as a string to substitute
    // rather than an AST node to match on. eg. in the following
    // `const ${name}Handler = useCallback(async () => $body, []);`
    // $name does not correspond to a node, but rather prepends a
    // string to "Handler", which will together combine into an
    // identifier.
    if context
        .compilation
        .lang
        .metavariable_bracket_regex()
        .is_match(source)
    {
        return if is_rhs {
            Ok(Pattern::Dynamic(
                dynamic_snippet_from_source(source, range, context).map(DynamicPattern::Snippet)?,
            ))
        } else {
            Err(CompileError::InvalidBracketedMetavariable)
        };
    }

    if context
        .compilation
        .lang
        .exact_variable_regex()
        .is_match(source.trim())
    {
        return match source.trim() {
            "$_" => Ok(Pattern::Underscore),
            "^_" => Ok(Pattern::Underscore),
            name => {
                let var = context.register_variable(name.to_owned(), range)?;
                Ok(Pattern::Variable(var))
            }
        };
    }

    let snippet_trees = context.compilation.lang.parse_snippet_contexts(source);
    let snippet_nodes = nodes_from_indices(&snippet_trees);
    if snippet_nodes.is_empty() {
        // not checking if is_rhs. So could potentially
        // be harder to find bugs where we expect the pattern
        // to parse. unfortunately got rid of check to support
        // passing non-node snippets as args.
        return Ok(Pattern::Dynamic(
            dynamic_snippet_from_source(source, range, context).map(DynamicPattern::Snippet)?,
        ));
    }

    let dynamic_snippet = dynamic_snippet_from_source(source, range, context)
        .map_or(None, |s| Some(DynamicPattern::Snippet(s)));
    Ok(Pattern::CodeSnippet(GritCodeSnippet {
        dynamic_snippet,
        source: source.to_owned(),
    }))
}

pub(crate) fn dynamic_snippet_from_source(
    raw_source: &str,
    source_range: ByteRange,
    context: &mut NodeCompilationContext,
) -> Result<DynamicSnippet, CompileError> {
    let source_string = raw_source
        .replace("\\n", "\n")
        .replace("\\$", "$")
        .replace("\\^", "^")
        .replace("\\`", "`")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\");
    let source = source_string.as_str();
    let metavariables = split_snippet(source, &context.compilation.lang);
    let mut parts = Vec::with_capacity(2 * metavariables.len() + 1);
    let mut last = 0;
    // Reverse the iterator so we go over the variables in ascending order.
    for (byte_range, var) in metavariables.into_iter().rev() {
        parts.push(DynamicSnippetPart::String(
            source[last..byte_range.start].to_string(),
        ));
        let range = ByteRange::new(
            source_range.start + byte_range.start,
            source_range.start + byte_range.start + var.len(),
        );
        if let Some(var) = context.vars.get(var.as_ref()) {
            context.vars_array[context.scope_index][*var]
                .locations
                .insert(range);
            parts.push(DynamicSnippetPart::Variable(Variable::new(
                context.scope_index,
                *var,
            )));
        } else if let Some(var) = context.global_vars.get(var.as_ref()) {
            parts.push(DynamicSnippetPart::Variable(Variable::new(
                GLOBAL_VARS_SCOPE_INDEX,
                *var,
            )));
        } else if var.starts_with("$GLOBAL_") {
            let variable = context.register_variable(var.to_string(), range)?;
            parts.push(DynamicSnippetPart::Variable(variable));
        } else {
            return Err(CompileError::UnknownVariable(var.to_string()));
        }
        last = byte_range.end;
    }
    parts.push(DynamicSnippetPart::String(source[last..].to_string()));

    Ok(DynamicSnippet { parts })
}

pub fn nodes_from_indices(indices: &[SnippetTree<GritTree>]) -> Vec<GritTargetNode> {
    indices
        .iter()
        .filter_map(snippet_nodes_from_index)
        .collect()
}

fn snippet_nodes_from_index(snippet: &SnippetTree<GritTree>) -> Option<GritTargetNode> {
    let mut snippet_root = snippet.tree.root_node();

    // find the outermost node with the same index as the snippet
    'outer: while snippet_root.start_byte() < snippet.snippet_start
        || snippet_root.end_byte() > snippet.snippet_end
    {
        let mut has_children = false;
        for child in snippet_root.clone().children() {
            has_children = true;

            if child.start_byte() <= snippet.snippet_start
                && child.end_byte() >= snippet.snippet_end
            {
                snippet_root = child;
                continue 'outer;
            }
        }

        if snippet_root.text() != snippet.source.trim() {
            return None;
        }

        if !has_children {
            return Some(snippet_root);
        }

        break;
    }

    // in order to handle white space and other superfluous
    // stuff in the snippet we assume the root
    // is correct as long as it's the largest node within
    // the snippet length. Maybe this is too permissive?
    let mut nodes = Vec::new();
    let root_start = snippet_root.start_byte();
    let root_end = snippet_root.end_byte();
    if root_start > snippet.snippet_start || root_end < snippet.snippet_end {
        return None;
    }
    while snippet_root.start_byte() == root_start && snippet_root.end_byte() == root_end {
        let first_child = snippet_root.children().next();
        nodes.push(snippet_root);
        if let Some(child) = first_child {
            snippet_root = child
        } else {
            break;
        }
    }
    nodes.last().cloned()
}

/// Takes a snippet with metavariables and returns a list of ranges and the
/// corresponding metavariables.
///
/// The ranges are in descending order.
pub fn split_snippet<'a>(snippet: &'a str, lang: &impl Language) -> Vec<(ByteRange, Cow<'a, str>)> {
    let mut ranges_and_metavars: Vec<(ByteRange, Cow<str>)> = Vec::new();

    let variable_regex = lang.metavariable_regex();
    let curly_var_regex = lang.metavariable_bracket_regex();

    for m in variable_regex.find_iter(snippet) {
        ranges_and_metavars.push(((m.start()..m.end()).into(), m.as_str().into()));
    }
    for m in curly_var_regex.find_iter(snippet) {
        let mut metavar: Cow<str> = m.as_str()[2..m.as_str().len() - 1].into();
        metavar.to_mut().insert(0, '$');
        ranges_and_metavars.push(((m.start()..m.end()).into(), metavar));
    }

    // Sort ranges in descending order
    ranges_and_metavars.sort_by(|a, b| b.0.start.cmp(&a.0.start));

    ranges_and_metavars
}
