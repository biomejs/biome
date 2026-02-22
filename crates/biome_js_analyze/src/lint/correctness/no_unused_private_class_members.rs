use crate::JsRuleAction;
use crate::services::semantic_class::{
    AnyPropertyMember, ClassMemberReference, ClassMemberReferences, SemanticClass,
};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsFormalParameter, JsClassDeclaration,
    TsAccessibilityModifier, TsPropertyParameter,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Text, TextRange, declare_node_union,
};
use biome_rule_options::no_unused_private_class_members::NoUnusedPrivateClassMembersOptions;

declare_lint_rule! {
    /// Disallow unused private class members
    ///
    /// Private class members that are declared and not used anywhere in the code are most likely an error due to incomplete refactoring.
    /// Such class members take up space in the code and can lead to confusion by readers.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class OnlyWrite {
    ///   #usedOnlyInWrite = 5;
    ///
    ///   method() {
    ///        this.#usedOnlyInWrite = 212;
    ///   }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    ///  class TsBioo {
    ///    private unusedProperty = 5;
    ///  }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    ///  class TsBioo {
    ///    private unusedMethod() {}
    ///  }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class UsedMember {
    ///   #usedMember = 42;
    ///
    ///   method() {
    ///        return this.#usedMember;
    ///   }
    /// }
    /// ```
    ///
    /// ## Caveats
    ///
    /// The rule currently considers that all TypeScript private members are used if it encounters a computed access.
    /// In the following example `member` is not reported. It is considered as used.
    ///
    /// ```ts
    ///  class TsBioo {
    ///    private member: number;
    ///
    ///    set_with_name(name: string, value: number) {
    ///      this[name] = value;
    ///    }
    ///  }
    /// ```
    ///
    pub NoUnusedPrivateClassMembers {
        version: "1.3.3",
        name: "noUnusedPrivateClassMembers",
        language: "js",
        sources: &[RuleSource::Eslint("no-unused-private-class-members").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyMember = AnyJsClassMember | TsPropertyParameter
}

#[derive(Debug, Clone)]
pub enum UnusedMemberAction {
    RemoveMember(AnyMember),
}

impl UnusedMemberAction {
    fn property_range(&self) -> Option<TextRange> {
        match self {
            Self::RemoveMember(member) => member.property_range(),
        }
    }
}

impl Rule for NoUnusedPrivateClassMembers {
    type Query = SemanticClass<JsClassDeclaration>;
    type State = UnusedMemberAction;
    type Signals = Box<[Self::State]>;
    type Options = NoUnusedPrivateClassMembersOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let private_members: Vec<AnyMember> = get_all_declared_private_members(node).collect();
        if private_members.is_empty() {
            Box::default()
        } else {
            let mut results = Vec::new();
            let class_member_references = ctx.model.class_member_references(&node.members());
            let unused_members = traverse_members_usage(private_members, &class_member_references);

            for member in unused_members {
                results.push(UnusedMemberAction::RemoveMember(member));
            }
            results.into_boxed_slice()
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            UnusedMemberAction::RemoveMember(_) => Some(RuleDiagnostic::new(
                rule_category!(),
                state.property_range(),
                markup! {
                    "This private class member is defined but never used."
                },
            )),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        match state {
            UnusedMemberAction::RemoveMember(member) => {
                mutation.remove_node(member.clone());
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove unused declaration." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

/// Check for private member usage
/// if the member usage is found, we remove it from the hashmap
fn traverse_members_usage(
    private_members: Vec<AnyMember>,
    class_member_references: &ClassMemberReferences,
) -> Vec<AnyMember> {
    let ClassMemberReferences { writes, reads } = class_member_references;
    let all_references: Vec<&ClassMemberReference> = reads.iter().chain(writes.iter()).collect();

    private_members
        .into_iter()
        .filter_map(|private_member| {
            if !all_references
                .iter()
                .any(|reference| private_member.match_class_member_reference(reference))
            {
                Some(private_member)
            } else {
                None
            }
        })
        .collect()
}

fn get_all_declared_private_members(
    class_declaration: &JsClassDeclaration,
) -> impl Iterator<Item = AnyMember> {
    class_declaration
        .members()
        .iter()
        .map(AnyMember::AnyJsClassMember)
        .chain(get_constructor_params(class_declaration))
        .filter(|member| member.is_private() == Some(true))
}

fn get_constructor_params(
    class_declaration: &JsClassDeclaration,
) -> impl Iterator<Item = AnyMember> {
    class_declaration
        .members()
        .iter()
        .find_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(member) => Some(member),
            _ => None,
        })
        .and_then(|constructor_member| constructor_member.parameters().ok())
        .into_iter()
        .flat_map(|constructor_params| {
            constructor_params
                .parameters()
                .iter()
                .filter_map(|param| match param.ok()? {
                    biome_js_syntax::AnyJsConstructorParameter::TsPropertyParameter(
                        ts_property,
                    ) => Some(ts_property.into()),
                    _ => None,
                })
        })
}

impl AnyMember {
    fn is_private(&self) -> Option<bool> {
        match self {
            Self::AnyJsClassMember(member) => {
                let is_es_private = matches!(
                    member.name().ok()??,
                    AnyJsClassMemberName::JsPrivateClassMemberName(_)
                );
                let is_ts_private = match member {
                    AnyJsClassMember::JsGetterClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsMethodClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsPropertyClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    AnyJsClassMember::JsSetterClassMember(member) => member
                        .modifiers()
                        .iter()
                        .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                        .any(|accessibility| accessibility.is_private()),
                    _ => false,
                };

                Some(is_es_private || is_ts_private)
            }
            Self::TsPropertyParameter(param) => Some(
                param
                    .modifiers()
                    .iter()
                    .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                    .any(|accessibility| accessibility.is_private()),
            ),
        }
    }

    // this logic can also be pushed to the SemanticClass model if needed
    fn property_range(&self) -> Option<TextRange> {
        match self {
            Self::AnyJsClassMember(member) => match member {
                AnyJsClassMember::JsGetterClassMember(member) => Some(member.name().ok()?.range()),
                AnyJsClassMember::JsMethodClassMember(member) => Some(member.name().ok()?.range()),
                AnyJsClassMember::JsPropertyClassMember(member) => {
                    Some(member.name().ok()?.range())
                }
                AnyJsClassMember::JsSetterClassMember(member) => Some(member.name().ok()?.range()),
                _ => None,
            },
            Self::TsPropertyParameter(ts_property) => match ts_property.formal_parameter().ok()? {
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
            },
        }
    }

    fn match_class_member_reference(&self, class_member_reference: &ClassMemberReference) -> bool {
        let ClassMemberReference { name, .. } = class_member_reference;

        if let Some(prop_name) = extract_member_text(self) {
            return prop_name.eq(name);
        }

        false
    }
}

/// Extracts the text from a property class member or constructor parameter
fn extract_member_text(property_or_param: &AnyMember) -> Option<Text> {
    if let Some(AnyPropertyMember::JsPropertyClassMember(member)) =
        AnyPropertyMember::cast(property_or_param.clone().into())
    {
        if let Ok(member_name) = member.name() {
            return Some(member_name.to_trimmed_text());
        }
        return None;
    }

    if let Some(AnyPropertyMember::TsPropertyParameter(parameter)) =
        AnyPropertyMember::cast(property_or_param.clone().into())
    {
        let name = parameter
            .formal_parameter()
            .ok()?
            .as_js_formal_parameter()?
            .binding()
            .ok()?;

        return Some(name.to_trimmed_text());
    }
    None
}
