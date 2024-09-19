use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsMemberExpression, JsCallExpression, JsSyntaxKind, T};
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Disallow use of `Object.prototype.hasOwnProperty.call()` and prefer use of `Object.hasOwn()`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Object.hasOwnProperty.call(obj, 'foo');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.prototype.hasOwnProperty.call(obj, 'foo');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// ({}).hasOwnProperty.call(obj, 'foo');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Object.hasOwn(object, 'foo');
    /// ```
    ///
    /// ```js
    /// (Object) => Object.hasOwnProperty.call(object, 'foo'); // shadowed Object
    /// ```
    ///
    pub UseObjectHasOwn {
        version: "next",
        name: "useObjectHasOwn",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-object-has-own")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseObjectHasOwn {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        let model = ctx.model();

        let callee = expression.callee().ok()?.omit_parentheses();
        let member_expr = AnyJsMemberExpression::cast(callee.into_syntax())?;

        let object = member_expr.object().ok()?.omit_parentheses();
        let obj_expr = AnyJsMemberExpression::cast(object.into_syntax())?;

        let variable_scope = model.scopes().find(|s| s.get_binding("Object").is_some());

        let is_call_member = member_expr.member_name()?.text() == "call";
        let is_has_own_property = obj_expr.member_name()?.text() == "hasOwnProperty";
        let is_valid_scope = variable_scope.is_none() || variable_scope?.is_global_scope();
        let has_left_hand = has_left_hand_object(obj_expr)?;

        if is_call_member && is_has_own_property && has_left_hand && is_valid_scope {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Use 'Object.hasOwn()' instead of 'Object.prototype.hasOwnProperty.call()'."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut callee = node.callee().ok()?;
        let is_callee_parenthesized = callee.as_js_parenthesized_expression().is_some();

        while let Some(paren_callee) = callee.as_js_parenthesized_expression() {
            callee = paren_callee.expression().ok()?;
        }

        let member_expr = AnyJsMemberExpression::cast(callee.clone().into_syntax())?;
        let member_syntax = member_expr.syntax();

        if member_syntax.has_comments_direct() || member_syntax.has_comments_descendants() {
            return None;
        }

        let mut mutation = ctx.root().begin();
        let mut object_identifier = String::from("Object");

        if !is_callee_parenthesized && can_previous_token_be_adjacent(node) {
            object_identifier.insert(0, ' ');
        }

        mutation.replace_node(
            callee,
            AnyJsExpression::from(make::js_static_member_expression(
                make::js_identifier_expression(make::js_reference_identifier(make::ident(
                    &object_identifier,
                )))
                .into(),
                make::token(T![.]),
                make::js_name(make::ident("hasOwn")).into(),
            )),
        );

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Use 'Object.hasOwn()' instead." },
            mutation,
        ))
    }
}

/// Checks if the given node is considered to be an access to a property of `Object.prototype`.
/// Returns `true` if `expression.object` is `Object`, `Object.prototype`, or `{}` (empty `JsObjectExpression`).
fn has_left_hand_object(member_expr: AnyJsMemberExpression) -> Option<bool> {
    let object = member_expr.object().ok()?.omit_parentheses();

    let node = match &object {
        AnyJsExpression::JsObjectExpression(obj_expr) => {
            if obj_expr.members().into_iter().count() == 0 {
                return Some(true);
            }
            object
        }
        AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsComputedMemberExpression(_) => {
            let obj_member_expr = AnyJsMemberExpression::cast(object.clone().into_syntax())?;

            if obj_member_expr.member_name()?.text() == "prototype" {
                obj_member_expr.object().ok()?.omit_parentheses()
            } else {
                object
            }
        }
        _ => object,
    };

    match &node {
        AnyJsExpression::JsIdentifierExpression(id_expr) => {
            if id_expr.name().ok()?.text() == "Object" {
                return Some(true);
            }
        }
        _ => (),
    };

    Some(false)
}

/// Checks if previous token of the given node can be adjacent when applying the action. Some
/// tokens are not valid such as `return`, `in` and `of`.
///
/// # Example
/// ```js
/// return{}.hasOwnProperty.call(a, b); // valid
/// returnObject.hasOwn(a, b); // invalid
/// ```
fn can_previous_token_be_adjacent(node: &JsCallExpression) -> bool {
    let mut prev_sibling = None;
    let mut current = node.syntax().clone();

    while AnyJsExpression::can_cast(current.kind()) {
        prev_sibling = current.prev_sibling_or_token();

        if let Some(parent) = current.parent() {
            current = parent;
        } else {
            break;
        }
    }

    if let Some(sibling) = prev_sibling {
        node.range().start() == sibling.text_range().end()
            && matches!(
                sibling.kind(),
                JsSyntaxKind::IN_KW | JsSyntaxKind::OF_KW | JsSyntaxKind::RETURN_KW
            )
    } else {
        false
    }
}
