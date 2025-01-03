use crate::{services::semantic::Semantic, utils::batch::JsBatchMutation, JsRuleAction};
use biome_analyze::{context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_factory::make::{js_literal_member_name, js_property_object_member};
use biome_js_semantic::{Reference, ReferencesExtensions};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsObjectMemberName, JsIdentifierBinding,
    JsIdentifierExpression, JsReferenceIdentifier, JsShorthandPropertyObjectMember,
    JsStringLiteralExpression, JsSyntaxKind, JsVariableDeclaration, JsVariableDeclarator,
    JsVariableDeclaratorList,
};
use biome_rowan::{AstNode, BatchMutationExt, SyntaxNodeCast, SyntaxToken};

declare_lint_rule! {
    /// Disallow the use of constants which its value is the upper-case version of its name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const FOO = "FOO";
    /// console.log(FOO);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let FOO = "FOO";
    /// console.log(FOO);
    /// ```
    ///
    /// ```js
    /// export const FOO = "FOO";
    /// console.log(FOO);
    /// ```
    ///
    /// ```js
    /// function f(FOO = "FOO") {
    ///     return FOO;
    /// }
    /// ```
    ///
    pub NoShoutyConstants {
        version: "1.0.0",
        name: "noShoutyConstants",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

/// Check for
/// a = "a" (true)
/// a = "b" (false)
fn is_id_and_string_literal_inner_text_equal(
    declarator: &JsVariableDeclarator,
) -> Option<(JsIdentifierBinding, JsStringLiteralExpression)> {
    let id = declarator.id().ok()?;
    let id = id.as_any_js_binding()?.as_js_identifier_binding()?;
    let name = id.name_token().ok()?;
    let id_text = name.text_trimmed();

    let expression = declarator.initializer()?.expression().ok()?;
    let literal = expression
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?;
    let literal_text = literal.inner_string_text().ok()?;

    if id_text.len() != literal_text.text().len() {
        return None;
    }

    for (from_id, from_literal) in id_text.chars().zip(literal_text.chars()) {
        if from_id != from_literal || from_id.is_lowercase() {
            return None;
        }
    }

    Some((id.clone(), literal.clone()))
}

pub struct State {
    literal: JsStringLiteralExpression,
    reference: Reference,
}

impl Rule for NoShoutyConstants {
    type Query = Semantic<JsVariableDeclarator>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let declarator = ctx.query();
        let declaration = declarator
            .parent::<JsVariableDeclaratorList>()?
            .parent::<JsVariableDeclaration>()?;

        if declaration.is_const() {
            if let Some((binding, literal)) = is_id_and_string_literal_inner_text_equal(declarator)
            {
                let model = ctx.model();
                if model.is_exported(&binding) {
                    return None;
                }

                if binding.all_references(model).count() > 1 {
                    return None;
                }

                return Some(State {
                    literal,
                    reference: binding.all_references(model).next()?,
                });
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let declarator = ctx.query();

        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            declarator.syntax().text_trimmed_range(),
            markup! {
                "Redundant constant declaration."
            },
        );

        let node = state.reference.syntax();
        diag = diag.detail(node.text_trimmed_range(), "Used here.");

        let diag = diag.note(
            markup! {"You should avoid declaring constants with a string that's the same
    value as the variable name. It introduces a level of unnecessary
    indirection when it's only two additional characters to inline."},
        );

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let root = ctx.root();
        let literal = AnyJsLiteralExpression::JsStringLiteralExpression(state.literal.clone());

        let mut batch = root.begin();

        batch.remove_js_variable_declarator(ctx.query());

        if let Some(node) = state
            .reference
            .syntax()
            .parent()?
            .cast::<JsIdentifierExpression>()
        {
            batch.replace_node(
                AnyJsExpression::JsIdentifierExpression(node),
                AnyJsExpression::AnyJsLiteralExpression(literal),
            );
        } else if let Some(node) = state
            .reference
            .syntax()
            .parent()?
            .cast::<JsShorthandPropertyObjectMember>()
        {
            // for replacing JsShorthandPropertyObjectMember
            let new_element = js_property_object_member(
                AnyJsObjectMemberName::JsLiteralMemberName(js_literal_member_name(
                    SyntaxToken::new_detached(
                        JsSyntaxKind::JS_LITERAL_MEMBER_NAME,
                        JsReferenceIdentifier::cast_ref(state.reference.syntax())?
                            .value_token()
                            .ok()?
                            .text(),
                        [],
                        [],
                    ),
                )),
                SyntaxToken::new_detached(JsSyntaxKind::COLON, ":", [], []),
                AnyJsExpression::AnyJsLiteralExpression(literal),
            );

            batch.replace_element(node.into_syntax().into(), new_element.into_syntax().into());
        } else {
            return None;
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the constant value directly" }.to_owned(),
            batch,
        ))
    }
}
