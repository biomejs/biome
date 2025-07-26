use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::class_member_analyzer::{
    ClassMemberAnalyzer, ClassPropMemberOrConstructorTsParam, ClassPropertyMutation,
};
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsConstructorParameter, AnyJsPropertyModifier,
    AnyTsPropertyParameterModifier, JsClassDeclaration, JsClassMemberList, JsSyntaxKind,
    JsSyntaxToken, TextRange, TsAccessibilityModifier, TsPropertyParameter, TsReadonlyModifier,
};
use biome_rowan::{
    AstNode, AstNodeExt, AstNodeList, AstSeparatedList, BatchMutationExt, Text, TriviaPiece,
};
use biome_rule_options::use_readonly_class_properties::UseReadonlyClassPropertiesOptions;
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
    type State = ClassPropMemberOrConstructorTsParam;
    type Signals = Box<[Self::State]>;
    type Options = UseReadonlyClassPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.query();
        let members = root.members();
        let private_only = !ctx.options().check_all_properties;
        let mutated_class_property_names = ClassMemberAnalyzer::mutated_properties(&members);

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
                if mutated_class_property_names.clone().into_iter().any(
                    |ClassPropertyMutation { name, .. }| {
                        if let Some(TextAndRange { text, .. }) =
                            extract_property_or_param_range_and_text(&prop_or_param.clone())
                        {
                            return name.eq(&text);
                        }

                        false
                    },
                ) {
                    None
                } else {
                    Some(prop_or_param.clone())
                }
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let TextAndRange { text, range } = extract_property_or_param_range_and_text(&node.clone())?;

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

        if let Some(ClassPropMemberOrConstructorTsParam::JsPropertyClassMember(member)) =
            ClassPropMemberOrConstructorTsParam::cast(original_node.clone())
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
        } else if let Some(ClassPropMemberOrConstructorTsParam::TsPropertyParameter(parameter)) =
            ClassPropMemberOrConstructorTsParam::cast(original_node.clone())
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
) -> impl Iterator<Item = ClassPropMemberOrConstructorTsParam> {
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

        let some_property = Some(ClassPropMemberOrConstructorTsParam::JsPropertyClassMember(
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
) -> Vec<ClassPropMemberOrConstructorTsParam> {
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
                Some(ClassPropMemberOrConstructorTsParam::TsPropertyParameter(
                    ts_property,
                ))
            }
            _ => None,
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

/// Extracts the range and text from a property class member or constructor parameter
fn extract_property_or_param_range_and_text(
    property_or_param: &ClassPropMemberOrConstructorTsParam,
) -> Option<TextAndRange> {
    if let Some(ClassPropMemberOrConstructorTsParam::JsPropertyClassMember(member)) =
        ClassPropMemberOrConstructorTsParam::cast(property_or_param.clone().into())
    {
        if let Ok(member_name) = member.name() {
            return Some(TextAndRange {
                text: member_name.to_trimmed_text(),
                range: member_name.range(),
            });
        }
        return None;
    }

    if let Some(ClassPropMemberOrConstructorTsParam::TsPropertyParameter(parameter)) =
        ClassPropMemberOrConstructorTsParam::cast(property_or_param.clone().into())
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
