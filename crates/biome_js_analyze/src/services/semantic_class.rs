use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, Visitor, VisitorContext, VisitorFinishContext,
};
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsClassMember, AnyJsExpression, AnyJsObjectBindingPatternMember,
    AnyJsRoot, JsArrayAssignmentPattern, JsArrowFunctionExpression, JsAssignmentExpression,
    JsClassDeclaration, JsClassMemberList, JsConstructorClassMember, JsFunctionBody, JsLanguage,
    JsObjectAssignmentPattern, JsObjectBindingPattern, JsPostUpdateExpression,
    JsPreUpdateExpression, JsPropertyClassMember, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator, TextRange,
    TsPropertyParameter,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxNode, Text, WalkEvent, declare_node_union,
};
use rustc_hash::FxHashSet;
use std::option::Option;

#[derive(Clone)]
pub struct SemanticClassServices {
    pub model: SemanticClassModel,
}

impl SemanticClassServices {
    pub fn model(&self) -> &SemanticClassModel {
        &self.model
    }
}

#[derive(Debug, Clone)]
pub struct SemanticClassModel {}

impl SemanticClassModel {
    pub fn class_member_references(&self, members: &JsClassMemberList) -> ClassMemberReferences {
        class_member_references(members)
    }
}

impl FromServices for SemanticClassServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let service: &SemanticClassModel = services.get_service().ok_or_else(|| {
            ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticClassModel"])
        })?;
        Ok(Self {
            model: service.clone(),
        })
    }
}

impl Phase for SemanticClassServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

pub struct SyntaxClassMemberReferencesVisitor {}

impl Visitor for SyntaxClassMemberReferencesVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        _event: &WalkEvent<JsSyntaxNode>,
        mut _ctx: VisitorContext<'_, '_, JsLanguage>,
    ) {
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        ctx.services.insert_service(SemanticClassModel {});
    }
}

pub struct SemanticClassMemberReferencesVisitor {}

impl Visitor for SemanticClassMemberReferencesVisitor {
    type Language = JsLanguage;

    fn visit(
        &mut self,
        event: &WalkEvent<JsSyntaxNode>,
        mut ctx: VisitorContext<'_, '_, JsLanguage>,
    ) {
        if let WalkEvent::Enter(node) = event
            && JsClassDeclaration::can_cast(node.kind())
        {
            ctx.match_query(node.clone());
        }
    }
}

#[derive(Clone)]
pub struct SemanticClass<N>(pub N);

impl QueryMatch for SemanticClass<JsClassDeclaration> {
    fn text_range(&self) -> TextRange {
        self.0.syntax().text_trimmed_range()
    }
}

impl<N> Queryable for SemanticClass<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = SemanticClassServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SyntaxClassMemberReferencesVisitor {});
        analyzer.add_visitor(Phases::Semantic, || SemanticClassMemberReferencesVisitor {});
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_service_bag: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

/// Represents how a class member is accessed within the code.
/// Variants:
///
/// - `Write`:
///   The member is being assigned to or mutated.
///   Example: `this.count = 10;`
///   This indicates the member’s value/state changes at this point.
///
/// - `MeaningfulRead`:
///   The member’s value is retrieved and used in a way that affects program logic.
///   Example: `if (this.enabled) { ... }` or `let x = this.value + 1;`
///   These reads influence control flow or computation.
///
/// - `TrivialRead`:
///   The member is accessed, but its value is not used in a way that
///   meaningfully affects logic.
///   Example: `this.value;` as a standalone expression, or a read that is optimized away.
///   This is mostly for distinguishing dead reads from truly meaningful ones.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum AccessKind {
    Write,
    MeaningfulRead,
    TrivialRead,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ClassMemberReference {
    pub name: Text,
    pub range: TextRange,
    pub access_kind: AccessKind,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct ClassMemberReferences {
    pub reads: FxHashSet<ClassMemberReference>,
    pub writes: FxHashSet<ClassMemberReference>,
}

declare_node_union! {
    pub AnyPropertyMember = JsPropertyClassMember | TsPropertyParameter
}

declare_node_union! {
    pub AnyCandidateForUsedInExpressionNode = AnyJsUpdateExpression | AnyJsObjectBindingPatternMember | JsStaticMemberExpression | AnyJsBindingPattern
}

declare_node_union! {
    pub AnyJsUpdateExpression = JsPreUpdateExpression | JsPostUpdateExpression
}

/// Collects all `this` property references used within the members of a JavaScript class.
///
/// This function traverses a `JsClassMemberList` and extracts property references from method bodies,
/// getters, setters, arrow functions assigned to properties, and constructors. It aggregates both
/// read and write references to `this` properties across all supported member types.
///
/// Returns a `ClassMemberReferences` struct containing the combined set of read and write references.
fn class_member_references(list: &JsClassMemberList) -> ClassMemberReferences {
    let all_references: Vec<ClassMemberReferences> = list
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
                    } else if let Some(static_member_expression) =
                        expression.as_js_static_member_expression()
                    {
                        return collect_class_property_reads_from_static_member(
                            static_member_expression,
                        );
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

    let mut combined_reads = FxHashSet::default();
    let mut combined_writes = FxHashSet::default();

    for refs in all_references {
        combined_reads.extend(refs.reads);
        combined_writes.extend(refs.writes);
    }

    ClassMemberReferences {
        reads: combined_reads,
        writes: combined_writes,
    }
}

/// Represents a function body and all `this` references (including aliases) valid within its lexical scope.
#[derive(Clone, Debug)]
struct FunctionThisReferences {
    scope: JsFunctionBody,
    this_references: FxHashSet<ClassMemberReference>,
}

/// A visitor that collects `this` references in nested function scopes,
/// while skipping class expressions and tracking inherited this references.
struct ThisScopeVisitor<'a> {
    skipped_ranges: Vec<TextRange>,
    inherited_this_references: &'a [ClassMemberReference],
    current_this_scopes: Vec<FunctionThisReferences>,
}
// Can not implement `Visitor` directly because it requires a new ctx that can not be created here
impl ThisScopeVisitor<'_> {
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
                        let current_scope = ThisScopeReferences::new(&body).local_this_references;
                        let mut scoped_this_references = FxHashSet::default();
                        scoped_this_references
                            .extend(self.inherited_this_references.iter().cloned());
                        scoped_this_references.extend(current_scope);

                        self.current_this_scopes.push(FunctionThisReferences {
                            scope: body.clone(),
                            this_references: scoped_this_references,
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
                        ThisScopeReferences::new(&body).local_this_references;
                    let mut scoped_this_references = FxHashSet::default();
                    scoped_this_references.extend(self.inherited_this_references.iter().cloned());
                    scoped_this_references.extend(current_scope_aliases.clone());

                    self.current_this_scopes.push(FunctionThisReferences {
                        scope: body.clone(),
                        this_references: scoped_this_references,
                    });
                }
            }

            WalkEvent::Leave(node) => {
                if let Some(last) = self.skipped_ranges.last()
                    && *last == node.text_range()
                {
                    self.skipped_ranges.pop();
                }
            }
        }
    }
}

/// Provides local or all `this` references found in a function body
struct ThisScopeReferences {
    /// Any js function body
    body: JsFunctionBody,
    /// this scope references found within the immediate function scope body, excludes nested scopes
    local_this_references: Vec<ClassMemberReference>,
}

impl ThisScopeReferences {
    fn new(body: &JsFunctionBody) -> Self {
        Self {
            body: body.clone(),
            local_this_references: Self::collect_local_this_references(body),
        }
    }

    /// Collects all `this` scope references in the function body and nested
    /// functions using `ThisScopeVisitor`, combining local and inherited ones
    /// into a list of `FunctionThisReferences`.
    fn collect_function_this_references(&self) -> Vec<FunctionThisReferences> {
        let mut visitor = ThisScopeVisitor {
            skipped_ranges: vec![],
            current_this_scopes: vec![],
            inherited_this_references: self.local_this_references.as_slice(),
        };

        let iter = self.body.syntax().preorder();
        for event in iter {
            visitor.visit(&event);
        }

        visitor.current_this_scopes
    }

    /// Collects local references of `this` in a function body.
    fn collect_local_this_references(body: &JsFunctionBody) -> Vec<ClassMemberReference> {
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
                let unwrapped = &expr.omit_parentheses();
                (unwrapped.syntax().first_token()?.text_trimmed() == "this").then(|| {
                    ClassMemberReference {
                        name: id.to_trimmed_text().clone(),
                        range: id.syntax().text_trimmed_range(),
                        access_kind: get_read_access_kind(
                            &AnyCandidateForUsedInExpressionNode::from(id),
                        ),
                    }
                })
            })
            .collect()
    }
}

/// Checks if a given expression is a reference to `this` or any of its aliases.
fn is_this_reference(
    js_expression: &AnyJsExpression,
    scoped_this_references: &[FunctionThisReferences],
) -> bool {
    if let Some(this_expr) = js_expression.as_js_this_expression() {
        let syntax = this_expr.syntax();

        return scoped_this_references
            .iter()
            .any(|FunctionThisReferences { scope, .. }| {
                is_within_scope_without_shadowing(syntax, scope.syntax())
            });
    }

    if let Some(js_identifier_expression) = js_expression.as_js_identifier_expression()
        && let Ok(name) = js_identifier_expression.name()
        && let Ok(value_token) = name.value_token()
    {
        let name_syntax = name.syntax();

        scoped_this_references.iter().any(
            |FunctionThisReferences {
                 this_references,
                 scope,
             }| {
                let is_alias = this_references.iter().any(|mutation| {
                    mutation
                        .name
                        .text()
                        .eq(value_token.token_text_trimmed().text())
                });

                let is_within_scope =
                    is_within_scope_without_shadowing(name_syntax, scope.syntax());

                is_alias && is_within_scope
            },
        )
    } else {
        false
    }
}

/// Provides methods to extract `this` references from array and object assignment patterns.
struct ThisPatternResolver {}

impl ThisPatternResolver {
    /// Extracts `this` references from array assignments (e.g., `[this.#value]` or `[...this.#value]`).
    /// Only applicable to writes.
    fn collect_array_assignment_names(
        array_assignment_pattern: &JsArrayAssignmentPattern,
        scoped_this_references: &[FunctionThisReferences],
    ) -> Vec<ClassMemberReference> {
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
                            Self::extract_this_member_reference(
                                assignment.as_js_static_member_assignment(),
                                scoped_this_references,
                                AccessKind::Write,
                            )
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
                            Self::extract_this_member_reference(
                                assignment.as_js_static_member_assignment(),
                                scoped_this_references,
                                AccessKind::Write,
                            )
                        })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Collects assignment names from a JavaScript object assignment pattern, e.g. `{...this.#value}`.
    /// Only applicable to writes.
    fn collect_object_assignment_names(
        assignment: &JsObjectAssignmentPattern,
        scoped_this_references: &[FunctionThisReferences],
    ) -> Vec<ClassMemberReference> {
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
                    return Self::extract_this_member_reference(
                        rest_params.target().ok()?.as_js_static_member_assignment(),
                        scoped_this_references,
                        AccessKind::Write,
                    );
                }
                if let Some(property) = prop
                    .node
                    .clone()
                    .ok()?
                    .as_js_object_assignment_pattern_property()
                {
                    return Self::extract_this_member_reference(
                        property
                            .pattern()
                            .ok()?
                            .as_any_js_assignment()?
                            .as_js_static_member_assignment(),
                        scoped_this_references,
                        AccessKind::Write,
                    );
                }
                None
            })
            .collect()
    }

    /// Extracts a class member reference from an assignment if it involves `this` or its aliases.
    ///
    /// Example:
    /// - `this.prop = value`
    /// - `this.#private = value`
    /// - `self.prop = value` (where `self` is a `this` alias)
    ///
    /// Returns a `ClassMemberReference` containing the member name and its range.
    fn extract_this_member_reference(
        operand: Option<&JsStaticMemberAssignment>,
        scoped_this_references: &[FunctionThisReferences],
        access_kind: AccessKind,
    ) -> Option<ClassMemberReference> {
        operand.and_then(|assignment| {
            if let Ok(object) = assignment.object()
                && is_this_reference(&object, scoped_this_references)
            {
                assignment.member().ok().and_then(|member| {
                    member
                        .as_js_name()
                        .map(|name| ClassMemberReference {
                            name: name.to_trimmed_text(),
                            range: name.syntax().text_trimmed_range(),
                            access_kind: access_kind.clone(),
                        })
                        .or_else(|| {
                            member
                                .as_js_private_name()
                                .map(|private_name| ClassMemberReference {
                                    name: private_name.to_trimmed_text(),
                                    range: private_name.syntax().text_trimmed_range(),
                                    access_kind,
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
/// Gathers reads and writes by analyzing the function body and its `this` references (and its aliases).
fn collect_references_from_body(
    member: &JsSyntaxNode,
    body: &JsFunctionBody,
) -> Option<ClassMemberReferences> {
    let scoped_this_references = ThisScopeReferences::new(body).collect_function_this_references();

    let mut reads = FxHashSet::default();
    let mut writes = FxHashSet::default();

    visit_references_in_body(member, &scoped_this_references, &mut writes, &mut reads);

    Some(ClassMemberReferences { reads, writes })
}

/// Traverses a JavaScript method or initializer body to collect references
/// to `this`-based class members, calling the provided callbacks for reads and writes.
///
/// It detects:
/// - Reads via `this.prop`, `this.#prop`, and compound assignments (e.g., `this.prop += 1`)
/// - Writes via assignments and destructuring patterns involving `this` or its aliases
fn visit_references_in_body(
    method_body_element: &JsSyntaxNode,
    scoped_this_references: &[FunctionThisReferences],
    writes: &mut FxHashSet<ClassMemberReference>,
    reads: &mut FxHashSet<ClassMemberReference>,
) {
    let iter = method_body_element.preorder();

    for event in iter {
        match event {
            WalkEvent::Enter(node) => {
                handle_object_binding_pattern(&node, scoped_this_references, reads);
                handle_static_member_expression(&node, scoped_this_references, reads);
                handle_assignment_expression(&node, scoped_this_references, reads, writes);
                if let Some(js_update_expression) = AnyJsUpdateExpression::cast_ref(&node) {
                    handle_pre_or_post_update_expression(
                        &js_update_expression,
                        scoped_this_references,
                        reads,
                        writes,
                    );
                }
            }
            WalkEvent::Leave(_) => {}
        }
    }
}

/// Detects `this` property reads in object destructuring bindings,
/// e.g. `const { foo, bar } = this;` or `let { x } = aliasForThis;`.
///
/// Extracts each property name from the binding pattern and records it
/// as a read reference if the initializer is `this` or a known `this` alias.
///
/// JavaScript example:
/// ```js
/// class Example {
///   method() {
///     const { foo, bar } = this;   // reads: foo, bar
///     let { x } = aliasForThis;    // reads: x (if aliasForThis is a known alias)
///   }
/// }
/// ```
fn handle_object_binding_pattern(
    node: &SyntaxNode<JsLanguage>,
    scoped_this_references: &[FunctionThisReferences],
    reads: &mut FxHashSet<ClassMemberReference>,
) {
    if let Some(binding) = JsObjectBindingPattern::cast_ref(node)
        && let Some(parent) = binding.syntax().parent()
        && let Some(variable_declarator) = JsVariableDeclarator::cast_ref(&parent)
        && let Some(initializer) = variable_declarator.initializer()
        && let Ok(expression) = initializer.expression()
    {
        for declarator in binding.properties() {
            if let Some(declarator) = declarator.ok()
                && is_this_reference(&expression, scoped_this_references)
            {
                reads.insert(ClassMemberReference {
                    name: declarator.clone().to_trimmed_text(),
                    range: declarator.clone().syntax().text_trimmed_range(),
                    access_kind: get_read_access_kind(&declarator.clone().into()),
                });
            }
        }
    }
}

/// Detects direct static property reads from `this` or its aliases,
/// e.g. `this.foo` or `aliasForThis.#privateProp`.
///
/// Adds the property name to the read references if the object
/// of the static member is `this` or a known `this` alias.
///
/// JavaScript example:
/// ```js
/// class Example {
///   method() {
///     console.log(this.foo);        // reads: foo
///     console.log(aliasForThis.bar); // reads: bar (if alias is known)
///     console.log(this.#secret);    // reads: #secret
///   }
/// }
/// ```
fn handle_static_member_expression(
    node: &SyntaxNode<JsLanguage>,
    scoped_this_references: &[FunctionThisReferences],
    reads: &mut FxHashSet<ClassMemberReference>,
) {
    if let Some(static_member) = JsStaticMemberExpression::cast_ref(node)
        && let Ok(object) = static_member.object()
        && is_this_reference(&object, scoped_this_references)
        && let Ok(member) = static_member.member()
    {
        reads.insert(ClassMemberReference {
            name: member.to_trimmed_text(),
            range: member.syntax().text_trimmed_range(),
            access_kind: get_read_access_kind(&static_member.into()),
        });
    }
}

/// Detects reads and writes to `this` properties inside assignment expressions.
///
/// - Compound assignments like `this.x += 1` produce a read and a write.
/// - Destructuring assignments like `({ a } = this)` produce reads.
/// - Assignments to `this` properties like `this.y = 2` produce writes.
///
/// JavaScript example:
/// ```js
/// class Example {
///   method() {
///     this.x += 1;            // read: x, write: x
///     [this.y] = [10];        // write: y
///     ({ a: this.z } = obj);  // write: z
///     ({ m } = this);         // read: m
///   }
/// }
/// ```
fn handle_assignment_expression(
    node: &SyntaxNode<JsLanguage>,
    scoped_this_references: &[FunctionThisReferences],
    reads: &mut FxHashSet<ClassMemberReference>,
    writes: &mut FxHashSet<ClassMemberReference>,
) {
    if let Some(assignment) = JsAssignmentExpression::cast_ref(node)
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
            && let Some(name) = ThisPatternResolver::extract_this_member_reference(
                operand.as_js_static_member_assignment(),
                scoped_this_references,
                AccessKind::MeaningfulRead,
            )
        {
            reads.insert(name.clone());
        }

        if let Some(array) = left.as_js_array_assignment_pattern().cloned() {
            for class_member_reference in
                ThisPatternResolver::collect_array_assignment_names(&array, scoped_this_references)
            {
                writes.insert(class_member_reference.clone());
            }
        }

        if let Some(object) = left.as_js_object_assignment_pattern().cloned() {
            for class_member_reference in ThisPatternResolver::collect_object_assignment_names(
                &object,
                scoped_this_references,
            ) {
                writes.insert(class_member_reference.clone());
            }
        }

        if let Some(assignment) = left.as_any_js_assignment().cloned()
            && let Some(name) = ThisPatternResolver::extract_this_member_reference(
                assignment.as_js_static_member_assignment(),
                scoped_this_references,
                AccessKind::Write,
            )
        {
            writes.insert(name.clone());
        }
    }
}

/// Detects reads and writes from increment/decrement operations on `this` properties,
/// e.g. `this.count++` or `--aliasForThis.value`.
///
/// These operations always produce both a read and a write reference.
///
/// JavaScript example:
/// ```js
/// class Example {
///   method() {
///     this.count++;           // read: count, write: count
///     --aliasForThis.value;   // read: value, write: value (if alias is known)
///   }
/// }
/// ```
fn handle_pre_or_post_update_expression(
    js_update_expression: &AnyJsUpdateExpression,
    scoped_this_references: &[FunctionThisReferences],
    reads: &mut FxHashSet<ClassMemberReference>,
    writes: &mut FxHashSet<ClassMemberReference>,
) {
    let operand = match js_update_expression {
        AnyJsUpdateExpression::JsPreUpdateExpression(expr) => expr.operand().ok(),
        AnyJsUpdateExpression::JsPostUpdateExpression(expr) => expr.operand().ok(),
    };

    if let Some(operand) = operand
        && let Some(name) = ThisPatternResolver::extract_this_member_reference(
            operand.as_js_static_member_assignment(),
            scoped_this_references,
            AccessKind::Write,
        )
    {
        writes.insert(name.clone());
        reads.insert(ClassMemberReference {
            name: name.name,
            range: name.range,
            access_kind: get_read_access_kind(&AnyCandidateForUsedInExpressionNode::from(
                js_update_expression.clone(),
            )),
        });
    }
}

/// Collects read and write references to `this` members within a class constructor body,
/// including any nested functions that capture `this` via aliasing.
fn collect_references_from_constructor(constructor_body: &JsFunctionBody) -> ClassMemberReferences {
    let all_descendants_fn_bodies_and_this_scopes: Vec<_> =
        ThisScopeReferences::new(constructor_body).collect_function_this_references();
    let mut reads = FxHashSet::default();
    let mut writes = FxHashSet::default();

    for this_scope in all_descendants_fn_bodies_and_this_scopes.iter() {
        visit_references_in_body(
            this_scope.scope.syntax(),
            std::slice::from_ref(this_scope),
            &mut writes,
            &mut reads,
        );
    }

    ClassMemberReferences { reads, writes }
}

/// Collects class property names read from a `this` static member expression,
/// such as `this.prop` or `this.#privateProp`.
///
/// This function extracts the property name and its source range from the
/// provided `JsStaticMemberExpression`, and records it as a read reference.
/// No write references are collected.
fn collect_class_property_reads_from_static_member(
    static_member: &JsStaticMemberExpression,
) -> Option<ClassMemberReferences> {
    let mut reads = FxHashSet::default();
    let writes = FxHashSet::default();

    if let Ok(member) = static_member.member() {
        let name = member.to_trimmed_text();
        reads.insert(ClassMemberReference {
            name,
            range: static_member.syntax().text_trimmed_range(),
            access_kind: get_read_access_kind(&AnyCandidateForUsedInExpressionNode::from(
                static_member.clone(),
            )),
        });
    }

    Some(ClassMemberReferences { reads, writes })
}

/// Checks whether a name is within its correct scope
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

/// Determines the kind of read access for a given node.
fn get_read_access_kind(node: &AnyCandidateForUsedInExpressionNode) -> AccessKind {
    if is_used_in_expression_context(node) {
        AccessKind::MeaningfulRead
    } else {
        AccessKind::TrivialRead
    }
}

/// Checks if the given node is used in an expression context
/// (e.g., return, call arguments, conditionals, binary expressions).
/// Not limited to `this` references.
/// It can be used for any node; additional cases may require extending the context checks.
fn is_used_in_expression_context(node: &AnyCandidateForUsedInExpressionNode) -> bool {
    node.syntax().ancestors().skip(1).any(|ancestor| {
        matches!(
            ancestor.kind(),
            JsSyntaxKind::JS_RETURN_STATEMENT
                | JsSyntaxKind::JS_CALL_ARGUMENTS
                | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
                | JsSyntaxKind::JS_LOGICAL_EXPRESSION
                | JsSyntaxKind::JS_THROW_STATEMENT
                | JsSyntaxKind::JS_AWAIT_EXPRESSION
                | JsSyntaxKind::JS_YIELD_EXPRESSION
                | JsSyntaxKind::JS_UNARY_EXPRESSION
                | JsSyntaxKind::JS_TEMPLATE_EXPRESSION
                | JsSyntaxKind::JS_CALL_EXPRESSION
                | JsSyntaxKind::JS_NEW_EXPRESSION
                | JsSyntaxKind::JS_IF_STATEMENT
                | JsSyntaxKind::JS_SWITCH_STATEMENT
                | JsSyntaxKind::JS_FOR_STATEMENT
                | JsSyntaxKind::JS_FOR_IN_STATEMENT
                | JsSyntaxKind::JS_FOR_OF_STATEMENT
                | JsSyntaxKind::JS_BINARY_EXPRESSION
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, Parse, parse};
    use biome_js_syntax::{AnyJsRoot, JsFileSource, JsObjectBindingPattern};
    use biome_rowan::AstNode;

    struct TestCase<'a> {
        description: &'a str,
        code: &'a str,
        expected_reads: Vec<(&'a str, AccessKind)>, // (name, is_meaningful_read)
        expected_writes: Vec<(&'a str, AccessKind)>, // (name, is_meaningful_read)
    }

    fn assert_reads(
        reads: &FxHashSet<ClassMemberReference>,
        expected: &[(&str, AccessKind)],
        description: &str,
    ) {
        for (expected_name, expected_access_kind) in expected {
            let found = reads
                .iter()
                .find(|r| r.name.clone().text() == *expected_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Case '{}' failed: expected to find read '{}'",
                        description, expected_name
                    )
                });

            assert_eq!(
                found.access_kind, *expected_access_kind,
                "Case '{}' failed: read '{}' access_kind mismatch",
                description, expected_name
            );
        }
    }

    fn assert_writes(
        writes: &FxHashSet<ClassMemberReference>,
        expected: &[(&str, AccessKind)],
        description: &str,
    ) {
        for (expected_name, expected_access_kind) in expected {
            let found = writes
                .iter()
                .find(|r| r.name.clone().text() == *expected_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Case '{}' failed: expected to find write '{}'",
                        description, expected_name
                    )
                });

            assert_eq!(
                found.access_kind, *expected_access_kind,
                "Case '{}' failed: write '{}' access_kind mismatch",
                description, expected_name
            );
        }
    }

    fn parse_ts(code: &str) -> Parse<AnyJsRoot> {
        let source = parse(code, JsFileSource::ts(), JsParserOptions::default());

        if source.has_errors() {
            panic!("syntax error")
        }

        source
    }

    fn parse_first_object_binding(syntax: &JsSyntaxNode) -> SyntaxNode<JsLanguage> {
        // Find the first JsObjectBindingPattern in the syntax tree
        syntax
            .descendants()
            .find_map(JsObjectBindingPattern::cast)
            .expect("No object binding pattern found")
            .syntax()
            .clone()
    }

    #[test]
    fn test_object_binding_reads_variants() {
        let test_cases = [
            TestCase {
                description: "reads from this",
                code: r#"
            class Example {
                method() {
                    const { foo, bar } = this;
                }
            }
        "#,
                expected_reads: vec![
                    ("foo", AccessKind::TrivialRead),
                    ("bar", AccessKind::TrivialRead),
                ],
                expected_writes: vec![],
            },
            TestCase {
                description: "reads from aliasForThis",
                code: r#"
            class Example {
                method() {
                    const aliasForThis = this;
                    const { baz, qux } = aliasForThis;
                }
            }
        "#,
                expected_reads: vec![
                    ("baz", AccessKind::TrivialRead),
                    ("qux", AccessKind::TrivialRead),
                ],
                expected_writes: vec![],
            },
        ];

        for case in test_cases {
            let parse = parse_ts(case.code);
            let syntax = parse.syntax();
            let body = syntax
                .descendants()
                .find_map(JsFunctionBody::cast)
                .expect("No function body found");

            let function_this_references =
                ThisScopeReferences::new(&body).collect_function_this_references();
            let node = parse_first_object_binding(body.syntax());
            let mut reads = FxHashSet::default();

            handle_object_binding_pattern(&node, &function_this_references, &mut reads);

            assert_reads(&reads, &case.expected_reads, case.description);
        }
    }

    #[test]
    fn test_static_member_reads_variants() {
        let test_cases = [
            TestCase {
                description: "reads static members from this",
                code: r#"
            class Example {
                method() {
                    console.log(this.foo);
                    console.log(this.bar);
                }
            }
        "#,
                expected_reads: vec![
                    ("foo", AccessKind::MeaningfulRead),
                    ("bar", AccessKind::MeaningfulRead),
                ],
                expected_writes: vec![],
            },
            TestCase {
                description: "reads static members from aliasForThis",
                code: r#"
            class Example {
                method() {
                    const aliasForThis = this;
                    aliasForThis.baz;
                    aliasForThis.qux;
                }
            }
        "#,
                expected_reads: vec![
                    ("baz", AccessKind::TrivialRead),
                    ("qux", AccessKind::TrivialRead),
                ],
                expected_writes: vec![],
            },
        ];

        for case in test_cases {
            let parse = parse_ts(case.code);
            let syntax = parse.syntax();
            let body = syntax
                .descendants()
                .find_map(JsFunctionBody::cast)
                .expect("No function body found");

            let function_this_references =
                ThisScopeReferences::new(&body).collect_function_this_references();

            let mut reads = FxHashSet::default();

            for member_expr in syntax
                .descendants()
                .filter_map(JsStaticMemberExpression::cast)
            {
                handle_static_member_expression(
                    &member_expr.syntax().clone(),
                    &function_this_references,
                    &mut reads,
                );
            }

            assert_reads(&reads, &case.expected_reads, case.description);
        }
    }

    #[test]
    fn test_assignment_expression_reads_and_writes_variants() {
        let test_cases = [
            TestCase {
                description: "assignment reads and writes with this",
                code: r#"
            class Example {
                method() {
                    this.x += 1;
                    [this.y] = [10];
                    ({ a: this.z } = obj);
                }
            }
        "#,
                expected_reads: vec![("x", AccessKind::MeaningfulRead)], // x is read due to +=
                expected_writes: vec![
                    ("x", AccessKind::Write),
                    ("y", AccessKind::Write),
                    ("z", AccessKind::Write),
                ],
            },
            TestCase {
                description: "assignment reads and writes with aliasForThis",
                code: r#"
            class Example {
                method() {
                    const aliasForThis = this;
                    [aliasForThis.value] = [42];
                    aliasForThis.x += 1;
                    [aliasForThis.y] = [10];
                    ({ a: aliasForThis.z } = obj);
                }
            }
        "#,
                expected_reads: vec![("x", AccessKind::MeaningfulRead)],
                expected_writes: vec![
                    ("x", AccessKind::Write),
                    ("y", AccessKind::Write),
                    ("z", AccessKind::Write),
                ],
            },
        ];

        for case in test_cases {
            let parse = parse_ts(case.code);
            let syntax = parse.syntax();
            let body = syntax
                .descendants()
                .find_map(JsFunctionBody::cast)
                .expect("No function body found");

            let function_this_references =
                ThisScopeReferences::new(&body).collect_function_this_references();

            let mut reads = FxHashSet::default();
            let mut writes = FxHashSet::default();

            for assignment_expr in syntax
                .descendants()
                .filter_map(JsAssignmentExpression::cast)
            {
                handle_assignment_expression(
                    &assignment_expr.syntax().clone(),
                    &function_this_references,
                    &mut reads,
                    &mut writes,
                );
            }

            assert_reads(&reads, &case.expected_reads, case.description);
            assert_writes(&writes, &case.expected_writes, case.description);
        }
    }

    #[test]
    fn test_pre_or_post_update_expression_reads_and_writes_variants() {
        let test_cases = [
            TestCase {
                description: "pre/post update expressions on this properties",
                code: r#"
                class AnyJsUpdateExpression {
                    method() {
                        this.count++;
                        --this.total;

                        if (this.inIfCondition++ > 5) {
                        }

                        return this.inReturn++;
                    }
                }
            "#,
                expected_reads: vec![
                    ("count", AccessKind::TrivialRead),
                    ("total", AccessKind::TrivialRead),
                    ("inIfCondition", AccessKind::MeaningfulRead),
                    ("inReturn", AccessKind::MeaningfulRead),
                ],
                expected_writes: vec![
                    ("count", AccessKind::Write),
                    ("total", AccessKind::Write),
                    ("inIfCondition", AccessKind::Write),
                    ("inReturn", AccessKind::Write),
                ],
            },
            TestCase {
                description: "pre/post update expressions on aliasForThis properties",
                code: r#"
                class Example {
                    method() {
                        const aliasForThis = this;
                        const anotherAlias = this;
                        aliasForThis.count++;
                        --anotherAlias.total;

                        return anotherAlias.inReturnIncrement++;
                    }
                }
            "#,
                expected_reads: vec![
                    ("count", AccessKind::TrivialRead),
                    ("total", AccessKind::TrivialRead),
                    ("inReturnIncrement", AccessKind::MeaningfulRead),
                ],
                expected_writes: vec![
                    ("count", AccessKind::Write),
                    ("total", AccessKind::Write),
                    ("inReturnIncrement", AccessKind::Write),
                ],
            },
        ];

        for case in test_cases {
            let parse = parse_ts(case.code);
            let syntax = parse.syntax();
            let body = syntax
                .descendants()
                .find_map(JsFunctionBody::cast)
                .expect("No function body found");

            let function_this_references =
                ThisScopeReferences::new(&body).collect_function_this_references();

            let mut reads = FxHashSet::default();
            let mut writes = FxHashSet::default();

            for node in syntax.descendants() {
                if let Some(js_update_expression) = AnyJsUpdateExpression::cast_ref(&node) {
                    handle_pre_or_post_update_expression(
                        &js_update_expression,
                        &function_this_references,
                        &mut reads,
                        &mut writes,
                    );
                }
            }

            assert_reads(&reads, &case.expected_reads, case.description);
            assert_writes(&writes, &case.expected_writes, case.description);
        }
    }

    mod is_used_in_expression_context_tests {
        use super::*;
        use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;

        fn extract_all_nodes(code: &str) -> Vec<AnyCandidateForUsedInExpressionNode> {
            let parsed = parse_ts(code);
            let root = parsed.syntax();

            let mut nodes = vec![];

            for descendant in root.descendants() {
                // 1) Skip the identifier that is the class name (e.g. `Test` in `class Test {}`)
                if AnyJsIdentifierBinding::can_cast(descendant.kind())
                    && let Some(parent) = descendant.parent()
                    && JsClassDeclaration::can_cast(parent.kind())
                {
                    continue;
                }

                // Try to cast the node itself
                if let Some(node) = AnyCandidateForUsedInExpressionNode::cast_ref(&descendant) {
                    nodes.push(node);
                }

                // If this is an assignment, also include LHS
                if let Some(assign_expr) = JsAssignmentExpression::cast_ref(&descendant) {
                    if let Ok(lhs) = assign_expr.left()
                        && let Some(node) =
                            AnyCandidateForUsedInExpressionNode::cast_ref(lhs.syntax())
                    {
                        nodes.push(node.clone());
                    }

                    if let Ok(rhs) = assign_expr.right()
                        && let Some(node) =
                            AnyCandidateForUsedInExpressionNode::cast_ref(rhs.syntax())
                    {
                        nodes.push(node.clone());
                    }
                }
            }

            nodes
        }

        struct TestCase<'a> {
            description: &'a str,
            code: &'a str,
            expected: Vec<(&'a str, bool)>, // (member name, is_meaningful_read)
        }

        fn run_test_cases(cases: &[TestCase]) {
            for case in cases {
                let nodes = extract_all_nodes(case.code);
                assert!(
                    !nodes.is_empty(),
                    "No match found for test case: '{}'",
                    case.description
                );

                // Ensure the number of nodes matches expected
                assert_eq!(
                    nodes.len(),
                    case.expected.len(),
                    "Number of nodes does not match expected for test case: '{}'",
                    case.description
                );

                for (node, (expected_name, expected_access_kind)) in
                    nodes.iter().zip(&case.expected)
                {
                    let meaningful_node =
                        AnyCandidateForUsedInExpressionNode::cast_ref(node.syntax())
                            .expect("Failed to cast node to AnyMeaningfulReadNode");

                    // Compare node name
                    let node_name = meaningful_node.to_trimmed_text();
                    assert_eq!(
                        &node_name, expected_name,
                        "Node name mismatch for test case: '{}'",
                        case.description
                    );

                    // Compare is_meaningful_read
                    let actual_meaningful = is_used_in_expression_context(&meaningful_node);
                    assert_eq!(
                        actual_meaningful, *expected_access_kind,
                        "Meaningful read mismatch for node '{}' in test case: '{}'",
                        expected_name, case.description
                    );
                }
            }
        }

        #[test]
        fn test_is_used_in_expression_contexts() {
            let cases = [
                TestCase {
                    description: "return statement",
                    code: r#"class Test {method() { return this.x; }}"#,
                    expected: vec![("this.x", true)],
                },
                TestCase {
                    description: "call arguments",
                    code: r#"class Test { method() { foo(this.y); } }"#,
                    expected: vec![("this.y", true)],
                },
                TestCase {
                    description: "conditional expression",
                    code: r#"class Test { method() { const a = this.z ? 1 : 2; } }"#,
                    expected: vec![("a", false), ("this.z", true)],
                },
                TestCase {
                    description: "logical expression",
                    code: r#"class Test { method() { const a = this.a && this.b; } }"#,
                    expected: vec![("a", false), ("this.a", true), ("this.b", true)],
                },
                TestCase {
                    description: "throw statement",
                    code: r#"class Test { method() { throw this.err; } }"#,
                    expected: vec![("this.err", true)],
                },
                TestCase {
                    description: "await expression",
                    code: r#"class Test { async method() { await this.promise; } }"#,
                    expected: vec![("this.promise", true)],
                },
                TestCase {
                    description: "yield expression",
                    code: r#"class Test { *method() { yield this.gen; } }"#,
                    expected: vec![("this.gen", true)],
                },
                TestCase {
                    description: "unary expression",
                    code: r#"class Test { method() { -this.num; } }"#,
                    expected: vec![("this.num", true)],
                },
                TestCase {
                    description: "template expression",
                    code: r#"class Test { method() { `${this.str}`; } }"#,
                    expected: vec![("this.str", true)],
                },
                TestCase {
                    description: "call expression callee",
                    code: r#"class Test { method() { this.func(); } }"#,
                    expected: vec![("this.func", true)],
                },
                TestCase {
                    description: "new expression",
                    code: r#"class Test { method() { new this.ClassName(); } }"#,
                    expected: vec![("this.ClassName", true)],
                },
                TestCase {
                    description: "if statement",
                    code: r#"class Test { method() { if(this.cond) {} } }"#,
                    expected: vec![("this.cond", true)],
                },
                TestCase {
                    description: "switch statement",
                    code: r#"class Test { method() { switch(this.val) {} } }"#,
                    expected: vec![("this.val", true)],
                },
                TestCase {
                    description: "for statement",
                    code: r#"class Test { method() { for(this.i = 0; this.i < 10; this.i++) {} } }"#, // First this.i = 0 is a write, so not a match at all
                    expected: vec![("this.i", true), ("this.i++", true)],
                },
                TestCase {
                    description: "binary expression",
                    code: r#"class Test { method() { const sum = this.a + this.b; } }"#,
                    expected: vec![("sum", false), ("this.a", true), ("this.b", true)],
                },
                TestCase {
                    description: "binary expression nested parenthesis",
                    code: r#"class Test { method() { const sum = (((this.a + ((this.b * 2))))); } }"#,
                    expected: vec![("sum", false), ("this.a", true), ("this.b", true)],
                },
                TestCase {
                    description: "nested logical and conditional expressions",
                    code: r#"class Test { method() { const val = foo(this.a && (this.b ? this.c : 7)); } }"#,
                    expected: vec![
                        ("val", false),
                        ("this.a", true),
                        ("this.b", true),
                        ("this.c", true),
                    ],
                },
            ];

            run_test_cases(&cases);
        }
    }
}
