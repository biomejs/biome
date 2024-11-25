use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsBindingPattern, AnyJsOptionalChainExpression,
    JsArrayAssignmentPatternElement, JsAssignmentExpression, JsAwaitExpression, JsCallExpression,
    JsComputedMemberExpression, JsConditionalExpression, JsExtendsClause, JsForOfStatement,
    JsInExpression, JsInitializerClause, JsInstanceofExpression, JsLogicalExpression,
    JsLogicalOperator, JsNewExpression, JsObjectAssignmentPatternProperty, JsObjectMemberList,
    JsParenthesizedExpression, JsSequenceExpression, JsSpread, JsStaticMemberExpression,
    JsTemplateExpression, JsVariableDeclarator, JsWithStatement,
};
use biome_rowan::{declare_node_union, AstNode, TextRange};

declare_lint_rule! {
    /// Disallow the use of optional chaining in contexts where the undefined value is not allowed.
    ///
    /// The optional chaining (?.) expression can short-circuit with a return value of undefined.
    /// Therefore, treating an evaluated optional chaining expression as a function, object, number, etc., can cause TypeError or unexpected results.
    /// Also, parentheses limit the scope of short-circuiting in chains.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// 1 in obj?.foo;
    /// ```
    ///
    /// ```cjs,expect_diagnostic
    /// with (obj?.foo);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (bar of obj?.foo);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// bar instanceof obj?.foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const { bar } = obj?.foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (obj?.foo)();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (baz?.bar).foo;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// (obj?.foo)?.();
    /// obj?.foo();
    /// (obj?.foo ?? bar)();
    /// obj?.foo.bar;
    /// obj.foo?.bar;
    /// foo?.()?.bar;
    /// ```
    ///
    pub NoUnsafeOptionalChaining {
        version: "1.0.0",
        name: "noUnsafeOptionalChaining",
        language: "js",
        sources: &[RuleSource::Eslint("no-unsafe-optional-chaining")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoUnsafeOptionalChaining {
    type Query = Ast<AnyJsOptionalChainExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        // need to check only optional chain nodes
        if !node.is_optional() {
            return None;
        }
        let mut node: RuleNode = RuleNode::cast_ref(node.syntax())?;
        let mut parent = node.parent::<RuleNode>();
        // parentheses limit the scope of short-circuiting in chains
        // (a?.b).c // here we have an error
        // a?.b.c // ok
        let mut is_inside_parenthesis = false;

        while let Some(current_parent) = parent.take() {
            match &current_parent {
                RuleNode::JsParenthesizedExpression(expression) => {
                    // parentheses limit the scope of short-circuiting in chains
                    is_inside_parenthesis = true;
                    parent = expression.parent::<RuleNode>()
                }
                RuleNode::JsAwaitExpression(expression) => parent = expression.parent::<RuleNode>(),
                RuleNode::JsExtendsClause(extends) => {
                    // class A extends obj?.foo {}
                    return Some(extends.syntax().text_trimmed_range());
                }
                RuleNode::JsNewExpression(expression) => {
                    // If we're here, it means we've found a error
                    // new a?.b
                    // new (a?.b)()
                    return Some(expression.syntax().text_trimmed_range());
                }
                RuleNode::JsLogicalExpression(expression) => {
                    match expression.operator().ok()? {
                        JsLogicalOperator::NullishCoalescing | JsLogicalOperator::LogicalOr => {
                            // for logical or and nullish we need to check only the right expression
                            // (a?.b || a?.b).c()
                            if expression.right().ok()?.syntax() == node.syntax() {
                                parent = expression.parent::<RuleNode>()
                            }
                        }
                        // for logical and we need check both branches
                        // (a?.b && a?.b).c()
                        JsLogicalOperator::LogicalAnd => parent = expression.parent::<RuleNode>(),
                    }
                }
                RuleNode::JsSequenceExpression(expression) => {
                    let is_last_in_sequence = expression.parent::<JsSequenceExpression>().is_none();

                    // need to check only the rightmost expression in the sequence
                    // a, b, c?.()
                    if is_last_in_sequence && expression.right().ok()?.syntax() == node.syntax() {
                        parent = expression.parent::<RuleNode>()
                    }
                }
                RuleNode::JsConditionalExpression(expression) => {
                    // need to check consequent and alternate branches
                    // (a ? obj?.foo : obj?.foo)();
                    // but not test expression
                    // (obj?.foo ? a : b)();
                    if node.syntax() == expression.consequent().ok()?.syntax()
                        || node.syntax() == expression.alternate().ok()?.syntax()
                    {
                        parent = expression.parent::<RuleNode>()
                    }
                }
                RuleNode::JsCallExpression(expression) => {
                    if expression.is_optional() {
                        // The current optional chain is inside another optional chain which will also be processed by the rule so we can skip current optional chain
                        // a?.b?.()
                        return None;
                    }

                    if is_inside_parenthesis {
                        // it means we've found a error because parentheses limit the scope
                        // (a?.b)()
                        return Some(expression.arguments().ok()?.syntax().text_trimmed_range());
                    }

                    // a()...
                    parent = expression.parent::<RuleNode>()
                }
                RuleNode::JsStaticMemberExpression(expression) => {
                    if expression.is_optional() {
                        // The current optional chain is inside another optional chain which will also be processed by the rule so we can skip current optional chain
                        // a?.b?.c
                        return None;
                    }

                    if is_inside_parenthesis {
                        // it means we've found a error because parentheses limit the scope
                        // (a?.b).c
                        return Some(expression.member().ok()?.syntax().text_trimmed_range());
                    }

                    // a.b....
                    parent = expression.parent::<RuleNode>()
                }
                RuleNode::JsComputedMemberExpression(expression) => {
                    if expression.is_optional() {
                        // The current optional chain is inside another optional chain which will also be processed by the rule so we can skip current optional chain
                        // a?.[b]?.[c]
                        return None;
                    }

                    if is_inside_parenthesis {
                        // it means we've found a error because parentheses limit the scope
                        // (a?.[b]).c
                        return Some(TextRange::new(
                            expression
                                .l_brack_token()
                                .ok()?
                                .text_trimmed_range()
                                .start(),
                            expression.r_brack_token().ok()?.text_trimmed_range().end(),
                        ));
                    }

                    // a[b]...
                    parent = expression.parent::<RuleNode>()
                }
                RuleNode::JsTemplateExpression(expression) => {
                    // a?.b``
                    // (a?.b)``
                    return Some(TextRange::new(
                        expression.l_tick_token().ok()?.text_trimmed_range().start(),
                        expression.r_tick_token().ok()?.text_trimmed_range().end(),
                    ));
                }
                RuleNode::JsForOfStatement(statement) => {
                    if node.syntax() == statement.expression().ok()?.syntax() {
                        // we can have an error only if we have an optional chain in the expression node
                        // for (foo of obj?.bar);
                        return Some(statement.syntax().text_trimmed_range());
                    }
                }
                RuleNode::JsWithStatement(statement) => {
                    if node.syntax() == statement.object().ok()?.syntax() {
                        // we can have an error only if we have an optional chain in the object part
                        // with (obj?.foo) {};
                        return Some(statement.syntax().text_trimmed_range());
                    }
                }
                RuleNode::JsInitializerClause(initializer) => {
                    if let Some(parent) = initializer.parent::<JsVariableDeclarator>() {
                        if matches!(
                            parent.id(),
                            Ok(AnyJsBindingPattern::JsObjectBindingPattern(_)
                                | AnyJsBindingPattern::JsArrayBindingPattern(_),)
                        ) {
                            return Some(parent.syntax().text_trimmed_range());
                        }
                    } else if let Some(parent) =
                        initializer.parent::<JsObjectAssignmentPatternProperty>()
                    {
                        if matches!(
                            parent.pattern(),
                            Ok(AnyJsAssignmentPattern::JsObjectAssignmentPattern(_)
                                | AnyJsAssignmentPattern::JsArrayAssignmentPattern(_),)
                        ) {
                            // ({bar: [ foo ] = obj?.prop} = {});
                            return Some(parent.range());
                        }
                    } else if let Some(parent) =
                        initializer.parent::<JsArrayAssignmentPatternElement>()
                    {
                        if matches!(
                            parent.pattern(),
                            Ok(AnyJsAssignmentPattern::JsObjectAssignmentPattern(_)
                                | AnyJsAssignmentPattern::JsArrayAssignmentPattern(_))
                        ) {
                            // [{ foo } = obj?.bar] = [];
                            return Some(parent.range());
                        }
                    }
                }
                RuleNode::JsAssignmentExpression(expression) => {
                    if matches!(
                        expression.left(),
                        Ok(AnyJsAssignmentPattern::JsObjectAssignmentPattern(_)
                            | AnyJsAssignmentPattern::JsArrayAssignmentPattern(_),)
                    ) {
                        return Some(expression.syntax().text_trimmed_range());
                    }
                }
                RuleNode::JsSpread(spread) => {
                    // it's not an error to have a spread inside object
                    // { ...a?.b }
                    if spread.parent::<JsObjectMemberList>().is_none() {
                        return Some(spread.syntax().text_trimmed_range());
                    }
                }
                RuleNode::JsInExpression(expression) => {
                    if node.syntax() == expression.object().ok()?.syntax() {
                        // we can have an error only if we have an optional chain in the object part
                        // a in foo?.bar;
                        return Some(expression.syntax().text_trimmed_range());
                    }
                }
                RuleNode::JsInstanceofExpression(expression) => {
                    if node.syntax() == expression.right().ok()?.syntax() {
                        // we can have an error only if we have an optional chain in the right part
                        // foo instanceof obj?.prop;
                        return Some(expression.syntax().text_trimmed_range());
                    }
                }
            };

            node = current_parent;
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.optional_chain_token()?.text_trimmed_range(),
                markup! {
                    "Unsafe usage of optional chaining."
                },
            )
            .detail(
                range,
                "If it short-circuits with 'undefined' the evaluation will throw TypeError here:",
            ),
        )
    }
}

declare_node_union! {
    /// Only these variants of the union can be part of an unsafe optional chain.
    pub RuleNode =
    JsLogicalExpression
    | JsSequenceExpression
    | JsConditionalExpression
    | JsAwaitExpression
    | JsParenthesizedExpression
    | JsCallExpression
    | JsNewExpression
    | JsStaticMemberExpression
    | JsComputedMemberExpression
    | JsTemplateExpression
    | JsForOfStatement
    | JsWithStatement
    | JsInitializerClause
    | JsAssignmentExpression
    | JsSpread
    | JsExtendsClause
    | JsInExpression
    | JsInstanceofExpression
}

impl From<AnyJsOptionalChainExpression> for RuleNode {
    fn from(node: AnyJsOptionalChainExpression) -> RuleNode {
        match node {
            AnyJsOptionalChainExpression::JsCallExpression(expression) => {
                RuleNode::JsCallExpression(expression)
            }
            AnyJsOptionalChainExpression::JsStaticMemberExpression(expression) => {
                RuleNode::JsStaticMemberExpression(expression)
            }
            AnyJsOptionalChainExpression::JsComputedMemberExpression(expression) => {
                RuleNode::JsComputedMemberExpression(expression)
            }
        }
    }
}
