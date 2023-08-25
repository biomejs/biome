use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    AnyJsCallArgument, AnyJsClass, AnyJsConstructorParameter, JsCallExpression,
    JsConstructorClassMember, TsPropertyParameter,
};
use rome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow unnecessary constructors.
    ///
    /// _ES2015_ provides a default class constructor if one is not specified.
    /// As such, providing an empty constructor or one that delegates into its parent is unnecessary.
    ///
    /// Source: https://typescript-eslint.io/rules/no-useless-constructor
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     constructor (a) {}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class B extends A {
    ///     constructor (a) {
    ///         super(a);
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class C {
    ///     /**
    ///      * Documented constructor.
    ///      */
    ///     constructor () {}
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// class A {
    ///     constructor (prop) {
    ///         this.prop = prop;
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// class B extends A {
    ///     constructor () {
    ///         super(5);
    ///     }
    /// }
    /// ```
    ///
    /// ```ts
    /// class C {
    ///     // Empty constructor with parameter properties are allowed.
    ///     constructor (private prop: number) {}
    /// }
    /// ```
    pub(crate) NoUselessConstructor {
        version: "1.0.0",
        name: "noUselessConstructor",
        recommended: true,
    }
}

impl Rule for NoUselessConstructor {
    type Query = Ast<JsConstructorClassMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let constructor = ctx.query();
        let is_not_public = constructor
            .modifiers()
            .iter()
            .any(|modifier| !modifier.is_public());
        if is_not_public {
            return None;
        }
        let has_parameter_property = constructor
            .parameters()
            .ok()?
            .parameters()
            .iter()
            .filter_map(|x| x.ok())
            .any(|x| TsPropertyParameter::can_cast(x.syntax().kind()));
        if has_parameter_property {
            return None;
        }
        let has_parent_class = constructor
            .syntax()
            .ancestors()
            .find_map(AnyJsClass::cast)
            .filter(|x| x.extends_clause().is_some())
            .is_some();
        let mut body_statements = constructor.body().ok()?.statements().iter();
        let Some(first) = body_statements.next() else {
            if has_parent_class {
                // A `super` call is missing.
                // Do not report as useless constructor.
                return None;
            }
            // empty body and no parent class
            return Some(());
        };
        if body_statements.count() != 0 {
            // There are more than one statement.
            return None;
        }
        let Some(js_expr) = first.as_js_expression_statement()?.expression().ok() else {
            return None;
        };
        let Some(js_call) = js_expr.as_js_call_expression() else {
            return None;
        };
        let is_super_call = js_call.callee().ok()?.as_js_super_expression().is_some();
        if !is_super_call {
            return None;
        }
        if !is_delegating_initialization(constructor, js_call) {
            return None;
        }
        // The constructor has a single statement:
        // a `super()` call that delegates initialization to the parent class
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let constructor = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            constructor.range(),
            markup! {
                "This constructor is unnecessary."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let constructor = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(constructor.clone());
        // Safely remove the constructor whether there is no comments.
        let applicability = if constructor.syntax().has_comments_descendants() {
            Applicability::MaybeIncorrect
        } else {
            Applicability::Always
        };
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability,
            message: markup! { "Remove the unnecessary constructor." }.to_owned(),
            mutation,
        })
    }
}

/// Is `constructor` delegating initialization via `super_call`?
///
/// This checks that constructors' **all** parameters are passed to the super-call in the same order.
fn is_delegating_initialization(
    constructor: &JsConstructorClassMember,
    super_call: &JsCallExpression,
) -> bool {
    let result = || {
        let parameters = constructor.parameters().ok()?.parameters().iter();
        let arguments = super_call.arguments().ok()?.args().iter();
        if parameters.clone().count() != arguments.clone().count() {
            return None;
        }
        let zipped = parameters.zip(arguments);
        for (param, arg) in zipped {
            let param = param.ok()?;
            let arg = arg.ok()?;
            match (param, arg) {
                (
                    AnyJsConstructorParameter::JsRestParameter(param),
                    AnyJsCallArgument::JsSpread(arg),
                ) => {
                    let param_name = param
                        .binding()
                        .ok()?
                        .as_any_js_binding()?
                        .as_js_identifier_binding()?
                        .name_token()
                        .ok()?;
                    let arg_name = arg
                        .argument()
                        .ok()?
                        .as_js_identifier_expression()?
                        .name()
                        .ok()?
                        .value_token()
                        .ok()?;
                    if param_name.text_trimmed() != arg_name.text_trimmed() {
                        return Some(false);
                    }
                }
                (
                    AnyJsConstructorParameter::AnyJsFormalParameter(param),
                    AnyJsCallArgument::AnyJsExpression(expr),
                ) => {
                    let param_name = param
                        .as_js_formal_parameter()?
                        .binding()
                        .ok()?
                        .as_any_js_binding()?
                        .as_js_identifier_binding()?
                        .name_token()
                        .ok()?;
                    let arg_name = expr
                        .as_js_identifier_expression()?
                        .name()
                        .ok()?
                        .value_token()
                        .ok()?;
                    if param_name.text_trimmed() != arg_name.text_trimmed() {
                        return Some(false);
                    }
                }
                (_, _) => {
                    return Some(false);
                }
            }
        }
        Some(true)
    };
    result().unwrap_or(false)
}
