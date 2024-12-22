use crate::{
    services::control_flow::AnyJsControlFlowRoot, services::semantic::Semantic, JsRuleAction,
};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsVariableDeclaration, JsModule, JsScript, JsSyntaxKind, TsGlobalDeclaration,
};

use crate::lint::style::use_const::ConstBindings;
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Disallow the use of `var`
    ///
    /// ECMAScript 6 allows programmers to create variables with block scope instead of function scope using the let and const keywords.
    ///
    /// Block scope is common in many other programming languages and helps programmers avoid mistakes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var foo = 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = 1;
    /// let bar = 1;
    ///```
    pub NoVar {
        version: "1.0.0",
        name: "noVar",
        language: "js",
        sources: &[RuleSource::Eslint("no-var")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoVar {
    type Query = Semantic<AnyJsVariableDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declaration = ctx.query();
        if declaration.is_var() {
            let ts_global_declaratio = &declaration
                .syntax()
                .ancestors()
                .find_map(TsGlobalDeclaration::cast);

            if ts_global_declaratio.is_some() {
                return None;
            }
            return Some(());
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let declaration = ctx.query();
        let var_scope = declaration
            .syntax()
            .ancestors()
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))?;
        let contextual_note = if JsScript::can_cast(var_scope.kind()) {
            markup! {
                "A variable declared with "<Emphasis>"var"</Emphasis>" in the global scope pollutes the global object."
            }
        } else if JsModule::can_cast(var_scope.kind()) {
            markup! {
                "A variable declared with "<Emphasis>"var"</Emphasis>" is accessible in the whole module. Thus, the variable can be accessed before its initialization and outside the block where it is declared."
            }
        } else {
            markup! {
                "A variable declared with "<Emphasis>"var"</Emphasis>" is accessible in the whole body of the function. Thus, the variable can be accessed before its initialization and outside the block where it is declared."
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            declaration.range(),
            markup! {
                "Use "<Emphasis>"let"</Emphasis>" or "<Emphasis>"const"</Emphasis>" instead of "<Emphasis>"var"</Emphasis>"."
            },
        ).note(contextual_note).note(
            markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/var">"MDN web docs"</Hyperlink>" for more details."
            }
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let declaration = ctx.query();
        let model = ctx.model();
        let maybe_const = ConstBindings::new(declaration, model);
        // When a `var` is initialized and re-assigned `maybe_const` is `None`.
        // In this case we fall back to `let`.
        // Otherwise, we check if the `var` can be "fixed" to a `const`.
        let replacing_token_kind = if maybe_const.filter(|x| x.can_fix).is_some() {
            JsSyntaxKind::CONST_KW
        } else {
            JsSyntaxKind::LET_KW
        };
        let mut mutation = ctx.root().begin();
        mutation.replace_token(
            declaration.kind_token().ok()?,
            make::token(replacing_token_kind),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use '"<Emphasis>{replacing_token_kind.to_string()?}</Emphasis>"' instead." }
                .to_owned(),
            mutation,
        ))
    }
}
