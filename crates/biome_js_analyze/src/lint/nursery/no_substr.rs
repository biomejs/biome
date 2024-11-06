use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsName, JsCallArguments, JsExpressionStatement, JsSyntaxToken, JsVariableStatement,
};
use biome_rowan::{declare_node_union, AstSeparatedList, BatchMutationExt, TextRange, TokenText};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce the use of `String.slice()` over `String.substr()` and `String.substring()`.
    ///
    /// `String.slice()` is preferred over `String.substr()` and `String.substring()` because it is a more popular option with clearer behavior,
    ///  and it has a consistent counterpart in arrays.
    ///
    /// Note that `String.substr`, `String.substring` and `String.slice` are not identical when arguments are passed.
    /// For detailed differences, refer to the MDN documentation:
    /// - [The difference between substring() and substr()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#the_difference_between_substring_and_substr)
    /// - [Differences between substring() and slice()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#differences_between_substring_and_slice)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo.substr();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.substring();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo.slice(beginIndex, endIndex);
    /// ```
    ///
    pub NoSubstr {
        version: "1.8.2",
        name: "noSubstr",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-string-slice")],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoSubstr {
    type Query = Ast<AnyJsStatement>;
    type State = NoSubstrState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let value_token = node.value_token()?;

        if matches!(value_token.text_trimmed(), "substr" | "substring") {
            Some(NoSubstrState { value_token })
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic_message = markup! {
            "Avoid using "{state.member_name().text()}" and consider using slice instead."
        }
        .to_owned();
        let note_message = markup! {
        <Emphasis>"slice"</Emphasis>" is more commonly used and has a less surprising behavior."
        }
        .to_owned();
        let mdn_link =
            markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#the_difference_between_substring_and_substr">"MDN web docs"</Hyperlink>" for more details."
        }
        .to_owned();
        Some(
            RuleDiagnostic::new(rule_category!(), state.span(), diagnostic_message)
                .note(note_message)
                .note(mdn_link),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let arguments = node.arguments()?;
        let args = arguments.args();

        if !args.is_empty() {
            // If the function has arguments, we cannot replace it with slice() as it has different behavior.
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substring#differences_between_substring_and_slice
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/substr#description
            return None;
        }

        let mut mutation = ctx.root().begin();
        let replaced_function = make::js_name(make::ident("slice"));
        mutation.replace_element(node.member()?.into(), replaced_function.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>".slice()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone)]
pub struct NoSubstrState {
    value_token: JsSyntaxToken,
}

impl NoSubstrState {
    fn member_name(&self) -> TokenText {
        self.value_token.token_text_trimmed()
    }

    fn span(&self) -> TextRange {
        self.value_token.text_range()
    }
}

// Helper union type to handle both JsExpressionStatement and JsVariableStatement.
// To handle arguments, we need to know the type of the statement.
declare_node_union! {
    pub AnyJsStatement = JsExpressionStatement | JsVariableStatement
}

impl AnyJsStatement {
    pub fn value_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsStatement::JsExpressionStatement(node) => {
                let callee = node
                    .expression()
                    .ok()?
                    .as_js_call_expression()?
                    .callee()
                    .ok()?;
                callee
                    .as_js_static_member_expression()?
                    .member()
                    .ok()?
                    .value_token()
                    .ok()
            }
            AnyJsStatement::JsVariableStatement(node) => {
                let declaration = node.declaration().ok()?;
                let declarators = declaration.declarators();
                declarators.into_iter().find_map(|declarator| {
                    let init = declarator.ok()?.initializer()?;
                    init.expression()
                        .ok()?
                        .as_js_static_member_expression()?
                        .member()
                        .ok()?
                        .value_token()
                        .ok()
                })
            }
        }
    }
    pub fn member(&self) -> Option<AnyJsName> {
        match self {
            AnyJsStatement::JsExpressionStatement(node) => {
                let callee = node
                    .expression()
                    .ok()?
                    .as_js_call_expression()?
                    .callee()
                    .ok()?;
                callee.as_js_static_member_expression()?.member().ok()
            }
            AnyJsStatement::JsVariableStatement(node) => {
                let declaration = node.declaration().ok()?;
                let declarators = declaration.declarators();
                declarators.into_iter().find_map(|declarator| {
                    let init = declarator.ok()?.initializer()?;
                    init.expression()
                        .ok()?
                        .as_js_static_member_expression()?
                        .member()
                        .ok()
                })
            }
        }
    }
    pub fn arguments(&self) -> Option<JsCallArguments> {
        match self {
            AnyJsStatement::JsExpressionStatement(node) => node
                .expression()
                .ok()?
                .as_js_call_expression()?
                .arguments()
                .ok(),
            _ => None,
        }
    }
}
