use biome_js_syntax::{
    AnyJsClassMember, AnyJsExpression, JsArrayAssignmentPattern, JsArrowFunctionExpression,
    JsAssignmentExpression, JsClassMemberList, JsConstructorClassMember, JsFunctionBody,
    JsLanguage, JsObjectAssignmentPattern, JsObjectBindingPattern, JsPostUpdateExpression,
    JsPreUpdateExpression, JsPropertyClassMember, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator, TextRange,
    TsPropertyParameter,
};

use biome_analyze::QueryMatch;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxNode, Text, WalkEvent, declare_node_union,
};
use std::collections::HashSet;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ClassMemberReference {
    pub name: Text,
    pub range: TextRange,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClassMemberReferences {
    pub reads: HashSet<ClassMemberReference>,
    pub writes: HashSet<ClassMemberReference>,
}

declare_node_union! {
    pub AnyPropertyMember = JsPropertyClassMember | TsPropertyParameter
}

/// Compute all `this`-based property reads and writes referenced by members of a JS/TS class.
///
/// Traverses the provided `JsClassMemberList` and analyzes supported member kinds (methods,
/// getters, setters, property initializers that are arrow functions or static member expressions,
/// and constructors). Returns a `ClassMemberReferences` containing two sets:
/// - `reads`: properties read via `this` (or aliases of `this`) within any member body.
/// - `writes`: properties written to via `this` (or aliases of `this`) within any member body.
///
/// Only documents the effective member-level references; it does not expose internal scope-tracking
/// details or alias resolution rules in the summary.
///
/// # Examples
///
/// ```no_run
/// // Given a parsed `JsClassMemberList` named `members`:
/// let refs = class_member_references(&members);
/// // `refs.reads` and `refs.writes` contain discovered `this` property references.
/// ```
pub fn class_member_references(list: &JsClassMemberList) -> ClassMemberReferences {
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

    let mut combined_reads = HashSet::new();
    let mut combined_writes = HashSet::new();

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
    this_references: HashSet<ClassMemberReference>,
}

/// A visitor that collects `this` references in nested function scopes,
/// while skipping class expressions and tracking inherited this references.
struct ThisScopeVisitor<'a> {
    skipped_ranges: Vec<TextRange>,
    inherited_this_references: &'a [ClassMemberReference],
    current_this_scopes: Vec<FunctionThisReferences>,
}
// can not implement `Visitor` directly because it requires a new ctx that can not be created here
impl ThisScopeVisitor<'_> {
    /// Handle a traversal event to track function scopes and `this` aliases within the visited tree.
    ///
    /// On `WalkEvent::Enter`, this method:
    /// - Skips traversing into class declarations and class expressions by recording their text ranges in `skipped_ranges`.
    /// - When encountering a `JsFunctionBody` (except when that body belongs to a constructor), computes the local `this` aliases for that body, merges them with any inherited aliases, and pushes a `FunctionThisReferences` for the body onto `current_this_scopes`.
    /// - When encountering an arrow function with a function body, performs the same alias collection and pushes a corresponding `FunctionThisReferences`.
    ///
    /// On `WalkEvent::Leave`, this method:
    /// - Pops the last skipped range if the current node matches it, resuming traversal after leaving a previously skipped class.
    ///
    /// Side effects:
    /// - May mutate `self.skipped_ranges` and `self.current_this_scopes`.
    /// - Reads `self.inherited_this_references` to propagate aliases into nested scopes.
    ///
    /// Parameters:
    /// - `event`: the walk event (entering or leaving a syntax node) to handle.
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
                        let mut scoped_this_references = HashSet::new();
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
                    let mut scoped_this_references = HashSet::new();
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
    /// Create a `ThisScopeReferences` for the given function body.
    ///
    /// This collects the local aliases of `this` (e.g., `const self = this`) that are
    /// visible within `body` and stores the cloned function body for later scope analysis.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Given a parsed `JsFunctionBody` named `body`:
    /// // let body: JsFunctionBody = ...;
    /// let refs = ThisScopeReferences::new(&body);
    /// ```
    pub fn new(body: &JsFunctionBody) -> Self {
        Self {
            body: body.clone(),
            local_this_references: Self::collect_local_this_references(body),
        }
    }

    /// Returns a list of `FunctionThisReferences` for `self.body`.
    ///
    /// Traverses the function body and its nested function scopes to collect all `this`
    /// references (including aliases inherited from the local scope) using
    /// `ThisScopeVisitor`. The returned vector contains one `FunctionThisReferences`
    /// entry per discovered function scope, combining local and inherited `this`
    /// aliases applicable within that scope.
    pub fn collect_function_this_references(&self) -> Vec<FunctionThisReferences> {
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

    /// Collects local aliases of `this` initialized in a function body.
    ///
    /// Scans top-level variable declarations in `body` and returns a list of
    /// ClassMemberReference entries for declarators whose initializer is a direct
    /// `this` expression (e.g. `const self = this;` or `const x = this;`).
    ///
    /// Only detects simple direct initializers that unwrap to a `this` token; more
    /// complex expressions or nested/deeper patterns are ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a parsed `body` representing:
    /// //   function foo() { const self = this; }
    /// // calling `collect_local_this_references(&body)` yields a vector
    /// // containing a ClassMemberReference for the `self` identifier.
    /// let refs = collect_local_this_references(&body);
    /// assert!(refs.len() >= 0);
    /// ```
    fn collect_local_this_references(body: &JsFunctionBody) -> Vec<ClassMemberReference> {
        body.statements()
            .iter()
            .filter_map(|node| node.as_js_variable_statement().cloned())
            .filter_map(|stmt| stmt.declaration().ok().map(|decl| decl.declarators()))
            .flat_map(|declarators| {
                // .into() not working here, JsVariableDeclaratorList is not implmenting it correctly
                declarators.into_iter().filter_map(|declaration| {
                    declaration.ok().map(|declarator| declarator.as_fields())
                })
            })
            .filter_map(|fields| {
                let id = fields.id.ok()?;
                let expr = fields.initializer?.expression().ok()?;
                let unwrapped = unwrap_expression(&expr);

                (unwrapped.syntax().first_token()?.text_trimmed() == "this").then(|| {
                    ClassMemberReference {
                        name: id.to_trimmed_text().clone(),
                        range: id.syntax().text_trimmed_range(),
                    }
                })
            })
            .collect()
    }
}

/// Returns true if the given expression denotes `this` or a known alias of `this` that is valid
/// within one of the provided function scopes.
///
/// The function recognizes:
/// - explicit `this` expressions, and
/// - identifier expressions that match a recorded `this` alias for a scope, provided the
///   identifier is not shadowed in that scope.
///
/// `scoped_this_references` is a list of FunctionThisReferences (each pairing a function scope with
/// the aliases that refer to `this` in or above that scope) used to validate both alias identity
/// and scope containment.
///
/// # Examples
///
/// ```
/// // Pseudocode-style example (types and helpers are defined in the crate).
/// // let expr = parse_expression("self"); // identifier expression
/// // let scopes = compute_scoped_this_references(...);
/// // assert!(is_this_reference(&expr, &scopes));
/// ```
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
    /// Collects `this`-based member references from an array assignment pattern.
    ///
    /// Scans the elements of `array_assignment_pattern` and returns a vec of
    /// `ClassMemberReference` for any entries that are assignments to `this` (including
    /// rest elements like `...[this.x]`). Only static member assignments (e.g. `this.x` / `this.#x`)
    /// that resolve to `this` or a known alias in `scoped_this_references` are returned.
    ///
    /// # Examples
    ///
    /// ```
    /// // Pseudo-usage: parse a snippet with an array assignment pattern like `[this.x, ...this.y]`,
    /// // obtain a `JsArrayAssignmentPattern` node and `scoped_this_references`, then:
    /// // let refs = collect_array_assignment_names(&array_pattern, &scoped_this_references);
    /// // assert!(refs.iter().any(|r| r.name == Text::from("x")));
    /// ```
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
                            )
                        })
                } else {
                    None
                }
            })
            .collect()
    }

    /// Collects `this`-based member references from an object assignment pattern (e.g. `{ a: this.x, ...this.#y }`).
    ///
    /// Scans the properties and rest element of the provided `JsObjectAssignmentPattern` and returns a
    /// vector of `ClassMemberReference` for each property that is a static member assignment rooted at
    /// `this` or an alias of `this` (as determined by `scoped_this_references`).
    ///
    /// - `assignment`: the object assignment pattern to inspect (destructuring assignment).
    /// - `scoped_this_references`: list of function-scoped `this` aliases used to resolve whether a static
    ///   member assignment is actually referencing `this` (or an alias) within the current scope.
    ///
    /// Returns a `Vec<ClassMemberReference>` containing a reference for each matched property/rest target.
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
                    );
                }
                None
            })
            .collect()
    }

    /// Extracts a class-member reference from a static member assignment when the assignment's object
    /// is `this` or an alias of `this`.
    ///
    /// Returns a `ClassMemberReference` containing the member's trimmed text and its text range when:
    /// - the assignment's object is `this` (e.g. `this.prop = ...`),
    /// - the assignment's object is a private name on `this` (e.g. `this.#private = ...`), or
    /// - the assignment's object is a known alias for `this` (e.g. `self.prop = ...` when `self` was aliased to `this`).
    ///
    /// The `scoped_this_references` parameter is the list of `FunctionThisReferences` for the current
    /// and enclosing function scopes used to resolve aliases that refer to `this`.
    ///
    /// Returns `None` if the operand is absent, the object is not `this`/an alias, or the member is not a
    /// plain property name or private name.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given an assignment representing `this.foo = bar`, this function returns a
    /// // ClassMemberReference with name `"foo"` and the source text range of that name.
    /// ```
    fn extract_this_member_reference(
        operand: Option<&JsStaticMemberAssignment>,
        scoped_this_references: &[FunctionThisReferences],
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
                        })
                        .or_else(|| {
                            member
                                .as_js_private_name()
                                .map(|private_name| ClassMemberReference {
                                    name: private_name.to_trimmed_text(),
                                    range: private_name.syntax().text_trimmed_range(),
                                })
                        })
                })
            } else {
                None
            }
        })
    }
}

/// Collects `this`-based member references (reads and writes) from a class member's function body.
///
/// Given a class member node (method, getter/setter, or property initializer) and its function body,
/// analyzes the body and all nested function scopes to find `this` property accesses and aliases,
/// returning two sets: reads and writes. Returns `None` if analysis cannot be performed for the
/// provided body (callers may unwrap when a valid body is known).
///
/// # Examples
///
/// ```no_run
/// // Given a parsed class member `member_node` and its function body `body`:
/// let refs = collect_references_from_body(&member_node, &body).unwrap();
/// // `refs.reads` and `refs.writes` are sets of `ClassMemberReference`.
/// ```
fn collect_references_from_body(
    member: &JsSyntaxNode,
    body: &JsFunctionBody,
) -> Option<ClassMemberReferences> {
    let scoped_this_references = ThisScopeReferences::new(body).collect_function_this_references();

    let mut reads = HashSet::new();
    let mut writes = HashSet::new();

    visit_references_in_body(member, &scoped_this_references, &mut writes, &mut reads);

    Some(ClassMemberReferences { reads, writes })
}

/// Traverse a method or initializer AST node and collect `this`-based member reads and writes.
///
/// This walks the subtree rooted at `method_body_element` in preorder and records occurrences of
/// `this` (or aliases of `this`) that access class members. It classifies references into:
/// - reads: property access expressions and static member reads (e.g., `this.prop`, `this.#prop`),
///   including reads implied by compound assignments (e.g., `this.prop += 1`); and
/// - writes: direct assignments and destructuring targets that assign into `this`-based members,
///   as well as updates from pre/post increment and decrement expressions.
///
/// The function updates the provided `writes` and `reads` sets in-place. `scoped_this_references`
/// describes the set of valid `this` aliases per nested function scope and is used to resolve
/// whether a given expression refers to `this`.
///
/// Parameters:
/// - `method_body_element`: root syntax node of the method/initializer body to analyze.
/// - `scoped_this_references`: scoped `this` alias information for nested function scopes.
/// - `writes`: destination set updated with discovered `this`-based write references.
/// - `reads`: destination set updated with discovered `this`-based read references.
///
/// Note: This function performs a best-effort traversal and ignores nodes that do not match the
/// handled patterns; it does not return an error.
///
/// # Examples
///
/// ```
/// // Conceptual example (types elided for brevity):
/// // let root: JsSyntaxNode = parse_method_body("foo() { this.a = 1; this.b += 2 }");
/// // let scopes = compute_scoped_this_references(&root);
/// // let mut reads = HashSet::new();
/// // let mut writes = HashSet::new();
/// // visit_references_in_body(&root, &scopes, &mut writes, &mut reads);
/// // assert!(writes.contains(&ClassMemberReference { name: text("a"), .. }));
/// // assert!(reads.contains(&ClassMemberReference { name: text("b"), .. }));
/// ```
fn visit_references_in_body(
    method_body_element: &JsSyntaxNode,
    scoped_this_references: &[FunctionThisReferences],
    writes: &mut HashSet<ClassMemberReference>,
    reads: &mut HashSet<ClassMemberReference>,
) {
    let iter = method_body_element.preorder();

    for event in iter {
        match event {
            WalkEvent::Enter(node) => {
                handle_object_binding_pattern(&node, scoped_this_references, reads);
                handle_static_member_expression(&node, scoped_this_references, reads);
                handle_assignment_expression(&node, scoped_this_references, reads, writes);
                handle_pre_or_post_update_expression(&node, scoped_this_references, reads, writes);
            }
            WalkEvent::Leave(_) => {}
        }
    }
}

/// Record reads of `this`-based properties found in an object binding pattern.
///
/// Looks for `JsObjectBindingPattern` nodes that are part of a `JsVariableDeclarator` with
/// an initializer expression that is `this` or a known alias of `this`. Each property name
/// in the object binding (e.g. `{ a, b }`) is inserted into `reads` as a `ClassMemberReference`
/// with its trimmed text and range.
///
/// The function is scope-aware: `scoped_this_references` is consulted to determine whether the
/// initializer expression refers to `this` (directly or via an alias) within the current nested
/// function scopes.
///
/// # Examples
///
/// ```js
/// // Detected reads:
/// const { foo, bar } = this;   // reads: foo, bar
/// let { x } = aliasForThis;    // reads: x (if aliasForThis is an alias for `this`)
/// ```
fn handle_object_binding_pattern(
    node: &SyntaxNode<JsLanguage>,
    scoped_this_references: &[FunctionThisReferences],
    reads: &mut HashSet<ClassMemberReference>,
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
                    name: declarator.to_trimmed_text(),
                    range: declarator.syntax().text_trimmed_range(),
                });
            }
        }
    }
}

/// Collects reads from static member expressions whose object is `this` or a known `this` alias.
///
/// If `node` is a `JsStaticMemberExpression` and its object resolves to `this` (or an alias
/// recorded in `scoped_this_references`), the member name and source range are inserted into
/// `reads` as a `ClassMemberReference`.
///
/// # Examples
///
/// ```js
/// // When visiting nodes, this will record reads for `foo`, `bar`, and `#secret`:
/// //   this.foo
/// //   aliasForThis.bar  // if `aliasForThis` is a known alias for `this`
/// //   this.#secret
/// ```
fn handle_static_member_expression(
    node: &SyntaxNode<JsLanguage>,
    scoped_this_references: &[FunctionThisReferences],
    reads: &mut HashSet<ClassMemberReference>,
) {
    if let Some(static_member) = JsStaticMemberExpression::cast_ref(node)
        && let Ok(object) = static_member.object()
        && is_this_reference(&object, scoped_this_references)
        && let Ok(member) = static_member.member()
    {
        reads.insert(ClassMemberReference {
            name: member.to_trimmed_text(),
            range: static_member.syntax().text_trimmed_range(),
        });
    }
}

/// Collects `this`-based member reads and writes found in an assignment expression.
///
/// This inspects an assignment expression node and updates `reads` and `writes` with
/// ClassMemberReference entries for any member accessed via `this` (or a scoped alias).
///
/// Behavior summary:
/// - Compound assignments (e.g. `this.x += 1`) are treated as both a read and a write for `x`.
/// - Direct assignments to `this` properties (e.g. `this.y = ...`) are treated as writes.
/// - Array/object assignment targets that bind to `this` (e.g. `[this.a] = ...` or `{ p: this.b } = ...`)
///   produce writes for the bound members.
/// - Destructuring assignments where the source is `this` (e.g. `({ m } = this)`) produce reads.
///
/// The function is scope-aware and resolves `this` aliases recorded in `scoped_this_references`.
///
/// # Examples
///
/// ```rust
/// // Pseudocode/illustrative example (actual use requires constructing a Js AST node):
/// // For assignment `this.x += 1` this function will insert `x` into both `reads` and `writes`.
/// // For `({ m } = this)` it will insert `m` into `reads`.
/// ```
fn handle_assignment_expression(
    node: &SyntaxNode<JsLanguage>,
    scoped_this_references: &[FunctionThisReferences],
    reads: &mut HashSet<ClassMemberReference>,
    writes: &mut HashSet<ClassMemberReference>,
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
            )
        {
            writes.insert(name.clone());
        }
    }
}

/// Marks `this`-based increment/decrement expressions as both a read and a write of the referenced member.
///
/// Detects pre- and post-update operations (e.g., `this.x++`, `--alias.y`) where the operand is a `this`
/// property or an alias for `this`. When such an expression is found, the referenced member is added to
/// both the reads and writes sets.
///
/// # Examples
///
/// ```js
/// // These produce both a read and a write for `count` and `value` respectively:
/// this.count++;
/// --aliasForThis.value;
/// ```
fn handle_pre_or_post_update_expression(
    node: &SyntaxNode<JsLanguage>,
    scoped_this_references: &[FunctionThisReferences],
    reads: &mut HashSet<ClassMemberReference>,
    writes: &mut HashSet<ClassMemberReference>,
) {
    let operand = JsPostUpdateExpression::cast_ref(node)
        .and_then(|expr| expr.operand().ok())
        .or_else(|| JsPreUpdateExpression::cast_ref(node).and_then(|expr| expr.operand().ok()));

    if let Some(operand) = operand
        && let Some(name) = ThisPatternResolver::extract_this_member_reference(
            operand.as_js_static_member_assignment(),
            scoped_this_references,
        )
    {
        writes.insert(name.clone());
        reads.insert(name.clone());
    }
}

/// Collects reads and writes of `this`-based class member accesses found inside a constructor body.
///
/// The returned `ClassMemberReferences` contains two sets:
/// - `reads`: members that are read (accessed) via `this` or a `this` alias.
/// - `writes`: members that are written (assigned to) via `this` or a `this` alias.
///
/// This function traverses the constructor body and all nested function scopes that may capture
/// `this` (including aliasing like `const self = this`), so references inside nested functions are
/// included in the result.
///
/// # Examples
///
/// ```no_run
/// // Given a parsed `constructor_body: JsFunctionBody`, collect `this` member references:
/// // let refs = collect_references_from_constructor(&constructor_body);
/// // assert!(refs.reads.len() >= 0);
/// ```
fn collect_references_from_constructor(constructor_body: &JsFunctionBody) -> ClassMemberReferences {
    let all_descendants_fn_bodies_and_this_scopes: Vec<_> =
        ThisScopeReferences::new(constructor_body).collect_function_this_references();
    let mut reads = HashSet::new();
    let mut writes = HashSet::new();

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
    let mut reads = HashSet::new();
    let writes = HashSet::new();

    if let Ok(member) = static_member.member() {
        let name = member.to_trimmed_text();
        reads.insert(ClassMemberReference {
            name,
            range: static_member.syntax().text_trimmed_range(),
        });
    }

    Some(ClassMemberReferences { reads, writes })
}

/// Recursively unwraps parenthesized expressions to return the innermost expression.
///
/// This returns a cloned inner `AnyJsExpression` when `expr` is a `JsParenthesizedExpression`
/// (unwrapping nested parentheses); otherwise it returns a clone of `expr`.
///
/// # Examples
///
/// ```rust
/// # // Example is ignored because constructing AST nodes requires the parser.
/// # fn example() {
/// # let paren_expr = /* a parenthesized AnyJsExpression like `(a)` */ unimplemented!();
/// # let inner = /* the `a` AnyJsExpression */ unimplemented!();
/// # let unwrapped = crate::unwrap_expression(&paren_expr);
/// # assert_eq!(unwrapped, inner);
/// # }
/// ```
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
/// Returns true if `name_syntax` is lexically inside `scope` without being shadowed by an intervening
/// function body or a class declaration/expression.
///
/// The function walks ancestors of `name_syntax` and returns true when `scope` is reached before any
/// of: `JS_FUNCTION_BODY`, `JS_CLASS_EXPRESSION`, or `JS_CLASS_DECLARATION`. If any of those nodes
/// appear first, the name would be outside the intended lexical scope (i.e., shadowed) and the
/// function returns false.
///
/// # Examples
///
/// ```no_run
/// // Given `name_node` and `scope_node` previously obtained from the AST:
/// let is_valid = is_within_scope_without_shadowing(&name_node, &scope_node);
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use biome_js_parser::{JsParserOptions, Parse, parse};
    use biome_js_syntax::{AnyJsRoot, JsFileSource, JsObjectBindingPattern};
    use biome_rowan::AstNode;

    struct TestCase<'a> {
        description: &'a str,
        code: &'a str,
        expected_reads: Vec<&'a str>,
        expected_writes: Vec<&'a str>,
    }

    /// Parse a TypeScript source string into an AST root, panicking on syntax errors.
    ///
    /// This is a thin test helper that runs the project parser with TypeScript file
    /// settings and default parser options. It returns the successfully parsed
    /// `Parse<AnyJsRoot>`.
    ///
    /// # Panics
    ///
    /// Panics if the provided source contains syntax errors.
    ///
    /// # Examples
    ///
    /// ```
    /// let root = parse_ts("const x: number = 1;");
    /// // `root` is the parsed AST; the call would have panicked on syntax errors.
    /// ```
    fn parse_ts(code: &str) -> Parse<AnyJsRoot> {
        let source = parse(code, JsFileSource::ts(), JsParserOptions::default());

        if source.has_errors() {
            panic!("syntax error")
        }

        source
    }

    /// Returns the first `JsObjectBindingPattern` node found inside `syntax`.
    ///
    /// The returned value is a cloned `SyntaxNode<JsLanguage>` for the first object binding pattern
    /// encountered during a preorder descent. This function will panic if no object binding pattern
    /// exists in the provided node.
    ///
    /// # Examples
    ///
    /// ```
    /// // Given a parsed root node `root` containing `let { a } = this;`
    /// let obj_binding = parse_first_object_binding(&root);
    /// // `obj_binding` is the SyntaxNode for `{ a }`
    /// ```
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
                expected_reads: vec!["foo", "bar"],
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
                expected_reads: vec!["baz", "qux"],
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
            let mut reads = HashSet::new();

            handle_object_binding_pattern(&node, &function_this_references, &mut reads);

            let names: Vec<_> = reads.into_iter().map(|r| r.name.to_string()).collect();

            for expected_reads in &case.expected_reads {
                assert!(
                    names.contains(&(*expected_reads).to_string()),
                    "Case '{}' failed: expected to find '{}'",
                    case.description,
                    expected_reads
                );
            }
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
                expected_reads: vec!["foo", "bar"],
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
                expected_reads: vec!["baz", "qux"],
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

            // Collect all static member expressions in the syntax
            let mut reads = HashSet::new();

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

            let names: Vec<_> = reads.into_iter().map(|r| r.name.to_string()).collect();

            for expected_reads in &case.expected_reads {
                assert!(
                    names.contains(&(*expected_reads).to_string()),
                    "Case '{}' failed: expected to find '{}'",
                    case.description,
                    expected_reads
                );
            }
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
                expected_reads: vec!["x"],
                expected_writes: vec!["x", "y", "z"],
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
                expected_reads: vec!["x"],
                expected_writes: vec!["x", "y", "z"],
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

            let mut reads = HashSet::new();
            let mut writes = HashSet::new();

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

            let read_names: Vec<_> = reads.into_iter().map(|r| r.name.to_string()).collect();
            let write_names: Vec<_> = writes.into_iter().map(|r| r.name.to_string()).collect();

            for expected_read in &case.expected_reads {
                assert!(
                    read_names.contains(&(*expected_read).to_string()),
                    "Case '{}' failed: expected to find read '{}'",
                    case.description,
                    expected_read
                );
            }

            for expected_write in &case.expected_writes {
                assert!(
                    write_names.contains(&(*expected_write).to_string()),
                    "Case '{}' failed: expected to find write '{}'",
                    case.description,
                    expected_write
                );
            }
        }
    }

    #[test]
    fn test_pre_or_post_update_expression_reads_and_writes_variants() {
        let test_cases = [
            TestCase {
                description: "pre/post update expressions on this properties",
                code: r#"
                class Example {
                    method() {
                        this.count++;
                        --this.total;
                    }
                }
            "#,
                expected_reads: vec!["count", "total"],
                expected_writes: vec!["count", "total"],
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
                    }
                }
            "#,
                expected_reads: vec!["count", "total"],
                expected_writes: vec!["count", "total"],
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

            let mut reads = HashSet::new();
            let mut writes = HashSet::new();

            for node in syntax.descendants() {
                handle_pre_or_post_update_expression(
                    &node,
                    &function_this_references,
                    &mut reads,
                    &mut writes,
                );
            }

            let read_names: Vec<_> = reads.into_iter().map(|r| r.name.to_string()).collect();
            let write_names: Vec<_> = writes.into_iter().map(|r| r.name.to_string()).collect();

            for expected_name in &case.expected_reads {
                assert!(
                    read_names.contains(&(*expected_name).to_string()),
                    "Case '{}' failed: expected to find read '{}'",
                    case.description,
                    expected_name
                );
            }

            for expected_name in &case.expected_writes {
                assert!(
                    write_names.contains(&(*expected_name).to_string()),
                    "Case '{}' failed: expected to find write '{}'",
                    case.description,
                    expected_name
                );
            }
        }
    }
}
