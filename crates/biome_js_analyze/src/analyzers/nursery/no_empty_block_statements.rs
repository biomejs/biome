use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize::{
    json::{has_only_known_keys, VisitJsonNode},
    DeserializationDiagnostic, VisitNode,
};
use biome_js_syntax::{
    JsBlockStatement, JsCatchClause, JsFunctionBody, JsStaticInitializationBlockClassMember,
    JsSwitchStatement, JsSyntaxKind,
};
use biome_json_syntax::JsonLanguage;
use biome_rowan::{declare_node_union, AstNode, AstNodeList, SyntaxNode, TextRange};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

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
    /// ## Options
    ///
    /// The rule provides one option that is detailed in the following subsections.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "allowEmptyCatch": true
    ///     }
    /// }
    /// ```
    ///
    /// ### allowEmptyCatch
    ///
    /// When set to true allows empty catch clauses (that is, which do not contain a comment)
    ///
    /// Default: false
    ///
    /// Examples of additional correct code for this rule with the { "allowEmptyCatch": true } option:
    ///
    /// ```jsx
    /// try {
    ///     doSomething();
    /// } catch (ex) {}
    ///
    /// try {
    ///     doSomething();
    /// }
    /// catch (ex) {}
    /// finally {
    ///     /* continue regardless of error */
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
    pub(crate) Query = JsBlockStatement | JsFunctionBody | JsStaticInitializationBlockClassMember | JsCatchClause | JsSwitchStatement
}

impl Rule for NoEmptyBlockStatements {
    type Query = Ast<Query>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoEmptyBlockStatementsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        use Query::*;
        let query = ctx.query();
        let options = ctx.options();

        match query {
            JsCatchClause(_) => check_catch(query, options),
            _ => check_block(query),
        }
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
        JsCatchClause(clause) => match clause.body() {
            Ok(catch_body) => catch_body.statements().len() == 0,
            Err(_) => false,
        },
    }
}

fn check_catch(query: &Query, options: &NoEmptyBlockStatementsOptions) -> Option<TextRange> {
    let is_empty = is_empty(query);
    let has_comments = query.syntax().has_comments_descendants();
    let text_range = query.syntax().text_range();

    if is_empty && !has_comments && !options.allow_empty_catch {
        Some(text_range)
    } else {
        None
    }
}

fn check_block(query: &Query) -> Option<TextRange> {
    let is_empty = is_empty(query);
    let has_comments = query.syntax().has_comments_descendants();
    let text_range = query.syntax().text_range();
    let parent: SyntaxNode<biome_js_syntax::JsLanguage> = query.syntax().parent()?;
    let is_catch = matches!(parent.kind(), JsSyntaxKind::JS_CATCH_CLAUSE);

    if is_empty && !has_comments && !is_catch {
        Some(text_range)
    } else {
        None
    }
}

/// Rule's options.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Bpaf)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NoEmptyBlockStatementsOptions {
    /// If `true`, then empty catch blocks are allowed
    #[bpaf(hide)]
    #[serde(
        default = "default_allow_empty_catch",
        skip_serializing_if = "is_default_allow_empty_catch"
    )]
    pub allow_empty_catch: bool,
}

const fn default_allow_empty_catch() -> bool {
    false
}

const fn is_default_allow_empty_catch(allow_empty_catch: &bool) -> bool {
    *allow_empty_catch == default_allow_empty_catch()
}

impl NoEmptyBlockStatementsOptions {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["allowEmptyCatch"];
}

impl Default for NoEmptyBlockStatementsOptions {
    fn default() -> Self {
        Self {
            allow_empty_catch: default_allow_empty_catch(),
        }
    }
}

impl VisitNode<JsonLanguage> for NoEmptyBlockStatementsOptions {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, Self::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        if name_text == "allowEmptyCatch" {
            self.allow_empty_catch = self.map_to_boolean(&value, name_text, diagnostics)?
        }
        Some(())
    }
}
