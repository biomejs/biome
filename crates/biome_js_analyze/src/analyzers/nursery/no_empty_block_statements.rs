use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{
    JsBlockStatement, JsFunctionBody, JsStaticInitializationBlockClassMember, JsSwitchStatement,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, TextRange};

declare_rule! {
    /// Disallow empty block statements and static blocks.
    ///
    /// Empty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code.
    ///
    /// This rule disallows empty block statements and static blocks. This rule ignores block statements which contain a comment (for example, in an empty catch or finally block of a try statement to indicate that execution should continue regardless of errors).
    /// This rule also ignores static blocks which contain a comment.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-empty-static-block/
    /// Source: https://eslint.org/docs/latest/rules/no-empty/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo () {}
    ///
    /// const foo = () => {}
    ///
    /// function fooWithNestedEmptyBlock() {
    ///     let a = 1;
    ///     function shouldFail(){}
    ///     return a
    ///  }
    ///
    /// const fooWithNestedEmptyBlock = () => {
    ///     let a = 1;
    ///     const shouldFail = () => {}
    ///     return a
    ///  }
    /// let someVar;
    /// if (someVar) {
    /// }
    ///
    /// while (someVar) {
    /// }
    ///
    /// switch(someVar) {
    /// }
    /// try {
    ///     doSomething();
    /// } catch(ex) {
    ///
    /// } finally {
    ///
    /// }
    ///
    // class Foo {
    //   static {}
    // }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// function foo () {let a;}
    ///
    /// const foo = () => {let a;}
    ///
    /// function fooWithComment() {
    ///   // should work
    /// }
    ///
    /// const barWithComment = () => {
    ///   // should work
    /// }
    ///
    /// function fooWithMultilineComment() {
    ///   /**
    ///    * this should also work
    ///    */
    /// }
    ///
    /// const barWithMultilineComment = () => {
    ///   /**
    ///    * this should also work
    ///    */
    /// }
    ///
    ///
    /// if (foo) {
    ///   // empty
    /// }
    ///
    /// while (foo) {
    ///   /* empty */
    /// }
    ///
    /// try {
    ///   doSomething();
    /// } catch (ex) {
    ///   // continue regardless of error
    /// }
    ///
    /// try {
    ///   doSomething();
    /// } finally {
    ///   /* continue regardless of error */
    /// }
    ///
    /// class Foo {
    ///   static {
    ///       bar();
    ///   }
    /// }
    ///
    /// class Foo {
    ///   static {
    ///       // comment
    ///   }
    /// }
    /// ```
    ///
    pub(crate) NoEmptyBlockStatements {
        version: "next",
        name: "noEmptyBlockStatements",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) Query = JsBlockStatement | JsFunctionBody | JsStaticInitializationBlockClassMember | JsSwitchStatement
}

impl Rule for NoEmptyBlockStatements {
    type Query = Ast<Query>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let is_empty = is_empty(query);
        let has_comments = query.syntax().has_comments_descendants();
        let text_range = query.syntax().text_range();

        if is_empty && !has_comments {
            return Some(text_range);
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "No empty blocks allowed."
                },
            )
            .note(markup! {
                "Empty static blocks and block statements, while not technically errors, usually occur due to refactoring that wasn’t completed. They can cause confusion when reading code."
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
