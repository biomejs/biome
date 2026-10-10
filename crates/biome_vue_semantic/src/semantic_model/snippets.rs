//! The snippet pass: places every embedded JavaScript tree in the host
//! document and summarizes its outermost expression.
//!
//! The summary is what lets a rule on the HTML side reason about a directive
//! value without reading the JavaScript tree, and what lets two snippets be
//! compared without their trees.

use super::builder::ModelBuilder;
use super::layout::{Layout, split_classes};
use super::model::*;
use biome_js_semantic::SemanticModel as JsSemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsArrowFunctionParameters, AnyJsExpression, AnyJsFunctionBody,
    AnyJsLiteralExpression, AnyJsObjectMember, AnyJsRoot, AnyJsStatement, JsLogicalOperator,
    JsSyntaxKind, JsSyntaxNode, JsUnaryOperator,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange, TextSize};
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

/// One embedded JavaScript tree of a single-file component.
pub struct SfcSnippet<'a> {
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
    layout: &mut Layout,
    snippets: &[SfcSnippet],
) -> Vec<SnippetId> {
    // Rows are stored in source order, whatever order the caller found the
    // snippets in.
    let mut order: Vec<usize> = (0..snippets.len()).collect();
    order.sort_by_key(|index| snippets[*index].content_range.start());
    let mut ids = vec![SnippetId::new(0); snippets.len()];

    for (row, index) in order.into_iter().enumerate() {
        let snippet = &snippets[index];
        let id = SnippetId::new(row);
        ids[index] = id;
        let host = find_host(builder, snippet.content_range);
        let first_operand = builder.data.operands.len() as u32;
        let root = root_shape(builder, snippet, host);
        builder.data.snippets.push(SnippetData {
            range: snippet.content_range,
            offset: snippet.content_offset,
            host,
            root,
            fingerprint: fingerprint(snippet.root.syntax()),
            operands: first_operand..builder.data.operands.len() as u32,
        });

        match host {
            SnippetHost::Script(block) => {
                builder.data.blocks[block.index()].snippet = Some(id);
            }
            SnippetHost::DirectiveValue(attr) => {
                let attr_data = &mut builder.data.attrs[attr.index()];
                attr_data.value = AttrValueData::Expr(id);
                let element = attr_data.element;
                if matches!(
                    &attr_data.name,
                    AttrName::Directive {
                        kind: DirectiveKind::Bind,
                        arg: DirectiveArg::Static(arg),
                        ..
                    } if arg.text() == "class"
                ) {
                    collect_class_entries(builder, layout, snippet, element);
                }
            }
            _ => {}
        }
    }

    for (chain, branch, attr) in &layout.branch_conditions {
        if let AttrValueData::Expr(snippet) = builder.data.attrs[attr.index()].value {
            builder.data.chains[chain.index()].branches[*branch].condition = Some(snippet);
        }
    }
    ids
}

/// Finds what holds the snippet whose text is at `range`.
fn find_host(builder: &ModelBuilder, range: TextRange) -> SnippetHost {
    for (index, block) in builder.data.blocks.iter().enumerate() {
        if block.content.contains_range(range) {
            match block.kind {
                BlockKind::Script => return SnippetHost::Script(BlockId::new(index)),
                BlockKind::Style => return SnippetHost::StyleVBind(BlockId::new(index)),
                _ => {}
            }
        }
    }

    // Attributes are stored in source order and do not overlap, so the only
    // candidate is the last one that starts at or before the snippet.
    let attrs = &builder.data.attrs;
    let index = attrs.partition_point(|attr| attr.range.start() <= range.start());
    if let Some(index) = index.checked_sub(1) {
        let attr = &attrs[index];
        if attr
            .value_range
            .is_some_and(|value| value.contains_range(range))
            || (range.is_empty() && attr.range.contains_range(range))
        {
            let id = AttrId::new(index);
            return match &attr.name {
                AttrName::Directive {
                    kind: DirectiveKind::For,
                    ..
                } => SnippetHost::VForIterable(id),
                AttrName::Directive { .. } => SnippetHost::DirectiveValue(id),
                AttrName::Plain(_) => SnippetHost::Unknown,
            };
        }
    }

    // Elements are stored parents first, so the last match is the innermost.
    let element = builder
        .data
        .elements
        .iter()
        .rposition(|element| element.range.contains_range(range))
        .map(ElementId::new);
    match element {
        Some(_) => SnippetHost::Interpolation(element),
        None => SnippetHost::Unknown,
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

fn root_shape(builder: &mut ModelBuilder, snippet: &SfcSnippet, host: SnippetHost) -> RootShape {
    match snippet.root {
        AnyJsRoot::JsVueSlotPropsRoot(_) => RootShape::SlotParams,
        AnyJsRoot::JsExpressionTemplateRoot(root) => match root.expression() {
            Some(expression) => expression_shape(builder, &expression, snippet.content_offset),
            None => RootShape::Invalid,
        },
        AnyJsRoot::JsModule(_) | AnyJsRoot::JsScript(_)
            if matches!(host, SnippetHost::Script(_)) =>
        {
            RootShape::Module
        }
        // An event handler may hold statements: `@click="count++; emit('x')"`.
        root => {
            let statements: Vec<_> = root
                .syntax()
                .descendants()
                .find(|node| {
                    matches!(
                        node.kind(),
                        JsSyntaxKind::JS_MODULE_ITEM_LIST | JsSyntaxKind::JS_STATEMENT_LIST
                    )
                })
                .into_iter()
                .flat_map(|list| list.children())
                .filter(|node| node.kind() != JsSyntaxKind::JS_EMPTY_STATEMENT)
                .collect();
            match statements.as_slice() {
                [] => RootShape::Invalid,
                [statement] => match AnyJsStatement::cast_ref(statement) {
                    Some(AnyJsStatement::JsExpressionStatement(statement)) => statement
                        .expression()
                        .map_or(RootShape::Invalid, |expression| {
                            expression_shape(builder, &expression, snippet.content_offset)
                        }),
                    _ => RootShape::Statements,
                },
                _ => RootShape::Statements,
            }
        }
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
            for element in template.elements().iter() {
                match element.as_js_template_chunk_element() {
                    Some(chunk) => {
                        if let Ok(token) = chunk.template_chunk_token() {
                            text.push_str(token.text_trimmed());
                        }
                    }
                    None => return RootShape::Other,
                }
            }
            RootShape::String(text.into())
        }
        AnyJsExpression::JsObjectExpression(_) => RootShape::Object,
        AnyJsExpression::JsArrayExpression(_) => RootShape::Array,
        AnyJsExpression::JsFunctionExpression(function) => RootShape::Function {
            params: function
                .parameters()
                .map_or(0, |parameters| parameters.items().len() as u8),
        },
        AnyJsExpression::JsArrowFunctionExpression(arrow) => {
            let (params, single) = match arrow.parameters() {
                Ok(AnyJsArrowFunctionParameters::AnyJsBinding(binding)) => {
                    (1, Some(binding.syntax().text_trimmed().to_string()))
                }
                Ok(AnyJsArrowFunctionParameters::JsParameters(parameters)) => {
                    let items = parameters.items();
                    let single = (items.len() == 1)
                        .then(|| items.iter().next()?.ok())
                        .flatten()
                        .map(|parameter| parameter.syntax().text_trimmed().to_string());
                    (items.len() as u8, single)
                }
                Err(_) => (0, None),
            };
            // `value => target = value`
            if let Some(param) = single
                && let Ok(AnyJsFunctionBody::AnyJsExpression(body)) = arrow.body()
                && let AnyJsExpression::JsAssignmentExpression(assignment) = body.omit_parentheses()
                && let (Ok(left), Ok(right)) = (assignment.left(), assignment.right())
                && right.syntax().text_trimmed() == param.as_str()
            {
                return RootShape::Assignment {
                    target: fingerprint(left.syntax()),
                    from_event: true,
                };
            }
            RootShape::Function { params }
        }
        AnyJsExpression::JsAssignmentExpression(assignment) => {
            match (assignment.left(), assignment.right()) {
                (Ok(left), Ok(right)) => RootShape::Assignment {
                    target: fingerprint(left.syntax()),
                    from_event: right.syntax().text_trimmed() == "$event",
                },
                _ => RootShape::Invalid,
            }
        }
        AnyJsExpression::JsUnaryExpression(unary) => match unary.operator() {
            Ok(JsUnaryOperator::LogicalNot) => RootShape::Not,
            _ => RootShape::Other,
        },
        AnyJsExpression::JsBinaryExpression(binary) => {
            match binary.operator_token().map(|token| token.kind()) {
                Ok(JsSyntaxKind::NEQ | JsSyntaxKind::NEQ2) => RootShape::NotEqual,
                _ => RootShape::Binary,
            }
        }
        AnyJsExpression::JsLogicalExpression(_) => {
            let mut group = 0;
            collect_operands(builder, &expression, offset, &mut group);
            RootShape::Logical
        }
        AnyJsExpression::JsConditionalExpression(_) => RootShape::Conditional,
        _ => RootShape::Other,
    }
}

/// Splits a condition on `||` into groups and each group on `&&` into
/// operands.
fn collect_operands(
    builder: &mut ModelBuilder,
    expression: &AnyJsExpression,
    offset: TextSize,
    group: &mut u16,
) {
    let expression = expression.clone().omit_parentheses();
    if let AnyJsExpression::JsLogicalExpression(logical) = &expression
        && let (Ok(operator), Ok(left), Ok(right)) =
            (logical.operator(), logical.left(), logical.right())
    {
        match operator {
            JsLogicalOperator::LogicalOr => {
                collect_operands(builder, &left, offset, group);
                *group += 1;
                collect_operands(builder, &right, offset, group);
                return;
            }
            JsLogicalOperator::LogicalAnd => {
                collect_operands(builder, &left, offset, group);
                collect_operands(builder, &right, offset, group);
                return;
            }
            JsLogicalOperator::NullishCoalescing => {}
        }
    }
    builder.data.operands.push(Operand {
        range: expression.syntax().text_trimmed_range() + offset,
        fingerprint: fingerprint(expression.syntax()),
        or_group: *group,
    });
}

/// Reads the classes a `:class` expression applies.
fn collect_class_entries(
    builder: &mut ModelBuilder,
    layout: &mut Layout,
    snippet: &SfcSnippet,
    element: ElementId,
) {
    let AnyJsRoot::JsExpressionTemplateRoot(root) = snippet.root else {
        return;
    };
    let Some(expression) = root.expression() else {
        return;
    };
    let mut open = false;
    class_entries_of(
        &expression,
        Certainty::Always,
        snippet.content_offset,
        element,
        layout,
        &mut open,
    );
    if open {
        builder.data.elements[element.index()].class_open = true;
    }
}

fn class_entries_of(
    expression: &AnyJsExpression,
    certainty: Certainty,
    offset: TextSize,
    element: ElementId,
    layout: &mut Layout,
    open: &mut bool,
) {
    let push_text = |text: &str, start: TextSize, layout: &mut Layout| {
        for (class, range) in split_classes(text, start + offset) {
            layout.class_entries.push((
                element,
                ClassEntryData {
                    name: class.into(),
                    range,
                    certainty,
                    from_expr: true,
                },
            ));
        }
    };
    match expression.clone().omit_parentheses() {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string),
        ) => {
            if let (Ok(text), Ok(token)) = (string.inner_string_text(), string.value_token()) {
                // The text starts after the opening quote.
                push_text(
                    text.text(),
                    token.text_trimmed_range().start() + TextSize::from(1),
                    layout,
                );
            }
        }
        AnyJsExpression::JsArrayExpression(array) => {
            for element_node in array.elements().iter().flatten() {
                match element_node {
                    AnyJsArrayElement::AnyJsExpression(item) => {
                        class_entries_of(&item, certainty, offset, element, layout, open);
                    }
                    AnyJsArrayElement::JsSpread(_) => *open = true,
                    AnyJsArrayElement::JsArrayHole(_) => {}
                }
            }
        }
        AnyJsExpression::JsObjectExpression(object) => {
            for member in object.members().iter().flatten() {
                let AnyJsObjectMember::JsPropertyObjectMember(property) = &member else {
                    // A shorthand member names a class after a variable.
                    match &member {
                        AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
                            if let Ok(name) = shorthand.name()
                                && let Ok(token) = name.value_token()
                            {
                                layout.class_entries.push((
                                    element,
                                    ClassEntryData {
                                        name: token.text_trimmed().into(),
                                        range: token.text_trimmed_range() + offset,
                                        certainty: Certainty::Conditional,
                                        from_expr: true,
                                    },
                                ));
                            }
                        }
                        _ => *open = true,
                    }
                    continue;
                };
                let Some(name) = member.name() else {
                    *open = true;
                    continue;
                };
                let Ok(name_node) = property.name() else {
                    continue;
                };
                let is_true = property
                    .value()
                    .is_ok_and(|value| value.syntax().text_trimmed() == "true");
                let name_start = name_node.syntax().text_trimmed_range().start();
                let quoted = name_node.syntax().text_trimmed() != name.text();
                let start = name_start + TextSize::from(u32::from(quoted));
                let member_certainty = if is_true && certainty == Certainty::Always {
                    Certainty::Always
                } else {
                    Certainty::Conditional
                };
                for (class, range) in split_classes(name.text(), start + offset) {
                    layout.class_entries.push((
                        element,
                        ClassEntryData {
                            name: class.into(),
                            range,
                            certainty: member_certainty,
                            from_expr: true,
                        },
                    ));
                }
            }
        }
        AnyJsExpression::JsConditionalExpression(conditional) => {
            for branch in [conditional.consequent(), conditional.alternate()]
                .into_iter()
                .flatten()
            {
                class_entries_of(
                    &branch,
                    Certainty::Conditional,
                    offset,
                    element,
                    layout,
                    open,
                );
            }
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            if let Ok(right) = logical.right() {
                class_entries_of(
                    &right,
                    Certainty::Conditional,
                    offset,
                    element,
                    layout,
                    open,
                );
            }
        }
        _ => *open = true,
    }
}
