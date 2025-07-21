use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsClassMember, AnyJsClassMemberName, AnyJsConstructorParameter,
    AnyJsPropertyModifier, AnyTsPropertyParameterModifier, JsArrayAssignmentPattern,
    JsArrowFunctionExpression, JsAssignmentExpression, JsAwaitExpression, JsBlockStatement,
    JsCallArgumentList, JsCallArguments, JsCallExpression, JsClassDeclaration, JsClassMemberList,
    JsConditionalExpression, JsConstructorClassMember, JsElseClause, JsExpressionStatement,
    JsFunctionBody, JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsIfStatement,
    JsInitializerClause, JsLanguage, JsMethodClassMember, JsMethodObjectMember,
    JsObjectAssignmentPattern, JsObjectExpression, JsObjectMemberList, JsParenthesizedExpression,
    JsPostUpdateExpression, JsPreUpdateExpression, JsPropertyClassMember, JsReturnStatement,
    JsSetterClassMember, JsSetterObjectMember, JsStatementList, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxToken, JsTemplateElement,
    JsTemplateElementList, JsTemplateExpression, JsThisExpression, JsVariableDeclaration,
    JsVariableDeclarator, JsVariableDeclaratorList, JsVariableStatement, TextRange,
    TsAccessibilityModifier, TsPropertyParameter, TsReadonlyModifier,
};
use biome_rowan::{
    AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxNode, Text,
    TriviaPiece, declare_node_union,
};
use biome_rule_options::use_readonly_class_properties::UseReadonlyClassPropertiesOptions;
use std::iter::once;
use std::vec::IntoIter;

declare_lint_rule! {
    /// Enforce marking members as `readonly` if they are never modified outside the constructor.
    ///
    /// This rule ensures that class properties, especially private ones, are marked as `readonly` if their values
    /// remain constant after being initialized. This helps improve code readability, maintainability, and ensures
    /// immutability where applicable.
    ///
    /// It can be configured to check only private members or all class properties.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// class Container {
    ///     private onlyModifiedInConstructor = 1;
    ///     constructor(
    ///         member1: number,
    ///     ) {
    ///         this.onlyModifiedInConstructor = onlyModifiedInConstructor;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Container {
    ///     constructor(
    ///        private constructorParameter: number,
    ///     ) {
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Container {
    ///     private neverModifiedMember = true;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Container {
    ///     #neverModifiedPrivateField = 3;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// class Container {
    ///     private readonly neverModifiedMember = true;
    ///     private readonly onlyModifiedInConstructor: number;
    ///     readonly #neverModifiedPrivateField = 3;
    ///
    ///     public constructor(
    ///         onlyModifiedInConstructor: number,
    ///         private readonly neverModifiedParameter: string,
    ///     ) {
    ///         this.onlyModifiedInConstructor = onlyModifiedInConstructor;
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `checkAllProperties`
    ///
    /// Checks whether all class properties (including public and protected) should be analyzed.
    /// By default, `checkAllProperties` is set to `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "checkAllProperties": true
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// class Example {
    ///     public constantValue = 42;
    ///
    ///     constructor(value: number) {
    ///         this.constantValue = value;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,use_options,expect_diagnostic
    /// class Example {
    ///     constructor(protected constructorParameter: string) {
    ///     }
    /// }
    /// ```
    ///
    pub UseReadonlyClassProperties {
        version: "2.1.0",
        name: "useReadonlyClassProperties",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-readonly").same()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseReadonlyClassProperties {
    type Query = Ast<JsClassDeclaration>;
    type State = PropOrParam;
    type Signals = Box<[Self::State]>;
    type Options = UseReadonlyClassPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let members = root.members();
        let private_only = !ctx.options().check_all_properties;
        let mut mutated_class_property_names = collect_mutated_class_property_names(&members);
        mutated_class_property_names.dedup();

        let constructor_params: Vec<_> =
            collect_non_readonly_constructor_parameters(root, private_only);
        let non_readonly_class_property_members =
            collect_non_readonly_class_member_properties(&members, private_only);

        constructor_params
            .clone()
            .into_iter()
            .chain(
                non_readonly_class_property_members.filter(|class_property_member| {
                    !constructor_params.clone().into_iter().any(|node| {
                        node.to_trimmed_text() == class_property_member.to_trimmed_text()
                    })
                }),
            )
            .filter_map(|prop_or_param| {
                if mutated_class_property_names
                    .clone()
                    .into_iter()
                    .any(|name| {
                        if let Some(TextAndRange { text, .. }) =
                            extract_range_and_text(&prop_or_param.clone())
                        {
                            return name.eq(&text);
                        }

                        false
                    })
                {
                    None
                } else {
                    Some(prop_or_param.clone())
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let TextAndRange { text, range } = extract_range_and_text(&node.clone())?;

        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Member '"{text.text()}"' is never reassigned."
            },
        ).note(markup! {
                "Using "<Emphasis>"readonly"</Emphasis>" improves code safety, clarity, and helps prevent unintended mutations."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, node: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let original_node = node.syntax();
        let readonly_token = make::ts_readonly_modifier(JsSyntaxToken::new_detached(
            JsSyntaxKind::TS_READONLY_MODIFIER,
            "readonly ",
            [],
            [TriviaPiece::whitespace(1)],
        ));

        if let Some(PropOrParam::JsPropertyClassMember(member)) =
            PropOrParam::cast(original_node.clone())
        {
            if let Ok(member_name) = member.name() {
                let replace_modifiers = make::js_property_modifier_list(
                    member
                        .modifiers()
                        .iter()
                        .chain(once(AnyJsPropertyModifier::TsReadonlyModifier(
                            readonly_token,
                        )))
                        .collect::<Vec<_>>(),
                );

                if let Some(modified_member) =
                    extract_property_member_name_trimmed_whitespace(member_name.clone())
                {
                    let mut builder =
                        make::js_property_class_member(replace_modifiers, modified_member);

                    if let Some(property_annotation) = member.property_annotation() {
                        builder = builder.with_property_annotation(property_annotation);
                    }

                    if let Some(semicolon_token) = member.semicolon_token() {
                        builder = builder.with_semicolon_token(semicolon_token);
                    }

                    if let Some(value) = member.value() {
                        builder = builder.with_value(value);
                    }

                    mutation.replace_node(member.clone(), builder.build());
                }
            }
        } else if let Some(PropOrParam::TsPropertyParameter(parameter)) =
            PropOrParam::cast(original_node.clone())
        {
            let replace_modifiers = make::ts_property_parameter_modifier_list(
                parameter
                    .modifiers()
                    .iter()
                    .chain(once(AnyTsPropertyParameterModifier::TsReadonlyModifier(
                        readonly_token,
                    )))
                    .collect::<Vec<_>>(),
            );

            if let Ok(formal_parameter) = parameter.formal_parameter() {
                let replace_parameter = make::ts_property_parameter(
                    parameter.decorators(),
                    replace_modifiers,
                    formal_parameter,
                );

                mutation.replace_node_discard_trivia(parameter.clone(), replace_parameter);
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add "<Emphasis>"readonly"</Emphasis>" decorator." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub AnyThisMemberLike = JsThisExpression | JsStaticMemberExpression
}

declare_node_union! {
    pub PropOrParam = JsPropertyClassMember | TsPropertyParameter
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

/// fn visit_fn_body_descendants will only visit the list of descendants listed here, more can be added if necessary
impl MethodBodyElementOrStatementList {
    pub fn syntax(&self) -> &SyntaxNode<JsLanguage> {
        match self {
            Self::CallArgumentsList(node) => node.syntax(),
            Self::MethodBodyElement(node) => node.syntax(),
            Self::ObjectMemberList(list) => list.syntax(),
            Self::StatementList(list) => list.syntax(),
            Self::TemplateElementList(list) => list.syntax(),
            Self::VariableDeclaratorList(list) => list.syntax(),
        }
    }

    pub fn as_js_function_body(&self) -> Option<JsFunctionBody> {
        match self {
            Self::MethodBodyElement(AnyJsClassMethodBodyElement::JsFunctionBody(body)) => {
                Some(body.clone())
            }
            _ => None,
        }
    }

    pub fn cast_ref(syntax_node: &SyntaxNode<JsLanguage>) -> Option<Self> {
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

#[derive(Debug)]
struct TextAndRange {
    text: Text,
    range: TextRange,
}

// currently only picks up aliases on top level of the function body, can optionally be extended to
// collect aliases from nested scopes, but that would require more complex logic to handle closures
#[derive(Clone, Debug)]
struct ThisAliasesAndTheirScope {
    scope: JsFunctionBody,
    aliases: Vec<Text>,
}

/// Collects mutable (not being `readonly`) class properties (excluding `static` and `accessor`),
/// If `private_only` is true, only private properties are included.
/// This is used to identify class properties that are candidates for being marked as `readonly`.
/// e.g. all properties in `class Container { private onlyModifiedInConstructor = 1; public paramTwo: number; }`
fn collect_non_readonly_class_member_properties(
    members: &JsClassMemberList,
    private_only: bool,
) -> impl Iterator<Item = PropOrParam> {
    members.iter().filter_map(move |member| {
        let property_class_member = member.as_js_property_class_member()?;

        let is_js_computed_name = property_class_member
            .name()
            .iter()
            .any(|name| name.as_js_computed_member_name().is_some());

        if property_class_member.modifiers().iter().any(|modifier| {
            modifier.as_ts_readonly_modifier().is_some()
                || modifier.as_js_static_modifier().is_some()
                || modifier.as_js_accessor_modifier().is_some()
        }) || is_js_computed_name
        {
            return None;
        }

        let some_property = Some(PropOrParam::JsPropertyClassMember(
            property_class_member.clone(),
        ));

        if !private_only {
            return some_property;
        }

        let is_private = matches!(
            member.name().ok()??,
            AnyJsClassMemberName::JsPrivateClassMemberName(_)
        ) || property_class_member.modifiers().iter().any(|x| {
            TsAccessibilityModifier::cast(x.into_syntax())
                .is_some_and(|modifier| modifier.is_private())
        });

        if is_private {
            return some_property;
        }
        None
    })
}

/// Collects all all mutable (non-readonly) constructor parameters from a given class declaration. If private_only is true, it only includes parameters with private visibility.
/// It returns a Vec<PropOrParam> representing these parameters, which are candidates for being marked as readonly.
/// e.g. constructor(private paramOne: string, public paramTwo: number) {} makes both paramOne and paramTwo classs member properties.
fn collect_non_readonly_constructor_parameters(
    class_declaration: &JsClassDeclaration,
    private_only: bool,
) -> Vec<PropOrParam> {
    class_declaration
        .members()
        .iter()
        .find_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(member) => Some(member),
            _ => None,
        })
        .into_iter()
        .filter_map(|constructor| constructor.parameters().ok())
        .flat_map(|constructor_params| constructor_params.parameters().iter())
        .filter_map(move |param| match param.ok()? {
            AnyJsConstructorParameter::TsPropertyParameter(ts_property)
                if is_non_readonly_and_optionally_private(&ts_property, private_only) =>
            {
                Some(PropOrParam::TsPropertyParameter(ts_property))
            }
            _ => None,
        })
        .collect()
}

/// Iterates over all members of a JavaScript class and collects the names of properties that are reassigned (mutated)
/// within class methods, setters, or the constructor.
/// It analyzes method and setter bodies for assignments and updates to this properties,
/// and also tracks mutations in the constructor.
/// The result is a Vec<Text> containing all property names that are updated anywhere in the class.
fn collect_names_from_class_member_body<T>(
    member: T,
    body: &JsFunctionBody,
) -> Option<IntoIter<Text>>
where
    T: Into<MethodBodyElementOrStatementList>,
{
    let this_aliases = collect_fn_body_this_aliases(body);
    let mut names = Vec::new();

    visit_fn_body_descendants(&member.into(), &this_aliases, &mut |name| {
        names.push(name);
    });

    Some(names.into_iter())
}

fn collect_mutated_class_property_names(members: &JsClassMemberList) -> Vec<Text> {
    members
        .iter()
        .filter_map(|member| match member {
            // assignments in class methods
            AnyJsClassMember::JsMethodClassMember(method) => {
                if let Ok(body) = method.body() {
                    collect_names_from_class_member_body(
                        MethodBodyElementOrStatementList::from(method.clone()),
                        &body,
                    )
                } else {
                    None
                }
            }
            // assignments in setters
            AnyJsClassMember::JsSetterClassMember(setter) => {
                if let Ok(body) = setter.body() {
                    collect_names_from_class_member_body(
                        MethodBodyElementOrStatementList::from(setter.clone()),
                        &body,
                    )
                } else {
                    None
                }
            }
            // assignments in getters, technically possible, but not recommended
            AnyJsClassMember::JsGetterClassMember(getter) => {
                if let Ok(body) = getter.body() {
                    collect_names_from_class_member_body(
                        MethodBodyElementOrStatementList::from(getter.clone()),
                        &body,
                    )
                } else {
                    None
                }
            }
            // assignments in property class member if it is an arrow function
            AnyJsClassMember::JsPropertyClassMember(property) => {
                if let Ok(expression) = property.value()?.expression() {
                    if let Some(arrow_function) =
                        JsArrowFunctionExpression::cast(expression.into_syntax())
                    {
                        if let Ok(any_js_body) = arrow_function.body() {
                            if let Some(body) = any_js_body.as_js_function_body() {
                                return collect_names_from_class_member_body(
                                    MethodBodyElementOrStatementList::from(arrow_function),
                                    body,
                                );
                            }
                        }
                    }
                };
                None
            }
            // assignments in constructor
            AnyJsClassMember::JsConstructorClassMember(constructor) => {
                if let Ok(body) = constructor.body() {
                    Some(collect_class_member_props_mutations_in_constructor(&body).into_iter())
                } else {
                    None
                }
            }
            _ => None,
        })
        .flatten()
        .collect::<Vec<_>>()
}

/// Checks recursively the assignment operand equals a reference to `this` (e.g. `this.privateProp`)
fn contains_this_or_static_member_kind(
    assignment: &JsStaticMemberAssignment,
    this_aliases: &[ThisAliasesAndTheirScope],
) -> bool {
    if let Ok(object) = assignment.object() {
        if object.as_js_this_expression().is_some() {
            return true;
        }

        if let Some(js_identifier_expression) = object.as_js_identifier_expression() {
            if let Ok(name) = js_identifier_expression.name() {
                if let Ok(value_token) = name.value_token() {
                    let name_syntax = name.syntax();

                    return this_aliases.iter().any(
                        |ThisAliasesAndTheirScope { aliases, scope }| {
                            aliases.contains(&Text::from(value_token.token_text_trimmed()))
                                && name_syntax
                                    .ancestors()
                                    .any(|ancestor| ancestor.key() == scope.syntax().key())
                        },
                    );
                }
            }
        }
    }

    false
}

fn visit_fn_body_descendants<F>(
    method_body_element: &MethodBodyElementOrStatementList,
    this_aliases: &[ThisAliasesAndTheirScope],
    on_name: &mut F,
) where
    F: FnMut(Text),
{
    method_body_element.syntax().children().for_each(|child| {
        if let Some(left) =
            JsAssignmentExpression::cast_ref(&child).and_then(|expr| expr.left().ok())
        {
            if let Some(assignment) = left.as_js_array_assignment_pattern().cloned() {
                for name in extract_js_array_assignment_pattern_names(&assignment, this_aliases) {
                    on_name(name);
                }
                return;
            }

            if let Some(assignment) = left.as_js_object_assignment_pattern().cloned() {
                for name in collect_js_object_assignment_pattern_names(&assignment, this_aliases) {
                    on_name(name);
                }
                return;
            }

            if let Some(assignment) = left.as_any_js_assignment().cloned() {
                if let Some(name) = extract_static_member_assignment_name(&assignment, this_aliases)
                {
                    on_name(name);
                }
                return;
            }
        }

        let operand = JsPostUpdateExpression::cast_ref(&child)
            .and_then(|expr| expr.operand().ok())
            .or_else(|| {
                JsPreUpdateExpression::cast_ref(&child.clone()).and_then(|expr| expr.operand().ok())
            });

        if let Some(operand) = operand {
            if let Some(name) = extract_static_member_assignment_name(&operand, this_aliases) {
                on_name(name);
            }
        } else if let Some(grand_child) = MethodBodyElementOrStatementList::cast_ref(&child) {
            visit_fn_body_descendants(&grand_child, this_aliases, on_name);
        } else {
            // uncomment the following line to debug what other entities should be added to MethodBodyElementOrStatementList
            // println!("child is {:?}", child);
        }
    });
}

/// Extracts the names of all properties assigned to this (or its aliases) within the array assignment pattern.
/// It handles both direct elements and rest elements (e.g., [this.prop, ...this.#private])
/// and extracts property names that are being assigned via destructuring.
/// This is useful for detecting which class properties are mutated through array destructuring assignments.
fn extract_js_array_assignment_pattern_names(
    array_assignment_pattern: &JsArrayAssignmentPattern,
    this_aliases: &[ThisAliasesAndTheirScope],
) -> Vec<Text> {
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
                        extract_static_member_assignment_name(assignment, this_aliases)
                    })
            }
            // [...this.#value]
            else if let Some(rest_element) = element.as_js_array_assignment_pattern_rest_element()
            {
                rest_element
                    .pattern()
                    .ok()?
                    .as_any_js_assignment()
                    .and_then(|assignment| {
                        extract_static_member_assignment_name(assignment, this_aliases)
                    })
            } else {
                None
            }
        })
        .collect()
}

/// Collects assignment names from a JavaScript object assignment pattern, e.g. `{...this.#value}`.
fn collect_js_object_assignment_pattern_names(
    assignment: &JsObjectAssignmentPattern,
    this_aliases: &[ThisAliasesAndTheirScope],
) -> Vec<Text> {
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
                return extract_static_member_assignment_name(
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
                return extract_static_member_assignment_name(
                    property.pattern().ok()?.as_any_js_assignment()?,
                    this_aliases,
                );
            }
            None
        })
        .collect()
}

/// Removes leading whitespace from `#privateProperty` names. Without this, the name might include
/// unwanted whitespace (e.g., "\n #privateProperty"). This ensures that when adding modifiers like
/// `readonly`, they are appended correctly without being affected by the whitespace.
fn extract_property_member_name_trimmed_whitespace(
    member_name: AnyJsClassMemberName,
) -> Option<AnyJsClassMemberName> {
    match member_name {
        AnyJsClassMemberName::JsPrivateClassMemberName(name) => {
            let hash_token = name.hash_token().ok()?;
            let new_hash_token = hash_token.with_leading_trivia([]);
            let trimmed = name.replace_token_discard_trivia(hash_token, new_hash_token)?;

            Some(AnyJsClassMemberName::JsPrivateClassMemberName(trimmed))
        }
        _ => Some(member_name),
    }
}

/// Determines if a TypeScript property parameter is mutable (not marked as readonly).
/// Optionally checks if it is private when the `private_only` flag is set to true.
fn is_non_readonly_and_optionally_private(param: &TsPropertyParameter, private_only: bool) -> bool {
    let is_mutable = param
        .modifiers()
        .into_iter()
        .all(|any_modifier| TsReadonlyModifier::cast(any_modifier.into_syntax()).is_none());

    let is_private = param.modifiers().iter().any(|modifier| {
        modifier
            .as_ts_accessibility_modifier()
            .is_some_and(|modifier| modifier.is_private())
    });

    is_mutable && (!private_only || is_private)
}

/// Extracts the name of a static member assignment from an AnyJsAssignment node.
/// Checks for this or static references, casts to a static member assignment, and retrieves the trimmed name (public or private).
fn extract_static_member_assignment_name(
    operand: &AnyJsAssignment,
    this_aliases: &[ThisAliasesAndTheirScope],
) -> Option<Text> {
    operand
        .as_js_static_member_assignment()
        .and_then(|assignment| {
            if contains_this_or_static_member_kind(assignment, this_aliases) {
                assignment.member().ok().and_then(|member| {
                    member
                        .as_js_name()
                        .map(|name| name.to_trimmed_text())
                        .or_else(|| {
                            member
                                .as_js_private_name()
                                .map(|private_name| private_name.to_trimmed_text())
                        })
                })
            } else {
                None
            }
        })
}

/// Extracts the range and text from a property class member or constructor parameter
fn extract_range_and_text(property_or_param: &PropOrParam) -> Option<TextAndRange> {
    if let Some(PropOrParam::JsPropertyClassMember(member)) =
        PropOrParam::cast(property_or_param.clone().into())
    {
        if let Ok(member_name) = member.name() {
            return Some(TextAndRange {
                text: member_name.to_trimmed_text(),
                range: member_name.range(),
            });
        }
        return None;
    }

    if let Some(PropOrParam::TsPropertyParameter(parameter)) =
        PropOrParam::cast(property_or_param.clone().into())
    {
        let name = parameter
            .formal_parameter()
            .ok()?
            .as_js_formal_parameter()?
            .binding()
            .ok()?;

        return Some(TextAndRange {
            text: name.to_trimmed_text(),
            range: name.range(),
        });
    }

    None
}

/// Extracts all mutations of class member props within function bodies found in CONSTRUCTOR only:
/// expression statements (or so called IIFE),
/// nested classes methods,
/// or inner functions
fn collect_class_member_props_mutations_in_constructor(
    constructor_body: &JsFunctionBody,
) -> Vec<Text> {
    let this_variable_aliases: Vec<_> =
        collect_this_variable_aliases_in_immediate_body_closure(constructor_body);

    let all_descendants_fn_bodies_and_this_aliases: Vec<_> =
        collect_descendants_of_body_this_aliases(
            &MethodBodyElementOrStatementList::from(constructor_body.clone()),
            &this_variable_aliases,
        );

    all_descendants_fn_bodies_and_this_aliases
        .iter()
        .flat_map(|this_aliases_and_their_scope| {
            let mut names = Vec::new();

            visit_fn_body_descendants(
                &MethodBodyElementOrStatementList::from(this_aliases_and_their_scope.scope.clone()),
                std::slice::from_ref(this_aliases_and_their_scope),
                &mut |name| {
                    names.push(name);
                },
            );

            names
        })
        .collect::<Vec<_>>()
}

/// Extracts all aliases of `this` variable in the immediate body closure and keeps the body for checking scope.
fn collect_fn_body_this_aliases(body: &JsFunctionBody) -> Vec<ThisAliasesAndTheirScope> {
    let this_variable_aliases: Vec<_> =
        collect_this_variable_aliases_in_immediate_body_closure(body);
    collect_descendants_of_body_this_aliases(
        &MethodBodyElementOrStatementList::from(body.clone()),
        &this_variable_aliases,
    )
}

/// Finds recursively function bodies in a syntax node AND collects all this aliases applicable to the current fn body.
/// e.g. var self = this; var another_self = this; ends up with this_aliases: [self, another_self]
/// Only collects aliases that are not directly owned by a constructor, as those are not relevant for the current scope.
fn collect_descendants_of_body_this_aliases(
    method_body_element_or_statement_list: &MethodBodyElementOrStatementList,
    parent_this_aliases: &[Text],
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
            let current_scope_aliases =
                collect_this_variable_aliases_in_immediate_body_closure(&body);
            let mut this_aliases = Vec::new();
            this_aliases.extend_from_slice(parent_this_aliases);
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
                update_fn_body_and_aliases(parent_this_aliases, &mut results, &body);
            }
        }
        // Check function expressions
        else if let Some(func_expr) = JsFunctionExpression::cast(child.clone()) {
            if let Ok(body) = func_expr.body() {
                update_fn_body_and_aliases(parent_this_aliases, &mut results, &body);
            }
        }
        // Check arrow functions with block bodies
        else if let Some(arrow_func) = JsArrowFunctionExpression::cast(child.clone()) {
            if let Ok(body) = arrow_func.body() {
                if let Some(block) = body.as_any_js_expression() {
                    if let Some(body) = JsFunctionBody::cast(block.syntax().clone()) {
                        update_fn_body_and_aliases(parent_this_aliases, &mut results, &body);
                    }
                }
            }
        }
        // Check method definitions
        else if let Some(method) = JsMethodObjectMember::cast(child.clone()) {
            if let Ok(body) = method.body() {
                update_fn_body_and_aliases(parent_this_aliases, &mut results, &body);
            }
        }
        // Recurse for other node types and append their results
        else if let Some(child) = MethodBodyElementOrStatementList::cast_ref(&child) {
            results.extend(collect_descendants_of_body_this_aliases(
                &child,
                parent_this_aliases,
            ));
        }
    }

    results
}

/// Update results with current scope aliases if found and returns the updated FnBodyAndThisAliases struct
fn update_fn_body_and_aliases(
    parent_this_aliases: &[Text],
    results: &mut Vec<ThisAliasesAndTheirScope>,
    body: &JsFunctionBody,
) {
    let current_scope_aliases = collect_this_variable_aliases_in_immediate_body_closure(body);
    let mut this_aliases = Vec::new();
    this_aliases.extend_from_slice(parent_this_aliases);
    this_aliases.extend(current_scope_aliases.clone());

    results.push(ThisAliasesAndTheirScope {
        scope: body.clone(),
        aliases: this_aliases,
    });
}

/// Process a js function body to find all reassignments/ aliases of this.
/// It only processes the top level of the function body scope
/// # Example
/// ``` js
/// var self = this;
/// const parent = this;
/// ```
/// produces vec![Text(self), Text(parent)]
fn collect_this_variable_aliases_in_immediate_body_closure(body: &JsFunctionBody) -> Vec<Text> {
    body.statements()
        .iter()
        .filter_map(|node| node.as_js_variable_statement().cloned())
        .filter_map(|stmt| stmt.declaration().ok().map(|decl| decl.declarators()))
        .flat_map(|declarators| {
            declarators
                .into_iter()
                .filter_map(|declaration| declaration.ok().map(|declarator| declarator.as_fields()))
        })
        .filter_map(|fields| {
            let id = fields.id.ok()?;
            let expr = fields.initializer?.expression().ok()?;

            (expr.syntax().first_token()?.text() == "this").then(|| id.to_trimmed_text().clone())
        })
        .collect()
}
