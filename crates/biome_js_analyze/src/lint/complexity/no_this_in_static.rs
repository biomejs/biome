use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsClass, AnyJsClassMember, AnyJsExpression, JsArrowFunctionExpression, JsSuperExpression,
    JsSyntaxToken, JsThisExpression,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutationExt, SyntaxResult};

use crate::{services::control_flow::AnyJsControlFlowRoot, JsRuleAction};

declare_lint_rule! {
    /// Disallow `this` and `super` in `static` contexts.
    ///
    /// In JavaScript, the `this` keyword in static contexts refers to the class (the constructor) instance,
    /// not an instance of the class. This can be confusing for developers coming from other languages where
    /// `this` typically refers to an instance of the class, not the class itself.
    ///
    /// Similarly, `super` in static contexts refers to the parent class, not an instance of the class.
    /// This can lead to unexpected behavior if not properly understood.
    ///
    /// This rule enforces the use of the class name itself to access static methods,
    /// which can make the code clearer and less prone to errors. It helps to prevent
    /// misunderstandings and bugs that can arise from the unique behavior of `this` and `super` in static contexts.
    ///
    /// ## Example
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    ///  class A {
    ///     static CONSTANT = 0;
    ///
    ///     static foo() {
    ///         this.CONSTANT;
    ///     }
    ///  }
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///  class B extends A {
    ///     static bar() {
    ///         super.CONSTANT;
    ///     }
    ///  }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class B extends A {
    ///     static ANOTHER_CONSTANT = A.CONSTANT + 1;
    ///
    ///     static foo() {
    ///         A.CONSTANT;
    ///         B.ANOTHER_CONSTANT;
    ///     }
    ///
    ///     bar() {
    ///         this.property;
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// class A {
    ///    static foo() {
    ///        doSomething()
    ///    }
    ///
    ///    bar() {
    ///      A.foo()
    ///    }
    /// }
    /// ```
    ///
    pub NoThisInStatic {
        version: "1.3.1",
        name: "noThisInStatic",
        language: "js",
        sources: &[RuleSource::EslintMysticatea("no-this-in-static")],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoThisInStatic {
    type Query = Ast<JsThisSuperExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let this_super_expression = ctx.query();
        let static_method = this_super_expression
            .syntax()
            .ancestors()
            .find(|x| {
                AnyJsControlFlowRoot::can_cast(x.kind())
                    && !JsArrowFunctionExpression::can_cast(x.kind())
            })
            .and_then(AnyJsClassMember::cast)
            .filter(|member| match member {
                AnyJsClassMember::JsGetterClassMember(member) => member
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some()),
                AnyJsClassMember::JsMethodClassMember(member) => member
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some()),
                AnyJsClassMember::JsSetterClassMember(member) => member
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some()),
                AnyJsClassMember::JsPropertyClassMember(member) => member
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some()),
                AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => true,
                _ => false,
            });
        static_method.is_some().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let this_super_expression = ctx.query();
        let this_super_token = this_super_expression.token().ok()?;
        let text = this_super_token.text_trimmed();
        let note = if let JsThisSuperExpression::JsSuperExpression(_) = this_super_expression {
            markup! {
                <Emphasis>"super"</Emphasis>" refers to a parent class."
            }
        } else {
            markup! {
                <Emphasis>"this"</Emphasis>" refers to the class."
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            this_super_expression.range(),
            markup! {
                "Using "<Emphasis>{text}</Emphasis>" in a "<Emphasis>"static"</Emphasis>" context can be confusing."
            },
        ).note(note))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let this_super_expression = ctx.query();
        let class = this_super_expression
            .syntax()
            .ancestors()
            .find_map(AnyJsClass::cast)?;
        let suggested_class_name = if let JsThisSuperExpression::JsSuperExpression(_) =
            this_super_expression
        {
            let extends_clause = class.extends_clause()?;
            let super_class_name = extends_clause.super_class().ok()?;
            let AnyJsExpression::JsIdentifierExpression(super_class_name) = super_class_name else {
                return None;
            };
            super_class_name
        } else {
            let class_name = class.id()?.as_js_identifier_binding()?.name_token().ok()?;
            make::js_identifier_expression(make::js_reference_identifier(class_name))
        };
        let expr = AnyJsExpression::cast_ref(this_super_expression.syntax())?;
        let mut mutation = ctx.root().begin();
        mutation.replace_node(expr, suggested_class_name.into());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the class name instead." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub JsThisSuperExpression = JsSuperExpression | JsThisExpression
}

impl JsThisSuperExpression {
    fn token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsThisSuperExpression::JsSuperExpression(expr) => expr.super_token(),
            JsThisSuperExpression::JsThisExpression(expr) => expr.this_token(),
        }
    }
}
