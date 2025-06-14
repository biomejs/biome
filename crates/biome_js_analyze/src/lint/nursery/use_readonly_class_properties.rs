use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsClassMember, AnyJsClassMemberName, AnyJsConstructorParameter,
    AnyJsPropertyModifier, AnyTsPropertyParameterModifier, JsAssignmentExpression,
    JsClassDeclaration, JsClassMemberList, JsLanguage, JsPostUpdateExpression,
    JsPreUpdateExpression, JsPropertyClassMember, JsStaticMemberExpression, JsSyntaxKind,
    JsSyntaxToken, JsThisExpression, TsAccessibilityModifier, TsPropertyParameter,
    TsReadonlyModifier,
};
use biome_rowan::{
    AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxNode, Text,
    declare_node_union,
};
use serde::{Deserialize, Serialize};

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
    /// This rule supports the following option:
    /// - `checkAllProperties` - Checks whether all class properties (including public and protected) should be analyzed. Defaults to false (only private).
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
        version: "2.0.0",
        name: "useReadonlyClassProperties",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-readonly")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseReadonlyClassProperties {
    type Query = Ast<JsClassDeclaration>;
    type State = PropertyClassMemberOrConstructorParam;
    type Signals = Box<[Self::State]>;
    type Options = ReadonlyClassPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let members = root.members();
        let private_only = !ctx.options().check_all_properties;
        let mut assignment_names = collect_all_class_property_updates(&members);

        let constructor_params: Vec<_> =
            filter_mutable_constructor_parameters(root, private_only).collect();

        let mutable_class_property_members = members
            .iter()
            .filter_map(|member| filter_mutable_non_static_class_property(&member, private_only));

        constructor_params
            .clone()
            .into_iter()
            .chain(
                mutable_class_property_members.filter(|class_property_member| {
                    !constructor_params.clone().into_iter().any(|node| {
                        node.to_trimmed_text() == class_property_member.to_trimmed_text()
                    })
                }),
            )
            .filter_map(|range_and_name| {
                if assignment_names.any(|name| name.eq(&range_and_name.to_trimmed_text())) {
                    None
                } else {
                    Some(range_and_name)
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let text_range = node.range();
        let text = node.to_trimmed_text();
        Some(RuleDiagnostic::new(
            rule_category!(),
            text_range,
            markup! {
                "Member '"{text.text()}"' is never reassigned. Mark it as "<Emphasis>"readonly"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, node: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let original_node = node.syntax();
        let readonly_token = make::ts_readonly_modifier(JsSyntaxToken::new_detached(
            JsSyntaxKind::TS_READONLY_MODIFIER,
            "readonly ",
            [],
            [],
        ));

        if let Some(PropertyClassMemberOrConstructorParam::JsPropertyClassMember(member)) =
            PropertyClassMemberOrConstructorParam::cast(original_node.clone())
        {
            if let Ok(member_name) = member.name() {
                let replace_modifiers = make::js_property_modifier_list(
                    member
                        .modifiers()
                        .iter()
                        .chain(std::iter::once(AnyJsPropertyModifier::TsReadonlyModifier(
                            readonly_token,
                        )))
                        .collect::<Vec<_>>(),
                );

                if let Some(modified_member) =
                    get_property_member_name_trimmed_whitespace(member_name.clone())
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
        } else if let Some(PropertyClassMemberOrConstructorParam::TsPropertyParameter(parameter)) =
            PropertyClassMemberOrConstructorParam::cast(original_node.clone())
        {
            let replace_modifiers = make::ts_property_parameter_modifier_list(
                parameter
                    .modifiers()
                    .iter()
                    .chain(std::iter::once(
                        AnyTsPropertyParameterModifier::TsReadonlyModifier(readonly_token),
                    ))
                    .collect::<Vec<_>>(),
            );

            let replace_parameter = make::ts_property_parameter(
                parameter.decorators(),
                replace_modifiers,
                parameter.formal_parameter().expect("REASON"),
            );

            mutation.replace_node_discard_trivia(parameter.clone(), replace_parameter);
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add `readonly` decorator." }.to_owned(),
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
    pub PropertyClassMemberOrConstructorParam = JsPropertyClassMember | TsPropertyParameter
}

/// Filters and returns mutable class properties (excluding `static`), optionally restricted to `private` ones, from a given class member.
fn filter_mutable_non_static_class_property(
    member: &AnyJsClassMember,
    private_only: bool,
) -> Option<PropertyClassMemberOrConstructorParam> {
    let property_class_member = member.as_js_property_class_member()?;

    if property_class_member.modifiers().iter().any(|modifier| {
        modifier.as_ts_readonly_modifier().is_some() || modifier.as_js_static_modifier().is_some()
    }) {
        return None;
    }

    let some_property = Some(
        PropertyClassMemberOrConstructorParam::JsPropertyClassMember(property_class_member.clone()),
    );

    if !private_only {
        return some_property;
    }

    let is_private = matches!(
        member.name().ok()??,
        AnyJsClassMemberName::JsPrivateClassMemberName(_)
    ) || property_class_member.modifiers().iter().any(|x| {
        TsAccessibilityModifier::cast(x.into_syntax()).is_some_and(|modifier| modifier.is_private())
    });

    if is_private {
        return some_property;
    }
    None
}

/// Filters and returns mutable constructor parameters, optionally restricted to private ones, from a given class declaration.
fn filter_mutable_constructor_parameters(
    class_declaration: &JsClassDeclaration,
    private_only: bool,
) -> impl Iterator<Item = PropertyClassMemberOrConstructorParam> {
    class_declaration
        .members()
        .iter()
        .find_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(member) => Some(member),
            _ => None,
        })
        .into_iter()
        .flat_map(|constructor| constructor.parameters().ok())
        .flat_map(|constructor_params| constructor_params.parameters().iter())
        .filter_map(move |param| match param.ok()? {
            AnyJsConstructorParameter::TsPropertyParameter(ts_property)
                if is_mutable_and_optionally_private(&ts_property, private_only) =>
            {
                Some(PropertyClassMemberOrConstructorParam::TsPropertyParameter(
                    ts_property,
                ))
            }
            _ => None,
        })
}

/// Collects all property reassignments within class method members or setters,
/// including assignments and updates to this or static properties.
fn collect_all_class_property_updates(members: &JsClassMemberList) -> impl Iterator<Item = Text> {
    members
        .iter()
        .filter_map(|member| match member {
            AnyJsClassMember::JsMethodClassMember(_) | AnyJsClassMember::JsSetterClassMember(_) => {
                Some(
                    find_all_assignment_names(member.syntax())
                        .collect::<Vec<_>>() // Collect into a Vec to own the data
                        .into_iter(),
                )
            }
            AnyJsClassMember::JsConstructorClassMember(_) => {
                println!("constructor, deal self executing immediate fn");
                None
            }
            _ => None,
        })
        .flatten()
}

/// Recursively checks if a syntax node contains references to `this` (e.g., `this.privateProp`)
/// or static member access (e.g., `ClassName.prototype.privateProp`).
fn contains_this_or_static_member_kind(assignment: &SyntaxNode<JsLanguage>) -> bool {
    for child in assignment.children() {
        if AnyThisMemberLike::can_cast(child.kind()) || contains_this_or_static_member_kind(&child)
        {
            return true;
        }
    }
    false
}

/// Recursively collects all assignment expressions and update expressions (e.g., = or ++) within a given syntax node.
fn find_all_assignment_names(node: &SyntaxNode<JsLanguage>) -> impl Iterator<Item = Text> + '_ {
    node.children().flat_map(|child| {
        if let Some(assignment) = JsAssignmentExpression::cast(child.clone())
            .and_then(|expr| expr.left().ok()?.as_any_js_assignment().cloned())
            .or_else(|| {
                JsPostUpdateExpression::cast(child.clone()).and_then(|expr| expr.operand().ok())
            })
            .or_else(|| {
                JsPreUpdateExpression::cast(child.clone()).and_then(|expr| expr.operand().ok())
            })
        {
            std::iter::once(assignment)
                .filter_map(|operand| extract_static_member_assignment_name(&operand))
                .collect::<Vec<_>>()
                .into_iter()
        } else {
            find_all_assignment_names(&child)
                .collect::<Vec<_>>()
                .into_iter()
        }
    })
}

/// Removes leading whitespace from `#privateProperty` names. Without this, the name might include
/// unwanted whitespace (e.g., "\n #privateProperty"). This ensures that when adding modifiers like
/// `readonly`, they are appended correctly without being affected by the whitespace.
fn get_property_member_name_trimmed_whitespace(
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
fn is_mutable_and_optionally_private(param: &TsPropertyParameter, private_only: bool) -> bool {
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

///Extracts the name of a static member assignment from an AnyJsAssignment node.
/// Checks for this or static references, casts to a static member assignment, and retrieves the trimmed name (public or private).
fn extract_static_member_assignment_name(operand: &AnyJsAssignment) -> Option<Text> {
    if contains_this_or_static_member_kind(operand.syntax()) {
        operand
            .as_js_static_member_assignment()
            .and_then(|assignment| {
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
            })
    } else {
        None
    }
}
