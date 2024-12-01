use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsFormalParameter, AnyJsName,
    JsAssignmentExpression, JsAssignmentOperator, JsClassDeclaration, JsSyntaxKind, JsSyntaxNode,
    TsAccessibilityModifier, TsPropertyParameter,
};
use biome_rowan::{
    declare_node_union, AstNode, AstNodeList, AstSeparatedList, BatchMutationExt,
    SyntaxNodeOptionExt, TextRange,
};
use rustc_hash::FxHashSet;

use crate::{utils::is_node_equal, JsRuleAction};

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
        sources: &[RuleSource::Eslint("no-unused-private-class-members")],
        recommended: false,
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
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let private_members: FxHashSet<AnyMember> = get_all_declared_private_members(node);
        if private_members.is_empty() {
            Vec::new()
        } else {
            traverse_members_usage(node.syntax(), private_members)
        }
        .into_boxed_slice()
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

/// Check for private member usage
/// if the member usage is found, we remove it from the hashmap
fn traverse_members_usage(
    syntax: &JsSyntaxNode,
    mut private_members: FxHashSet<AnyMember>,
) -> Vec<AnyMember> {
    let iter = syntax.preorder();

    for event in iter {
        match event {
            biome_rowan::WalkEvent::Enter(node) => {
                if let Some(js_name) = AnyJsName::cast(node) {
                    private_members.retain(|private_member| {
                        let member_being_used =
                            private_member.match_js_name(&js_name) == Some(true);
                        let is_write_only =
                            is_write_only(&js_name) == Some(true) && !private_member.is_accessor();
                        let is_in_update_expression = is_in_update_expression(&js_name);

                        if member_being_used && is_in_update_expression {
                            return true;
                        }

                        if member_being_used && is_write_only {
                            return true;
                        }

                        false
                    });

                    if private_members.is_empty() {
                        break;
                    }
                }
            }
            biome_rowan::WalkEvent::Leave(_) => continue,
        }
    }

    private_members.into_iter().collect()
}

fn get_all_declared_private_members(
    class_declaration: &JsClassDeclaration,
) -> FxHashSet<AnyMember> {
    class_declaration
        .members()
        .iter()
        .map(AnyMember::AnyJsClassMember)
        .chain(get_constructor_params(class_declaration))
        .filter(|member| member.is_private() == Some(true))
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

    if let Some(constructor_member) = constructor_member {
        if let Ok(constructor_params) = constructor_member.parameters() {
            return constructor_params
                .parameters()
                .iter()
                .filter_map(|param| match param.ok()? {
                    biome_js_syntax::AnyJsConstructorParameter::TsPropertyParameter(
                        ts_property,
                    ) => Some(ts_property.into()),
                    _ => None,
                })
                .collect();
        }
    }

    FxHashSet::default()
}

/// Check whether the provided `AnyJsName` is part of a potentially write-only assignment expression.
/// This function inspects the syntax tree around the given `AnyJsName` to check whether it is involved in an assignment operation and whether that assignment can be write-only.
///
/// # Returns
///
/// - `Some(true)`: If the `js_name` is in a write-only assignment.
/// - `Some(false)`: If the `js_name` is in a assignments that also reads like shorthand operators
/// - `None`: If the parent is not present or grand parent is not a JsAssignmentExpression
///
/// # Examples of write only expressions
///
/// ```js
/// this.usedOnlyInWrite = 2;
/// this.usedOnlyInWrite = this.usedOnlyInWrite;
/// ```
///
fn is_write_only(js_name: &AnyJsName) -> Option<bool> {
    let parent = js_name.syntax().parent()?;
    let grand_parent = parent.parent()?;
    let assignment_expression = JsAssignmentExpression::cast(grand_parent)?;
    let left = assignment_expression.left().ok()?;

    if !is_node_equal(left.syntax(), &parent) {
        return Some(false);
    }

    if !matches!(
        assignment_expression.operator(),
        Ok(JsAssignmentOperator::Assign)
    ) {
        let kind = assignment_expression.syntax().parent().kind();
        return Some(
            kind.is_some_and(|kind| matches!(kind, JsSyntaxKind::JS_EXPRESSION_STATEMENT)),
        );
    }

    Some(true)
}

fn is_in_update_expression(js_name: &AnyJsName) -> bool {
    let grand_parent = js_name.syntax().grand_parent();

    grand_parent.kind().is_some_and(|kind| {
        matches!(
            kind,
            JsSyntaxKind::JS_POST_UPDATE_EXPRESSION | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
        )
    })
}

impl AnyMember {
    fn is_accessor(&self) -> bool {
        matches!(
            self.syntax().kind(),
            JsSyntaxKind::JS_SETTER_CLASS_MEMBER | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
        )
    }

    fn is_private(&self) -> Option<bool> {
        match self {
            AnyMember::AnyJsClassMember(member) => {
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
            AnyMember::TsPropertyParameter(param) => Some(
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
            AnyMember::AnyJsClassMember(member) => match member {
                AnyJsClassMember::JsGetterClassMember(member) => Some(member.name().ok()?.range()),
                AnyJsClassMember::JsMethodClassMember(member) => Some(member.name().ok()?.range()),
                AnyJsClassMember::JsPropertyClassMember(member) => {
                    Some(member.name().ok()?.range())
                }
                AnyJsClassMember::JsSetterClassMember(member) => Some(member.name().ok()?.range()),
                _ => None,
            },
            AnyMember::TsPropertyParameter(ts_property) => {
                match ts_property.formal_parameter().ok()? {
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

    fn match_js_name(&self, js_name: &AnyJsName) -> Option<bool> {
        let value_token = js_name.value_token().ok()?;
        let token = value_token.text_trimmed();

        match self {
            AnyMember::AnyJsClassMember(member) => match member {
                AnyJsClassMember::JsGetterClassMember(member) => {
                    Some(member.name().ok()?.name()?.text() == token)
                }
                AnyJsClassMember::JsMethodClassMember(member) => {
                    Some(member.name().ok()?.name()?.text() == token)
                }
                AnyJsClassMember::JsPropertyClassMember(member) => {
                    Some(member.name().ok()?.name()?.text() == token)
                }
                AnyJsClassMember::JsSetterClassMember(member) => {
                    Some(member.name().ok()?.name()?.text() == token)
                }
                _ => None,
            },
            AnyMember::TsPropertyParameter(ts_property) => {
                match ts_property.formal_parameter().ok()? {
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
                            .text_trimmed()
                            == token,
                    ),
                }
            }
        }
    }
}
