use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsFormalParameter, JsInitializerClause, JsSyntaxToken, TsPropertyParameter};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, Direction};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce default function parameters and optional function parameters to be last.
    ///
    /// Default and optional parameters that precede a required parameter cannot be omitted at call site.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f(a = 0, b) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(a, b = 0, c) {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function f(a: number, b?: number, c: number) {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Foo {
    ///     constructor(readonly a = 10, readonly b: number) {}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f(a, b = 0) {}
    /// ```
    ///
    /// ```ts
    /// function f(a: number, b?: number, c = 0) {}
    /// ```
    ///
    /// ```ts
    /// function f(a: number, b = 0, c?: number) {}
    /// ```
    ///
    pub UseDefaultParameterLast {
        version: "1.0.0",
        name: "useDefaultParameterLast",
        language: "js",
        sources: &[
            RuleSource::Eslint("default-param-last"),
            RuleSource::EslintTypeScript("default-param-last")
        ],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyFormalParameter = JsFormalParameter | TsPropertyParameter
}

impl AnyFormalParameter {
    pub(crate) fn is_required(&self) -> bool {
        self.question_mark_token().is_none() && self.initializer().is_none()
    }

    pub(crate) fn initializer(&self) -> Option<JsInitializerClause> {
        match self {
            AnyFormalParameter::JsFormalParameter(x) => x.initializer(),
            AnyFormalParameter::TsPropertyParameter(x) => x
                .formal_parameter()
                .ok()?
                .as_js_formal_parameter()?
                .initializer(),
        }
    }

    pub(crate) fn question_mark_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyFormalParameter::JsFormalParameter(x) => x.question_mark_token(),
            AnyFormalParameter::TsPropertyParameter(x) => x
                .formal_parameter()
                .ok()?
                .as_js_formal_parameter()?
                .question_mark_token(),
        }
    }
}

impl Rule for UseDefaultParameterLast {
    type Query = Ast<AnyFormalParameter>;
    type State = AnyFormalParameter;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let formal_param = ctx.query();
        if formal_param.is_required() {
            return None;
        }
        let last_required_param = formal_param
            .syntax()
            .siblings(Direction::Next)
            .filter_map(AnyFormalParameter::cast)
            .filter(|x| x.is_required())
            .last();
        last_required_param
    }

    fn diagnostic(
        ctx: &RuleContext<Self>,
        last_required_param: &Self::State,
    ) -> Option<RuleDiagnostic> {
        let formal_param = ctx.query();
        let param_kind = if formal_param.question_mark_token().is_some() {
            "optional"
        } else {
            "default"
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            formal_param.range(),
            markup! {
                "This "<Emphasis>{param_kind}" parameter"</Emphasis>" should follow the last "<Emphasis>"required parameter"</Emphasis>" or should be a "<Emphasis>"required parameter"</Emphasis>"."
            },
        ).detail(
            last_required_param.range(),
            markup! {
                "The last "<Emphasis>"required parameter"</Emphasis>" is here:"
            },
        ).note(
            markup! {
                "A "<Emphasis>{param_kind}" parameter"</Emphasis>" that precedes a "<Emphasis>"required parameter"</Emphasis>" cannot be omitted at call site."
            }
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let opt_param = ctx.query();
        let mut mutation = ctx.root().begin();
        if opt_param.question_mark_token().is_some() {
            let question_mark = opt_param.question_mark_token()?;
            let prev_token = question_mark.prev_token()?;
            let new_token =
                prev_token.append_trivia_pieces(question_mark.trailing_trivia().pieces());
            mutation.replace_token_discard_trivia(prev_token, new_token);
            mutation.remove_token(question_mark);
        } else {
            let initializer = opt_param.initializer()?;
            let prev_token = initializer.syntax().prev_sibling()?.last_token()?;
            let new_token = prev_token
                .trim_trailing_trivia()
                .append_trivia_pieces(initializer.syntax().last_trailing_trivia()?.pieces());
            mutation.replace_token_discard_trivia(prev_token, new_token);
            mutation.remove_node(initializer);
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {"Turn the parameter into a "<Emphasis>"required parameter"</Emphasis>"."}
                .to_owned(),
            mutation,
        ))
    }
}
