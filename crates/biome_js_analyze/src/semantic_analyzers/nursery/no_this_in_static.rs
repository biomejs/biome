use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    JsCallExpression, JsClassDeclaration, JsMethodClassMember, JsSuperExpression, JsThisExpression,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
///# Disallow `this`/`super` in static methods (no-this-in-static)
///
///`this` keyword on static methods refers the class (the constructor) instance.
///However, probably it's confusing maintainers since this behavior is different to
///most other languages.
///
///This rule enforces a use of class itself to access static methods.
///
///## Rule Details
///
///Examples of **incorrect** code for this rule:
///
///```js
///
///class A {
///    static foo() {
///        doSomething()
///    }
///
///    static bar() {
///        this.foo()   //ERROR: Instead of this.foo() use A.foo()
///    }
///}
/// ```
///
/// ```js
///
/// class A {
///    static foo() {
///        doSomething()
///    }
/// }
///
/// class B extends A {
///    static foo() {
///        super.foo()  //ERROR: Instead of super.foo() use A.foo()
///    }
/// }
///```
///
///Examples of **correct** code for this rule:
///
///```js
///
///class A {
///    static foo() {
///        doSomething()
///    }
///}
///
///class B extends A {
///    static foo() {
///        A.foo()
///    }
///}
///```
///
/// ```js
///
///class A {
///    static foo() {
///        doSomething()
///    }
///
///    bar() {
///      A.foo()
///    }
///}
///```
///
    pub(crate) NoThisInStatic {
        version: "next",
        name: "noThisInStatic",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsThisSuperExpression = JsSuperExpression | JsThisExpression
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
            .find_map(JsMethodClassMember::cast)
            .filter(|member| {
                member
                    .modifiers()
                    .iter()
                    .any(|modifier| modifier.as_js_static_modifier().is_some())
            });

        if static_method.is_some() {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let this_super_expression = ctx.query();

        let call_expression = this_super_expression
            .syntax()
            .ancestors()
            .find_map(JsCallExpression::cast)?;

        let class_declaration = this_super_expression
            .syntax()
            .ancestors()
            .find_map(JsClassDeclaration::cast)?;

        let class_name_str = class_declaration.id().ok()?.text();
        let call_expression_str = call_expression.text();

        let extended_class_name = class_declaration
            .extends_clause()
            .and_then(|with_extends_clause| with_extends_clause.super_class().ok())
            .map(|node| node.text());

        let mut recommendation_str = call_expression_str.replace("this", &class_name_str);

        if let Some(extended_class_name_str) = extended_class_name {
            recommendation_str = recommendation_str.replace("super", &extended_class_name_str);
        }

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                this_super_expression.range(),
                markup! {
                    "Unexpected "<Emphasis>{this_super_expression.text()}</Emphasis>"."
                },
            )
            .note(markup! {
                            "Function "<Emphasis>{call_expression_str}</Emphasis>" is static, so `"<Emphasis>{this_super_expression.text()}"."</Emphasis>"` refers to the class (the constructor) instance."
            })
            .note(markup! {
                            "Instead of "<Emphasis>{call_expression_str}</Emphasis>" use "<Emphasis>{recommendation_str}</Emphasis>"."
                        }),
        )
    }
}
