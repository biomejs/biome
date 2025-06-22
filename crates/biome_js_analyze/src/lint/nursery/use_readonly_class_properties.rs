use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsClassMember, AnyJsClassMemberName, AnyJsConstructorParameter,
    AnyJsPropertyModifier, AnyTsPropertyParameterModifier, JsArrayAssignmentPattern,
    JsArrowFunctionExpression, JsAssignmentExpression, JsClassDeclaration, JsClassMemberList,
    JsConstructorClassMember, JsExpressionStatement, JsFunctionBody, JsFunctionExpression,
    JsLanguage, JsMethodClassMember, JsMethodObjectMember, JsObjectAssignmentPattern,
    JsParenthesizedExpression, JsPostUpdateExpression, JsPreUpdateExpression,
    JsPropertyClassMember, JsReturnStatement, JsSetterClassMember, JsStatementList,
    JsStaticMemberAssignment, JsStaticMemberExpression, JsSyntaxKind, JsSyntaxToken,
    JsThisExpression, TextRange, TsAccessibilityModifier, TsPropertyParameter, TsReadonlyModifier,
};
use biome_rowan::{
    AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxNode, Text,
    TriviaPiece, declare_node_union,
};
use serde::{Deserialize, Serialize};
use std::iter::once;

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
        version: "next",
        name: "useReadonlyClassProperties",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-readonly")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseReadonlyClassProperties {
    type Query = Ast<JsClassDeclaration>;
    type State = PropOrParam;
    type Signals = Box<[Self::State]>;
    type Options = ReadonlyClassPropertiesOptions;

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

/// Rule's options
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct ReadonlyClassPropertiesOptions {
    /// When `true`, the keywords `public`, `protected`, and `private` are analyzed by the rule.
    #[serde(default, skip_serializing_if = "is_default")]
    pub check_all_properties: bool,
}

fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

declare_node_union! {
    pub AnyThisMemberLike = JsThisExpression | JsStaticMemberExpression
}

declare_node_union! {
    pub PropOrParam = JsPropertyClassMember | TsPropertyParameter
}

declare_node_union! {
    pub AnyJsClassMethodBodyElement = JsMethodClassMember | JsSetterClassMember | JsFunctionBody |
    JsReturnStatement | JsExpressionStatement | JsParenthesizedExpression
}

enum MethodBodyElementOrStatementList<'a> {
    MethodBodyElement(&'a AnyJsClassMethodBodyElement),
    StatementList(&'a JsStatementList),
}

impl<'a> MethodBodyElementOrStatementList<'a> {
    pub fn syntax(&self) -> &SyntaxNode<JsLanguage> {
        match self {
            MethodBodyElementOrStatementList::MethodBodyElement(node) => node.syntax(),
            MethodBodyElementOrStatementList::StatementList(list) => list.syntax(),
        }
    }
}

#[derive(Debug)]
struct TextAndRange {
    text: Text,
    range: TextRange,
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
fn collect_mutated_class_property_names(members: &JsClassMemberList) -> Vec<Text> {
    members
        .iter()
        .filter_map(|member| match member {
            // assignments in class methods
            AnyJsClassMember::JsMethodClassMember(method) => {
                if let Ok(body) = method.body() {
                    let this_aliases = collect_class_member_props_mutations(&body);
                    Some(
                        collect_all_assignment_names(
                            &MethodBodyElementOrStatementList::MethodBodyElement(
                                &AnyJsClassMethodBodyElement::from(method.clone()),
                            ),
                            &this_aliases,
                        )
                        .collect::<Vec<_>>()
                        .into_iter(),
                    )
                } else {
                    None
                }
            }
            // assignments in setters
            AnyJsClassMember::JsSetterClassMember(setter) => {
                if let Ok(body) = setter.body() {
                    let this_aliases = collect_class_member_props_mutations(&body);
                    Some(
                        collect_all_assignment_names(
                            &MethodBodyElementOrStatementList::MethodBodyElement(
                                &AnyJsClassMethodBodyElement::from(setter.clone()),
                            ),
                            &this_aliases,
                        )
                        .collect::<Vec<_>>()
                        .into_iter(),
                    )
                } else {
                    None
                }
            }
            // assignments in constructor
            AnyJsClassMember::JsConstructorClassMember(constructor) => {
                if let Ok(body) = constructor.body() {
                    Some(collect_class_member_props_mutations(&body).into_iter())
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
    this_aliases: &[Text],
) -> bool {
    if let Ok(object) = assignment.object() {
        let js_this_assignment = object.as_js_this_expression();

        if js_this_assignment.is_some() {
            return true;
        }

        if let Some(js_identifier_expression) = object.as_js_identifier_expression() {
            if let Ok(name) = js_identifier_expression.name() {
                return this_aliases
                    .iter()
                    .any(|alias| alias.eq(&name.to_trimmed_text()));
            }
        }
    }

    false
}

/// Recursively traverses a class method or setter body (or related AST node)
/// and collects the names of all properties assigned to this (or its aliases) within that body.
/// It handles assignment expressions, update expressions, array/object destructuring, and nested method bodies,
/// returning an iterator of property names (Text) that are mutated.
/// This is used to detect which class properties are reassigned inside methods or setters.
fn collect_all_assignment_names(
    method_body_element: &MethodBodyElementOrStatementList,
    this_aliases: &[Text],
) -> impl Iterator<Item = Text> {
    method_body_element
        .syntax()
        .children()
        .flat_map(move |child| {
            // Try to handle assignment expressions
            if let Some(expr) = JsAssignmentExpression::cast_ref(&child) {
                if let Ok(left) = expr.left() {
                    // Handle array assignment pattern
                    if let Some(assignment) = left.as_js_array_assignment_pattern().cloned() {
                        return extract_js_array_assignment_pattern_names(
                            &assignment,
                            this_aliases,
                        )
                        .into_iter();
                    }

                    if let Some(assignment) = left.as_js_object_assignment_pattern().cloned() {
                        return collect_js_object_assignment_pattern_names(
                            &assignment,
                            this_aliases,
                        )
                        .into_iter();
                    }
                    // Handle regular assignment
                    if let Some(assignment) = left.as_any_js_assignment().cloned() {
                        if let Some(name) =
                            extract_static_member_assignment_name(&assignment, this_aliases)
                        {
                            return vec![name].into_iter();
                        }
                        return Vec::new().into_iter();
                    }
                }
            }

            // Handle update expressions (pre and post)
            let operand = JsPostUpdateExpression::cast(child.clone())
                .and_then(|expr| expr.operand().ok())
                .or_else(|| {
                    JsPreUpdateExpression::cast(child.clone()).and_then(|expr| expr.operand().ok())
                });

            if let Some(operand) = operand {
                if let Some(name) = extract_static_member_assignment_name(&operand, this_aliases) {
                    return vec![name].into_iter();
                }
                return Vec::new().into_iter();
            }

            if let Some(allowed_child) = AnyJsClassMethodBodyElement::cast_ref(&child) {
                return collect_all_assignment_names(
                    &MethodBodyElementOrStatementList::MethodBodyElement(&allowed_child),
                    this_aliases,
                )
                .collect::<Vec<_>>()
                .into_iter();
            }

            if let Some(statement_list) = JsStatementList::cast_ref(&child) {
                return collect_all_assignment_names(
                    &MethodBodyElementOrStatementList::StatementList(&statement_list),
                    this_aliases,
                )
                .collect::<Vec<_>>()
                .into_iter();
            }

            Vec::new().into_iter()
        })
}

/// Extracts the names of all properties assigned to this (or its aliases) within the array assignment pattern.
/// It handles both direct elements and rest elements (e.g., [this.prop, ...this.#private])
/// and extracts property names that are being assigned via destructuring.
/// This is useful for detecting which class properties are mutated through array destructuring assignments.
fn extract_js_array_assignment_pattern_names(
    array_assignment_pattern: &JsArrayAssignmentPattern,
    this_aliases: &[Text],
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
    this_aliases: &[Text],
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
    this_aliases: &[Text],
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

/// Extracts all mutations of class member props within function bodies found in constructor:
/// expression statements (or so called IIFE),
/// nested classes methods,
/// or inner functions
fn collect_class_member_props_mutations(body: &JsFunctionBody) -> Vec<Text> {
    let this_variable_aliases: Vec<_> =
        collect_this_variable_aliases_in_immediate_body_closure(body);

    let all_descendants_fn_bodies_and_this_aliases: Vec<_> =
        collect_nested_function_bodies_with_this_aliases(body.syntax(), &this_variable_aliases);

    all_descendants_fn_bodies_and_this_aliases
        .iter()
        .flat_map(|FnBodyAndThisAliases { body, this_aliases }| {
            collect_all_assignment_names(
                &MethodBodyElementOrStatementList::MethodBodyElement(
                    &AnyJsClassMethodBodyElement::from(body.clone()),
                ),
                this_aliases,
            )
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Debug)]
struct FnBodyAndThisAliases {
    body: JsFunctionBody,
    this_aliases: Vec<Text>,
}
/// Finds recursively function bodies in a syntax node AND collects all this aliases applicable to the current fn body.
/// e.g. var self = this; var another_self = this; ends up with this_aliases: [self, another_self]
/// Assumes to be within a single method or constructor only
fn collect_nested_function_bodies_with_this_aliases(
    node: &SyntaxNode<JsLanguage>,
    parent_this_aliases: &[Text],
) -> Vec<FnBodyAndThisAliases> {
    let mut results = Vec::new();

    // First check if this node itself is a function body
    if let Some(body) = JsFunctionBody::cast(node.clone()) {
        // Only add if it's not directly owned by a constructor
        if node
            .parent()
            .and_then(JsConstructorClassMember::cast)
            .is_none()
        {
            let current_scope_aliases =
                collect_this_variable_aliases_in_immediate_body_closure(&body);
            let mut this_aliases = Vec::new();
            this_aliases.extend_from_slice(parent_this_aliases);
            this_aliases.extend(current_scope_aliases.clone());

            results.push(FnBodyAndThisAliases {
                body: body.clone(),
                this_aliases,
            });
        }
    }

    // Collect function bodies from children
    for child in node.children() {
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
        else {
            results.extend(collect_nested_function_bodies_with_this_aliases(
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
    results: &mut Vec<FnBodyAndThisAliases>,
    body: &JsFunctionBody,
) {
    let current_scope_aliases = collect_this_variable_aliases_in_immediate_body_closure(body);
    let mut this_aliases = Vec::new();
    this_aliases.extend_from_slice(parent_this_aliases);
    this_aliases.extend(current_scope_aliases.clone());

    results.push(FnBodyAndThisAliases {
        body: body.clone(),
        this_aliases,
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
