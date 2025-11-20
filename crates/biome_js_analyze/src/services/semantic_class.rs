use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
    VisitorFinishContext,
};
use biome_js_semantic::{SemanticEventExtractor, SemanticModel, SemanticModelBuilder};
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsClassMember, AnyJsComputedMember, AnyJsExpression,
    AnyJsObjectBindingPatternMember, AnyJsRoot, AnyTsType, JsArrayAssignmentPattern,
    JsArrowFunctionExpression, JsAssignmentExpression, JsClassDeclaration, JsClassMemberList,
    JsConstructorClassMember, JsFormalParameter, JsFunctionBody, JsGetterClassMember,
    JsIdentifierExpression, JsLanguage, JsMethodClassMember, JsObjectAssignmentPattern,
    JsObjectBindingPattern, JsPostUpdateExpression, JsPreUpdateExpression, JsPropertyClassMember,
    JsSetterClassMember, JsStaticMemberAssignment, JsStaticMemberExpression, JsSyntaxKind,
    JsSyntaxNode, JsThisExpression, JsVariableDeclarator, TextRange, TsIndexSignatureClassMember,
    TsPropertyParameter, TsReferenceType, TsStringLiteralType, TsTypeAliasDeclaration, TsUnionType,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxNode, SyntaxNodePtr, Text, WalkEvent,
    declare_node_union,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;
use std::option::Option;

#[derive(Debug, Clone)]
pub struct NamedClassMember {
    pub name: Text,
    pub range: TextRange,
}

#[derive(Clone)]
pub struct SemanticClassServices {
    semantic_class: SemanticClassModel,
    semantic: SemanticModel,
}

impl SemanticClassServices {
    pub fn semantic_class(&self) -> &SemanticClassModel {
        &self.semantic_class
    }

    pub fn semantic(&self) -> &SemanticModel {
        &self.semantic
    }
}

#[derive(Debug, Clone)]
pub struct SemanticClassModel {
    pub semantic: SemanticModel,
    named_members_cache: RefCell<FxHashMap<SyntaxNodePtr<JsLanguage>, Option<NamedClassMember>>>,
}

impl SemanticClassModel {
    pub fn new(semantic: SemanticModel) -> Self {
        Self {
            semantic,
            named_members_cache: RefCell::new(Default::default()),
        }
    }

    pub fn class_member_references(&self, members: &JsClassMemberList) -> ClassMemberReferences {
        class_member_references(&self.semantic, members)
    }

    pub fn extract_named_member(
        &self,
        any_class_member: &AnyNamedClassMember,
    ) -> Option<NamedClassMember> {
        let ptr = SyntaxNodePtr::new(any_class_member.syntax());
        if let Some(cached) = self.named_members_cache.borrow().get(&ptr) {
            return cached.clone();
        }

        let result = extract_named_member(any_class_member);
        self.named_members_cache
            .borrow_mut()
            .insert(ptr, result.clone());
        result
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
            semantic_class: service.clone(),
            semantic: service.semantic.clone(),
        })
    }
}

impl Phase for SemanticClassServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

pub struct SyntaxClassMemberReferencesVisitor {
    extractor: SemanticEventExtractor,
    builder: SemanticModelBuilder,
}

impl SyntaxClassMemberReferencesVisitor {
    pub(crate) fn new(root: AnyJsRoot) -> Self {
        Self {
            extractor: SemanticEventExtractor::default(),
            builder: SemanticModelBuilder::new(root),
        }
    }
}

impl Visitor for SyntaxClassMemberReferencesVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, _ctx: VisitorContext<JsLanguage>) {
        match event {
            WalkEvent::Enter(node) => {
                self.builder.push_node(node);
                self.extractor.enter(node);
            }
            WalkEvent::Leave(node) => {
                self.extractor.leave(node);
            }
        }

        while let Some(e) = self.extractor.pop() {
            self.builder.push_event(e);
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        let semantic = self.builder.build();

        ctx.services
            .insert_service(SemanticClassModel::new(semantic));
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

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || {
            SyntaxClassMemberReferencesVisitor::new(root.clone())
        });
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
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
    /// Represents any class member that has a name (public, private, or TypeScript-specific).
    pub AnyNamedClassMember =
      JsPropertyClassMember         // class Foo { bar = 1; }
      | JsMethodClassMember           // class Foo { baz() {} }
      | JsGetterClassMember           // class Foo { get qux() {} }
      | JsSetterClassMember           // class Foo { set quux(v) {} }
      | TsPropertyParameter           // constructor(public numbered: number) {}
      | TsIndexSignatureClassMember   // class Foo { [key: string]: number }
    // we also need to add accessor at some point claas Foo { accessor bar: string; }
}

declare_node_union! {
    pub AnyCandidateForUsedInExpressionNode = AnyJsExpression | AnyJsUpdateExpression | AnyJsObjectBindingPatternMember | JsStaticMemberExpression | AnyJsBindingPattern | JsStaticMemberAssignment | AnyJsComputedMember
}

declare_node_union! {
    pub AnyJsUpdateExpression = JsPreUpdateExpression | JsPostUpdateExpression
}

fn to_named(name: &impl AstNode) -> Option<NamedClassMember> {
    Some(NamedClassMember {
        name: name.to_trimmed_text(),
        range: name.range(),
    })
}

/// Extracts the name and range from a method, property, or constructor parameter.
/// Returns `None` for index signatures, since they don’t have a traditional name.
fn extract_named_member(any_class_member: &AnyNamedClassMember) -> Option<NamedClassMember> {
    match any_class_member {
        AnyNamedClassMember::JsMethodClassMember(member) => {
            let name_node = member.name().ok()?;
            to_named(&name_node)
        }

        AnyNamedClassMember::JsGetterClassMember(getter) => {
            let name_node = getter.name().ok()?;
            to_named(&name_node)
        }

        AnyNamedClassMember::JsSetterClassMember(setter) => {
            let name_node = setter.name().ok()?;
            to_named(&name_node)
        }

        AnyNamedClassMember::JsPropertyClassMember(member) => {
            let name_node = member.name().ok()?;
            to_named(&name_node)
        }

        AnyNamedClassMember::TsPropertyParameter(parameter) => {
            let name_node = parameter
                .formal_parameter()
                .ok()?
                .as_js_formal_parameter()?
                .binding()
                .ok()?;
            to_named(&name_node)
        }

        AnyNamedClassMember::TsIndexSignatureClassMember(_) => None,
    }
}

/// Collects all `this` property references used within the members of a JavaScript class.
///
/// This function traverses a `JsClassMemberList` and extracts property references from method bodies,
/// getters, setters, arrow functions assigned to properties, and constructors. It aggregates both
/// read and write references to `this` properties across all supported member types.
///
/// Returns a `ClassMemberReferences` struct containing the combined set of read and write references.
fn class_member_references(
    semantic: &SemanticModel,
    list: &JsClassMemberList,
) -> ClassMemberReferences {
    list.iter()
        .filter_map(|member| match member {
            AnyJsClassMember::JsMethodClassMember(method) => method
                .body()
                .ok()
                .and_then(|body| collect_references_from_body(semantic, method.syntax(), &body)),
            AnyJsClassMember::JsSetterClassMember(setter) => setter
                .body()
                .ok()
                .and_then(|body| collect_references_from_body(semantic, setter.syntax(), &body)),
            AnyJsClassMember::JsGetterClassMember(getter) => getter
                .body()
                .ok()
                .and_then(|body| collect_references_from_body(semantic, getter.syntax(), &body)),
            AnyJsClassMember::JsPropertyClassMember(property) => {
                property.value()?.expression().ok().and_then(|expr| {
                    if let Some(arrow) = JsArrowFunctionExpression::cast(expr.syntax().clone()) {
                        arrow.body().ok()?.as_js_function_body().and_then(|body| {
                            collect_references_from_body(semantic, arrow.syntax(), body)
                        })
                    } else {
                        expr.as_js_static_member_expression().map(|static_member| {
                            collect_class_property_reads_from_static_member(static_member)
                        })
                    }
                })
            }
            AnyJsClassMember::JsConstructorClassMember(constructor) => constructor
                .body()
                .ok()
                .map(|body| collect_references_from_constructor(semantic, &body)),
            _ => None,
        })
        .fold(
            ClassMemberReferences {
                reads: FxHashSet::default(),
                writes: FxHashSet::default(),
            },
            |mut acc, refs| {
                acc.reads.extend(refs.reads);
                acc.writes.extend(refs.writes);
                acc
            },
        )
}

/// Represents a function body and all `this` references (including aliases) valid within its lexical scope.
#[derive(Clone, Debug)]
struct FunctionThisAliases {
    scope: JsFunctionBody,
    this_aliases: FxHashSet<Text>,
}

/// A visitor that collects `this` references in nested function scopes,
/// while skipping class expressions and tracking inherited this references.
struct ThisScopeVisitor {
    skipped_ranges: Vec<TextRange>,
    inherited_this_aliases: FxHashSet<Text>,
    current_this_scopes: Vec<FunctionThisAliases>,
}
// Can not implement `Visitor` directly because it requires a new ctx that can not be created here
impl ThisScopeVisitor {
    fn visit(&mut self, event: &WalkEvent<SyntaxNode<JsLanguage>>) {
        match event {
            WalkEvent::Enter(node) => {
                // Skip nodes inside already-handled ranges (e.g., nested classes)
                if self
                    .skipped_ranges
                    .iter()
                    .any(|range| range.contains_range(node.text_range()))
                {
                    return;
                }

                match node.kind() {
                    // Skip nested classes entirely
                    JsSyntaxKind::JS_CLASS_EXPRESSION | JsSyntaxKind::JS_CLASS_DECLARATION => {
                        self.skipped_ranges.push(node.text_range());
                    }

                    // Regular function body (non-constructor)
                    JsSyntaxKind::JS_FUNCTION_BODY => {
                        if let Some(body) = JsFunctionBody::cast_ref(node) {
                            let is_constructor = node
                                .parent()
                                .and_then(JsConstructorClassMember::cast)
                                .is_some();

                            if !is_constructor {
                                let current_scope =
                                    ThisScopeReferences::new(&body).local_this_aliases;
                                let mut scoped_this_references = FxHashSet::default();
                                scoped_this_references
                                    .extend(self.inherited_this_aliases.iter().cloned());
                                scoped_this_references.extend(current_scope);

                                self.current_this_scopes.push(FunctionThisAliases {
                                    scope: body.clone(),
                                    this_aliases: scoped_this_references,
                                });
                            }
                        }
                    }

                    // Arrow functions
                    JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                        if let Some(func_expr) = JsArrowFunctionExpression::cast_ref(node)
                            && let Some(body) = func_expr
                                .body()
                                .ok()
                                .and_then(|b| b.as_js_function_body().cloned())
                        {
                            let current_scope_aliases =
                                ThisScopeReferences::new(&body).local_this_aliases;
                            let mut scoped_this_references = FxHashSet::default();
                            scoped_this_references
                                .extend(self.inherited_this_aliases.iter().cloned());
                            scoped_this_references.extend(current_scope_aliases.clone());

                            self.current_this_scopes.push(FunctionThisAliases {
                                scope: body.clone(),
                                this_aliases: scoped_this_references,
                            });
                        }
                    }

                    // Everything else: do nothing
                    _ => {}
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
    local_this_aliases: FxHashSet<Text>,
}

impl ThisScopeReferences {
    fn new(body: &JsFunctionBody) -> Self {
        Self {
            body: body.clone(),
            local_this_aliases: Self::collect_local_this_aliases(body),
        }
    }

    /// Collects all `this` scope references in the function body and nested
    /// functions using `ThisScopeVisitor`, combining local and inherited ones
    /// into a list of `FunctionThisAliases`.
    fn collect_function_this_aliases(&self) -> Vec<FunctionThisAliases> {
        let mut visitor = ThisScopeVisitor {
            skipped_ranges: vec![],
            current_this_scopes: vec![],
            inherited_this_aliases: self.local_this_aliases.clone(),
        };

        let iter = self.body.syntax().preorder();
        for event in iter {
            visitor.visit(&event);
        }

        visitor.current_this_scopes
    }

    /// Collects local this aliases of `this` in a function body.
    fn collect_local_this_aliases(body: &JsFunctionBody) -> FxHashSet<Text> {
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

                // Only direct `this` assignments (not this.prop)
                if JsThisExpression::can_cast(unwrapped.syntax().kind()) {
                    Some(id.syntax().text_trimmed().into_text())
                } else {
                    None
                }
            })
            .collect()
    }
}

/// Checks if a given expression is a reference to `this` or any of its aliases.
fn is_this_reference(
    js_expression: &AnyJsExpression,
    scoped_this_references: &[FunctionThisAliases],
) -> bool {
    // Direct `this` expression
    if let Some(this_expr) = js_expression.as_js_this_expression() {
        let syntax = this_expr.syntax();
        return scoped_this_references.iter().any(|func_scope| {
            is_within_scope_without_shadowing(syntax, func_scope.scope.syntax())
        });
    }

    // Identifier alias
    if let Some(js_identifier_expression) = js_expression.as_js_identifier_expression()
        && let Ok(name) = js_identifier_expression.name()
        && let Ok(value_token) = name.value_token()
    {
        let name_syntax = name.syntax();

        scoped_this_references.iter().any(
            |FunctionThisAliases {
                 scope,
                 this_aliases,
             }| {
                if !this_aliases.contains(value_token.token_text_trimmed().text()) {
                    return false; // not an alias → skip expensive scope check
                }

                is_within_scope_without_shadowing(name_syntax, scope.syntax())
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
        scoped_this_references: &[FunctionThisAliases],
    ) -> Vec<ClassMemberReference> {
        array_assignment_pattern
            .elements()
            .iter()
            .filter_map(|element| {
                let element = element.as_ref().ok()?;

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
        scoped_this_references: &[FunctionThisAliases],
    ) -> Vec<ClassMemberReference> {
        assignment
            .properties()
            .elements()
            .filter_map(|prop| {
                if let Some(rest_params) = prop
                    .node
                    .as_ref()
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
                    .as_ref()
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
        scoped_this_references: &[FunctionThisAliases],
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
    semantic: &SemanticModel,
    member: &JsSyntaxNode,
    body: &JsFunctionBody,
) -> Option<ClassMemberReferences> {
    let scoped_this_references = ThisScopeReferences::new(body).collect_function_this_aliases();
    let mut reads = FxHashSet::default();
    let mut writes = FxHashSet::default();

    visit_references_in_body(
        semantic,
        member,
        &scoped_this_references,
        &mut writes,
        &mut reads,
    );

    Some(ClassMemberReferences { reads, writes })
}

/// Traverses a JavaScript method or initializer body to collect references
/// to `this`-based class members, calling the provided callbacks for reads and writes.
///
/// It detects:
/// - Reads via `this.prop`, `this.#prop`, and compound assignments (e.g., `this.prop += 1`)
/// - Writes via assignments and destructuring patterns involving `this` or its aliases
fn visit_references_in_body(
    semantic: &SemanticModel,
    method_body_element: &JsSyntaxNode,
    scoped_this_references: &[FunctionThisAliases],
    writes: &mut FxHashSet<ClassMemberReference>,
    reads: &mut FxHashSet<ClassMemberReference>,
) {
    let iter = method_body_element.preorder();

    for event in iter {
        match event {
            WalkEvent::Enter(node) => {
                match node.kind() {
                    JsSyntaxKind::JS_OBJECT_BINDING_PATTERN => {
                        handle_object_binding_pattern(&node, scoped_this_references, reads);
                    }
                    JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
                    | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                        handle_dynamic_member_expression(
                            &node,
                            scoped_this_references,
                            semantic,
                            reads,
                        );
                    }
                    JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                        handle_assignment_expression(&node, scoped_this_references, reads, writes);
                    }
                    JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                        handle_static_member_expression(&node, scoped_this_references, reads);
                    }
                    JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                    | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => {
                        // Handle both ++a and a++ in the same handler
                        if let Some(update_expr) = AnyJsUpdateExpression::cast_ref(&node) {
                            handle_pre_or_post_update_expression(
                                &update_expr,
                                scoped_this_references,
                                reads,
                                writes,
                            );
                        }
                    }
                    _ => {}
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
    scoped_this_references: &[FunctionThisAliases],
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
                let name = declarator.to_trimmed_text(); // allocate only the text
                let range = declarator.syntax().text_trimmed_range();
                reads.insert(ClassMemberReference {
                    name,
                    range,
                    access_kind: get_read_access_kind(&AnyCandidateForUsedInExpressionNode::from(
                        declarator.clone(),
                    )),
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
    scoped_this_references: &[FunctionThisAliases],
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

/// we assume that any usage in an expression context is meaningful read, and writes are much less likely
/// so skip the dynamic writes
fn handle_dynamic_member_expression(
    node: &SyntaxNode<JsLanguage>,
    scoped_this_references: &[FunctionThisAliases],
    semantic: &SemanticModel,
    reads: &mut FxHashSet<ClassMemberReference>,
) {
    if let Some(dynamic_member) = AnyJsComputedMember::cast(node.clone())
        && let Ok(object) = dynamic_member.object()
        && is_this_reference(&object, scoped_this_references)
        && let Ok(member_expr) = dynamic_member.member()
        && let Some(id_expr) = JsIdentifierExpression::cast_ref(member_expr.syntax())
        && let Some(ty) = resolve_formal_param_type(semantic, &id_expr)
        && let Some(ts_union_type) = TsUnionType::cast(ty.syntax().clone())
            .or_else(|| resolve_reference_to_union(semantic, &ty))
    {
        let items: Vec<_> = extract_literal_types(&ts_union_type);

        for item in items.iter() {
            reads.insert(ClassMemberReference {
                // we keep the range of the dynamic accessed member
                range: member_expr.range(),
                // swap the name for the actual resolved type
                name: item.clone(),

                access_kind: get_read_access_kind(&AnyCandidateForUsedInExpressionNode::from(
                    dynamic_member.clone(),
                )),
            });
        }
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
    scoped_this_references: &[FunctionThisAliases],
    reads: &mut FxHashSet<ClassMemberReference>,
    writes: &mut FxHashSet<ClassMemberReference>,
) {
    if let Some(assignment) = JsAssignmentExpression::cast_ref(node)
        && let Ok(left) = assignment.left()
    {
        // Compound assignment -> meaningful read
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
            reads.insert(name);
        }

        // Array assignment pattern
        if let Some(array) = left.as_js_array_assignment_pattern() {
            for class_member_reference in
                ThisPatternResolver::collect_array_assignment_names(array, scoped_this_references)
            {
                writes.insert(class_member_reference);
            }
        }

        // Object assignment pattern
        if let Some(object) = left.as_js_object_assignment_pattern() {
            for class_member_reference in
                ThisPatternResolver::collect_object_assignment_names(object, scoped_this_references)
            {
                match class_member_reference.access_kind {
                    AccessKind::Write => writes.insert(class_member_reference),
                    _ => reads.insert(class_member_reference),
                };
            }
        }

        // Plain assignment
        if let Some(assignment) = left.as_any_js_assignment()
            && let Some(name) = ThisPatternResolver::extract_this_member_reference(
                assignment.as_js_static_member_assignment(),
                scoped_this_references,
                AccessKind::Write,
            )
        {
            writes.insert(name.clone());

            // If it is used in expression context, a write can be still a meaningful read e.g.
            // class Used { #val; getVal() { return this.#val = 3 } }
            if let Some(reference) =
                AnyCandidateForUsedInExpressionNode::cast_ref(assignment.syntax())
                && is_used_in_expression_context(&reference)
            {
                reads.insert({
                    ClassMemberReference {
                        name: name.name,
                        range: name.range,
                        access_kind: AccessKind::MeaningfulRead,
                    }
                });
            }
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
    scoped_this_references: &[FunctionThisAliases],
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
fn collect_references_from_constructor(
    semantic: &SemanticModel,
    constructor_body: &JsFunctionBody,
) -> ClassMemberReferences {
    let all_descendants_fn_bodies_and_this_scopes: Vec<_> =
        ThisScopeReferences::new(constructor_body).collect_function_this_aliases();
    let mut reads = FxHashSet::default();
    let mut writes = FxHashSet::default();

    for this_scope in all_descendants_fn_bodies_and_this_scopes.iter() {
        visit_references_in_body(
            semantic,
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
) -> ClassMemberReferences {
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

    ClassMemberReferences { reads, writes }
}

/// Checks whether a name is within its correct scope
fn is_within_scope_without_shadowing(
    name_syntax: &SyntaxNode<JsLanguage>,
    scope: &SyntaxNode<JsLanguage>,
) -> bool {
    let scope_key = scope.key();
    for ancestor in name_syntax.ancestors() {
        if ancestor.key() == scope_key {
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
    let node_syntax = node.syntax();
    node_syntax.ancestors().any(|ancestor| {
        is_class_initializer_rhs(&ancestor)
            || is_assignment_expression_context(node, &ancestor)
            || is_general_expression_context(&ancestor)
    })
}

/// Returns `true` if the given `node` appears on the **right-hand side of a class property initializer**.
///
/// Example:
/// ```js
/// class Foo {
///     #x = 42;
///     y = this.#x; // RHS (`this.#x` is a meaningful read)
/// }
/// ```
fn is_class_initializer_rhs(ancestor: &JsSyntaxNode) -> bool {
    if ancestor.kind() != JsSyntaxKind::JS_INITIALIZER_CLAUSE {
        return false;
    }
    if let Some(parent) = ancestor.parent() {
        parent.kind() == JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER
    } else {
        false
    }
}

/// Checks if the given `node` occurs in an assignment expression context
/// where its value is meaningfully used.
///
/// - **RHS of an assignment** counts as a read (meaningful use).
/// - **LHS inside an object destructuring pattern** also counts as a read.
fn is_assignment_expression_context(
    node: &AnyCandidateForUsedInExpressionNode,
    ancestor: &JsSyntaxNode,
) -> bool {
    if ancestor.kind() != JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION {
        return false;
    }
    let node_range = node.syntax().text_trimmed_range();
    if let Some(assignment) = JsAssignmentExpression::cast_ref(ancestor) {
        if let Ok(rhs) = assignment.right()
            && rhs.syntax().text_trimmed_range().contains_range(node_range)
        {
            return true;
        }

        if let Ok(lhs) = assignment.left()
            && lhs.syntax().kind() == JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN
            && lhs.syntax().text_trimmed_range().contains_range(node_range)
        {
            return true;
        }
    }
    false
}

/// Checks if the given `ancestor` node represents a context
/// where a value is used (read) in an expression, such as a return statement,
/// call argument, conditional, logical expression, etc.
fn is_general_expression_context(ancestor: &JsSyntaxNode) -> bool {
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
}

/// Extracts the immediate string literal types only from a union like `A | B | C`.
/// Filters out any non string literal type.
/// Does not recurse into nested unions.
fn extract_literal_types(union: &TsUnionType) -> Vec<Text> {
    extract_shallow_union_members(union)
        .iter()
        .filter_map(|item| {
            if let Some(literal_type) = TsStringLiteralType::cast(item.syntax().clone()) {
                return Some(Text::new_owned(Box::from(
                    literal_type
                        .to_trimmed_text()
                        .trim_matches(&['"', '\''][..]),
                )));
            }

            None
        })
        .collect()
}

/// Extracts the immediate types from a union like `A | B | C`.
/// Does not recurse into nested unions.
fn extract_shallow_union_members(union: &TsUnionType) -> Vec<AnyTsType> {
    union.types().into_iter().flatten().collect()
}

/// Attempts to resolve the type annotation of a formal parameter for the given identifier expression.
/// - Looks up the binding for the identifier expression in the semantic model.
/// - Checks if the binding corresponds to a `JsFormalParameter`.
/// - If so, extracts and returns its type annotation.
fn resolve_formal_param_type(
    model: &SemanticModel,
    id_expr: &JsIdentifierExpression,
) -> Option<AnyTsType> {
    let ref_ident = id_expr.name().ok()?;
    let binding = model.binding(&ref_ident)?;
    let parent_node = binding.syntax().parent()?;

    // Only proceed if parent is a formal parameter
    let js_param = JsFormalParameter::cast_ref(&parent_node)?;
    let type_annotation = js_param.type_annotation()?;
    type_annotation.ty().ok()
}

/// Resolves a type reference to its aliased union type, if the reference points to a union.
fn resolve_reference_to_union(model: &SemanticModel, ty: &AnyTsType) -> Option<TsUnionType> {
    let ts_reference_type = TsReferenceType::cast_ref(ty.syntax())?;
    let ref_name = ts_reference_type.name().ok()?;
    let ref_ident = ref_name.as_js_reference_identifier()?;

    let binding = model.binding(ref_ident)?;
    let parent_node = binding.syntax().parent()?;

    let type_alias = TsTypeAliasDeclaration::cast_ref(&parent_node)?;
    let ty = type_alias.ty().ok()?;

    TsUnionType::cast_ref(ty.syntax())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::semantic_class::FxHashSet;
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
        for (expected_name, _) in expected {
            reads
                .iter()
                .find(|r| r.name.clone().text() == *expected_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Case '{}' failed: expected to find read '{}', but none was found in {:#?}",
                        description, expected_name, reads
                    )
                });
        }
    }

    fn assert_writes(
        writes: &FxHashSet<ClassMemberReference>,
        expected: &[(&str, AccessKind)],
        description: &str,
    ) {
        for (expected_name, _) in expected {
            writes
                .iter()
                .find(|r| r.name.clone().text() == *expected_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Case '{}' failed: expected to find write '{}' in {:#?}",
                        description, expected_name, writes
                    )
                });
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
                ThisScopeReferences::new(&body).collect_function_this_aliases();
            let node = parse_first_object_binding(body.syntax());
            let mut reads = FxHashSet::default();

            handle_object_binding_pattern(&node, &function_this_references, &mut reads);

            assert_reads(&reads, case.expected_reads.as_slice(), case.description);
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
                ThisScopeReferences::new(&body).collect_function_this_aliases();

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
                expected_writes: vec![("x", AccessKind::Write), ("y", AccessKind::Write)],
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
                expected_writes: vec![("x", AccessKind::Write), ("y", AccessKind::Write)],
            },
            TestCase {
                description: "assignment reads and writes with return expression",
                code: r#"class Used { #val = 1; getVal() { return this.#val = this.#val } }"#,
                expected_reads: vec![("#val", AccessKind::MeaningfulRead)],
                expected_writes: vec![("#val", AccessKind::Write)],
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
                ThisScopeReferences::new(&body).collect_function_this_aliases();

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
                ThisScopeReferences::new(&body).collect_function_this_aliases();

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

        struct TestCase<'a> {
            description: &'a str,
            code: &'a str,
            expected: Vec<(&'a str, bool)>, // (identifier text, is_meaningful_read)
        }

        fn parse_this_member_nodes_from_code(
            code: &str,
        ) -> Vec<AnyCandidateForUsedInExpressionNode> {
            let parsed = parse_ts(code);
            let root = parsed.syntax();
            let mut nodes = vec![];

            for descendant in root.descendants() {
                // Static member: this.x or this.#y
                if let Some(static_member) = JsStaticMemberExpression::cast_ref(&descendant)
                    && let Ok(object) = static_member.object()
                    && object.as_js_this_expression().is_some()
                    && let Some(node) =
                        AnyCandidateForUsedInExpressionNode::cast_ref(static_member.syntax())
                {
                    nodes.push(node.clone());
                }
            }

            nodes
        }

        fn run_test_cases(cases: &[TestCase]) {
            for case in cases {
                let nodes = parse_this_member_nodes_from_code(case.code);
                assert!(
                    !nodes.is_empty(),
                    "No nodes found for test case: {}",
                    case.description
                );
                assert_eq!(
                    nodes.len(),
                    case.expected.len(),
                    "Number of nodes does not match expected for '{}'",
                    case.description
                );

                for (node, (expected_name, expected_flag)) in nodes.iter().zip(&case.expected) {
                    let name = node.to_trimmed_text();
                    assert_eq!(
                        &name, expected_name,
                        "Node name mismatch for '{}'",
                        case.description
                    );

                    let actual_flag = is_used_in_expression_context(node);
                    assert_eq!(
                        actual_flag, *expected_flag,
                        "Meaningful read mismatch for '{}' in '{}'",
                        expected_name, case.description
                    );
                }
            }
        }

        #[test]
        fn test_major_expression_contexts() {
            let cases = [
                TestCase {
                    description: "return statement",
                    code: r#"class Test { method() { return this.x; } }"#,
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
                    expected: vec![("this.z", true)],
                },
                TestCase {
                    description: "logical expression",
                    code: r#"class Test { method() { const a = this.a && this.b; } }"#,
                    expected: vec![("this.a", true), ("this.b", true)],
                },
                TestCase {
                    description: "unary expression",
                    code: r#"class Test { method() { -this.num; } }"#,
                    expected: vec![("this.num", true)],
                },
                TestCase {
                    description: "template literal",
                    code: r#"class Test { method() { `${this.str}`; } }"#,
                    expected: vec![("this.str", true)],
                },
                TestCase {
                    description: "binary expression",
                    code: r#"class Test { method() { const sum = this.a + this.b; } }"#,
                    expected: vec![("this.a", true), ("this.b", true)],
                },
                TestCase {
                    description: "assignment RHS",
                    code: r#"class Test { method() { this.x = 5 + this.x; } }"#,
                    expected: vec![("this.x", true)],
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
                    code: r#"class Test { method() { for(this.i = 0; this.i < 10; this.i++) {} } }"#,
                    expected: vec![("this.i", true)],
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
            ];

            run_test_cases(&cases);
        }
    }

    mod extract_named_member_tests {
        use crate::services::semantic_class::AnyNamedClassMember;
        use crate::services::semantic_class::extract_named_member;
        use crate::services::semantic_class::tests::parse_ts;
        use biome_js_syntax::JsClassDeclaration;
        use biome_rowan::{AstNode, AstNodeList};

        fn extract_first_member(src: &str) -> AnyNamedClassMember {
            let parse = parse_ts(src);
            let root = parse.syntax();
            let class = root
                .descendants()
                .find_map(JsClassDeclaration::cast)
                .unwrap();
            let members: Vec<_> = class.members().iter().collect();
            let first = members.first().unwrap();

            AnyNamedClassMember::cast((*first).clone().into()).unwrap()
        }

        #[test]
        fn extracts_method_name() {
            let member = extract_first_member("class A { foo() {} }");
            let named = extract_named_member(&member).unwrap();
            assert_eq!(named.name, "foo");
        }

        #[test]
        fn extracts_property_name() {
            let member = extract_first_member("class A { bar = 1 }");
            let named = extract_named_member(&member).unwrap();
            assert_eq!(named.name, "bar");
        }

        #[test]
        fn extracts_getter_name() {
            let member = extract_first_member("class A { get baz() { return 1 } }");
            let named = extract_named_member(&member).unwrap();
            assert_eq!(named.name, "baz");
        }

        #[test]
        fn extracts_setter_name() {
            let member = extract_first_member("class A { set qux(v) {} }");
            let named = extract_named_member(&member).unwrap();
            assert_eq!(named.name, "qux");
        }

        #[test]
        fn returns_none_for_index_signature() {
            let member = extract_first_member("class A { [key: string]: number }");
            assert!(extract_named_member(&member).is_none());
        }
    }
}
