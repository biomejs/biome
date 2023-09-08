use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsAssignmentPattern, AnyJsBindingPattern, AnyJsExpression, JsAssignmentExpression,
    JsAssignmentWithDefault, JsAwaitExpression, JsCallExpression, JsComputedMemberExpression,
    JsConditionalExpression, JsExtendsClause, JsForOfStatement, JsInExpression,
    JsInitializerClause, JsInstanceofExpression, JsLogicalExpression, JsLogicalOperator,
    JsNewExpression, JsObjectAssignmentPatternProperty, JsObjectMemberList,
    JsParenthesizedExpression, JsSequenceExpression, JsSpread, JsStaticMemberExpression,
    JsTemplateExpression, JsVariableDeclarator, JsWithStatement,
};
use rome_rowan::{declare_node_union, AstNode, TextRange};

declare_rule! {
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
    /// ## Valid
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
    pub(crate) NoUnsafeOptionalChaining {
        version: "1.0.0",
        name: "noUnsafeOptionalChaining",
        recommended: true,
    }
}

impl Rule for NoUnsafeOptionalChaining {
    type Query = Ast<QueryNode>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query_node = ctx.query();

        // need to check only optional chain nodes
        if !query_node.is_optional() {
            return None;
        }
        let mut node: RuleNode = RuleNode::cast_ref(query_node.syntax())?;
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
                            return Some(parent.syntax().text_trimmed_range());
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
                RuleNode::JsAssignmentWithDefault(assigment) => {
                    if matches!(
                        assigment.pattern(),
                        Ok(AnyJsAssignmentPattern::JsObjectAssignmentPattern(_)
                            | AnyJsAssignmentPattern::JsArrayAssignmentPattern(_))
                    ) {
                        // [{ foo } = obj?.bar] = [];
                        return Some(assigment.syntax().text_trimmed_range());
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
        let query_node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                query_node.range(),
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
    pub(crate) QueryNode = JsCallExpression | JsStaticMemberExpression | JsComputedMemberExpression
}

impl QueryNode {
    pub fn is_optional(&self) -> bool {
        match self {
            QueryNode::JsCallExpression(expression) => expression.is_optional(),
            QueryNode::JsStaticMemberExpression(expression) => expression.is_optional(),
            QueryNode::JsComputedMemberExpression(expression) => expression.is_optional(),
        }
    }

    pub fn range(&self) -> Option<TextRange> {
        let token = match self {
            QueryNode::JsCallExpression(expression) => expression.optional_chain_token(),
            QueryNode::JsStaticMemberExpression(expression) => expression.operator_token().ok(),
            QueryNode::JsComputedMemberExpression(expression) => expression.optional_chain_token(),
        };

        Some(token?.text_trimmed_range())
    }
}

impl From<QueryNode> for AnyJsExpression {
    fn from(node: QueryNode) -> AnyJsExpression {
        match node {
            QueryNode::JsCallExpression(expression) => expression.into(),
            QueryNode::JsStaticMemberExpression(expression) => expression.into(),
            QueryNode::JsComputedMemberExpression(expression) => expression.into(),
        }
    }
}

impl From<QueryNode> for RuleNode {
    fn from(node: QueryNode) -> RuleNode {
        match node {
            QueryNode::JsCallExpression(expression) => RuleNode::JsCallExpression(expression),
            QueryNode::JsStaticMemberExpression(expression) => {
                RuleNode::JsStaticMemberExpression(expression)
            }
            QueryNode::JsComputedMemberExpression(expression) => {
                RuleNode::JsComputedMemberExpression(expression)
            }
        }
    }
}

declare_node_union! {
    /// Only these variants of the union can be part of an unsafe optional chain.
    pub(crate) RuleNode =
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
    | JsAssignmentWithDefault
}
