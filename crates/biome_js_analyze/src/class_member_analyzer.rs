use biome_js_syntax::{
    AnyJsAssignment, AnyJsClassMember, AnyJsExpression, JsArrayAssignmentPattern,
    JsArrowFunctionExpression, JsAssignmentExpression, JsAwaitExpression, JsBlockStatement,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsClassMemberList,
    JsConditionalExpression, JsConstructorClassMember, JsElseClause, JsExpressionStatement,
    JsFunctionBody, JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsIfStatement,
    JsInitializerClause, JsLanguage, JsMethodClassMember, JsMethodObjectMember,
    JsObjectAssignmentPattern, JsObjectBindingPattern, JsObjectExpression, JsObjectMemberList,
    JsParenthesizedExpression, JsPostUpdateExpression, JsPreUpdateExpression,
    JsPropertyClassMember, JsReturnStatement, JsSetterClassMember, JsSetterObjectMember,
    JsStatementList, JsStaticMemberExpression, JsSyntaxKind, JsTemplateElement,
    JsTemplateElementList, JsTemplateExpression, JsVariableDeclaration, JsVariableDeclarator,
    JsVariableDeclaratorList, JsVariableStatement, TextRange, TsPropertyParameter,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxNode, Text, declare_node_union};
use std::collections::HashSet;
use std::vec::IntoIter;

pub trait ClassMemberAnalyzer {
    fn write_properties(&self) -> HashSet<ClassPropertyReference>;
    // class methods and properties
    fn read_members(&self) -> HashSet<ClassPropertyReference>;
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ClassPropertyReference {
    pub name: Text,
    pub range: TextRange,
}

impl ClassMemberAnalyzer for JsClassMemberList {
    fn write_properties(&self) -> HashSet<ClassPropertyReference> {
        self.visit_members(
            Self::collect_write_references_from_constructor,
            Self::collect_write_references_from_method_body,
            Self::collect_write_references_from_property_member,
        )
    }

    fn read_members(&self) -> HashSet<ClassPropertyReference> {
        self.visit_members(
            Self::collect_read_references_from_constructor,
            Self::collect_read_references_from_method_body,
            Self::collect_read_references_from_property_member,
        )
    }
}

impl ClassMemberAnalyzerVisitor for JsClassMemberList {
    fn visit_members<F, G, S>(
        &self,
        visit_constructor_references: F,
        visit_method_body_references: G,
        visit_member_initializer_references: S,
    ) -> HashSet<ClassPropertyReference>
    where
        F: Fn(&JsFunctionBody) -> Vec<ClassPropertyReference>,
        G: Fn(
            MethodBodyElementOrStatementList,
            &JsFunctionBody,
        ) -> Option<IntoIter<ClassPropertyReference>>,
        S: Fn(&JsStaticMemberExpression) -> Option<IntoIter<ClassPropertyReference>>,
    {
        self.iter()
            .filter_map(|member| match member {
                AnyJsClassMember::JsMethodClassMember(method) => {
                    method.body().ok().and_then(|body| {
                        visit_method_body_references(
                            MethodBodyElementOrStatementList::from(method.clone()),
                            &body,
                        )
                    })
                }
                AnyJsClassMember::JsSetterClassMember(setter) => {
                    setter.body().ok().and_then(|body| {
                        visit_method_body_references(
                            MethodBodyElementOrStatementList::from(setter.clone()),
                            &body,
                        )
                    })
                }
                AnyJsClassMember::JsGetterClassMember(getter) => {
                    getter.body().ok().and_then(|body| {
                        visit_method_body_references(
                            MethodBodyElementOrStatementList::from(getter.clone()),
                            &body,
                        )
                    })
                }
                AnyJsClassMember::JsPropertyClassMember(property) => {
                    if let Ok(expression) = property.value()?.expression() {
                        if let Some(arrow_function) =
                            JsArrowFunctionExpression::cast(expression.clone().into_syntax())
                        {
                            if let Ok(any_js_body) = arrow_function.body() {
                                if let Some(body) = any_js_body.as_js_function_body() {
                                    return visit_method_body_references(
                                        MethodBodyElementOrStatementList::from(arrow_function),
                                        body,
                                    );
                                }
                            }
                        }

                        if let Some(static_member_expression) =
                            expression.as_js_static_member_expression()
                        {
                            return visit_member_initializer_references(&static_member_expression);
                        }

                        // println!("expression is {:?}", expression);
                    };
                    None
                }
                AnyJsClassMember::JsConstructorClassMember(constructor) => constructor
                    .body()
                    .ok()
                    .map(|body| visit_constructor_references(&body).into_iter()),
                _ => None,
            })
            .flatten()
            .collect::<HashSet<_>>()
    }
}

// currently only picks up aliases on top level of the function body, can optionally be extended to
// collect aliases from nested scopes, but that would require more complex logic to handle closures
#[derive(Clone, Debug)]
struct ThisAliasesAndTheirScope {
    scope: JsFunctionBody,
    aliases: HashSet<ClassPropertyReference>,
}

declare_node_union! {
    pub ClassPropMemberOrConstructorTsParam = JsPropertyClassMember | TsPropertyParameter
}

declare_node_union! {
    pub AnyJsClassMethodBodyElement =
    JsArrowFunctionExpression |
    JsBlockStatement |
    JsCallArguments |
    JsCallExpression |
    JsConditionalExpression |
    JsConstructorClassMember |
    JsElseClause |
    JsExpressionStatement |
    JsFunctionBody |
    JsGetterClassMember |
    JsGetterObjectMember |
    JsIfStatement |
    JsInitializerClause |
    JsMethodClassMember |
    JsMethodObjectMember |
    JsObjectExpression |
    JsParenthesizedExpression |
    JsReturnStatement |
    JsSetterClassMember |
    JsSetterObjectMember |
    JsTemplateElement |
    JsTemplateExpression |
    JsVariableDeclaration |
    JsVariableDeclarator |
    JsVariableStatement |
    JsAwaitExpression
}

#[derive(Debug)]
enum MethodBodyElementOrStatementList {
    CallArgumentsList(JsCallArgumentList),
    MethodBodyElement(AnyJsClassMethodBodyElement),
    ObjectMemberList(JsObjectMemberList),
    StatementList(JsStatementList),
    TemplateElementList(JsTemplateElementList),
    VariableDeclaratorList(JsVariableDeclaratorList),
}

impl<T> From<T> for MethodBodyElementOrStatementList
where
    T: Into<AnyJsClassMethodBodyElement>,
{
    fn from(member: T) -> Self {
        Self::MethodBodyElement(member.into())
    }
}

/// fn collect_nested_this_aliases will only visit the list of descendants listed here, more can be added if necessary
impl MethodBodyElementOrStatementList {
    fn syntax(&self) -> &SyntaxNode<JsLanguage> {
        match self {
            Self::CallArgumentsList(node) => node.syntax(),
            Self::MethodBodyElement(node) => node.syntax(),
            Self::ObjectMemberList(list) => list.syntax(),
            Self::StatementList(list) => list.syntax(),
            Self::TemplateElementList(list) => list.syntax(),
            Self::VariableDeclaratorList(list) => list.syntax(),
        }
    }

    fn as_js_function_body(&self) -> Option<JsFunctionBody> {
        match self {
            Self::MethodBodyElement(AnyJsClassMethodBodyElement::JsFunctionBody(body)) => {
                Some(body.clone())
            }
            _ => None,
        }
    }

    fn cast_ref(syntax_node: &SyntaxNode<JsLanguage>) -> Option<Self> {
        JsObjectMemberList::cast_ref(syntax_node)
            .map(|e| Self::ObjectMemberList(e.clone()))
            .or_else(|| {
                JsStatementList::cast_ref(syntax_node).map(|e| Self::StatementList(e.clone()))
            })
            .or_else(|| {
                JsVariableDeclaratorList::cast_ref(syntax_node)
                    .map(|e| Self::VariableDeclaratorList(e.clone()))
            })
            .or_else(|| {
                JsCallArgumentList::cast_ref(syntax_node)
                    .map(|e| Self::CallArgumentsList(e.clone()))
            })
            .or_else(|| {
                AnyJsClassMethodBodyElement::cast_ref(syntax_node)
                    .map(|e| Self::MethodBodyElement(e.clone()))
            })
            .or_else(|| {
                JsTemplateElementList::cast_ref(syntax_node)
                    .map(|e| Self::TemplateElementList(e.clone()))
            })
    }
}

trait ClassMemberAnalyzerVisitor {
    fn visit_members<F, G, S>(
        &self,
        collect_constructor_member_references: F,
        collect_body_member_references: G,
        visit_member_initializer_references: S,
    ) -> HashSet<ClassPropertyReference>
    where
        F: Fn(&JsFunctionBody) -> Vec<ClassPropertyReference>,
        G: Fn(
            MethodBodyElementOrStatementList,
            &JsFunctionBody,
        ) -> Option<IntoIter<ClassPropertyReference>>,
        S: Fn(&JsStaticMemberExpression) -> Option<IntoIter<ClassPropertyReference>>;
}

trait ThisAliasResolver {
    fn collect_this_aliases_in_closure(body: &JsFunctionBody) -> Vec<ClassPropertyReference>;

    fn collect_nested_this_aliases(
        element: &MethodBodyElementOrStatementList,
        parent_aliases: &[ClassPropertyReference],
    ) -> Vec<ThisAliasesAndTheirScope>;

    fn update_fn_body_and_aliases(
        parent_aliases: &[ClassPropertyReference],
        results: &mut Vec<ThisAliasesAndTheirScope>,
        body: &JsFunctionBody,
    );

    fn collect_fn_body_this_aliases(body: &JsFunctionBody) -> Vec<ThisAliasesAndTheirScope>;

    fn is_this_or_alias(
        object: &AnyJsExpression,
        this_aliases: &[ThisAliasesAndTheirScope],
    ) -> bool;
}

impl ThisAliasResolver for JsClassMemberList {
    /// Process a js function body to find all reassignments/ aliases of this.
    /// It only processes the top level of the function body scope
    /// # Example
    /// ``` js
    /// var self = this;
    /// const parent = this;
    /// ```
    /// produces vec![Text(self), Text(parent)]
    fn collect_this_aliases_in_closure(body: &JsFunctionBody) -> Vec<ClassPropertyReference> {
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

                (expr.syntax().first_token()?.text() == "this").then(|| ClassPropertyReference {
                    name: id.to_trimmed_text().clone(),
                    range: id.syntax().text_trimmed_range(),
                })
            })
            .collect()
    }

    /// Finds recursively function bodies in a syntax node AND collects all this aliases applicable to the current fn body.
    /// e.g. var self = this; var another_self = this; ends up with this_aliases: [self, another_self]
    /// Only collects aliases that are not directly owned by a constructor, as those are not relevant for the current scope.
    fn collect_nested_this_aliases(
        method_body_element_or_statement_list: &MethodBodyElementOrStatementList,
        parent_this_aliases: &[ClassPropertyReference],
    ) -> Vec<ThisAliasesAndTheirScope> {
        let mut results = Vec::new();

        // First check if this node itself is a function body
        if let Some(body) = method_body_element_or_statement_list.as_js_function_body() {
            // Only add if it's not directly owned by a constructor
            if method_body_element_or_statement_list
                .syntax()
                .parent()
                .and_then(JsConstructorClassMember::cast)
                .is_none()
            {
                let current_scope_aliases = Self::collect_this_aliases_in_closure(&body);
                let mut this_aliases = HashSet::new();
                this_aliases.extend(parent_this_aliases.iter().cloned());
                this_aliases.extend(current_scope_aliases.clone());

                results.push(ThisAliasesAndTheirScope {
                    scope: body.clone(),
                    aliases: this_aliases,
                });
            }
        }

        // Collect function bodies from children
        for child in method_body_element_or_statement_list.syntax().children() {
            if child.kind() == JsSyntaxKind::JS_CLASS_EXPRESSION {
                // Skip class expressions, scope of `this` changes to the nested class
                break;
            }

            // Check arrow function expressions
            if let Some(func_expr) = JsArrowFunctionExpression::cast(child.clone()) {
                if let Some(body) = func_expr
                    .body()
                    .ok()
                    .and_then(|body| body.as_js_function_body().cloned())
                {
                    Self::update_fn_body_and_aliases(parent_this_aliases, &mut results, &body);
                }
            }
            // Check function expressions
            else if let Some(func_expr) = JsFunctionExpression::cast(child.clone()) {
                if let Ok(body) = func_expr.body() {
                    Self::update_fn_body_and_aliases(parent_this_aliases, &mut results, &body);
                }
            }
            // Check arrow functions with block bodies
            else if let Some(arrow_func) = JsArrowFunctionExpression::cast(child.clone()) {
                if let Ok(body) = arrow_func.body() {
                    if let Some(block) = body.as_any_js_expression() {
                        if let Some(body) = JsFunctionBody::cast(block.syntax().clone()) {
                            Self::update_fn_body_and_aliases(
                                parent_this_aliases,
                                &mut results,
                                &body,
                            );
                        }
                    }
                }
            }
            // Check method definitions
            else if let Some(method) = JsMethodObjectMember::cast(child.clone()) {
                if let Ok(body) = method.body() {
                    Self::update_fn_body_and_aliases(parent_this_aliases, &mut results, &body);
                }
            }
            // Recurse for other node types and append their results
            else if let Some(child) = MethodBodyElementOrStatementList::cast_ref(&child) {
                results.extend(Self::collect_nested_this_aliases(
                    &child,
                    parent_this_aliases,
                ));
            }
        }

        results
    }

    /// Update results with current scope aliases if found and returns the updated FnBodyAndThisAliases struct
    fn update_fn_body_and_aliases(
        parent_this_aliases: &[ClassPropertyReference],
        results: &mut Vec<ThisAliasesAndTheirScope>,
        body: &JsFunctionBody,
    ) {
        let current_scope_aliases = Self::collect_this_aliases_in_closure(body);
        let mut this_aliases = HashSet::new();
        this_aliases.extend(parent_this_aliases.iter().cloned());
        this_aliases.extend(current_scope_aliases.clone());

        results.push(ThisAliasesAndTheirScope {
            scope: body.clone(),
            aliases: this_aliases,
        });
    }

    /// Extracts all aliases of `this` variable in the immediate body closure and keeps the body for checking scope.
    fn collect_fn_body_this_aliases(body: &JsFunctionBody) -> Vec<ThisAliasesAndTheirScope> {
        let this_variable_aliases: Vec<_> = Self::collect_this_aliases_in_closure(body);
        Self::collect_nested_this_aliases(
            &MethodBodyElementOrStatementList::from(body.clone()),
            &this_variable_aliases,
        )
    }

    /// Checks recursively the assignment operand equals a reference to `this` (e.g. `this.privateProp`)
    fn is_this_or_alias(
        object: &AnyJsExpression,
        this_aliases: &[ThisAliasesAndTheirScope],
    ) -> bool {
        if object.as_js_this_expression().is_some() {
            return true;
        }

        if let Some(js_identifier_expression) = object.as_js_identifier_expression() {
            if let Ok(name) = js_identifier_expression.name() {
                if let Ok(value_token) = name.value_token() {
                    let name_syntax = name.syntax();

                    return this_aliases.iter().any(
                        |ThisAliasesAndTheirScope { aliases, scope }| {
                            let is_alias = aliases.iter().any(|mutation| {
                                mutation
                                    .name
                                    .text()
                                    .eq(value_token.token_text_trimmed().text())
                            });

                            let is_within_scope = name_syntax
                                .ancestors()
                                .any(|ancestor| ancestor.key() == scope.syntax().key());

                            is_alias && is_within_scope
                        },
                    );
                }
            }
        }

        false
    }
}

trait MemberReadVisitor {
    fn collect_read_references_from_method_body<T>(
        member: T,
        body: &JsFunctionBody,
    ) -> Option<IntoIter<ClassPropertyReference>>
    where
        T: Into<MethodBodyElementOrStatementList>;

    fn collect_read_references_from_property_member(
        js_static_member_expression: &JsStaticMemberExpression,
    ) -> Option<IntoIter<ClassPropertyReference>>;

    fn collect_read_references_from_constructor(
        constructor_body: &JsFunctionBody,
    ) -> Vec<ClassPropertyReference>;

    fn visit_read_references_in_body<F>(
        element: &MethodBodyElementOrStatementList,
        this_aliases: &[ThisAliasesAndTheirScope],
        on_name: &mut F,
    ) where
        F: FnMut(ClassPropertyReference);
}

impl MemberReadVisitor for JsClassMemberList {
    /// Iterates over all members of a JavaScript class and collects the names of members that are readonly accessed
    /// within class methods, setters, or the constructor.
    /// It analyzes method and setter bodies for assignments and updates to this properties,
    fn collect_read_references_from_method_body<T>(
        member: T,
        body: &JsFunctionBody,
    ) -> Option<IntoIter<ClassPropertyReference>>
    where
        T: Into<MethodBodyElementOrStatementList>,
    {
        let this_aliases = Self::collect_fn_body_this_aliases(body);
        let mut names = Vec::new();

        Self::visit_read_references_in_body(&member.into(), &this_aliases, &mut |name| {
            names.push(name);
        });

        Some(names.into_iter())
    }

    fn collect_read_references_from_property_member(
        static_member: &JsStaticMemberExpression,
    ) -> Option<IntoIter<ClassPropertyReference>> {
        let mut names = Vec::new();

        if let Some(member) = static_member.member().ok() {
            let name = member.to_trimmed_text();
            names.push(ClassPropertyReference {
                name,
                range: static_member.syntax().text_trimmed_range(),
            });
        }

        Some(names.into_iter())
    }

    /// Extracts all read only members within function bodies found in CONSTRUCTOR only:
    /// expression statements (or so called IIFE),
    /// nested classes methods,
    /// or inner functions
    fn collect_read_references_from_constructor(
        constructor_body: &JsFunctionBody,
    ) -> Vec<ClassPropertyReference> {
        let this_variable_aliases: Vec<_> = Self::collect_this_aliases_in_closure(constructor_body);

        let all_descendants_fn_bodies_and_this_aliases: Vec<_> = Self::collect_nested_this_aliases(
            &MethodBodyElementOrStatementList::from(constructor_body.clone()),
            &this_variable_aliases,
        );

        all_descendants_fn_bodies_and_this_aliases
            .iter()
            .flat_map(|this_aliases_and_their_scope| {
                let mut names = Vec::new();

                Self::visit_read_references_in_body(
                    &MethodBodyElementOrStatementList::from(
                        this_aliases_and_their_scope.scope.clone(),
                    ),
                    std::slice::from_ref(this_aliases_and_their_scope),
                    &mut |name| {
                        names.push(name);
                    },
                );

                names
            })
            .collect::<Vec<_>>()
    }
    // todo check if type can be simplified !!!!
    fn visit_read_references_in_body<F>(
        method_body_element: &MethodBodyElementOrStatementList,
        this_aliases: &[ThisAliasesAndTheirScope],
        on_name: &mut F,
    ) where
        F: FnMut(ClassPropertyReference),
    {
        let iter = method_body_element.syntax().preorder();

        for event in iter {
            match event {
                biome_rowan::WalkEvent::Enter(node) => {
                    // check if right hand side is `this` and left hand side is `{one, two}` e.g. `const {one, two} = this;`
                    if let Some(binding) = JsObjectBindingPattern::cast_ref(&node) {
                        if let Some(parent) = binding.syntax().parent()
                            && let Some(variable_declarator) =
                                JsVariableDeclarator::cast_ref(&parent)
                            && let Some(initializer) = variable_declarator.initializer()
                            && let Ok(expression) = initializer.expression()
                        {
                            for declarator in binding.properties() {
                                if let Some(declarator) = declarator.ok()
                                    && Self::is_this_or_alias(&expression, this_aliases)
                                {
                                    on_name(ClassPropertyReference {
                                        name: declarator.to_trimmed_text(),
                                        range: declarator.syntax().text_trimmed_range(),
                                    });
                                }
                            }
                        }
                    } else if let Some(static_member) = JsStaticMemberExpression::cast_ref(&node) {
                        if let Ok(object) = static_member.object() {
                            if Self::is_this_or_alias(&object, this_aliases) {
                                if let Ok(member) = static_member.member() {
                                    on_name(ClassPropertyReference {
                                        name: member.to_trimmed_text(),
                                        range: static_member.syntax().text_trimmed_range(),
                                    });
                                }
                            }
                        }
                    } else if let Some(operand) = JsPostUpdateExpression::cast_ref(&node)
                        .and_then(|expr| expr.operand().ok())
                        .or_else(|| {
                            JsPreUpdateExpression::cast_ref(&node.clone())
                                .and_then(|expr| expr.operand().ok())
                        })
                    {
                        if let Some(name) = Self::extract_static_assignment_name(&operand, this_aliases) {
                            on_name(name);
                        }
                    } else {
                        // uncomment the following line to debug what other entities should be potentially processed
                         println!("node is {:?}: {:?}",node, node.to_string());
                    }
                }
                biome_rowan::WalkEvent::Leave(_) => {}
            }
        }
    }
}

trait PropertyWritesVisitor {
    fn collect_write_references_from_method_body<T>(
        member: T,
        body: &JsFunctionBody,
    ) -> Option<IntoIter<ClassPropertyReference>>
    where
        T: Into<MethodBodyElementOrStatementList>;

    fn collect_write_references_from_property_member(
        static_member: &JsStaticMemberExpression,
    ) -> Option<IntoIter<ClassPropertyReference>>;

    fn collect_write_references_from_constructor(
        constructor_body: &JsFunctionBody,
    ) -> Vec<ClassPropertyReference>;

    fn visit_write_references_in_body<F>(
        element: &MethodBodyElementOrStatementList,
        this_aliases: &[ThisAliasesAndTheirScope],
        on_name: &mut F,
    ) where
        F: FnMut(ClassPropertyReference);
}

impl PropertyWritesVisitor for JsClassMemberList {
    /// Iterates over all members of a JavaScript class and collects the names of properties that are reassigned (mutated)
    /// within class methods, setters, or the constructor.
    /// It analyzes method and setter bodies for assignments and updates to this properties,
    /// and also tracks mutations in the constructor.
    /// The result is a Vec<ClassPropertyMutation> containing all property names that are updated anywhere in the class.
    fn collect_write_references_from_method_body<T>(
        member: T,
        body: &JsFunctionBody,
    ) -> Option<IntoIter<ClassPropertyReference>>
    where
        T: Into<MethodBodyElementOrStatementList>,
    {
        let this_aliases = Self::collect_fn_body_this_aliases(body);
        let mut names = Vec::new();

        Self::visit_write_references_in_body(&member.into(), &this_aliases, &mut |name| {
            names.push(name);
        });

        Some(names.into_iter())
    }
    // we will eventually need this to handle more complex cases such as this.prop = { method: () => this.anotherProp++} etc.
    fn collect_write_references_from_property_member(
        _js_static_member_expression: &JsStaticMemberExpression,
    ) -> Option<IntoIter<ClassPropertyReference>> {
        let names = Vec::new();

        Some(names.into_iter())
    }

    /// Extracts all mutations of class member props within function bodies found in CONSTRUCTOR only:
    /// expression statements (or so called IIFE),
    /// nested classes methods,
    /// or inner functions
    fn collect_write_references_from_constructor(
        constructor_body: &JsFunctionBody,
    ) -> Vec<ClassPropertyReference> {
        let this_variable_aliases: Vec<_> = Self::collect_this_aliases_in_closure(constructor_body);

        let all_descendants_fn_bodies_and_this_aliases: Vec<_> = Self::collect_nested_this_aliases(
            &MethodBodyElementOrStatementList::from(constructor_body.clone()),
            &this_variable_aliases,
        );

        all_descendants_fn_bodies_and_this_aliases
            .iter()
            .flat_map(|this_aliases_and_their_scope| {
                let mut names = Vec::new();

                Self::visit_write_references_in_body(
                    &MethodBodyElementOrStatementList::from(
                        this_aliases_and_their_scope.scope.clone(),
                    ),
                    std::slice::from_ref(this_aliases_and_their_scope),
                    &mut |name| {
                        names.push(name);
                    },
                );

                names
            })
            .collect::<Vec<_>>()
    }

    fn visit_write_references_in_body<F>(
        method_body_element: &MethodBodyElementOrStatementList,
        this_aliases: &[ThisAliasesAndTheirScope],
        on_name: &mut F,
    ) where
        F: FnMut(ClassPropertyReference),
    {
        method_body_element.syntax().children().for_each(|child| {
            if let Some(left) =
                JsAssignmentExpression::cast_ref(&child).and_then(|expr| expr.left().ok())
            {
                if let Some(assignment) = left.as_js_array_assignment_pattern().cloned() {
                    for name in Self::collect_array_assignment_names(&assignment, this_aliases) {
                        on_name(name);
                    }
                    return;
                }

                if let Some(assignment) = left.as_js_object_assignment_pattern().cloned() {
                    for name in Self::collect_object_assignment_names(&assignment, this_aliases) {
                        on_name(name);
                    }
                    return;
                }

                if let Some(assignment) = left.as_any_js_assignment().cloned() {
                    if let Some(name) =
                        Self::extract_static_assignment_name(&assignment, this_aliases)
                    {
                        on_name(name);
                    }
                    return;
                }
            }

            let operand = JsPostUpdateExpression::cast_ref(&child)
                .and_then(|expr| expr.operand().ok())
                .or_else(|| {
                    JsPreUpdateExpression::cast_ref(&child.clone())
                        .and_then(|expr| expr.operand().ok())
                });

            if let Some(operand) = operand {
                if let Some(name) = Self::extract_static_assignment_name(&operand, this_aliases) {
                    on_name(name);
                }
            } else if let Some(grand_child) = MethodBodyElementOrStatementList::cast_ref(&child) {
                Self::visit_write_references_in_body(&grand_child, this_aliases, on_name);
            } else {
                // uncomment the following line to debug what other entities should be added to MethodBodyElementOrStatementList
                // println!("child is {:?}", child);
            }
        });
    }
}

/// deals with destructuring of [this.prop, ...this.#private] and {this.prop, ...this.#private}
trait ThisPatternResolver {
    fn collect_array_assignment_names(
        array_assignment_pattern: &JsArrayAssignmentPattern,
        this_aliases: &[ThisAliasesAndTheirScope],
    ) -> Vec<ClassPropertyReference>;

    fn collect_object_assignment_names(
        assignment: &JsObjectAssignmentPattern,
        this_aliases: &[ThisAliasesAndTheirScope],
    ) -> Vec<ClassPropertyReference>;

    fn extract_static_assignment_name(
        operand: &AnyJsAssignment,
        this_aliases: &[ThisAliasesAndTheirScope],
    ) -> Option<ClassPropertyReference>;
}

impl ThisPatternResolver for JsClassMemberList {
    /// Extracts the names of all properties assigned to this (or its aliases) within the array assignment pattern.
    /// It handles both direct elements and rest elements (e.g., [this.prop, ...this.#private])
    /// and extracts property names that are being assigned via destructuring.
    /// This is useful for detecting which class properties are mutated through array destructuring assignments.
    fn collect_array_assignment_names(
        array_assignment_pattern: &JsArrayAssignmentPattern,
        this_aliases: &[ThisAliasesAndTheirScope],
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
        this_aliases: &[ThisAliasesAndTheirScope],
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
        this_aliases: &[ThisAliasesAndTheirScope],
    ) -> Option<ClassPropertyReference> {
        operand
            .as_js_static_member_assignment()
            .and_then(|assignment| {
                if let Ok(object) = assignment.object() {
                    if Self::is_this_or_alias(&object, this_aliases) {
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
                } else {
                    None
                }
            })
    }
}
