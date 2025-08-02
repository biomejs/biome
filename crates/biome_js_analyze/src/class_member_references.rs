use biome_js_syntax::{
    AnyJsAssignment, AnyJsClassMember, AnyJsExpression, JsArrayAssignmentPattern,
    JsArrowFunctionExpression, JsAssignmentExpression, JsClassMemberList, JsConstructorClassMember,
    JsFunctionBody, JsLanguage, JsObjectAssignmentPattern, JsObjectBindingPattern,
    JsPostUpdateExpression, JsPreUpdateExpression, JsPropertyClassMember, JsStaticMemberExpression,
    JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator, TextRange, TsPropertyParameter,
};

use biome_analyze::QueryMatch;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxNode, Text, WalkEvent, declare_node_union,
};
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ClassPropertyReference {
    pub name: Text,
    pub range: TextRange,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct References {
    pub reads: HashSet<ClassPropertyReference>,
    pub writes: HashSet<ClassPropertyReference>,
}

declare_node_union! {
    pub ClassPropMemberOrConstructorTsParam = JsPropertyClassMember | TsPropertyParameter
}

/// Collects all `this` property references used within the members of a JavaScript class.
///
/// This function traverses a `JsClassMemberList` and extracts property references from method bodies,
/// getters, setters, arrow functions assigned to properties, and constructors. It aggregates both
/// read and write references to `this` properties across all supported member types.
///
/// Returns a `References` struct containing the combined set of read and write references.
pub fn class_member_references(list: &JsClassMemberList) -> References {
    let all_references: Vec<References> = list
        .iter()
        .filter_map(|member| match member {
            AnyJsClassMember::JsMethodClassMember(method) => method
                .body()
                .ok()
                .and_then(|body| collect_references_from_body(method.syntax(), &body)),
            AnyJsClassMember::JsSetterClassMember(setter) => setter
                .body()
                .ok()
                .and_then(|body| collect_references_from_body(setter.syntax(), &body)),
            AnyJsClassMember::JsGetterClassMember(getter) => getter
                .body()
                .ok()
                .and_then(|body| collect_references_from_body(getter.syntax(), &body)),
            AnyJsClassMember::JsPropertyClassMember(property) => {
                if let Ok(expression) = property.value()?.expression() {
                    if let Some(arrow_function) =
                        JsArrowFunctionExpression::cast(expression.clone().into_syntax())
                    {
                        if let Ok(any_js_body) = arrow_function.body()
                            && let Some(body) = any_js_body.as_js_function_body()
                        {
                            return collect_references_from_body(arrow_function.syntax(), body);
                        }
                    }

                    if let Some(static_member_expression) =
                        expression.as_js_static_member_expression()
                    {
                        return collect_references_from_property_member(static_member_expression);
                    }
                };
                None
            }
            AnyJsClassMember::JsConstructorClassMember(constructor) => constructor
                .body()
                .ok()
                .map(|body| collect_references_from_constructor(&body)),
            _ => None,
        })
        .collect();

    let mut combined_reads = HashSet::new();
    let mut combined_writes = HashSet::new();

    for refs in all_references {
        combined_reads.extend(refs.reads);
        combined_writes.extend(refs.writes);
    }

    References {
        reads: combined_reads,
        writes: combined_writes,
    }
}

/// Represents a function body and all `this` aliases valid within its lexical scope.
#[derive(Clone, Debug)]
struct ScopedThisAliases {
    scope: JsFunctionBody,
    aliases: HashSet<ClassPropertyReference>,
}

/// A visitor that collects `this` aliases in nested function scopes,
/// while skipping class expressions and tracking inherited aliases.
struct ScopedThisAliasVisitor<'a> {
    skipped_ranges: Vec<TextRange>,
    inherited_aliases: &'a [ClassPropertyReference],
    scoped_this_aliases: Vec<ScopedThisAliases>,
}
// can not implement `Visitor` directly because it requires a new ctx that can not be created here
impl ScopedThisAliasVisitor<'_> {
    fn visit(&mut self, event: &WalkEvent<SyntaxNode<JsLanguage>>) {
        match event {
            WalkEvent::Enter(node) => {
                if self
                    .skipped_ranges
                    .iter()
                    .any(|range| range.contains_range(node.text_range()))
                {
                    return;
                }

                if node.kind() == JsSyntaxKind::JS_CLASS_EXPRESSION {
                    self.skipped_ranges.push(node.text_range());
                    return;
                }

                if node.kind() == JsSyntaxKind::JS_CLASS_DECLARATION {
                    self.skipped_ranges.push(node.text_range());
                    return;
                }

                if let Some(body) = JsFunctionBody::cast_ref(node) {
                    // Only process if not part of constructor
                    let is_constructor = node
                        .parent()
                        .and_then(JsConstructorClassMember::cast)
                        .is_some();

                    if !is_constructor {
                        let current_scope_aliases =
                            ThisAliasResolver::collect_local_this_aliases(&body);
                        let mut this_aliases = HashSet::new();
                        this_aliases.extend(self.inherited_aliases.iter().cloned());
                        this_aliases.extend(current_scope_aliases);

                        self.scoped_this_aliases.push(ScopedThisAliases {
                            scope: body.clone(),
                            aliases: this_aliases,
                        });
                    }
                }

                if let Some(func_expr) = JsArrowFunctionExpression::cast_ref(node)
                    && let Some(body) = func_expr
                        .body()
                        .ok()
                        .and_then(|b| b.as_js_function_body().cloned())
                {
                    let current_scope_aliases =
                        ThisAliasResolver::collect_local_this_aliases(&body);
                    let mut this_aliases = HashSet::new();
                    this_aliases.extend(self.inherited_aliases.iter().cloned());
                    this_aliases.extend(current_scope_aliases.clone());

                    self.scoped_this_aliases.push(ScopedThisAliases {
                        scope: body.clone(),
                        aliases: this_aliases,
                    });
                }
            }

            WalkEvent::Leave(node) => {
                if let Some(last) = self.skipped_ranges.last() {
                    if *last == node.text_range() {
                        self.skipped_ranges.pop();
                    }
                }
            }
        }
    }
}

/// `ThisAliasResolver` provides methods to collect and check `this` aliases within JavaScript functions.
///
/// - `collect_local_this_aliases`: Collects local aliases of `this` in a function body.
/// - `collect_all_nested_this_aliases`: Recursively collects `this` aliases in nested function bodies.
/// - `is_this_or_alias`: Checks if a given expression is a reference to `this` or any of its aliases.
struct ThisAliasResolver {}

impl ThisAliasResolver {
    fn collect_local_this_aliases(body: &JsFunctionBody) -> Vec<ClassPropertyReference> {
        body.statements()
            .iter()
            .filter_map(|node| node.as_js_variable_statement().cloned())
            .filter_map(|stmt| stmt.declaration().ok().map(|decl| decl.declarators()))
            .flat_map(|declarators| {
                declarators.into_iter().filter_map(|declaration| {
                    declaration.ok().map(|declarator| declarator.as_fields())
                })
            })
            .filter_map(|fields| {
                let id = fields.id.ok()?;
                let expr = fields.initializer?.expression().ok()?;
                let unwrapped = unwrap_expression(&expr);

                (unwrapped.syntax().first_token()?.text_trimmed() == "this").then(|| {
                    ClassPropertyReference {
                        name: id.to_trimmed_text().clone(),
                        range: id.syntax().text_trimmed_range(),
                    }
                })
            })
            .collect()
    }

    fn collect_all_nested_this_aliases(
        body_element: &JsSyntaxNode,
        parent_this_aliases: &[ClassPropertyReference],
    ) -> Vec<ScopedThisAliases> {
        let mut visitor = ScopedThisAliasVisitor {
            skipped_ranges: vec![],
            scoped_this_aliases: vec![],
            inherited_aliases: parent_this_aliases,
        };

        let iter = body_element.preorder();

        for event in iter {
            visitor.visit(&event);
        }

        visitor.scoped_this_aliases
    }

    fn is_this_or_alias(
        js_expression: &AnyJsExpression,
        this_aliases: &[ScopedThisAliases],
    ) -> bool {
        if let Some(this_expr) = js_expression.as_js_this_expression() {
            let syntax = this_expr.syntax();

            return this_aliases.iter().any(|ScopedThisAliases { scope, .. }| {
                is_within_scope_without_shadowing(syntax, scope.syntax())
            });
        }

        if let Some(js_identifier_expression) = js_expression.as_js_identifier_expression()
            && let Ok(name) = js_identifier_expression.name()
            && let Ok(value_token) = name.value_token()
        {
            let name_syntax = name.syntax();

            this_aliases
                .iter()
                .any(|ScopedThisAliases { aliases, scope }| {
                    let is_alias = aliases.iter().any(|mutation| {
                        mutation
                            .name
                            .text()
                            .eq(value_token.token_text_trimmed().text())
                    });

                    let is_within_scope =
                        is_within_scope_without_shadowing(name_syntax, scope.syntax());

                    is_alias && is_within_scope
                })
        } else {
            false
        }
    }
}

/// `ThisPatternResolver` provides methods to extract `this` references from array and object assignment patterns.
///
/// - `collect_array_assignment_names`: Extracts `this` references from array assignments (e.g., `[this.#value]` or `[...this.#value]`).
/// - `collect_object_assignment_names`: Extracts `this` references from object assignments (e.g., `{...this.#value}`).
/// - `extract_static_assignment_name`: Extracts the name of a static member assignment, checking if it's referencing `this` or its aliases.
struct ThisPatternResolver {}
impl ThisPatternResolver {
    fn collect_array_assignment_names(
        array_assignment_pattern: &JsArrayAssignmentPattern,
        this_aliases: &[ScopedThisAliases],
    ) -> Vec<ClassPropertyReference> {
        array_assignment_pattern
            .elements()
            .iter()
            .filter_map(|element| {
                let element = element.clone().ok()?;

                // [this.#value]
                if let Some(pattern_element) = element.as_js_array_assignment_pattern_element() {
                    pattern_element
                        .pattern()
                        .ok()?
                        .as_any_js_assignment()
                        .and_then(|assignment| {
                            Self::extract_static_assignment_name(assignment, this_aliases)
                        })
                }
                // [...this.#value]
                else if let Some(rest_element) =
                    element.as_js_array_assignment_pattern_rest_element()
                {
                    rest_element
                        .pattern()
                        .ok()?
                        .as_any_js_assignment()
                        .and_then(|assignment| {
                            Self::extract_static_assignment_name(assignment, this_aliases)
                        })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Collects assignment names from a JavaScript object assignment pattern, e.g. `{...this.#value}`.
    fn collect_object_assignment_names(
        assignment: &JsObjectAssignmentPattern,
        this_aliases: &[ScopedThisAliases],
    ) -> Vec<ClassPropertyReference> {
        assignment
            .properties()
            .elements()
            .filter_map(|prop| {
                if let Some(rest_params) = prop
                    .node
                    .clone()
                    .ok()?
                    .as_js_object_assignment_pattern_rest()
                {
                    return Self::extract_static_assignment_name(
                        &rest_params.target().ok()?,
                        this_aliases,
                    );
                }
                if let Some(property) = prop
                    .node
                    .clone()
                    .ok()?
                    .as_js_object_assignment_pattern_property()
                {
                    return Self::extract_static_assignment_name(
                        property.pattern().ok()?.as_any_js_assignment()?,
                        this_aliases,
                    );
                }
                None
            })
            .collect()
    }

    /// Extracts the name of a static member assignment from an AnyJsAssignment node.
    /// Checks for this or static references, casts to a static member assignment, and retrieves the trimmed name.
    fn extract_static_assignment_name(
        operand: &AnyJsAssignment,
        this_aliases: &[ScopedThisAliases],
    ) -> Option<ClassPropertyReference> {
        operand
            .as_js_static_member_assignment()
            .and_then(|assignment| {
                if let Ok(object) = assignment.object()
                    && ThisAliasResolver::is_this_or_alias(&object, this_aliases)
                {
                    assignment.member().ok().and_then(|member| {
                        member
                            .as_js_name()
                            .map(|name| ClassPropertyReference {
                                name: name.to_trimmed_text(),
                                range: name.syntax().text_trimmed_range(),
                            })
                            .or_else(|| {
                                member.as_js_private_name().map(|private_name| {
                                    ClassPropertyReference {
                                        name: private_name.to_trimmed_text(),
                                        range: private_name.syntax().text_trimmed_range(),
                                    }
                                })
                            })
                    })
                } else {
                    None
                }
            })
    }
}

/// Collects `this`-based member references from a class method or property initializer body.
/// Gathers reads and writes by analyzing the function body and its `this` aliases.
fn collect_references_from_body(
    member: &JsSyntaxNode,
    body: &JsFunctionBody,
) -> Option<References> {
    let this_variable_aliases: Vec<_> = ThisAliasResolver::collect_local_this_aliases(body);
    let this_aliases =
        ThisAliasResolver::collect_all_nested_this_aliases(body.syntax(), &this_variable_aliases);

    let mut reads = Vec::new();
    let mut writes = Vec::new();

    visit_references_in_body(
        member,
        &this_aliases,
        &mut |name| {
            writes.push(name);
        },
        &mut |name| {
            reads.push(name);
        },
    );

    Some(References {
        reads: reads.into_iter().collect(),
        writes: writes.into_iter().collect(),
    })
}

/// Traverses a JavaScript method or initializer body to collect references
/// to `this`-based class members, calling the provided callbacks for reads and writes.
///
/// It detects:
/// - Reads via `this.prop`, `this.#prop`, and compound assignments (e.g., `this.prop += 1`)
/// - Writes via assignments and destructuring patterns involving `this` or its aliases
fn visit_references_in_body<F, S>(
    method_body_element: &JsSyntaxNode,
    this_aliases: &[ScopedThisAliases],
    on_write_match: &mut F,
    on_read_match: &mut S,
) where
    F: FnMut(ClassPropertyReference),
    S: FnMut(ClassPropertyReference),
{
    let iter = method_body_element.preorder();

    for event in iter {
        match event {
            WalkEvent::Enter(node) => {
                if let Some(binding) = JsObjectBindingPattern::cast_ref(&node)
                    && let Some(parent) = binding.syntax().parent()
                    && let Some(variable_declarator) = JsVariableDeclarator::cast_ref(&parent)
                    && let Some(initializer) = variable_declarator.initializer()
                    && let Ok(expression) = initializer.expression()
                {
                    for declarator in binding.properties() {
                        if let Some(declarator) = declarator.ok()
                            && ThisAliasResolver::is_this_or_alias(&expression, this_aliases)
                        {
                            on_read_match(ClassPropertyReference {
                                name: declarator.to_trimmed_text(),
                                range: declarator.syntax().text_trimmed_range(),
                            });
                        }
                    }
                }

                if let Some(static_member) = JsStaticMemberExpression::cast_ref(&node)
                    && let Ok(object) = static_member.object()
                    && ThisAliasResolver::is_this_or_alias(&object, this_aliases)
                    && let Ok(member) = static_member.member()
                {
                    on_read_match(ClassPropertyReference {
                        name: member.to_trimmed_text(),
                        range: static_member.syntax().text_trimmed_range(),
                    });
                }

                if let Some(assignment) = JsAssignmentExpression::cast_ref(&node)
                    && let Ok(left) = assignment.left()
                {
                    if let Ok(operator) = assignment.operator_token()
                        && let Some(operand) = left.as_any_js_assignment()
                        && matches!(
                            operator.kind(),
                            JsSyntaxKind::PIPE2EQ
                                | JsSyntaxKind::AMP2EQ
                                | JsSyntaxKind::SLASHEQ
                                | JsSyntaxKind::STAREQ
                                | JsSyntaxKind::PERCENTEQ
                                | JsSyntaxKind::PLUSEQ
                                | JsSyntaxKind::QUESTION2EQ
                        )
                        && let Some(name) = ThisPatternResolver::extract_static_assignment_name(
                            operand,
                            this_aliases,
                        )
                    {
                        on_read_match(name);
                    }

                    if let Some(array) = left.as_js_array_assignment_pattern().cloned() {
                        ThisPatternResolver::collect_array_assignment_names(&array, this_aliases)
                            .into_iter()
                            .for_each(&mut *on_write_match);
                    }

                    if let Some(object) = left.as_js_object_assignment_pattern().cloned() {
                        ThisPatternResolver::collect_object_assignment_names(&object, this_aliases)
                            .into_iter()
                            .for_each(&mut *on_write_match);
                    }

                    if let Some(assignment) = left.as_any_js_assignment().cloned()
                        && let Some(name) = ThisPatternResolver::extract_static_assignment_name(
                            &assignment,
                            this_aliases,
                        )
                    {
                        on_write_match(name);
                    }
                }

                let operand = JsPostUpdateExpression::cast_ref(&node)
                    .and_then(|expr| expr.operand().ok())
                    .or_else(|| {
                        JsPreUpdateExpression::cast_ref(&node.clone())
                            .and_then(|expr| expr.operand().ok())
                    });

                if let Some(operand) = operand
                    && let Some(name) =
                        ThisPatternResolver::extract_static_assignment_name(&operand, this_aliases)
                {
                    on_write_match(name.clone());
                    on_read_match(name);
                }
            }
            WalkEvent::Leave(_) => {}
        }
    }
}

/// Collects read and write references to `this` members within a class constructor body,
/// including any nested functions that capture `this` via aliasing.
fn collect_references_from_constructor(constructor_body: &JsFunctionBody) -> References {
    let this_variable_aliases: Vec<_> =
        ThisAliasResolver::collect_local_this_aliases(constructor_body);

    let all_descendants_fn_bodies_and_this_aliases: Vec<_> =
        ThisAliasResolver::collect_all_nested_this_aliases(
            constructor_body.syntax(),
            &this_variable_aliases,
        );
    let mut reads = Vec::new();
    let mut writes = Vec::new();

    for this_scope in all_descendants_fn_bodies_and_this_aliases.iter() {
        visit_references_in_body(
            this_scope.scope.syntax(),
            std::slice::from_ref(this_scope),
            &mut |name| writes.push(name),
            &mut |name| reads.push(name),
        );
    }

    References {
        reads: reads.into_iter().collect(),
        writes: writes.into_iter().collect(),
    }
}

/// Collects references to class members accessed through a static member expression,
/// such as `this.prop` or `this.#privateProp`.
fn collect_references_from_property_member(
    static_member: &JsStaticMemberExpression,
) -> Option<References> {
    let mut reads = Vec::new();
    let writes = Vec::new();

    if let Ok(member) = static_member.member() {
        let name = member.to_trimmed_text();
        reads.push(ClassPropertyReference {
            name,
            range: static_member.syntax().text_trimmed_range(),
        });
    }

    Some(References {
        reads: reads.into_iter().collect(),
        writes: writes.into_iter().collect(),
    })
}

/// Recursively unwraps expressions like parentheses to find the inner expression.
fn unwrap_expression(expr: &AnyJsExpression) -> AnyJsExpression {
    match expr {
        AnyJsExpression::JsParenthesizedExpression(paren_expr) => {
            if let Ok(inner) = paren_expr.expression() {
                let cloned_inner = inner.clone();
                unwrap_expression(&cloned_inner)
            } else {
                expr.clone()
            }
        }
        _ => expr.clone(),
    }
}

fn is_within_scope_without_shadowing(
    name_syntax: &SyntaxNode<JsLanguage>,
    scope: &SyntaxNode<JsLanguage>,
) -> bool {
    for ancestor in name_syntax.ancestors() {
        if ancestor.key() == scope.key() {
            return true;
        }

        match ancestor.kind() {
            JsSyntaxKind::JS_FUNCTION_BODY
            | JsSyntaxKind::JS_CLASS_EXPRESSION
            | JsSyntaxKind::JS_CLASS_DECLARATION => return false,
            _ => {}
        }
    }

    false
}
