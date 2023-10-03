use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    JsMethodClassMember, JsSuperExpression, JsThisExpression,
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
    
        Some(RuleDiagnostic::new(
            rule_category!(),
            this_super_expression.range(),
            markup! {
                "Unexpected "<Emphasis>{this_super_expression.text()}</Emphasis>"."
            }),
        )
    }
}
