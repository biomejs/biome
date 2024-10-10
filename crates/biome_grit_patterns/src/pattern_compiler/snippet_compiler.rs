use super::compilation_context::NodeCompilationContext;
use crate::{
    grit_code_snippet::GritCodeSnippet,
    grit_context::GritQueryContext,
    grit_node_patterns::{GritLeafNodePattern, GritNodePattern, GritNodePatternArg},
    grit_target_node::{GritSyntaxSlot, GritTargetNode, GritTargetSyntaxKind},
    grit_tree::GritTargetTree,
    CompileError, GritTargetLanguage,
};
use grit_pattern_matcher::{
    constants::GLOBAL_VARS_SCOPE_INDEX,
    pattern::{
        is_reserved_metavariable, DynamicPattern, DynamicSnippet, DynamicSnippetPart, List,
        Pattern, RegexLike, RegexPattern, Variable, VariableSource,
    },
};
use grit_util::{traverse, Ast, AstNode, ByteRange, GritMetaValue, Language, Order, SnippetTree};
use std::{borrow::Cow, collections::BTreeMap};

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
        .matches_bracket_metavariable(source)
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
        .matches_exact_metavariable(source.trim())
    {
        return match source.trim() {
            "$_" | "^_" => Ok(Pattern::Underscore),
            name => {
                let var = context.register_variable(name.to_owned(), range);
                Ok(Pattern::Variable(var))
            }
        };
    }

    let snippet_trees = context.compilation.lang.parse_snippet_contexts(source);
    let snippet_nodes = nodes_from_trees(&snippet_trees);
    if snippet_nodes.is_empty() {
        // not checking if is_rhs. So could potentially
        // be harder to find bugs where we expect the pattern
        // to parse. unfortunately got rid of check to support
        // passing non-node snippets as args.
        return Ok(Pattern::Dynamic(
            dynamic_snippet_from_source(source, range, context).map(DynamicPattern::Snippet)?,
        ));
    }

    let patterns: Vec<(GritTargetSyntaxKind, Pattern<GritQueryContext>)> = snippet_nodes
        .into_iter()
        .map(|node| {
            let range_map = metavariable_range_mapping(&node, &context.compilation.lang);
            let pattern = pattern_from_node(&node, range, &range_map, context, is_rhs)?;
            Ok((node.kind(), pattern))
        })
        .collect::<Result<_, CompileError>>()?;
    let dynamic_snippet = dynamic_snippet_from_source(source, range, context)
        .map_or(None, |s| Some(DynamicPattern::Snippet(s)));
    Ok(Pattern::CodeSnippet(GritCodeSnippet {
        patterns,
        dynamic_snippet,
        source: source.to_owned(),
    }))
}

pub(crate) fn dynamic_snippet_from_source(
    raw_source: &str,
    source_range: ByteRange,
    context: &mut NodeCompilationContext,
) -> Result<DynamicSnippet, CompileError> {
    let source = unescape(raw_source);
    let metavariables = split_snippet(&source, &context.compilation.lang);
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
            if let VariableSource::Compiled { locations, .. } =
                &mut context.vars_array[context.scope_index][*var]
            {
                locations.insert(range);
            }
            parts.push(DynamicSnippetPart::Variable(Variable::new(
                context.scope_index,
                *var,
            )));
        } else if let Some(var) = context.global_vars.get(var.as_ref()) {
            parts.push(DynamicSnippetPart::Variable(Variable::new(
                GLOBAL_VARS_SCOPE_INDEX.into(),
                *var,
            )));
        } else if var.starts_with("$GLOBAL_") {
            let variable = context.register_variable(var.to_string(), range);
            parts.push(DynamicSnippetPart::Variable(variable));
        } else {
            return Err(CompileError::UnknownVariable(var.to_string()));
        }
        last = byte_range.end;
    }
    parts.push(DynamicSnippetPart::String(source[last..].to_string()));

    Ok(DynamicSnippet { parts })
}

fn nodes_from_trees(snippets: &[SnippetTree<GritTargetTree>]) -> Vec<GritTargetNode> {
    snippets.iter().filter_map(node_from_tree).collect()
}

/// Finds the outermost node containing the parsed snippet, but not any snippet
/// context.
///
/// Snippets get parsed with surrounding _context_ strings. Because of this, the
/// root node of the snippet tree isn't necessarily the root node of the source
/// snippet. Instead, it's the root node of the snippet with surrounding
/// context. This function descends from the root node into the tree, to find
/// the outermost node containing the parsed snippet, while stripping off the
/// part of the tree that resulted from the given context.
fn node_from_tree(snippet: &SnippetTree<GritTargetTree>) -> Option<GritTargetNode> {
    let mut snippet_root = snippet.tree.root_node();

    // find the outermost node with the same index as the snippet
    'outer: while snippet_root.start_byte() <= snippet.snippet_start
        || snippet_root.end_byte() >= snippet.snippet_end
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
    let mut last_node = None;
    let root_start = snippet_root.start_byte();
    let root_end = snippet_root.end_byte();
    if root_start > snippet.snippet_start || root_end < snippet.snippet_end {
        return None;
    }
    while snippet_root.start_byte() == root_start && snippet_root.end_byte() == root_end {
        let first_child = snippet_root.children().next();
        last_node = Some(snippet_root);
        if let Some(child) = first_child {
            snippet_root = child
        } else {
            break;
        }
    }
    last_node
}

/// Creates a pattern from the snippet node.
///
/// The snippet node is the one returned from [`node_from_tree()`].
fn pattern_from_node(
    node: &GritTargetNode,
    context_range: ByteRange,
    range_map: &BTreeMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Pattern<GritQueryContext>, CompileError> {
    let metavariable = metavariable_descendent(node, context_range, range_map, context, is_rhs)?;
    if let Some(metavariable) = metavariable {
        return Ok(metavariable);
    }

    let Some(slots) = node.slots() else {
        let content = node.text();
        let lang = &context.compilation.lang;
        let pattern = if let Some(regex_pattern) = lang
            .matches_replaced_metavariable(content)
            .then(|| implicit_metavariable_regex(node, context_range, range_map, context))
            .flatten()
        {
            Pattern::Regex(Box::new(regex_pattern))
        } else {
            Pattern::AstLeafNode(GritLeafNodePattern::new(node.kind(), content, lang)?)
        };

        return Ok(pattern);
    };

    let kind = node.kind();
    let args = slots
        .filter(|slot| {
            !context.compilation.lang.is_disregarded_snippet_field(
                kind,
                slot.index(),
                node.child_by_slot_index(slot.index()),
            )
        })
        .map(|slot| pattern_arg_from_slot(slot, context_range, range_map, context, is_rhs))
        .collect::<Result<Vec<GritNodePatternArg>, CompileError>>()?;

    Ok(Pattern::AstNode(Box::new(GritNodePattern { kind, args })))
}

fn pattern_arg_from_slot(
    slot: GritSyntaxSlot,
    context_range: ByteRange,
    range_map: &BTreeMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<GritNodePatternArg, CompileError> {
    if slot.contains_list() {
        let mut nodes_list: Vec<Pattern<GritQueryContext>> = match &slot {
            GritSyntaxSlot::Node(node) => node
                .named_children()
                .map(|n| pattern_from_node(&n, context_range, range_map, context, is_rhs))
                .collect::<Result<_, CompileError>>()?,
            _ => Vec::new(),
        };
        Ok(GritNodePatternArg::new(
            slot.index(),
            if nodes_list.len() == 1
                && matches!(
                    nodes_list.first(),
                    Some(Pattern::Variable(_) | Pattern::Underscore)
                )
            {
                nodes_list.pop().unwrap()
            } else {
                Pattern::List(Box::new(List::new(nodes_list)))
            },
        ))
    } else if let GritSyntaxSlot::Node(node) = slot {
        let pattern = pattern_from_node(&node, context_range, range_map, context, is_rhs)?;
        Ok(GritNodePatternArg::new(node.index(), pattern))
    } else {
        let pattern = Pattern::Dynamic(DynamicPattern::Snippet(DynamicSnippet {
            parts: vec![DynamicSnippetPart::String(String::new())],
        }));
        Ok(GritNodePatternArg::new(slot.index(), pattern))
    }
}

fn implicit_metavariable_regex(
    node: &GritTargetNode,
    context_range: ByteRange,
    range_map: &BTreeMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
) -> Option<RegexPattern<GritQueryContext>> {
    let source = node.text();
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
        let variable = text_to_var(name, range, context_range, range_map, context).ok()?;
        match variable {
            SnippetValue::Dots => return None,
            SnippetValue::Underscore => regex_string.push_str(uncapture_string),
            SnippetValue::Variable(var) => {
                regex_string.push_str(capture_string);
                variables.push(var);
            }
        }
    }

    if last < source.len() {
        regex_string.push_str(&regex::escape(&source[last..]));
    }
    let regex = RegexLike::Regex(regex_string);
    Some(RegexPattern::new(regex, variables))
}

fn metavariable_descendent(
    node: &GritTargetNode,
    context_range: ByteRange,
    range_map: &BTreeMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Option<Pattern<GritQueryContext>>, CompileError> {
    if !context.compilation.lang.is_metavariable(node) {
        return Ok(None);
    }

    let name = node.text();
    if is_reserved_metavariable(name, Some(&context.compilation.lang)) && !is_rhs {
        return Err(CompileError::ReservedMetavariable(
            name.trim_start_matches(context.compilation.lang.metavariable_prefix_substitute())
                .to_string(),
        ));
    }

    let range = node.byte_range();
    text_to_var(name, range, context_range, range_map, context).map(|s| Some(s.into()))
}

fn metavariable_range_mapping(
    node: &GritTargetNode,
    lang: &GritTargetLanguage,
) -> BTreeMap<ByteRange, ByteRange> {
    let mut ranges = metavariable_ranges(node, lang);
    let snippet_start = node.start_byte() as usize;

    // assumes metavariable ranges do not enclose one another
    ranges.sort_by_key(|r| r.start);

    let mut map = BTreeMap::new();
    for range in ranges {
        let start_byte = range.start - snippet_start;
        let end_byte = range.end - snippet_start;
        let new_range = ByteRange::new(start_byte, end_byte);
        map.insert(range, new_range);
    }

    map
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

fn node_sub_variables(node: &GritTargetNode, lang: &GritTargetLanguage) -> Vec<ByteRange> {
    let mut ranges = Vec::new();
    if node.has_children() {
        return ranges;
    }

    let source = node.text();
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

#[derive(Debug)]
enum SnippetValue {
    Dots,
    Underscore,
    Variable(Variable),
}

impl From<SnippetValue> for Pattern<GritQueryContext> {
    fn from(value: SnippetValue) -> Self {
        match value {
            SnippetValue::Dots => Pattern::Dots,
            SnippetValue::Underscore => Pattern::Underscore,
            SnippetValue::Variable(v) => Pattern::Variable(v),
        }
    }
}

fn text_to_var(
    name: &str,
    range: ByteRange,
    context_range: ByteRange,
    range_map: &BTreeMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
) -> Result<SnippetValue, CompileError> {
    let meta_value = context
        .compilation
        .lang
        .snippet_metavariable_to_grit_metavariable(name)
        .ok_or_else(|| CompileError::MetavariableNotFound(name.to_string()))?;
    match meta_value {
        GritMetaValue::Dots => Ok(SnippetValue::Dots),
        GritMetaValue::Underscore => Ok(SnippetValue::Underscore),
        GritMetaValue::Variable(name) => {
            let range = *range_map
                .get(&range)
                .ok_or(CompileError::InvalidMetavariableRange(range))?;
            let var = context.register_variable(name, range + context_range.start);
            Ok(SnippetValue::Variable(var))
        }
    }
}

fn unescape(raw_string: &str) -> String {
    let mut result = String::with_capacity(raw_string.len());
    let mut is_escape = false;
    for c in raw_string.chars() {
        if is_escape {
            result.push(match c {
                'n' => '\n',
                c => c,
            });
            is_escape = false;
        } else if c == '\\' {
            is_escape = true;
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        grit_built_in_functions::BuiltIns, grit_js_parser::GritJsParser,
        pattern_compiler::compilation_context::CompilationContext, JsTargetLanguage,
    };
    use grit_util::Parser;
    use regex::Regex;

    #[test]
    fn test_node_from_tree() {
        let snippet = GritJsParser.parse_snippet("", "console.log('hello')", "");
        let node = node_from_tree(&snippet).expect("no node found");
        let formatted = format!("{node:#?}");
        insta::assert_snapshot!(&formatted, @r###"
        GritTargetNode {
            node: JsLanguage(
                Node(
                    0: JS_CALL_EXPRESSION@0..20
                      0: JS_STATIC_MEMBER_EXPRESSION@0..11
                        0: JS_IDENTIFIER_EXPRESSION@0..7
                          0: JS_REFERENCE_IDENTIFIER@0..7
                            0: IDENT@0..7 "console" [] []
                        1: DOT@7..8 "." [] []
                        2: JS_NAME@8..11
                          0: IDENT@8..11 "log" [] []
                      1: (empty)
                      2: (empty)
                      3: JS_CALL_ARGUMENTS@11..20
                        0: L_PAREN@11..12 "(" [] []
                        1: JS_CALL_ARGUMENT_LIST@12..19
                          0: JS_STRING_LITERAL_EXPRESSION@12..19
                            0: JS_STRING_LITERAL@12..19 "'hello'" [] []
                        2: R_PAREN@19..20 ")" [] []
                    ,
                ),
            ),
        }
        "###);
    }

    #[test]
    fn test_pattern_from_node() {
        let built_ins = BuiltIns::default();
        let compilation_context = CompilationContext::new(
            None,
            GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
            &built_ins,
        );
        let mut vars = BTreeMap::new();
        let mut vars_array = Vec::new();
        let mut global_vars = BTreeMap::new();
        let mut diagnostics = Vec::new();
        let mut context = NodeCompilationContext::new(
            &compilation_context,
            &mut vars,
            &mut vars_array,
            &mut global_vars,
            &mut diagnostics,
        );

        let snippet_source = "console.log('hello')";
        let snippet = GritJsParser.parse_snippet("", snippet_source, "");
        let node = node_from_tree(&snippet).expect("no node found");
        let range = ByteRange::new(0, snippet_source.len());
        let range_map = metavariable_range_mapping(&node, &context.compilation.lang);
        let pattern = pattern_from_node(&node, range, &range_map, &mut context, false)
            .expect("cannot compile pattern from node");
        let formatted = format!("{pattern:#?}");
        let snapshot = Regex::new("normalizer: 0x[0-9a-f]{16}")
            .unwrap()
            .replace_all(&formatted, "normalizer: [address redacted]");

        insta::assert_snapshot!(&snapshot, @r###"
        AstNode(
            GritNodePattern {
                kind: JsSyntaxKind(
                    JS_CALL_EXPRESSION,
                ),
                args: [
                    GritNodePatternArg {
                        slot_index: 0,
                        pattern: AstNode(
                            GritNodePattern {
                                kind: JsSyntaxKind(
                                    JS_STATIC_MEMBER_EXPRESSION,
                                ),
                                args: [
                                    GritNodePatternArg {
                                        slot_index: 0,
                                        pattern: AstNode(
                                            GritNodePattern {
                                                kind: JsSyntaxKind(
                                                    JS_IDENTIFIER_EXPRESSION,
                                                ),
                                                args: [
                                                    GritNodePatternArg {
                                                        slot_index: 0,
                                                        pattern: AstNode(
                                                            GritNodePattern {
                                                                kind: JsSyntaxKind(
                                                                    JS_REFERENCE_IDENTIFIER,
                                                                ),
                                                                args: [
                                                                    GritNodePatternArg {
                                                                        slot_index: 0,
                                                                        pattern: AstLeafNode(
                                                                            GritLeafNodePattern {
                                                                                kind: JsSyntaxKind(
                                                                                    IDENT,
                                                                                ),
                                                                                equivalence_class: None,
                                                                                text: "console",
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 1,
                                        pattern: AstLeafNode(
                                            GritLeafNodePattern {
                                                kind: JsSyntaxKind(
                                                    DOT,
                                                ),
                                                equivalence_class: None,
                                                text: ".",
                                            },
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 2,
                                        pattern: AstNode(
                                            GritNodePattern {
                                                kind: JsSyntaxKind(
                                                    JS_NAME,
                                                ),
                                                args: [
                                                    GritNodePatternArg {
                                                        slot_index: 0,
                                                        pattern: AstLeafNode(
                                                            GritLeafNodePattern {
                                                                kind: JsSyntaxKind(
                                                                    IDENT,
                                                                ),
                                                                equivalence_class: None,
                                                                text: "log",
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                        ),
                                    },
                                ],
                            },
                        ),
                    },
                    GritNodePatternArg {
                        slot_index: 1,
                        pattern: Dynamic(
                            Snippet(
                                DynamicSnippet {
                                    parts: [
                                        String(
                                            "",
                                        ),
                                    ],
                                },
                            ),
                        ),
                    },
                    GritNodePatternArg {
                        slot_index: 2,
                        pattern: Dynamic(
                            Snippet(
                                DynamicSnippet {
                                    parts: [
                                        String(
                                            "",
                                        ),
                                    ],
                                },
                            ),
                        ),
                    },
                    GritNodePatternArg {
                        slot_index: 3,
                        pattern: AstNode(
                            GritNodePattern {
                                kind: JsSyntaxKind(
                                    JS_CALL_ARGUMENTS,
                                ),
                                args: [
                                    GritNodePatternArg {
                                        slot_index: 0,
                                        pattern: AstLeafNode(
                                            GritLeafNodePattern {
                                                kind: JsSyntaxKind(
                                                    L_PAREN,
                                                ),
                                                equivalence_class: None,
                                                text: "(",
                                            },
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 1,
                                        pattern: List(
                                            List {
                                                patterns: [
                                                    AstNode(
                                                        GritNodePattern {
                                                            kind: JsSyntaxKind(
                                                                JS_STRING_LITERAL_EXPRESSION,
                                                            ),
                                                            args: [
                                                                GritNodePatternArg {
                                                                    slot_index: 0,
                                                                    pattern: AstLeafNode(
                                                                        GritLeafNodePattern {
                                                                            kind: JsSyntaxKind(
                                                                                JS_STRING_LITERAL,
                                                                            ),
                                                                            equivalence_class: Some(
                                                                                LeafEquivalenceClass {
                                                                                    representative: "hello",
                                                                                    class: [
                                                                                        LeafNormalizer {
                                                                                            kind: JsSyntaxKind(
                                                                                                JS_STRING_LITERAL,
                                                                                            ),
                                                                                            normalizer: [address redacted],
                                                                                        },
                                                                                        LeafNormalizer {
                                                                                            kind: JsSyntaxKind(
                                                                                                JS_STRING_LITERAL_EXPRESSION,
                                                                                            ),
                                                                                            normalizer: [address redacted],
                                                                                        },
                                                                                    ],
                                                                                },
                                                                            ),
                                                                            text: "'hello'",
                                                                        },
                                                                    ),
                                                                },
                                                            ],
                                                        },
                                                    ),
                                                ],
                                            },
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 2,
                                        pattern: AstLeafNode(
                                            GritLeafNodePattern {
                                                kind: JsSyntaxKind(
                                                    R_PAREN,
                                                ),
                                                equivalence_class: None,
                                                text: ")",
                                            },
                                        ),
                                    },
                                ],
                            },
                        ),
                    },
                ],
            },
        )
        "###);
    }

    #[test]
    fn test_pattern_with_metavariables_from_node() {
        let built_ins = BuiltIns::default();
        let compilation_context = CompilationContext::new(
            None,
            GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
            &built_ins,
        );
        let mut vars = BTreeMap::new();
        let mut vars_array = vec![Vec::new()];
        let mut global_vars = BTreeMap::new();
        let mut diagnostics = Vec::new();
        let mut context = NodeCompilationContext::new(
            &compilation_context,
            &mut vars,
            &mut vars_array,
            &mut global_vars,
            &mut diagnostics,
        );

        let snippet_source = "µfn && µfn()";
        let range = ByteRange::new(0, snippet_source.len());
        let pattern = parse_snippet_content(snippet_source, range, &mut context, false)
            .expect("cannot parse snippet");
        let formatted = format!("{pattern:#?}");
        let snapshot = Regex::new("normalizer: 0x[0-9a-f]{16}")
            .unwrap()
            .replace_all(&formatted, "normalizer: [address redacted]");

        insta::assert_snapshot!(&snapshot, @r###"
        CodeSnippet(
            GritCodeSnippet {
                patterns: [
                    (
                        JsSyntaxKind(
                            JS_PROPERTY_OBJECT_MEMBER,
                        ),
                        AstNode(
                            GritNodePattern {
                                kind: JsSyntaxKind(
                                    JS_PROPERTY_OBJECT_MEMBER,
                                ),
                                args: [
                                    GritNodePatternArg {
                                        slot_index: 0,
                                        pattern: Variable(
                                            Variable {
                                                internal: Static(
                                                    VariableScope {
                                                        scope: 0,
                                                        index: 0,
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 1,
                                        pattern: Dynamic(
                                            Snippet(
                                                DynamicSnippet {
                                                    parts: [
                                                        String(
                                                            "",
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 2,
                                        pattern: AstNode(
                                            GritNodePattern {
                                                kind: JsSyntaxKind(
                                                    JS_LOGICAL_EXPRESSION,
                                                ),
                                                args: [
                                                    GritNodePatternArg {
                                                        slot_index: 0,
                                                        pattern: Dynamic(
                                                            Snippet(
                                                                DynamicSnippet {
                                                                    parts: [
                                                                        String(
                                                                            "",
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    GritNodePatternArg {
                                                        slot_index: 1,
                                                        pattern: AstLeafNode(
                                                            GritLeafNodePattern {
                                                                kind: JsSyntaxKind(
                                                                    AMP2,
                                                                ),
                                                                equivalence_class: None,
                                                                text: "&&",
                                                            },
                                                        ),
                                                    },
                                                    GritNodePatternArg {
                                                        slot_index: 2,
                                                        pattern: AstNode(
                                                            GritNodePattern {
                                                                kind: JsSyntaxKind(
                                                                    JS_CALL_EXPRESSION,
                                                                ),
                                                                args: [
                                                                    GritNodePatternArg {
                                                                        slot_index: 0,
                                                                        pattern: Variable(
                                                                            Variable {
                                                                                internal: Static(
                                                                                    VariableScope {
                                                                                        scope: 0,
                                                                                        index: 0,
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ),
                                                                    },
                                                                    GritNodePatternArg {
                                                                        slot_index: 1,
                                                                        pattern: Dynamic(
                                                                            Snippet(
                                                                                DynamicSnippet {
                                                                                    parts: [
                                                                                        String(
                                                                                            "",
                                                                                        ),
                                                                                    ],
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    GritNodePatternArg {
                                                                        slot_index: 2,
                                                                        pattern: Dynamic(
                                                                            Snippet(
                                                                                DynamicSnippet {
                                                                                    parts: [
                                                                                        String(
                                                                                            "",
                                                                                        ),
                                                                                    ],
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    GritNodePatternArg {
                                                                        slot_index: 3,
                                                                        pattern: AstNode(
                                                                            GritNodePattern {
                                                                                kind: JsSyntaxKind(
                                                                                    JS_CALL_ARGUMENTS,
                                                                                ),
                                                                                args: [
                                                                                    GritNodePatternArg {
                                                                                        slot_index: 0,
                                                                                        pattern: AstLeafNode(
                                                                                            GritLeafNodePattern {
                                                                                                kind: JsSyntaxKind(
                                                                                                    L_PAREN,
                                                                                                ),
                                                                                                equivalence_class: None,
                                                                                                text: "(",
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    GritNodePatternArg {
                                                                                        slot_index: 1,
                                                                                        pattern: List(
                                                                                            List {
                                                                                                patterns: [],
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    GritNodePatternArg {
                                                                                        slot_index: 2,
                                                                                        pattern: AstLeafNode(
                                                                                            GritLeafNodePattern {
                                                                                                kind: JsSyntaxKind(
                                                                                                    R_PAREN,
                                                                                                ),
                                                                                                equivalence_class: None,
                                                                                                text: ")",
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                        ),
                                    },
                                ],
                            },
                        ),
                    ),
                    (
                        JsSyntaxKind(
                            JS_LOGICAL_EXPRESSION,
                        ),
                        AstNode(
                            GritNodePattern {
                                kind: JsSyntaxKind(
                                    JS_LOGICAL_EXPRESSION,
                                ),
                                args: [
                                    GritNodePatternArg {
                                        slot_index: 0,
                                        pattern: Variable(
                                            Variable {
                                                internal: Static(
                                                    VariableScope {
                                                        scope: 0,
                                                        index: 0,
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 1,
                                        pattern: AstLeafNode(
                                            GritLeafNodePattern {
                                                kind: JsSyntaxKind(
                                                    AMP2,
                                                ),
                                                equivalence_class: None,
                                                text: "&&",
                                            },
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 2,
                                        pattern: AstNode(
                                            GritNodePattern {
                                                kind: JsSyntaxKind(
                                                    JS_CALL_EXPRESSION,
                                                ),
                                                args: [
                                                    GritNodePatternArg {
                                                        slot_index: 0,
                                                        pattern: Variable(
                                                            Variable {
                                                                internal: Static(
                                                                    VariableScope {
                                                                        scope: 0,
                                                                        index: 0,
                                                                    },
                                                                ),
                                                            },
                                                        ),
                                                    },
                                                    GritNodePatternArg {
                                                        slot_index: 1,
                                                        pattern: Dynamic(
                                                            Snippet(
                                                                DynamicSnippet {
                                                                    parts: [
                                                                        String(
                                                                            "",
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    GritNodePatternArg {
                                                        slot_index: 2,
                                                        pattern: Dynamic(
                                                            Snippet(
                                                                DynamicSnippet {
                                                                    parts: [
                                                                        String(
                                                                            "",
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    GritNodePatternArg {
                                                        slot_index: 3,
                                                        pattern: AstNode(
                                                            GritNodePattern {
                                                                kind: JsSyntaxKind(
                                                                    JS_CALL_ARGUMENTS,
                                                                ),
                                                                args: [
                                                                    GritNodePatternArg {
                                                                        slot_index: 0,
                                                                        pattern: AstLeafNode(
                                                                            GritLeafNodePattern {
                                                                                kind: JsSyntaxKind(
                                                                                    L_PAREN,
                                                                                ),
                                                                                equivalence_class: None,
                                                                                text: "(",
                                                                            },
                                                                        ),
                                                                    },
                                                                    GritNodePatternArg {
                                                                        slot_index: 1,
                                                                        pattern: List(
                                                                            List {
                                                                                patterns: [],
                                                                            },
                                                                        ),
                                                                    },
                                                                    GritNodePatternArg {
                                                                        slot_index: 2,
                                                                        pattern: AstLeafNode(
                                                                            GritLeafNodePattern {
                                                                                kind: JsSyntaxKind(
                                                                                    R_PAREN,
                                                                                ),
                                                                                equivalence_class: None,
                                                                                text: ")",
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                        ),
                                    },
                                ],
                            },
                        ),
                    ),
                    (
                        JsSyntaxKind(
                            JSX_TEXT,
                        ),
                        AstNode(
                            GritNodePattern {
                                kind: JsSyntaxKind(
                                    JSX_TEXT,
                                ),
                                args: [
                                    GritNodePatternArg {
                                        slot_index: 0,
                                        pattern: AstLeafNode(
                                            GritLeafNodePattern {
                                                kind: JsSyntaxKind(
                                                    JSX_TEXT_LITERAL,
                                                ),
                                                equivalence_class: None,
                                                text: "µfn && µfn()",
                                            },
                                        ),
                                    },
                                ],
                            },
                        ),
                    ),
                    (
                        JsSyntaxKind(
                            JS_PROPERTY_OBJECT_MEMBER,
                        ),
                        AstNode(
                            GritNodePattern {
                                kind: JsSyntaxKind(
                                    JS_PROPERTY_OBJECT_MEMBER,
                                ),
                                args: [
                                    GritNodePatternArg {
                                        slot_index: 0,
                                        pattern: Variable(
                                            Variable {
                                                internal: Static(
                                                    VariableScope {
                                                        scope: 0,
                                                        index: 0,
                                                    },
                                                ),
                                            },
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 1,
                                        pattern: Dynamic(
                                            Snippet(
                                                DynamicSnippet {
                                                    parts: [
                                                        String(
                                                            "",
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ),
                                    },
                                    GritNodePatternArg {
                                        slot_index: 2,
                                        pattern: AstNode(
                                            GritNodePattern {
                                                kind: JsSyntaxKind(
                                                    JS_LOGICAL_EXPRESSION,
                                                ),
                                                args: [
                                                    GritNodePatternArg {
                                                        slot_index: 0,
                                                        pattern: Dynamic(
                                                            Snippet(
                                                                DynamicSnippet {
                                                                    parts: [
                                                                        String(
                                                                            "",
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                        ),
                                                    },
                                                    GritNodePatternArg {
                                                        slot_index: 1,
                                                        pattern: AstLeafNode(
                                                            GritLeafNodePattern {
                                                                kind: JsSyntaxKind(
                                                                    AMP2,
                                                                ),
                                                                equivalence_class: None,
                                                                text: "&&",
                                                            },
                                                        ),
                                                    },
                                                    GritNodePatternArg {
                                                        slot_index: 2,
                                                        pattern: AstNode(
                                                            GritNodePattern {
                                                                kind: JsSyntaxKind(
                                                                    JS_CALL_EXPRESSION,
                                                                ),
                                                                args: [
                                                                    GritNodePatternArg {
                                                                        slot_index: 0,
                                                                        pattern: Variable(
                                                                            Variable {
                                                                                internal: Static(
                                                                                    VariableScope {
                                                                                        scope: 0,
                                                                                        index: 0,
                                                                                    },
                                                                                ),
                                                                            },
                                                                        ),
                                                                    },
                                                                    GritNodePatternArg {
                                                                        slot_index: 1,
                                                                        pattern: Dynamic(
                                                                            Snippet(
                                                                                DynamicSnippet {
                                                                                    parts: [
                                                                                        String(
                                                                                            "",
                                                                                        ),
                                                                                    ],
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    GritNodePatternArg {
                                                                        slot_index: 2,
                                                                        pattern: Dynamic(
                                                                            Snippet(
                                                                                DynamicSnippet {
                                                                                    parts: [
                                                                                        String(
                                                                                            "",
                                                                                        ),
                                                                                    ],
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                    GritNodePatternArg {
                                                                        slot_index: 3,
                                                                        pattern: AstNode(
                                                                            GritNodePattern {
                                                                                kind: JsSyntaxKind(
                                                                                    JS_CALL_ARGUMENTS,
                                                                                ),
                                                                                args: [
                                                                                    GritNodePatternArg {
                                                                                        slot_index: 0,
                                                                                        pattern: AstLeafNode(
                                                                                            GritLeafNodePattern {
                                                                                                kind: JsSyntaxKind(
                                                                                                    L_PAREN,
                                                                                                ),
                                                                                                equivalence_class: None,
                                                                                                text: "(",
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    GritNodePatternArg {
                                                                                        slot_index: 1,
                                                                                        pattern: List(
                                                                                            List {
                                                                                                patterns: [],
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                    GritNodePatternArg {
                                                                                        slot_index: 2,
                                                                                        pattern: AstLeafNode(
                                                                                            GritLeafNodePattern {
                                                                                                kind: JsSyntaxKind(
                                                                                                    R_PAREN,
                                                                                                ),
                                                                                                equivalence_class: None,
                                                                                                text: ")",
                                                                                            },
                                                                                        ),
                                                                                    },
                                                                                ],
                                                                            },
                                                                        ),
                                                                    },
                                                                ],
                                                            },
                                                        ),
                                                    },
                                                ],
                                            },
                                        ),
                                    },
                                ],
                            },
                        ),
                    ),
                ],
                source: "µfn && µfn()",
                dynamic_snippet: Some(
                    Snippet(
                        DynamicSnippet {
                            parts: [
                                String(
                                    "µfn && µfn()",
                                ),
                            ],
                        },
                    ),
                ),
            },
        )
        "###);
    }
}
