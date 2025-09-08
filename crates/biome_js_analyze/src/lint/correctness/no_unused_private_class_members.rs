use crate::{
    JsRuleAction,
    services::semantic::Semantic,
    utils::{is_node_equal, rename::RenameSymbolExtensions},
};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsClassMember, AnyJsClassMemberName, AnyJsComputedMember, AnyJsExpression,
    AnyJsFormalParameter, AnyJsName, JsAssignmentExpression, JsClassDeclaration, JsSyntaxKind,
    JsSyntaxNode, TsAccessibilityModifier, TsPropertyParameter,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, SyntaxNodeOptionExt, TextRange,
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
    type Query = Semantic<JsClassDeclaration>;
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
            let unused_members = traverse_members_usage(node.syntax(), private_members);

            for member in unused_members {
                match &member {
                    AnyMember::AnyJsClassMember(_) => {
                        results.push(UnusedMemberAction::RemoveMember(member));
                    }
                    AnyMember::TsPropertyParameter(ts_property_param) => {
                        // Check if the parameter is also unused in constructor body using semantic analysis
                        let should_rename =
                            check_ts_property_parameter_usage(ctx, ts_property_param);
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
                        let new_name = format!("_{name_trimmed}");
                        if !mutation.rename_node_declaration(
                            ctx.model(),
                            identifier_binding,
                            &new_name,
                        ) {
                            return None;
                        }
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
    syntax: &JsSyntaxNode,
    mut private_members: Vec<AnyMember>,
) -> Vec<AnyMember> {
    // `true` is at least one member is a TypeScript private member like `private member`.
    // The other private members are sharp members `#member`.
    let mut ts_private_count = private_members
        .iter()
        .filter(|member| !member.is_private_sharp())
        .count();

    for node in syntax.descendants() {
        match AnyJsName::try_cast(node) {
            Ok(js_name) => {
                private_members.retain(|private_member| {
                    let member_being_used = private_member.match_js_name(&js_name) == Some(true);

                    if !member_being_used {
                        return true;
                    }

                    let is_write_only =
                        is_write_only(&js_name) == Some(true) && !private_member.is_accessor();
                    let is_in_update_expression = is_in_update_expression(&js_name);

                    if is_in_update_expression || is_write_only {
                        return true;
                    }

                    if !private_member.is_private_sharp() {
                        ts_private_count -= 1;
                    }

                    false
                });

                if private_members.is_empty() {
                    break;
                }
            }
            Err(node) => {
                if ts_private_count != 0
                    && let Some(computed_member) = AnyJsComputedMember::cast(node)
                    && matches!(
                        computed_member.object(),
                        Ok(AnyJsExpression::JsThisExpression(_))
                    )
                {
                    // We consider that all TypeScript private members are used in expressions like `this[something]`.
                    private_members.retain(|private_member| private_member.is_private_sharp());
                    ts_private_count = 0;
                }
            }
        }
    }

    private_members
}

/// Check if a TsPropertyParameter is also unused as a function parameter
fn check_ts_property_parameter_usage(
    ctx: &RuleContext<NoUnusedPrivateClassMembers>,
    ts_property_param: &TsPropertyParameter,
) -> bool {
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

        if identifier_binding
            .all_references(ctx.model())
            .next()
            .is_some()
        {
            return false;
        }

        return true;
    }

    false
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
/// # Examples of expressions that are NOT write-only
///
/// ```js
/// return this.#val++;   // increment expression used as return value
/// return this.#val = 1; // assignment used as expression
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

    // If it's not a direct child of expression statement, its result is being used
    let kind = assignment_expression.syntax().parent().kind();
    Some(kind.is_some_and(|kind| matches!(kind, JsSyntaxKind::JS_EXPRESSION_STATEMENT)))
}

fn is_in_update_expression(js_name: &AnyJsName) -> bool {
    let Some(grand_parent) = js_name.syntax().grand_parent() else {
        return false;
    };

    // If it's not a direct child of expression statement, its result is being used
    let kind = grand_parent.parent().kind();
    if !kind.is_some_and(|kind| matches!(kind, JsSyntaxKind::JS_EXPRESSION_STATEMENT)) {
        return false;
    }

    matches!(
        grand_parent.kind(),
        JsSyntaxKind::JS_POST_UPDATE_EXPRESSION | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
    )
}

impl AnyMember {
    fn is_accessor(&self) -> bool {
        matches!(
            self.syntax().kind(),
            JsSyntaxKind::JS_SETTER_CLASS_MEMBER | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
        )
    }

    /// Returns `true` if it is a private property starting with `#`.
    fn is_private_sharp(&self) -> bool {
        if let Self::AnyJsClassMember(member) = self {
            matches!(
                member.name(),
                Ok(Some(AnyJsClassMemberName::JsPrivateClassMemberName(_)))
            )
        } else {
            false
        }
    }

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

    fn match_js_name(&self, js_name: &AnyJsName) -> Option<bool> {
        let value_token = js_name.value_token().ok()?;
        let token = value_token.text_trimmed();

        match self {
            Self::AnyJsClassMember(member) => match member {
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
                        .text_trimmed()
                        == token,
                ),
            },
        }
    }
}
