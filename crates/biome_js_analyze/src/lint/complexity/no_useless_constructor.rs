use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsClass, AnyJsConstructorParameter, AnyJsFormalParameter,
    JsCallExpression, JsConstructorClassMember,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow unnecessary constructors.
    ///
    /// _ES2015_ provides a default class constructor if one is not specified.
    /// As such, providing an empty constructor or one that delegates into its parent is unnecessary.
    ///
    /// The rule ignores:
    ///
    /// - decorated classes;
    /// - constructors with at least one [parameter property](https://www.typescriptlang.org/docs/handbook/2/classes.html#parameter-properties);
    /// - `private` and `protected` constructors.
    ///
    /// ## Caveat
    ///
    /// This rule reports on constructors whose sole purpose is to make a parent constructor public.
    /// See the last invalid example.
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
    /// ```ts,expect_diagnostic
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
    /// ```js,expect_diagnostic
    /// class A {
    ///     protected constructor() {
    ///         this.prop = 1;
    ///     }
    /// }
    ///
    /// class B extends A {
    ///     // Make the parent constructor public.
    ///     constructor () {
    ///         super();
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
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
    ///
    /// ```ts
    /// class D {
    ///   constructor(public arg: number){}
    /// }
    ///
    /// class F extends D {
    ///   // constructor with default parameters are allowed.
    ///   constructor(arg = 4) {
    ///     super(arg)
    ///   }
    /// }
    /// ```
    ///
    /// ```ts
    /// @Decorator
    /// class C {
    ///     constructor (prop: number) {}
    /// }
    /// ```
    pub NoUselessConstructor {
        version: "1.0.0",
        name: "noUselessConstructor",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-useless-constructor"),
            RuleSource::EslintTypeScript("no-useless-constructor"),
        ],
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
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
        for parameter in constructor.parameters().ok()?.parameters() {
            let decorators = match parameter.ok()? {
                AnyJsConstructorParameter::AnyJsFormalParameter(
                    AnyJsFormalParameter::JsBogusParameter(_)
                    | AnyJsFormalParameter::JsMetavariable(_),
                )
                | AnyJsConstructorParameter::TsPropertyParameter(_) => {
                    // Ignore constructors with Bogus parameters or parameter properties
                    return None;
                }
                AnyJsConstructorParameter::AnyJsFormalParameter(
                    AnyJsFormalParameter::JsFormalParameter(parameter),
                ) => parameter.decorators(),
                AnyJsConstructorParameter::JsRestParameter(parameter) => parameter.decorators(),
            };
            if !decorators.is_empty() {
                // Ignore constructors with decorated parameters
                return None;
            }
        }
        let class = constructor.syntax().ancestors().find_map(AnyJsClass::cast);
        if let Some(class) = &class {
            if !class.decorators().is_empty() {
                // Ignore decorated classes
                return None;
            }
        }
        let mut body_statements = constructor.body().ok()?.statements().iter();
        let Some(first) = body_statements.next() else {
            let has_parent_class = class.and_then(|x| x.extends_clause()).is_some();
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
        let js_expr = first.as_js_expression_statement()?.expression().ok()?;
        let js_call = js_expr.as_js_call_expression()?;
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
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the unnecessary constructor." }.to_owned(),
            mutation,
        ))
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
        let parameters = constructor.parameters().ok()?.parameters();
        let arguments = super_call.arguments().ok()?.args();
        if parameters.len() != arguments.len() {
            return None;
        }
        let zipped = parameters.iter().zip(arguments.iter());
        for (param, arg) in zipped {
            let param = param.ok()?;
            let arg = arg.ok()?;
            match (param, arg) {
                (
                    AnyJsConstructorParameter::AnyJsFormalParameter(
                        AnyJsFormalParameter::JsFormalParameter(param),
                    ),
                    _,
                ) if param.initializer().is_some() => {
                    return Some(false);
                }
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
