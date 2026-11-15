//! The snippet pass: places every embedded JavaScript tree in the host
//! document and summarizes its outermost expression.
//!
//! The summary lets a rule on the HTML side reason about an expression
//! without reading the JavaScript tree, and lets two snippets be compared
//! without their trees.

use super::builder::ModelBuilder;
use super::layout::{ExpectedSnippet, Layout};
use super::model::*;
use biome_js_semantic::SemanticModel as JsSemanticModel;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression,
    AnyJsRoot, JsLogicalOperator, JsSyntaxKind, JsSyntaxNode, JsUnaryOperator,
};
use biome_languages::JsFileSource;
use biome_languages::javascript::SvelteEmbeddingKind;
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TextSize};
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

/// One embedded JavaScript tree of a component.
pub struct SvelteSnippet<'a> {
    pub root: &'a AnyJsRoot,
    /// The semantic model of `root`.
    pub js: &'a JsSemanticModel,
    /// How `root` was parsed.
    pub source: JsFileSource,
    /// The range of the snippet's text in the host document.
    pub content_range: TextRange,
    /// The position in the host document of the start of `root`.
    pub content_offset: TextSize,
}

/// Creates a row for each snippet and returns the id given to each, in the
/// order of `snippets`.
pub(crate) fn collect_snippets(
    builder: &mut ModelBuilder,
    layout: &Layout,
    snippets: &[SvelteSnippet],
) -> Vec<SnippetId> {
    // Rows are stored in source order, whatever order the caller found the
    // snippets in.
    let mut order: Vec<_> = (0..snippets.len()).collect();
    order.sort_by_key(|index| snippets[*index].content_range.start());
    let mut ids = vec![SnippetId::new(0); snippets.len()];

    for (row, index) in order.into_iter().enumerate() {
        let snippet = &snippets[index];
        let id = SnippetId::new(row);
        ids[index] = id;
        let expected = find_host(builder, layout, snippet);
        let first_operand = builder.data.operands.len() as u32;
        let root = root_shape(builder, snippet, expected.host);
        builder.data.snippets.push(SnippetData {
            range: snippet.content_range,
            offset: snippet.content_offset,
            host: expected.host,
            root,
            assignability: assignability(snippet.root),
            fingerprint: fingerprint(snippet.root.syntax()),
            operands: first_operand..builder.data.operands.len() as u32,
            has_comments: has_comments(snippet.root.syntax()),
            scope: Some(expected.scope),
            in_snippet_block: expected.in_snippet_block,
            facts: ValueFacts::default(),
        });
        link(builder, layout, id, expected.host);
    }
    ids
}

/// Finds what holds `snippet`, and where its names are looked up.
fn find_host(builder: &ModelBuilder, layout: &Layout, snippet: &SvelteSnippet) -> ExpectedSnippet {
    let range = snippet.content_range;
    let embedding = snippet.source.as_embedding_kind();
    let is_source = matches!(
        embedding,
        biome_languages::javascript::JsEmbeddingKind::Svelte {
            embedding_kind: SvelteEmbeddingKind::Source,
            ..
        }
    );
    let is_generics = embedding.is_generics_list();
    for (index, script) in builder.data.scripts.iter().enumerate() {
        let element = &builder.data.elements[script.element.index()];
        let id = ScriptId::new(index);
        if is_source && script.content.contains_range(range) {
            return ExpectedSnippet {
                host: SnippetHost::Script(id),
                scope: script.scope,
                in_snippet_block: false,
            };
        }
        if is_generics && element.start_tag.contains_range(range) {
            return ExpectedSnippet {
                host: SnippetHost::Generics(id),
                scope: script.scope,
                in_snippet_block: false,
            };
        }
    }
    layout
        .expected
        .get(&range)
        .copied()
        .unwrap_or_else(|| ExpectedSnippet {
            host: SnippetHost::Unknown,
            scope: builder
                .data
                .template_scope
                .expect("the layout pass creates the template scope"),
            in_snippet_block: false,
        })
}

/// Attaches the snippet `id` to the row that holds it.
fn link(builder: &mut ModelBuilder, layout: &Layout, id: SnippetId, host: SnippetHost) {
    let data = &mut builder.data;
    match host {
        SnippetHost::Script(script) => data.scripts[script.index()].snippet = Some(id),
        SnippetHost::Generics(script) => data.scripts[script.index()].generics = Some(id),
        SnippetHost::AttrValue(attr) => {
            if let AttrValueData::Expr(snippet) = &mut data.attrs[attr.index()].value {
                *snippet = Some(id);
            }
            if layout.runes_attr == Some(attr)
                && let RootShape::Bool(value) = data.snippets[id.index()].root
            {
                data.runes_option = Some(value);
            }
        }
        SnippetHost::AttrPart(attr, index) => {
            if let AttrValueData::Interpolated(parts) = &mut data.attrs[attr.index()].value
                && let Some(ValuePartData::Expr(snippet)) = parts.get_mut(index as usize)
            {
                *snippet = Some(id);
            }
        }
        SnippetHost::BindGetter(attr) => {
            if let AttrValueData::FunctionBinding { get, .. } = &mut data.attrs[attr.index()].value
            {
                *get = Some(id);
            }
        }
        SnippetHost::BindSetter(attr) => {
            if let AttrValueData::FunctionBinding { set, .. } = &mut data.attrs[attr.index()].value
            {
                *set = Some(id);
            }
        }
        SnippetHost::Condition(branch) => data.branches[branch.index()].condition = Some(id),
        SnippetHost::EachList(block)
        | SnippetHost::AwaitPromise(block)
        | SnippetHost::KeyExpression(block)
        | SnippetHost::SnippetSignature(block) => {
            data.blocks[block.index()].expression = Some(id);
        }
        SnippetHost::EachKey(block) => data.blocks[block.index()].key = Some(id),
        SnippetHost::Text(_)
        | SnippetHost::ConstTag
        | SnippetHost::DeclarationTag
        | SnippetHost::RenderTag
        | SnippetHost::HtmlTag
        | SnippetHost::DebugTag
        | SnippetHost::Unknown => {}
    }
}

/// Returns the expression a template snippet holds.
pub(crate) fn root_expression(root: &AnyJsRoot) -> Option<AnyJsExpression> {
    match root {
        AnyJsRoot::JsExpressionTemplateRoot(root) => root.expression(),
        AnyJsRoot::JsExpressionSnippet(root) => root.expression().ok(),
        _ => None,
    }
}

/// Hashes the tokens of `node`, ignoring whitespace and comments.
pub(crate) fn fingerprint(node: &JsSyntaxNode) -> u64 {
    let mut hasher = FxHasher::default();
    for token in node.descendants_tokens(biome_rowan::Direction::Next) {
        if token.kind() == JsSyntaxKind::EOF {
            continue;
        }
        (token.kind() as u16).hash(&mut hasher);
        token.text_trimmed().hash(&mut hasher);
    }
    hasher.finish()
}

fn has_comments(node: &JsSyntaxNode) -> bool {
    node.descendants_tokens(biome_rowan::Direction::Next)
        .any(|token| token.has_leading_comments() || token.has_trailing_comments())
}

fn root_shape(builder: &mut ModelBuilder, snippet: &SvelteSnippet, host: SnippetHost) -> RootShape {
    match snippet.root {
        AnyJsRoot::JsModule(_) | AnyJsRoot::JsScript(_)
            if matches!(host, SnippetHost::Script(_)) =>
        {
            RootShape::Module
        }
        AnyJsRoot::JsSvelteSnippetRoot(_) => RootShape::SnippetSignature,
        AnyJsRoot::JsSvelteDeclarationRoot(_) | AnyJsRoot::JsSvelteLegacyConstRoot(_) => {
            RootShape::Declaration
        }
        root => match root_expression(root) {
            Some(expression) => expression_shape(builder, &expression, snippet.content_offset),
            None => RootShape::Invalid,
        },
    }
}

fn expression_shape(
    builder: &mut ModelBuilder,
    expression: &AnyJsExpression,
    offset: TextSize,
) -> RootShape {
    let expression = expression.clone().omit_parentheses();
    match &expression {
        AnyJsExpression::JsIdentifierExpression(_) => RootShape::Identifier,
        AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsComputedMemberExpression(_) => RootShape::MemberChain,
        AnyJsExpression::JsCallExpression(_) => RootShape::Call,
        AnyJsExpression::AnyJsLiteralExpression(literal) => match literal {
            AnyJsLiteralExpression::JsStringLiteralExpression(string) => string
                .inner_string_text()
                .map_or(RootShape::Invalid, |text| {
                    RootShape::String(text.text().into())
                }),
            AnyJsLiteralExpression::JsBooleanLiteralExpression(boolean) => {
                boolean.value_token().map_or(RootShape::Invalid, |token| {
                    RootShape::Bool(token.text_trimmed() == "true")
                })
            }
            AnyJsLiteralExpression::JsNullLiteralExpression(_) => RootShape::Null,
            AnyJsLiteralExpression::JsNumberLiteralExpression(_) => RootShape::Number,
            _ => RootShape::Other,
        },
        AnyJsExpression::JsTemplateExpression(template) => {
            if template.tag().is_some() {
                return RootShape::Other;
            }
            let mut text = String::new();
            for element in template.elements() {
                match element.as_js_template_chunk_element() {
                    Some(chunk) => {
                        if let Ok(token) = chunk.template_chunk_token() {
                            text.push_str(token.text_trimmed());
                        }
                    }
                    None => return RootShape::Template,
                }
            }
            RootShape::String(text.into())
        }
        AnyJsExpression::JsObjectExpression(_) => RootShape::Object,
        AnyJsExpression::JsArrayExpression(_) => RootShape::Array,
        AnyJsExpression::JsClassExpression(_) => RootShape::Class,
        AnyJsExpression::JsFunctionExpression(function) => RootShape::Function {
            params: function
                .parameters()
                .map_or(0, |parameters| parameters.items().len() as u8),
        },
        AnyJsExpression::JsArrowFunctionExpression(arrow) => RootShape::Function {
            params: match arrow.parameters() {
                Ok(AnyJsArrowFunctionParameters::AnyJsBinding(_)) => 1,
                Ok(AnyJsArrowFunctionParameters::JsParameters(parameters)) => {
                    parameters.items().len() as u8
                }
                Err(_) => 0,
            },
        },
        AnyJsExpression::JsAssignmentExpression(assignment) => match assignment.left() {
            Ok(left) => RootShape::Assignment {
                target: fingerprint(left.syntax()),
            },
            Err(_) => RootShape::Invalid,
        },
        AnyJsExpression::JsUnaryExpression(unary) => match unary.operator() {
            Ok(JsUnaryOperator::LogicalNot) => RootShape::Not,
            _ => RootShape::Other,
        },
        AnyJsExpression::JsBinaryExpression(_) => RootShape::Binary,
        AnyJsExpression::JsLogicalExpression(_) => {
            let mut group = 0;
            collect_operands(builder, &expression, offset, &mut group, false);
            RootShape::Logical
        }
        AnyJsExpression::JsConditionalExpression(_) => RootShape::Conditional,
        _ => RootShape::Other,
    }
}

fn assignability(root: &AnyJsRoot) -> Assignability {
    let Some(expression) = root_expression(root).and_then(AnyJsExpression::omit_type_wrappers)
    else {
        return Assignability::NotAssignable;
    };
    let member = match expression {
        AnyJsExpression::JsIdentifierExpression(_) => return Assignability::Assignable,
        AnyJsExpression::JsStaticMemberExpression(member) => AnyJsMemberExpression::from(member),
        AnyJsExpression::JsComputedMemberExpression(member) => AnyJsMemberExpression::from(member),
        _ => return Assignability::NotAssignable,
    };
    if member.is_optional_chain() {
        Assignability::OptionalChain
    } else {
        Assignability::Assignable
    }
}

/// Splits a condition on its outermost `||` chain into groups and each group
/// on `&&` into operands.
///
/// An `||` inside an `&&` group stays one operand: splitting it would make
/// `a && (b || c)` read as `(a && b) || c`.
fn collect_operands(
    builder: &mut ModelBuilder,
    expression: &AnyJsExpression,
    offset: TextSize,
    group: &mut u16,
    in_and: bool,
) {
    let expression = expression.clone().omit_parentheses();
    if let AnyJsExpression::JsLogicalExpression(logical) = &expression
        && let (Ok(operator), Ok(left), Ok(right)) =
            (logical.operator(), logical.left(), logical.right())
    {
        match operator {
            JsLogicalOperator::LogicalOr if !in_and => {
                collect_operands(builder, &left, offset, group, false);
                *group += 1;
                collect_operands(builder, &right, offset, group, false);
                return;
            }
            JsLogicalOperator::LogicalAnd => {
                collect_operands(builder, &left, offset, group, true);
                collect_operands(builder, &right, offset, group, true);
                return;
            }
            _ => {}
        }
    }
    builder.data.operands.push(Operand {
        range: expression.syntax().text_trimmed_range() + offset,
        fingerprint: fingerprint(expression.syntax()),
        or_group: *group,
    });
}
