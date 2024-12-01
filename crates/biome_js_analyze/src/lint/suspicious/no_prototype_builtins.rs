use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{self};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsCallExpression, JsSyntaxKind,
    TextRange, T,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};

declare_lint_rule! {
    /// Disallow direct use of `Object.prototype` builtins.
    ///
    /// ECMAScript 5.1 added `Object.create` which allows the creation of an object with a custom prototype.
    /// This pattern is often used for objects used as Maps. However, this pattern can lead to errors
    /// if something else relies on prototype properties/methods.
    /// Moreover, the methods could be shadowed, this can lead to random bugs and denial of service
    /// vulnerabilities. For example, calling `hasOwnProperty` directly on parsed JSON like `{"hasOwnProperty": 1}` could lead to vulnerabilities.
    /// To avoid subtle bugs like this, you should call these methods from `Object.prototype`.
    /// For example, `foo.isPrototypeOf(bar)` should be replaced with `Object.prototype.isPrototypeOf.call(foo, "bar")`
    /// As for the `hasOwn` method, `foo.hasOwn("bar")` should be replaced with `Object.hasOwn(foo, "bar")`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.hasOwnProperty("bar");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.isPrototypeOf(bar);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.propertyIsEnumerable("bar");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Object.hasOwnProperty.call(foo, "bar");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var valid = Object.hasOwn(foo, "bar");
    /// var valid = Object.prototype.isPrototypeOf.call(foo, bar);
    /// var valid = {}.propertyIsEnumerable.call(foo, "bar");
    /// ```
    ///
    pub NoPrototypeBuiltins {
        version: "1.1.0",
        name: "noPrototypeBuiltins",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-prototype-builtins"),
            RuleSource::Eslint("prefer-object-has-own")
        ],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

pub struct RuleState {
    prototype_builtins_method_name: String,
    text_range: TextRange,
    has_call_fn: bool,
}

impl Rule for NoPrototypeBuiltins {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?.omit_parentheses();
        let member_expr = AnyJsMemberExpression::cast(callee.into_syntax())?;
        let member_name = member_expr.member_name()?;
        let member_name_text = member_name.text();

        if is_prototype_builtins(member_name_text) {
            return Some(RuleState {
                prototype_builtins_method_name: member_name_text.to_string(),
                text_range: member_name.range(),
                has_call_fn: false,
            });
        }

        if member_name_text == "call" {
            let object = member_expr.object().ok()?.omit_parentheses();
            let obj_expr = AnyJsMemberExpression::cast(object.into_syntax())?;
            let obj_name = obj_expr.member_name()?;
            let obj_name_text = obj_name.text();

            if obj_name_text == "hasOwnProperty"
                && has_left_hand_object(&obj_expr)?
                && is_global_object(ctx.model())
            {
                return Some(RuleState {
                    prototype_builtins_method_name: obj_name_text.to_string(),
                    text_range: call_expr.range(),
                    has_call_fn: true,
                });
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            state.text_range,
            markup! {
                "Do not access Object.prototype method '"{&state.prototype_builtins_method_name}"' from target object."
            },
        );

        if state.prototype_builtins_method_name == "hasOwnProperty" {
            Some(
                diag.note(markup! {
                    "It's recommended using "<Emphasis>"Object.hasOwn()"</Emphasis>" instead of using "<Emphasis>"Object.hasOwnProperty()"</Emphasis>"."
                })
                .note(markup! {
                    "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwn">"MDN web docs"</Hyperlink>" for more details."
                }),
            )
        } else {
            Some(diag)
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        if node.is_optional() || state.prototype_builtins_method_name != "hasOwnProperty" {
            return None;
        }

        let mut callee = node.callee().ok()?;
        let is_callee_parenthesized = callee.as_js_parenthesized_expression().is_some();

        while let Some(paren_callee) = callee.as_js_parenthesized_expression() {
            callee = paren_callee.expression().ok()?;
        }

        let member_expr = AnyJsMemberExpression::cast(callee.clone().into_syntax())?;
        let member_syntax = member_expr.syntax();

        if member_expr.is_optional_chain()
            || member_syntax.has_comments_direct()
            || member_syntax.has_comments_descendants()
        {
            return None;
        }

        let mut mutation = ctx.root().begin();
        let has_own_expr = build_has_own_expr(node, is_callee_parenthesized);

        // foo.hasOwnProperty('bar') -> Object.hasOwn(foo, 'bar')
        // foo.bar.hasOwnProperty('bar') -> Object.hasOwn(foo.bar, 'bar')
        // foo.["bar"].hasOwnProperty('bar') -> Object.hasOwn(foo.["bar"], 'bar')
        if !state.has_call_fn {
            let callee_arg = AnyJsCallArgument::AnyJsExpression(member_expr.object().ok()?);
            let existing_arg = node.arguments().ok()?.args().into_iter().next()?.ok()?;

            mutation.replace_node(
                node.clone(),
                make::js_call_expression(
                    has_own_expr,
                    make::js_call_arguments(
                        make::token(T!['(']),
                        make::js_call_argument_list(
                            [callee_arg.trim_trivia()?, existing_arg],
                            [make::token(T![,])
                                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")])],
                        ),
                        make::token(T![')']),
                    ),
                )
                .build(),
            );
        } else {
            mutation.replace_node(callee, has_own_expr);
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use 'Object.hasOwn()' instead." },
            mutation,
        ))
    }
}

/// Checks if the `Object.prototype` builtins called directly.
fn is_prototype_builtins(token_text: &str) -> bool {
    matches!(
        token_text,
        "hasOwnProperty" | "isPrototypeOf" | "propertyIsEnumerable"
    )
}

fn is_global_object(semantic: &SemanticModel) -> bool {
    semantic
        .scopes()
        .find(|s| s.get_binding("Object").is_some())
        .map_or(true, |s| s.is_global_scope())
}

/// Checks if the given node is considered to be an access to a property of `Object.prototype`.
/// Returns `true` if `expression.object` is `Object`, `Object.prototype`, or `{}` (empty `JsObjectExpression`).
fn has_left_hand_object(member_expr: &AnyJsMemberExpression) -> Option<bool> {
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

    if let AnyJsExpression::JsIdentifierExpression(id_expr) = &node {
        if id_expr.name().ok()?.syntax().text_trimmed() == "Object" {
            return Some(true);
        }
    }

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

fn build_has_own_expr(node: &JsCallExpression, is_callee_parenthesized: bool) -> AnyJsExpression {
    let mut object_identifier = String::from("Object");

    if !is_callee_parenthesized && can_previous_token_be_adjacent(node) {
        object_identifier.insert(0, ' ');
    }

    AnyJsExpression::from(make::js_static_member_expression(
        make::js_identifier_expression(make::js_reference_identifier(make::ident(
            &object_identifier,
        )))
        .into(),
        make::token(T![.]),
        make::js_name(make::ident("hasOwn")).into(),
    ))
}
