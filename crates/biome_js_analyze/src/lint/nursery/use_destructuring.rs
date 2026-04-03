use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsExpression,
    AnyJsLiteralExpression, AnyJsName, JsAssignmentExpression, JsAssignmentOperator,
    JsVariableDeclaration, JsVariableDeclarator,
};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::use_destructuring::UseDestructuringOptions;

declare_lint_rule! {
    /// Require destructuring from arrays and/or objects
    ///
    /// With JavaScript ES6, a new syntax was added for creating variables from an array index or object property,
    /// called destructuring. This rule enforces usage of destructuring instead of accessing a property through a member expression.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var foo = array[0];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var bar = foo.bar;
    /// ```
    ///
    ///
    /// ### Valid
    ///
    /// ```js
    /// var [foo] = array;
    /// ```
    ///
    /// ```js
    /// var { bar } = foo;
    /// ```
    ///
    /// ```ts
    /// // Variables with type annotations are ignored
    /// const foo: string = object.foo;
    /// ```
    ///
    pub UseDestructuring {
        version: "2.3.9",
        name: "useDestructuring",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("prefer-destructuring").same()],
    }
}

impl Rule for UseDestructuring {
    type Query = Ast<UseDestructuringQuery>;
    type State = UseDestructuringState;
    type Signals = Option<Self::State>;
    type Options = UseDestructuringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let options = ctx.options();

        match query {
            UseDestructuringQuery::JsAssignmentExpression(node) => {
                let config = options.assignment_expression();
                if !config.array && !config.object {
                    return None;
                }

                let op = node.operator().ok()?;
                if op != JsAssignmentOperator::Assign {
                    return None;
                }
                let left = node.left().ok()?;
                let right = node.right().ok()?;

                if let AnyJsAssignmentPattern::AnyJsAssignment(
                    AnyJsAssignment::JsIdentifierAssignment(expr),
                ) = left
                {
                    let ident = expr.name_token().ok()?;
                    let state = should_suggest_destructuring(ident.text_trimmed(), &right)?;
                    return match state {
                        UseDestructuringState::Array if config.array => {
                            Some(UseDestructuringState::Array)
                        }
                        UseDestructuringState::Object { .. } if config.object => {
                            Some(UseDestructuringState::Object {
                                is_assignment: true,
                            })
                        }
                        _ => None,
                    };
                }

                None
            }
            UseDestructuringQuery::JsVariableDeclarator(node) => {
                let config = options.variable_declarator();
                if !config.array && !config.object {
                    return None;
                }

                let initializer = node.initializer()?;
                let declaration = JsVariableDeclaration::cast(node.syntax().parent()?.parent()?)?;
                let has_await_using = declaration.await_token().is_some();
                if declaration.kind().ok()?.text_trimmed() == "using" || has_await_using {
                    return None;
                }

                if node.variable_annotation().is_some() {
                    return None;
                }

                let left = node.id().ok()?;
                let right = initializer.expression().ok()?;

                if let AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(expr)) =
                    left
                {
                    let ident = expr.name_token().ok()?;
                    let state = should_suggest_destructuring(ident.text_trimmed(), &right)?;
                    return match state {
                        UseDestructuringState::Array if config.array => {
                            Some(UseDestructuringState::Array)
                        }
                        UseDestructuringState::Object { .. } if config.object => {
                            Some(UseDestructuringState::Object {
                                is_assignment: false,
                            })
                        }
                        _ => None,
                    };
                }

                None
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        match state {
            UseDestructuringState::Array => {
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        node.range(),
                        markup! {
                            "Use array destructuring instead of accessing array elements by index."
                        },
                    )
                    .note(markup! {
                        "Array destructuring is more readable and expressive than accessing individual elements by index."
                    })
                    .note(markup! {
                        "Replace the array index access with array destructuring syntax."
                    }),
                )
            }
            UseDestructuringState::Object { is_assignment } => {
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "Use object destructuring instead of accessing object properties."
                    },
                )
                .note(markup! {
                    "Object destructuring is more readable and expressive than accessing individual properties."
                });

                Some(if *is_assignment {
                    diagnostic.note(markup! {
                        "Wrap the assignment in parentheses to use object destructuring: "<Emphasis>"({ prop } = object)"</Emphasis>"."
                    })
                } else {
                    diagnostic.note(markup! {
                        "Replace the property access with object destructuring syntax."
                    })
                })
            }
        }
    }
}

declare_node_union! {
    pub UseDestructuringQuery = JsVariableDeclarator |  JsAssignmentExpression
}

fn should_suggest_destructuring(
    left: &str,
    right: &AnyJsExpression,
) -> Option<UseDestructuringState> {
    match right {
        AnyJsExpression::JsComputedMemberExpression(expr) => {
            if expr.is_optional_chain() {
                return None;
            }

            let member = expr.member().ok()?;
            if let AnyJsExpression::AnyJsLiteralExpression(expr) = member {
                if matches!(expr, AnyJsLiteralExpression::JsNumberLiteralExpression(_)) {
                    return Some(UseDestructuringState::Array);
                }

                let value = expr.value_token().ok()?;

                if left == value.text_trimmed() {
                    return Some(UseDestructuringState::Object { is_assignment: false });
                }
            }

            None
        }
        AnyJsExpression::JsStaticMemberExpression(expr) => {
            if matches!(expr.member().ok()?, AnyJsName::JsPrivateName(_))
                || matches!(expr.object().ok()?, AnyJsExpression::JsSuperExpression(_))
            {
                return None;
            }

            if expr.is_optional_chain() {
                return None;
            }
            let member = expr.member().ok()?.value_token().ok()?;
            if left == member.text_trimmed() {
                return Some(UseDestructuringState::Object { is_assignment: false });
            }
            None
        }
        _ => None,
    }
}

pub enum UseDestructuringState {
    Object { is_assignment: bool },
    Array,
}
