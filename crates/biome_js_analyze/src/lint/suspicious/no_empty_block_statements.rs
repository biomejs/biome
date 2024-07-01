use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsConstructorParameter, JsBlockStatement, JsConstructorClassMember, JsFunctionBody,
    JsStaticInitializationBlockClassMember, JsSwitchStatement,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, SyntaxNodeCast};

declare_lint_rule! {
    /// Disallow empty block statements and static blocks.
    ///
    /// Empty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code.
    ///
    /// This rule disallows empty block statements and static blocks.
    /// This rule ignores block statements or static blocks which contain a comment (for example, in an empty catch or finally block of a try statement to indicate that execution should continue regardless of errors).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function emptyFunctionBody () {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     doSomething();
    /// } catch(ex) {
    ///
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   static {}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo () {
    ///     doSomething();
    /// }
    /// ```
    ///
    /// ```js
    /// try {
    ///   doSomething();
    /// } catch (ex) {
    ///   // continue regardless of error
    /// }
    /// ```
    ///
    pub NoEmptyBlockStatements {
        version: "1.3.0",
        name: "noEmptyBlockStatements",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-empty"),
            RuleSource::Eslint("no-empty-static-block"),
            RuleSource::Eslint("no-empty-function"),
            RuleSource::EslintTypeScript("no-empty-function"),
        ],
        recommended: false,
    }
}

declare_node_union! {
    pub Query = JsBlockStatement | JsFunctionBody | JsStaticInitializationBlockClassMember | JsSwitchStatement
}

impl Rule for NoEmptyBlockStatements {
    type Query = Ast<Query>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let is_empty = is_empty(query);
        let has_comments = query.syntax().has_comments_descendants();
        let is_constructor_with_ts_param_props_or_private =
            is_constructor_with_ts_param_props_or_private(query);

        (is_empty && !has_comments && !is_constructor_with_ts_param_props_or_private).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let query = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                query.range(),
                markup! {
                    "Unexpected empty block."
                },
            )
            .note(markup! {
                "Empty blocks are usually the result of an incomplete refactoring. Remove the empty block or add a comment inside it if it is intentional."
            }),
        )
    }
}

fn is_empty(query: &Query) -> bool {
    use Query::*;
    match query {
        JsFunctionBody(body) => body.directives().len() == 0 && body.statements().len() == 0,
        JsBlockStatement(block) => block.statements().len() == 0,
        JsStaticInitializationBlockClassMember(block) => block.statements().len() == 0,
        JsSwitchStatement(statement) => statement.cases().len() == 0,
    }
}

/// Check if the function is a constructor with TypeScript parameter properties, or a private/protected constructor.
///
/// https://www.typescriptlang.org/docs/handbook/2/classes.html#parameter-properties
fn is_constructor_with_ts_param_props_or_private(query: &Query) -> bool {
    let Query::JsFunctionBody(body) = query else {
        return false;
    };

    let Some(constructor) = body
        .syntax()
        .parent()
        .and_then(|node| node.cast::<JsConstructorClassMember>())
    else {
        return false;
    };

    let Ok(params) = constructor.parameters() else {
        return false;
    };
    let is_param_props = params
        .parameters()
        .into_iter()
        .any(|param| matches!(param, Ok(AnyJsConstructorParameter::TsPropertyParameter(_))));
    let is_private = constructor
        .modifiers()
        .into_iter()
        .any(|modifier| modifier.is_private() || modifier.is_protected());
    is_param_props || is_private
}
