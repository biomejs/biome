use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsLanguage};
use biome_rowan::{
    AstSeparatedList, BatchMutationExt, NodeOrToken, SyntaxToken, TextRange, TokenText,
};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `String.trimStart()` and `String.trimEnd()` over `String.trimLeft()` and `String.trimRight()`.
    ///
    /// While `String.trimLeft()` and `String.trimRight()` are aliases for `String.trimStart()` and `String.trimEnd()`,
    /// only using the latter pair ensures consistency and is preferable for their direction-independent wording.
    ///
    /// Note that `String.trimStart()` and `String.trimEnd()` methods do not take any parameters. Any arguments passed to these methods will be ignored.
    /// See the MDN documentation for more details:
    /// - [String.prototype.trimStart()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart)
    /// - [String.prototype.trimEnd()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = bar.trimLeft();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = bar.trimRight();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = bar.trimStart();
    /// ```
    ///
    /// ```js
    /// const foo = bar.trimEnd();
    /// ```
    ///
    pub UseTrimStartEnd {
        version: "next",
        name: "useTrimStartEnd",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-string-trim-start-end")],
        fix_kind: FixKind::Safe,
    }
}

#[derive(Debug, Clone)]
pub struct UseTrimStartEndState {
    member_name: TokenText,
    span: TextRange,
    replaced_member_name: String,
}

impl Rule for UseTrimStartEnd {
    type Query = Ast<JsCallExpression>;
    type State = UseTrimStartEndState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let arguments = node.arguments().ok()?;
        let args = arguments.args();

        if !args.is_empty() {
            // If arguments are present, it suggests this function call may not be intended for `String.trimStart()` or `String.trimEnd()`,
            // as these methods do not accept parameters according to the specification:
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart#parameters
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd#parameters
            return None;
        }

        let callee = node.callee().ok()?;
        let token = generate_syntax_token(callee)?;
        let suggested_name = suggested_name(token.clone());

        Some(UseTrimStartEndState {
            member_name: token.token_text_trimmed(),
            span: token.text_range(),
            replaced_member_name: suggested_name,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let member_name = state.member_name.text();
        let replaced_member_name = state.replaced_member_name.clone();

        let diagnostic_message = markup! {
            "Use "{replaced_member_name}" instead of "{member_name}"."
        }
        .to_owned();
        let note_message = {
            markup! {
                ""{member_name}"() is an alias for "{replaced_member_name}"."
            }
            .to_owned()
        };

        Some(
            RuleDiagnostic::new(rule_category!(), state.span, diagnostic_message)
                .note(note_message),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let token = generate_syntax_token(callee)?;

        let member_name = state.member_name.text();
        let replaced_member_name = state.replaced_member_name.clone();
        let replaced_function = make::js_name(make::ident(&replaced_member_name));
        let mut mutation = ctx.root().begin();
        mutation.replace_element(NodeOrToken::Token(token), replaced_function.into());

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>{member_name}</Emphasis>" with "<Emphasis>{replaced_member_name}</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}

fn generate_syntax_token(callee: AnyJsExpression) -> Option<SyntaxToken<JsLanguage>> {
    let token = if let AnyJsExpression::JsComputedMemberExpression(expression) = callee {
        let member = expression.member().ok()?;
        match member {
            AnyJsExpression::AnyJsLiteralExpression(literal) => literal.value_token().ok(),
            AnyJsExpression::JsTemplateExpression(element) => {
                element.elements().into_iter().find_map(|x| {
                    x.as_js_template_chunk_element()
                        .and_then(|chunk| chunk.template_chunk_token().ok())
                })
            }
            _ => None,
        }
    } else if let AnyJsExpression::JsStaticMemberExpression(expression) = callee {
        expression.member().ok()?.value_token().ok()
    } else {
        None
    };
    token
}

// Handle "'text'" and "\"text\"" and "text" cases
fn suggested_name(text: SyntaxToken<JsLanguage>) -> String {
    let trimmed = text.text_trimmed();
    let first_char = trimmed.chars().next();
    let last_char = trimmed.chars().last();

    let is_single_quoted = first_char == Some('\'') && last_char == Some('\'');
    let is_double_quoted = first_char == Some('"') && last_char == Some('"');

    let unquoted = trimmed.trim_matches(|c| c == '\'' || c == '"');

    let cleaned = match unquoted {
        "trimLeft" => "trimStart",
        "trimRight" => "trimEnd",
        _ => unquoted,
    };

    if is_single_quoted {
        format!("'{}'", cleaned)
    } else if is_double_quoted {
        format!("\"{}\"", cleaned)
    } else {
        cleaned.to_string()
    }
}
