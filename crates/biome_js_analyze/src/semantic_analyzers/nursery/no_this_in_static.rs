use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    JsCallExpression, JsClassDeclaration, JsMethodClassMember, JsSuperExpression, JsThisExpression,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
///  Disallow `this`/`super` in static methods
///
///  In JavaScript, the `this` keyword within static methods refers to the class (the constructor) instance, 
///  not an instance of the class. This can be confusing for developers coming from other languages where 
///  `this` typically refers to an instance of the class, not the class itself. 
///
///  Similarly, `super` in static methods also refers to the parent class, not an instance of the parent class. 
///  This can lead to unexpected behavior if not properly understood.
///
///  This rule enforces the use of the class name itself to access static methods, 
///  which can make the code clearer and less prone to errors. It helps to prevent 
///  misunderstandings and bugs that can arise from the unique behavior of `this` and `super` in static methods.
/// 
///  Source: https://github.com/mysticatea/eslint-plugin/blob/master/docs/rules/no-this-in-static.md
///
///  ## Example
///
///  ### Invalid
/// 
/// ```js,expect_diagnostic
///
///  class A {
///     static foo() {
///         doSomething()
///     }
///
///     static bar() {
///         this.foo()
///     }
///  }
/// ```
///
/// ```js,expect_diagnostic
///  class A {
///     static foo() {
///         doSomething()
///     }
///  }
///
///  class B extends A {
///     static foo() {
///         super.foo()
///     }
///  }
/// ```
///
///  ### Valid
///
///  ```js
///  class A {
///      static foo() {
///          doSomething()
///      }
///  }
///
///  class B extends A {
///      static foo() {
///          A.foo()
///      }
///  }
///  ```
///
///  ```js
///  class A {
///     static foo() {
///         doSomething()
///     }
///
///     bar() {
///       A.foo()
///     }
///  }
///  ```
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
