use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    JsBreakStatement, JsContinueStatement, JsDoWhileStatement, JsForInStatement, JsForOfStatement,
    JsForStatement, JsSwitchStatement, JsSyntaxToken, JsWhileStatement, TextRange,
};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_break_in_nested_loop::NoBreakInNestedLoopOptions;

use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow `break` statements inside nested loops and inside `switch` statements in loops.
    ///
    /// When a loop or a `switch` statement is inside another loop, it's easy to misread which
    /// one a `break` statement exits. Move the inner loop or `switch` statement into a separate
    /// function, so that each `break` statement clearly belongs to one loop.
    ///
    /// A `break` or `continue` statement with a label, such as `break outer;`, isn't reported,
    /// because the label names the statement it applies to. Statements inside a function that is
    /// written inside a loop are checked separately from that loop.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (const item of items) {
    ///     for (const child of item.children) {
    ///         if (child.done) {
    ///             break;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (const item of items) {
    ///     switch (item.type) {
    ///         case "hidden":
    ///             break;
    ///         default:
    ///             process(item);
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function processChildren(item) {
    ///     for (const child of item.children) {
    ///         if (child.done) {
    ///             break;
    ///         }
    ///     }
    /// }
    ///
    /// for (const item of items) {
    ///     processChildren(item);
    /// }
    ///
    /// for (const item of items) {
    ///     for (const child of item.children) {
    ///         check(child);
    ///     }
    /// }
    ///
    /// outer: for (const item of items) {
    ///     for (const child of item.children) {
    ///         if (child.done) {
    ///             break outer;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `checkContinue`
    ///
    /// Type: `boolean`
    ///
    /// Default: `false`
    ///
    /// Also report `continue` statements without a label inside nested loops and inside
    /// `switch` statements in loops. It's off by default so that early `continue` statements in
    /// an inner loop, such as `if (!item.isValid) continue;`, are allowed.
    ///
    /// A `continue` statement inside a `switch` statement doesn't move to the next `case`: it
    /// skips to the next iteration of the loop around the `switch` statement. When this option is
    /// enabled, add a label to that loop and use it in the `continue` statement to make this clear.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "checkContinue": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// for (const group of groups) {
    ///     for (const item of group) {
    ///         if (!item.isValid) {
    ///             continue;
    ///         }
    ///         save(item);
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options,expect_diagnostic
    /// for (const item of items) {
    ///     switch (item.type) {
    ///         case "hidden":
    ///             continue;
    ///     }
    ///     process(item);
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// itemsLoop: for (const item of items) {
    ///     switch (item.type) {
    ///         case "hidden":
    ///             continue itemsLoop;
    ///     }
    ///     process(item);
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - The [`useEarlyContinue`](https://biomejs.dev/linter/rules/use-early-continue/) rule
    ///   suggests early `continue` statements, which `checkContinue` reports inside nested loops.
    ///
    pub NoBreakInNestedLoop {
        version: "2.6.10",
        name: "noBreakInNestedLoop",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-break-in-nested-loop").same()],
        recommended: false,
        severity: Severity::Information,
    }
}

declare_node_union! {
    pub AnyJsJumpStatement = JsBreakStatement | JsContinueStatement
}

declare_node_union! {
    /// A statement that a `break` or `continue` statement without a label can apply to.
    pub AnyJsJumpTarget =
        JsDoWhileStatement
        | JsForInStatement
        | JsForOfStatement
        | JsForStatement
        | JsSwitchStatement
        | JsWhileStatement
}

pub enum NestedJump {
    /// The statement applies to `target`, which is inside `outer_loop`.
    InsideNestedStatement {
        target: AnyJsJumpTarget,
        outer_loop: AnyJsJumpTarget,
    },
    /// A `continue` statement is inside a `switch` statement, and applies to `target_loop` around
    /// that `switch` statement.
    ContinueInSwitch { target_loop: AnyJsJumpTarget },
}

impl Rule for NoBreakInNestedLoop {
    type Query = Ast<AnyJsJumpStatement>;
    type State = NestedJump;
    type Signals = Option<Self::State>;
    type Options = NoBreakInNestedLoopOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let is_break = match node {
            AnyJsJumpStatement::JsBreakStatement(statement) => {
                if statement.label_token().is_some() {
                    return None;
                }
                true
            }
            AnyJsJumpStatement::JsContinueStatement(statement) => {
                if statement.label_token().is_some() || !ctx.options().check_continue() {
                    return None;
                }
                false
            }
        };

        let mut target: Option<AnyJsJumpTarget> = None;
        let mut is_inside_switch = false;
        for ancestor in node.syntax().ancestors().skip(1) {
            // A `break` or `continue` statement can't reach a loop outside the function or
            // class member that contains it.
            if AnyJsControlFlowRoot::can_cast(ancestor.kind()) {
                return None;
            }
            let Some(statement) = AnyJsJumpTarget::cast(ancestor) else {
                continue;
            };
            if let Some(target) = &target {
                if statement.is_loop() {
                    return Some(NestedJump::InsideNestedStatement {
                        target: target.clone(),
                        outer_loop: statement,
                    });
                }
            } else if is_break || statement.is_loop() {
                if is_inside_switch {
                    return Some(NestedJump::ContinueInSwitch {
                        target_loop: statement,
                    });
                }
                target = Some(statement);
            } else {
                // A `continue` statement skips over `switch` statements to the closest loop.
                is_inside_switch = true;
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let keyword = node.keyword_token().ok()?;
        let keyword = keyword.text_trimmed();
        let diagnostic = match state {
            NestedJump::InsideNestedStatement { target, outer_loop } => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>{keyword}</Emphasis>" statement applies to a "{target}" that is inside an outer loop."
                },
            )
            .detail(
                target.keyword_range()?,
                markup! {
                    "The "<Emphasis>{keyword}</Emphasis>" statement applies to this "{target}"."
                },
            )
            .detail(
                outer_loop.keyword_range()?,
                markup! {
                    "This is the outer loop."
                },
            )
            .note(markup! {
                "When loops and "<Emphasis>"switch"</Emphasis>" statements are nested, it's easy to misread which one a "<Emphasis>{keyword}</Emphasis>" statement applies to."
            })
            .note(markup! {
                "Move the inner "{target}" into a separate function."
            }),
            NestedJump::ContinueInSwitch { target_loop } => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>"continue"</Emphasis>" statement is inside a "<Emphasis>"switch"</Emphasis>" statement, but it skips to the next iteration of the loop instead of moving to the next "<Emphasis>"case"</Emphasis>"."
                },
            )
            .detail(
                target_loop.keyword_range()?,
                markup! {
                    "The "<Emphasis>"continue"</Emphasis>" statement applies to this loop."
                },
            )
            .note(markup! {
                "Add a label to the loop and use it in the "<Emphasis>"continue"</Emphasis>" statement, as in "<Emphasis>"continue label;"</Emphasis>", to make it clear which loop it applies to."
            }),
        };
        Some(diagnostic)
    }
}

impl AnyJsJumpStatement {
    fn keyword_token(&self) -> biome_rowan::SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsBreakStatement(statement) => statement.break_token(),
            Self::JsContinueStatement(statement) => statement.continue_token(),
        }
    }
}

impl AnyJsJumpTarget {
    const fn is_loop(&self) -> bool {
        !matches!(self, Self::JsSwitchStatement(_))
    }

    /// Returns the range of the keyword that starts the statement, such as `for` or `switch`.
    fn keyword_range(&self) -> Option<TextRange> {
        Some(self.syntax().first_token()?.text_trimmed_range())
    }
}

impl biome_console::fmt::Display for AnyJsJumpTarget {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        if self.is_loop() {
            fmt.write_str("loop")
        } else {
            fmt.write_markup(markup! { <Emphasis>"switch"</Emphasis>" statement" })
        }
    }
}
