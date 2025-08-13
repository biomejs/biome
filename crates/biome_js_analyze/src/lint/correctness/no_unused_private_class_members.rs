use crate::JsRuleAction;
use crate::class_member_references::{
    ClassMemberReference, ClassMemberReferences, class_member_references,
};
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsFormalParameter, JsClassDeclaration, JsSyntaxKind,
    TsAccessibilityModifier, TsPropertyParameter,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Text, TextRange, declare_node_union,
};
use biome_rule_options::no_unused_private_class_members::NoUnusedPrivateClassMembersOptions;
use rustc_hash::FxHashSet;
use std::collections::HashSet;

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
    ///	    this.#usedOnlyInWrite = 212;
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
    ///	    return this.#usedMember;
    ///   }
    /// }
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

impl Rule for NoUnusedPrivateClassMembers {
    type Query = Ast<JsClassDeclaration>;
    type State = AnyMember;
    type Signals = Box<[Self::State]>;
    type Options = NoUnusedPrivateClassMembersOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let private_members = get_all_declared_private_members(node);

        let ClassMemberReferences { reads, writes } = class_member_references(&node.members());

        private_members
            .iter()
            .filter_map(|private_member| {
                let is_read = reads
                    .iter()
                    .any(|ClassMemberReference { name, .. }| private_member.match_js_name(name));

                let is_write = writes
                    .iter()
                    .any(|ClassMemberReference { name, .. }| private_member.match_js_name(name));

                let is_write_only = !is_read && is_write && private_member.is_accessor();

                if is_write_only {
                    return None;
                }

                if !is_read {
                    Some(private_member.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<AnyMember>>()
            .into()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.property_range(),
            markup! {
                "This private class member is defined but never used."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        mutation.remove_node(state.clone());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove unused declaration." }.to_owned(),
            mutation,
        ))
    }
}

fn get_all_declared_private_members(class_declaration: &JsClassDeclaration) -> HashSet<AnyMember> {
    class_declaration
        .members()
        .iter()
        .map(AnyMember::AnyJsClassMember)
        .chain(get_constructor_params(class_declaration))
        .filter(|member| member.is_private())
        .collect()
}

fn get_constructor_params(class_declaration: &JsClassDeclaration) -> FxHashSet<AnyMember> {
    let constructor_member = class_declaration
        .members()
        .iter()
        .find_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(member) => Some(member),
            _ => None,
        });

    if let Some(constructor_member) = constructor_member
        && let Ok(constructor_params) = constructor_member.parameters()
    {
        return constructor_params
            .parameters()
            .iter()
            .filter_map(|param| match param.ok()? {
                biome_js_syntax::AnyJsConstructorParameter::TsPropertyParameter(ts_property) => {
                    Some(ts_property.into())
                }
                _ => None,
            })
            .collect();
    }

    FxHashSet::default()
}

impl AnyMember {
    fn is_accessor(&self) -> bool {
        matches!(
            self.syntax().kind(),
            JsSyntaxKind::JS_SETTER_CLASS_MEMBER | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
        )
    }

    fn is_private(&self) -> bool {
        match self {
            Self::AnyJsClassMember(member) => {
                let name = member.name().ok().flatten();

                let is_es_private = matches!(
                    name,
                    Some(AnyJsClassMemberName::JsPrivateClassMemberName(_))
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

                is_es_private || is_ts_private
            }
            Self::TsPropertyParameter(param) => param
                .modifiers()
                .iter()
                .filter_map(|x| TsAccessibilityModifier::cast(x.into_syntax()))
                .any(|accessibility| accessibility.is_private()),
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

    fn match_js_name(&self, text: &Text) -> bool {
        let token = text.text().trim();
        let token = token.strip_prefix('#').unwrap_or(token);

        match self {
            Self::AnyJsClassMember(member) => match member {
                AnyJsClassMember::JsGetterClassMember(member) => member
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_some_and(|name| name.text().eq(token)),
                AnyJsClassMember::JsMethodClassMember(member) => member
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_some_and(|name| name.text().eq(token)),
                AnyJsClassMember::JsPropertyClassMember(member) => member
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_some_and(|name| name.text().eq(token)),
                AnyJsClassMember::JsSetterClassMember(member) => member
                    .name()
                    .ok()
                    .and_then(|n| n.name())
                    .is_some_and(|name| name.text().eq(token)),
                _ => false,
            },
            Self::TsPropertyParameter(ts_property) => ts_property
                .formal_parameter()
                .ok()
                .and_then(|param| match param {
                    AnyJsFormalParameter::JsBogusParameter(_)
                    | AnyJsFormalParameter::JsMetavariable(_) => None,
                    AnyJsFormalParameter::JsFormalParameter(param) => param
                        .binding()
                        .ok()?
                        .as_any_js_binding()?
                        .as_js_identifier_binding()?
                        .name_token()
                        .ok(),
                })
                .is_some_and(|name_token| name_token.text().eq(token)),
        }
    }
}
