use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsClassMember, AnyJsClassMemberName,
    AnyJsConstructorParameter, AnyJsFormalParameter, AnyJsPropertyModifier,
    AnyTsPropertyParameterModifier, JsAssignmentExpression, JsClassDeclaration, JsLanguage,
    JsPropertyClassMember, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, TsPropertyParameter,
};
use biome_rowan::{
    declare_node_union, syntax::SyntaxTrivia, AstNode, AstNodeExt, AstNodeList, AstSeparatedList,
    BatchMutationExt, SyntaxElement, TextRange, TriviaPiece, WalkEvent,
};
use rustc_hash::FxHashSet;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_lint_rule! {
    /// Require private members to be marked as `readonly` if they're never modified outside of the constructor
    ///
    /// Private member variables (whether using the private modifier or private # fields) are never permitted
    /// to be modified outside of their declaring class. If that class never modifies their value,
    /// they may safely be marked as readonly.
    ///
    /// This rule reports on private members are marked as readonly
    /// if they're never modified outside of the constructor.
    ///
    /// ## Examples
    ///
    /// ```typescript
    /// class Container {
    ///     // These member variables could be marked as readonly
    ///     private neverModifiedMember = true;
    ///     private onlyModifiedInConstructor: number;
    ///     #neverModifiedPrivateField = 3;
    ///
    ///     public constructor(
    ///         onlyModifiedInConstructor: number,
    ///         // Private parameter properties can also be marked as readonly
    ///         private neverModifiedParameter: string,
    ///     ) {
    ///         this.onlyModifiedInConstructor = onlyModifiedInConstructor;
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// This rule accepts the following options:
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "checkAllProperties": false
    ///     }
    /// }
    /// ```
    ///
    /// ### checkAllProperties
    ///
    /// Check on all properties (`public` and `protected` properties). Default: false.
    ///
    pub UseReadonlyClassProperties {
        version: "1.0.0",
        name: "useReadonlyClassProperties",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-readonly")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyClassProperties = JsPropertyClassMember | TsPropertyParameter
}

impl Rule for UseReadonlyClassProperties {
    type Query = Ast<JsClassDeclaration>;
    type State = AnyClassProperties;
    type Signals = Box<[Self::State]>;
    type Options = ReadonlyClassPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let class_declaration = ctx.query();
        let options = ctx.options();
        let eligible_properties =
            get_eligible_properties(class_declaration, !options.check_all_properties);
        if eligible_properties.is_empty() {
            Vec::new()
        } else {
            find_properties_need_add_readonly(class_declaration.syntax(), eligible_properties)
        }
        .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut name = String::new();
        if let Some(token) = state.text_trimmed() {
            name = token;
        }
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            markup! {
                "Member '"{name}"' is never reassigned; mark it as `readonly`."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.replace_element_discard_trivia(state.into_syntax(), state.replace_syntax());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add `readonly` decorator." }.to_owned(),
            mutation,
        ))
    }
}

/// Rule's options.
#[derive(
    Default, Debug, Clone, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReadonlyClassPropertiesOptions {
    /// If `true`, then check all class properties member
    ///
    /// If `false`, then only check private members
    #[serde(default, skip_serializing_if = "is_default")]
    pub check_all_properties: bool,
}

fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}

fn get_constructor_eligible_params(
    class_declaration: &JsClassDeclaration,
    only_private: bool,
) -> FxHashSet<AnyClassProperties> {
    let constructor_member =
        class_declaration
            .members()
            .into_iter()
            .find_map(|member| match member {
                AnyJsClassMember::JsConstructorClassMember(constructor) => Some(constructor),
                _ => None,
            });

    if let Some(constructor_member) = constructor_member {
        if let Ok(constructor_params) = constructor_member.parameters() {
            return constructor_params
                .parameters()
                .iter()
                .filter_map(|parameter| match parameter.ok()? {
                    AnyJsConstructorParameter::TsPropertyParameter(property_parameter) => {
                        let mut eligible = false;
                        let modifiers = property_parameter.modifiers();
                        modifiers.iter().for_each(|modifier| match modifier {
                            AnyTsPropertyParameterModifier::TsAccessibilityModifier(
                                accessibility_modifier,
                            ) => {
                                eligible = !only_private || accessibility_modifier.is_private();
                            }
                            AnyTsPropertyParameterModifier::TsReadonlyModifier(_) => {
                                eligible = false; // self has `readonly` modifier ignore it
                            }
                            _ => {}
                        });

                        if eligible {
                            return Some(property_parameter.into());
                        }

                        None
                    }
                    _ => None,
                })
                .collect();
        }
    }

    FxHashSet::default()
}

fn get_eligible_property(
    property_class_member: &JsPropertyClassMember,
    only_private: bool,
) -> bool {
    let modifiers = property_class_member.modifiers();

    if modifiers.is_empty() {
        if !only_private {
            return true;
        }
        let class_member_name = property_class_member.name();
        if let Ok(AnyJsClassMemberName::JsPrivateClassMemberName(_)) = class_member_name {
            return true;
        }
        return false;
    }

    let mut eligible = false;
    modifiers.iter().for_each(|modifier| match modifier {
        AnyJsPropertyModifier::TsAccessibilityModifier(accessibility_modifier) => {
            eligible = !only_private || accessibility_modifier.is_private();
        }
        AnyJsPropertyModifier::TsReadonlyModifier(_) => {
            eligible = false;
        }
        _ => {}
    });

    eligible
}

fn get_eligible_properties(
    class_declaration: &JsClassDeclaration,
    only_private: bool,
) -> FxHashSet<AnyClassProperties> {
    class_declaration
        .members()
        .iter()
        .filter_map(|class_member| match class_member {
            AnyJsClassMember::JsPropertyClassMember(property_class_member) => {
                let eligible = get_eligible_property(&property_class_member, only_private);
                if eligible {
                    return Some(property_class_member.into());
                }
                None
            }
            _ => None,
        })
        .chain(get_constructor_eligible_params(
            class_declaration,
            only_private,
        ))
        .collect()
}

fn get_property_name(assignment: AnyJsAssignment) -> Option<String> {
    match assignment {
        AnyJsAssignment::JsStaticMemberAssignment(static_member_assignment) => {
            let member = static_member_assignment.member();
            if let Ok(member) = member {
                let value_token = member.value_token();
                if let Ok(value_token) = value_token {
                    let mut name = String::from(value_token.text_trimmed());
                    if member.as_js_private_name().is_some() {
                        name = format!("#{}", name);
                    }
                    return Some(name);
                }
            }
            None
        }
        _ => None,
    }
}

fn find_properties_need_add_readonly(
    syntax: &JsSyntaxNode,
    mut properties: FxHashSet<AnyClassProperties>,
) -> Vec<AnyClassProperties> {
    let mut constructor_member = false;
    let mut changed_properties: FxHashSet<String> = FxHashSet::default();
    let events = syntax.preorder();

    for event in events {
        match event {
            WalkEvent::Enter(syntax_node) => match syntax_node.kind() {
                JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => constructor_member = true,
                JsSyntaxKind::JS_METHOD_CLASS_MEMBER => constructor_member = false,
                JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                    if constructor_member {
                        continue; // skip constructor assignment
                    }
                    let assignment_expression =
                        JsAssignmentExpression::unwrap_cast(syntax_node.clone());
                    let assignment_pattern = assignment_expression.left();
                    if let Ok(AnyJsAssignmentPattern::AnyJsAssignment(assignment)) =
                        assignment_pattern
                    {
                        let name = get_property_name(assignment);
                        if let Some(name) = name {
                            changed_properties.insert(name);
                        }
                    }
                }
                _ => {}
            },
            WalkEvent::Leave(_) => {}
        }
    }

    if !changed_properties.is_empty() {
        properties.retain(|property| {
            let name = property.text_trimmed();
            if let Some(name) = name {
                return !changed_properties.contains(&name);
            }
            true
        });
    }

    properties.into_iter().collect()
}

fn get_replace_class_member_name(
    class_member_name: AnyJsClassMemberName,
) -> Option<(AnyJsClassMemberName, SyntaxTrivia<JsLanguage>)> {
    match class_member_name {
        AnyJsClassMemberName::JsComputedMemberName(_) => None,
        AnyJsClassMemberName::JsMetavariable(_) => None,
        AnyJsClassMemberName::JsLiteralMemberName(literal_member_name) => {
            let value_token = literal_member_name.value();
            if let Ok(value_token) = value_token {
                let leading = value_token.leading_trivia();
                let replace_value_token = value_token.with_leading_trivia([]);
                let replace_class_member_name = literal_member_name
                    .replace_token_discard_trivia(value_token, replace_value_token);
                if let Some(replace_class_member_name) = replace_class_member_name {
                    return Some((
                        AnyJsClassMemberName::JsLiteralMemberName(replace_class_member_name),
                        leading,
                    ));
                }
            }
            None
        }
        AnyJsClassMemberName::JsPrivateClassMemberName(private_class_member_name) => {
            let hash_token = private_class_member_name.hash_token();
            if let Ok(hash_token) = hash_token {
                let leading = hash_token.leading_trivia();
                let replace_hash_token = hash_token.with_leading_trivia([]);
                let replace_class_member_name = private_class_member_name
                    .replace_token_discard_trivia(hash_token, replace_hash_token);
                if let Some(replace_class_member_name) = replace_class_member_name {
                    return Some((
                        AnyJsClassMemberName::JsPrivateClassMemberName(replace_class_member_name),
                        leading,
                    ));
                }
            }
            None
        }
    }
}

impl AnyClassProperties {
    pub fn text_trimmed(&self) -> Option<String> {
        match self {
            AnyClassProperties::JsPropertyClassMember(property_class_member) => {
                let class_member_name = property_class_member.name();
                if let Ok(class_member_name) = class_member_name {
                    let member_name = class_member_name.name();
                    if let Some(member_name) = member_name {
                        let mut name = String::from(member_name.text());
                        if class_member_name
                            .as_js_private_class_member_name()
                            .is_some()
                        {
                            name = format!("#{}", name);
                        }
                        return Some(name);
                    }
                }
                None
            }
            AnyClassProperties::TsPropertyParameter(property_parameter) => {
                match property_parameter.formal_parameter().ok()? {
                    AnyJsFormalParameter::JsFormalParameter(formal_parameter) => Some(
                        formal_parameter
                            .binding()
                            .ok()?
                            .as_any_js_binding()?
                            .as_js_identifier_binding()?
                            .name_token()
                            .ok()?
                            .text_trimmed()
                            .to_string(),
                    ),
                    _ => None,
                }
            }
        }
    }

    pub fn range(&self) -> Option<TextRange> {
        match self {
            AnyClassProperties::JsPropertyClassMember(property_class_member) => {
                Some(property_class_member.name().ok()?.range())
            }
            AnyClassProperties::TsPropertyParameter(property_parameter) => {
                match property_parameter.formal_parameter().ok()? {
                    AnyJsFormalParameter::JsBogusParameter(_)
                    | AnyJsFormalParameter::JsMetavariable(_) => None,
                    AnyJsFormalParameter::JsFormalParameter(param) => Some(
                        param
                            .binding()
                            .ok()?
                            .as_any_js_binding()?
                            .as_js_identifier_binding()?
                            .name_token()
                            .ok()?
                            .text_range(),
                    ),
                }
            }
        }
    }

    pub fn into_syntax(&self) -> SyntaxElement<JsLanguage> {
        let syntax = match self {
            AnyClassProperties::JsPropertyClassMember(property_class_member) => {
                property_class_member.clone().into_syntax()
            }
            AnyClassProperties::TsPropertyParameter(property_parameter) => {
                property_parameter.clone().into_syntax()
            }
        };
        SyntaxElement::Node(syntax)
    }

    pub fn replace_syntax(&self) -> SyntaxElement<JsLanguage> {
        match self {
            AnyClassProperties::JsPropertyClassMember(member) => {
                let class_member_name = member.name();
                if let Ok(class_member_name) = class_member_name {
                    let mut class_member_name = class_member_name;
                    let modifiers = member.modifiers();
                    let mut replace_modifiers: Vec<AnyJsPropertyModifier> = Vec::new();

                    let mut readonly_token = JsSyntaxToken::new_detached(
                        JsSyntaxKind::TS_READONLY_MODIFIER,
                        "readonly ",
                        [],
                        [TriviaPiece::whitespace(1)],
                    );
                    if modifiers.is_empty() {
                        let replace_items =
                            get_replace_class_member_name(class_member_name.clone());
                        if let Some((replace_class_member_name, replace_leading)) = replace_items {
                            class_member_name = replace_class_member_name;
                            let replace_readonly_token =
                                readonly_token.with_leading_trivia_pieces(replace_leading.pieces());
                            readonly_token = replace_readonly_token;
                        }
                    }

                    modifiers
                        .iter()
                        .for_each(|modifier| replace_modifiers.push(modifier));

                    replace_modifiers.push(AnyJsPropertyModifier::TsReadonlyModifier(
                        make::ts_readonly_modifier(readonly_token),
                    ));

                    let modifiers = make::js_property_modifier_list(replace_modifiers);
                    let mut builder = make::js_property_class_member(modifiers, class_member_name);

                    let property_annotation = member.property_annotation();
                    if let Some(property_annotation) = property_annotation {
                        builder = builder.with_property_annotation(property_annotation);
                    }
                    let semicolon_token = member.semicolon_token();
                    if let Some(semicolon_token) = semicolon_token {
                        builder = builder.with_semicolon_token(semicolon_token);
                    }
                    let value = member.value();
                    if let Some(value) = value {
                        builder = builder.with_value(value);
                    }

                    let replace_member = builder.build();
                    return SyntaxElement::Node(replace_member.into_syntax());
                }
                self.into_syntax()
            }
            AnyClassProperties::TsPropertyParameter(parameter) => {
                let formal_parameter = parameter.formal_parameter();
                if let Ok(formal_parameter) = formal_parameter {
                    let decorators = parameter.decorators();
                    let modifiers = parameter.modifiers();

                    let mut replace_modifiers: Vec<AnyTsPropertyParameterModifier> = Vec::new();
                    modifiers
                        .iter()
                        .for_each(|modifier| replace_modifiers.push(modifier));
                    replace_modifiers.push(AnyTsPropertyParameterModifier::TsReadonlyModifier(
                        make::ts_readonly_modifier(JsSyntaxToken::new_detached(
                            JsSyntaxKind::TS_READONLY_MODIFIER,
                            "readonly ",
                            [],
                            [TriviaPiece::whitespace(1)],
                        )),
                    ));

                    let modifiers = make::ts_property_parameter_modifier_list(replace_modifiers);
                    let replace_parameter =
                        make::ts_property_parameter(decorators, modifiers, formal_parameter);
                    return SyntaxElement::Node(replace_parameter.into_syntax());
                }
                self.into_syntax()
            }
        }
    }
}
