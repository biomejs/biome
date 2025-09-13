use std::collections::HashSet;
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
    JsSyntaxKind, TsAccessibilityModifier, TsPropertyParameter,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Text, TextRange,
    declare_node_union,
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
    RemovePrivateModifier {
        member: AnyMember,
        rename_with_underscore: bool,
    },
}

impl UnusedMemberAction {
    fn property_range(&self) -> Option<TextRange> {
        match self {
            Self::RemoveMember(member) => member.property_range(),
            Self::RemovePrivateModifier { member, .. } => member.property_range(),
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
                match &member {
                    AnyMember::AnyJsClassMember(_) => {
                        results.push(UnusedMemberAction::RemoveMember(member));
                    }
                    AnyMember::TsPropertyParameter(ts_property_param) => {
                        // Check if the parameter is also unused in constructor body using semantic analysis
                        let should_rename = check_ts_property_parameter_usage(ts_property_param);
                        results.push(UnusedMemberAction::RemovePrivateModifier {
                            member,
                            rename_with_underscore: should_rename,
                        });
                    }
                }
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
            UnusedMemberAction::RemovePrivateModifier {
                rename_with_underscore,
                ..
            } => {
                if *rename_with_underscore {
                    Some(RuleDiagnostic::new(
                        rule_category!(),
                        state.property_range(),
                        markup! {
                            "This private class member is defined but never used."
                        },
                    ))
                } else {
                    Some(RuleDiagnostic::new(
                        rule_category!(),
                        state.property_range(),
                        markup! {
                            "This parameter is never used outside of the constructor."
                        },
                    ))
                }
            }
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
            UnusedMemberAction::RemovePrivateModifier {
                member,
                rename_with_underscore,
            } => {
                if let AnyMember::TsPropertyParameter(ts_property_param) = member {
                    // Remove the private modifier
                    let modifiers = ts_property_param.modifiers();
                    for modifier in modifiers.iter() {
                        if let Some(accessibility_modifier) =
                            TsAccessibilityModifier::cast(modifier.into_syntax())
                            && accessibility_modifier.is_private()
                        {
                            mutation.remove_node(accessibility_modifier);
                            break;
                        }
                    }
                    // If needed, rename with underscore prefix
                    if *rename_with_underscore
                        && let Ok(AnyJsFormalParameter::JsFormalParameter(param)) =
                        ts_property_param.formal_parameter()
                    {
                        let binding = param.binding().ok()?;
                        let identifier_binding =
                            binding.as_any_js_binding()?.as_js_identifier_binding()?;
                        let name_token = identifier_binding.name_token().ok()?;
                        let name_trimmed = name_token.text_trimmed();
                        let _new_name = format!("_{name_trimmed}");
                        // if !mutation.rename_node_declaration(
                        //     ctx.model(),
                        //     identifier_binding,
                        //     &new_name,
                        // ) {
                        //     return None;
                        // }
                    }
                }
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove private modifier" }.to_owned(),
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
    let ClassMemberReferences { reads, writes } = class_member_references;

    private_members
        .into_iter()
        .filter(|private_member| {
            println!("reads: {reads:?}, writes: {writes:?}");
            let is_read = reads
                .iter()
                .any(|read| private_member.match_class_member_reference(read));
            let is_write = writes
                .iter()
                .any(|write| private_member.match_class_member_reference(write));

            if !is_read && !is_write {
                // No references at all
                return true;
            }

            // let is_write_only = !is_read && is_write && !private_member.is_accessor();
            let all_refs: HashSet<_> = reads.iter().chain(writes.iter()).collect();
            // todo add check if has update usage for writes too;
            let has_update_usage = all_refs.into_iter().any(|reference| {
                !is_in_expression_statement(reference)
                    && private_member.match_class_member_reference(reference)
            });
            if !has_update_usage {
                return true;
            } else {
                !(is_read || is_write)
            }
        })
        .collect()
}

/// Check if a TsPropertyParameter is also unused as a function parameter
fn check_ts_property_parameter_usage(ts_property_param: &TsPropertyParameter) -> bool {
    if let Ok(AnyJsFormalParameter::JsFormalParameter(param)) = ts_property_param.formal_parameter()
        && let Ok(binding) = param.binding()
        && let Some(identifier_binding) = binding
        .as_any_js_binding()
        .and_then(|b| b.as_js_identifier_binding())
    {
        let name_token = match identifier_binding.name_token() {
            Ok(token) => token,
            Err(_) => return false,
        };

        let name = name_token.text_trimmed();

        if name.starts_with('_') {
            return false;
        }

        return true;
    }

    false
}

fn get_all_declared_private_members(
    class_declaration: &JsClassDeclaration,
) -> impl Iterator<Item=AnyMember> {
    class_declaration
        .members()
        .iter()
        .map(AnyMember::AnyJsClassMember)
        .chain(get_constructor_params(class_declaration))
        .filter(|member| member.is_private() == Some(true))
}

fn get_constructor_params(
    class_declaration: &JsClassDeclaration,
) -> impl Iterator<Item=AnyMember> {
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

fn is_in_expression_statement(reference: &ClassMemberReference) -> bool {
    let maybe_kind = reference.parent_statement_kind.clone();
    if let Some(kind) = maybe_kind {
        return JsSyntaxKind::JS_EXPRESSION_STATEMENT == kind;
    }

    false
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

        if let Some(prop_name) = extract_property_or_param_range_and_text(self) {
            // println!("Comparing member '{:?}' with reference '{name}'", prop_name);
            return prop_name.eq(name);
        }

        false
    }
}

/// Extracts the text from a property class member or constructor parameter
fn extract_property_or_param_range_and_text(property_or_param: &AnyMember) -> Option<Text> {
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
