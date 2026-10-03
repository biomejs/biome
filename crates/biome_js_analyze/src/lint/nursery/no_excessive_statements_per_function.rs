use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsFunction, AnyJsRoot, JsBlockStatement, JsConstructorClassMember, JsFunctionBody,
    JsGetterClassMember, JsGetterObjectMember, JsMethodClassMember, JsMethodObjectMember,
    JsSetterClassMember, JsSetterObjectMember, JsStaticInitializationBlockClassMember,
};
use biome_rowan::{AstNode, AstNodeList, TextRange, WalkEvent, declare_node_union};
use biome_rule_options::no_excessive_statements_per_function::NoExcessiveStatementsPerFunctionOptions;

declare_lint_rule! {
    /// Enforce a maximum number of statements allowed in a function.
    ///
    /// Functions with many statements tend to do too many things at once,
    /// which makes them harder to read, test, and maintain.
    ///
    /// The rule checks every kind of function: function declarations and expressions,
    /// arrow functions, methods, getters, setters, and constructors.
    ///
    /// The rule counts every statement in the function body, including the statements
    /// inside braces (`{ }`), such as the body of an `if` statement, a loop, or a `try`/`catch`.
    /// A statement without braces, such as `b` in `if (a) b;`, or a statement in a `switch` case,
    /// doesn't add to the count.
    /// The statements of a nested function count towards the nested function only.
    ///
    /// Class static blocks (`static { }` inside a class) aren't checked,
    /// and their statements don't count towards the function that contains the class.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     const foo1 = 1;
    ///     const foo2 = 2;
    ///     const foo3 = 3;
    ///     const foo4 = 4;
    ///     const foo5 = 5;
    ///     const foo6 = 6;
    ///     const foo7 = 7;
    ///     const foo8 = 8;
    ///     const foo9 = 9;
    ///     const foo10 = 10;
    ///     const foo11 = 11;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo(items) {
    ///     const result = [];
    ///     for (const item of items) {
    ///         if (item.enabled) {
    ///             const value = item.value;
    ///             const label = item.label;
    ///             const description = item.description;
    ///             result.push({ value, label, description });
    ///         } else {
    ///             const fallback = item.fallback;
    ///             const reason = item.reason;
    ///             result.push({ fallback, reason });
    ///         }
    ///     }
    ///     return result;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo() {
    ///     const foo1 = 1;
    ///     const foo2 = 2;
    ///     const foo3 = 3;
    ///     const foo4 = 4;
    ///     const foo5 = 5;
    ///     const foo6 = 6;
    ///     const foo7 = 7;
    ///     const foo8 = 8;
    ///     const foo9 = 9;
    ///     return function () {
    ///         // The statements of this function don't count towards `foo`.
    ///         let bar;
    ///         let baz;
    ///         return 42;
    ///     };
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `max`
    ///
    /// The maximum number of statements allowed in a function.
    ///
    /// Default: `10`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "max": 2
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// function foo() {
    ///     const a = 1;
    ///     const b = 2;
    ///     return a + b;
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// function foo() {
    ///     const a = 1;
    ///     return a + 2;
    /// }
    /// ```
    ///
    /// ### `ignoreTopLevelFunctions`
    ///
    /// When set to `true`, a top-level function (a function that isn't nested in another function)
    /// isn't checked, as long as it's the only top-level function in the file.
    /// This is useful for files whose code is entirely wrapped in a single function,
    /// such as `(function () { ... })()` or `define(function () { ... })`.
    /// If the file contains several top-level functions, all of them are checked.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "max": 2,
    ///         "ignoreTopLevelFunctions": true
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// (function () {
    ///     const a = 1;
    ///     const b = 2;
    ///     const c = 3;
    /// })();
    ///
    /// (function () {
    ///     const a = 1;
    /// })();
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// (function () {
    ///     const a = 1;
    ///     const b = 2;
    ///     const c = 3;
    /// })();
    /// ```
    ///
    pub NoExcessiveStatementsPerFunction {
        version: "next",
        name: "noExcessiveStatementsPerFunction",
        language: "js",
        sources: &[RuleSource::Eslint("max-statements").same()],
        severity: Severity::Warning,
        recommended: false,
    }
}

impl Rule for NoExcessiveStatementsPerFunction {
    type Query = Ast<JsFunctionBody>;
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = NoExcessiveStatementsPerFunctionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let body = ctx.query();
        let options = ctx.options();

        let count = count_statements(body);
        if count <= usize::from(options.max()) {
            return None;
        }

        if options.ignore_top_level_functions() {
            let function = body.parent::<AnyStatementCountBoundary>()?;
            if is_top_level(&function) && !has_multiple_top_level_functions(&ctx.root()) {
                return None;
            }
        }

        Some(count)
    }

    fn diagnostic(ctx: &RuleContext<Self>, count: &Self::State) -> Option<RuleDiagnostic> {
        let body = ctx.query();
        let max = ctx.options().max();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                function_head_range(body)?,
                markup! {
                    "This function has too many statements ("{count}"). Maximum allowed is "{max}"."
                },
            )
            .note(markup! {
                "Functions with many statements are harder to read, test, and maintain."
            })
            .note(markup! {
                "Consider splitting this function into smaller functions."
            }),
        )
    }
}

declare_node_union! {
    /// A node whose body is a separate scope for statement counting.
    ///
    /// Statements inside such a node never count towards an enclosing function.
    pub AnyStatementCountBoundary =
        AnyJsFunction
        | JsMethodClassMember
        | JsMethodObjectMember
        | JsConstructorClassMember
        | JsGetterClassMember
        | JsSetterClassMember
        | JsGetterObjectMember
        | JsSetterObjectMember
        | JsStaticInitializationBlockClassMember
}

/// Counts the statements of a function body, including the directives and the statements
/// of nested block statements, but excluding the statements of nested functions and
/// class static blocks.
fn count_statements(body: &JsFunctionBody) -> usize {
    let mut count = body.directives().len() + body.statements().len();
    let mut events = body.syntax().preorder();
    while let Some(event) = events.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if AnyStatementCountBoundary::KIND_SET.matches(node.kind()) {
            events.skip_subtree();
        } else if let Some(block) = JsBlockStatement::cast(node) {
            count += block.statements().len();
        }
    }
    count
}

fn is_top_level(function: &AnyStatementCountBoundary) -> bool {
    !function
        .syntax()
        .ancestors()
        .skip(1)
        .any(|ancestor| AnyStatementCountBoundary::KIND_SET.matches(ancestor.kind()))
}

/// Returns `true` if the file has more than one function that isn't nested in another
/// function or in a class static block.
///
/// Expression-bodied arrow functions are top-level functions too.
fn has_multiple_top_level_functions(root: &AnyJsRoot) -> bool {
    let mut count = 0;
    let mut events = root.syntax().preorder();
    while let Some(event) = events.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if AnyStatementCountBoundary::KIND_SET.matches(node.kind()) {
            events.skip_subtree();
            if !JsStaticInitializationBlockClassMember::can_cast(node.kind()) {
                count += 1;
                if count > 1 {
                    return true;
                }
            }
        }
    }
    false
}

/// Returns the range from the start of the function to the end of the token preceding its body,
/// so the diagnostic doesn't span the whole function.
///
/// Decorators are part of the range, so a suppression comment placed above them applies.
fn function_head_range(body: &JsFunctionBody) -> Option<TextRange> {
    let function = body.parent::<AnyStatementCountBoundary>()?;
    let last_head_token = body.syntax().first_token()?.prev_token()?;
    Some(TextRange::new(
        function.syntax().text_trimmed_range().start(),
        last_head_token.text_trimmed_range().end(),
    ))
}
