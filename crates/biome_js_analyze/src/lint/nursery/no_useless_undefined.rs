use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    JsArrayBindingPatternElement, JsArrowFunctionExpression, JsCallExpression, JsFormalParameter,
    JsObjectBindingPattern, JsReturnStatement, JsVariableStatement, JsYieldArgument,
};
use biome_rowan::{declare_node_union, AstNode, TextRange};

declare_lint_rule! {
    /// Disallow useless `undefined`.
    ///
    /// `undefined` is the default value for new variables, parameters, return statements, etc… so specifying it doesn't make any difference.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let foo = undefined;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const {foo = undefined} = bar;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const noop = () => undefined;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///    return undefined;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function* foo() {
    ///   yield undefined;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo(bar = undefined) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo({bar = undefined}) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo(undefined);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let foo;
    /// const {foo} = bar;
    /// function foo() {
    ///   return;
    /// }
    /// function* foo() {
    ///   yield;
    /// }
    /// function foo(bar) {}
    /// function foo({bar}) {}
    /// foo();
    /// ```
    ///
    pub NoUselessUndefined {
        version: "next",
        name: "noUselessUndefined",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-undefined")],
        recommended: false,
    }
}

declare_node_union! {
    pub RuleQuery = JsVariableStatement
        | JsObjectBindingPattern
        | JsYieldArgument
        | JsReturnStatement
        | JsArrayBindingPatternElement
        | JsCallExpression
        | JsArrowFunctionExpression
        | JsFormalParameter
}

static FUNCTION_NAMES: &'static [&'static str; 28] = &[
    // Function#bind()
    "bind",
    // Compare function names
    "is",
    "equal",
    "notEqual",
    "strictEqual",
    "notStrictEqual",
    "propertyVal",
    "notPropertyVal",
    "not",
    "include",
    "property",
    "toBe",
    "toHaveBeenCalledWith",
    "toContain",
    "toContainEqual",
    "toEqual",
    "same",
    "notSame",
    "strictSame",
    "strictNotSame",
    // `array.push(undefined)`
    "push",
    // `array.unshift(undefined)`
    "unshift",
    // `array.includes(undefined)`
    "includes",
    // `set.add(undefined)`
    "add",
    // `set.has(undefined)`
    "has",
    // `map.set(foo, undefined)`
    "set",
    // `React.createContext(undefined)`
    "createContext",
    // https://vuejs.org/api/reactivity-core.html#ref
    "ref",
];

fn should_ignore(name: &str) -> bool {
    FUNCTION_NAMES.contains(&name) ||
    // setState(undefined), setXXX(undefined)
    name.starts_with("set")
}

impl Rule for NoUselessUndefined {
    type Query = Ast<RuleQuery>;
    type State = (String, TextRange);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut signals = vec![];

        match node {
            // let foo = undefined;
            RuleQuery::JsVariableStatement(statement) => {
                let Ok(node) = statement.declaration() else {
                    return signals;
                };

                let let_or_var = node.is_let() || node.is_var();
                if !let_or_var {
                    return signals;
                }

                for declarator in node.declarators() {
                    let Ok(decl) = declarator else { continue };
                    if let Some(initializer) = decl.initializer() {
                        let js_reference_identifier = initializer
                            .expression()
                            .ok()
                            .and_then(|expression| expression.as_js_reference_identifier());
                        if let Some(keyword) = js_reference_identifier {
                            if keyword.is_undefined() {
                                signals.push((keyword.text().to_string(), keyword.range()));
                            }
                        }
                    }
                }
            }
            // foo(bar, undefined, undefined);
            RuleQuery::JsCallExpression(js_call_expr) => {
                // check if it's a bind call: foo.bind(bar, undefined)
                if let Ok(callee) = js_call_expr.callee() {
                    if let Some(member_expr) = callee.as_js_static_member_expression() {
                        if let Ok(member) = member_expr.member() {
                            if should_ignore(&member.text().as_str()) {
                                return signals;
                            }
                        }
                    }
                };

                let Some(js_call_argument_list) = js_call_expr.arguments().ok() else {
                    return signals;
                };

                for (idx, argument) in js_call_argument_list.args().into_iter().enumerate() {
                    if idx == 0 {
                        continue;
                    }
                    if let Some(argument) = argument.ok() {
                        if let Some(expr) = argument.as_any_js_expression() {
                            if let Some(keyword) = expr.as_js_reference_identifier() {
                                if keyword.is_undefined() {
                                    signals.push((keyword.text().to_string(), keyword.range()));
                                }
                            }
                        }
                    }
                }
            }
            // { a: undefined }
            RuleQuery::JsObjectBindingPattern(js_object_binding_pattern) => {
                for property in js_object_binding_pattern.properties() {
                    if let Ok(property) = property {
                        if let Some(prop) =
                            property.as_js_object_binding_pattern_shorthand_property()
                        {
                            if let Some(init) = prop.init() {
                                if let Ok(expr) = init.expression() {
                                    if let Some(keyword) = expr.as_js_reference_identifier() {
                                        if keyword.is_undefined() {
                                            signals.push((
                                                keyword.text().to_string(),
                                                keyword.range(),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                return signals;
            }
            // function foo([bar = undefined]) {}
            RuleQuery::JsArrayBindingPatternElement(js_array_binding_pattern_element) => {
                if let Some(init) = js_array_binding_pattern_element.init() {
                    if let Ok(expr) = init.expression() {
                        if let Some(keyword) = expr.as_js_reference_identifier() {
                            if keyword.is_undefined() {
                                signals.push((keyword.text().to_string(), keyword.range()));
                            }
                        }
                    }
                }
            }
            // yield undefined
            RuleQuery::JsYieldArgument(yield_argument) => {
                if yield_argument.star_token().is_some() {
                    return signals;
                }

                if let Ok(expression) = yield_argument.expression() {
                    if let Some(keyword) = expression.as_js_reference_identifier() {
                        if keyword.is_undefined() {
                            signals.push((keyword.text().to_string(), keyword.range()));
                        }
                    }
                }
            }
            // return undefined
            RuleQuery::JsReturnStatement(js_return_statement) => {
                if let Some(argument) = js_return_statement.argument() {
                    if let Some(keyword) = argument.as_js_reference_identifier() {
                        if keyword.is_undefined() {
                            signals.push((keyword.text().to_string(), keyword.range()));
                        }
                    }
                }
            }
            // const noop = () => undefined
            RuleQuery::JsArrowFunctionExpression(js_arrow_function_expression) => {
                if let Some(body) = js_arrow_function_expression.body().ok() {
                    if let Some(expr) = body.as_any_js_expression() {
                        if let Some(keyword) = expr.as_js_reference_identifier() {
                            if keyword.is_undefined() {
                                signals.push((keyword.text().to_string(), keyword.range()));
                            }
                        }
                    }
                }
            }
            // function foo(bar = undefined) {}
            RuleQuery::JsFormalParameter(js_formal_parameter) => {
                if let Some(init) = js_formal_parameter.initializer() {
                    if let Ok(expr) = init.expression() {
                        if let Some(keyword) = expr.as_js_reference_identifier() {
                            if keyword.is_undefined() {
                                signals.push((keyword.text().to_string(), keyword.range()));
                            }
                        }
                    }
                }
            }
        };

        signals
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.1,
                markup! {
                    "Don't use unnecessary "<Emphasis>"undefined"</Emphasis>"."
                },
            )
            .note(markup! {
                ""<Emphasis>"undefined"</Emphasis>" is the default value for new variables, parameters, return statements, etc… so specifying it doesn't make any difference."
            }),
        )
    }

    // fn action(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<()> {}
}
