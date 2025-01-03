use crate::services::semantic::Semantic;
use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{self};
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{JsIdentifierAssignment, JsSyntaxKind};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};

declare_lint_rule! {
    /// Prevents from having `const` variables being re-assigned.
    ///
    /// Trying to assign a value to a `const` will cause an `TypeError` when the code is executed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = 1;
    /// a = 4;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 2;
    /// a += 1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 1;
    /// ++a;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = 1, b = 2;
    ///
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 10;
    /// let b = 10;
    /// b = 20;
    /// ```
    ///
    pub NoConstAssign {
        version: "1.0.0",
        name: "noConstAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-const-assign")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoConstAssign {
    type Query = Semantic<JsIdentifierAssignment>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let id_binding = model.binding(node)?.tree();
        let decl = id_binding.declaration()?;
        if let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) =
            decl.parent_binding_pattern_declaration().unwrap_or(decl)
        {
            if declarator.declaration()?.is_const() {
                return Some(id_binding.range());
            }
        };
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = node.name_token().ok()?;
        let name = name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {"Can't assign "<Emphasis>{name}</Emphasis>" because it's a constant"},
            )
            .detail(
                state,
                markup! {"This is where the variable is defined as constant"},
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let model = ctx.model();
        let mut mutation = ctx.root().begin();
        let decl = model.binding(node)?.tree().declaration()?;
        if let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) =
            decl.parent_binding_pattern_declaration().unwrap_or(decl)
        {
            let const_token = declarator.declaration()?.kind_token().ok()?;
            let let_token = make::token(JsSyntaxKind::LET_KW);
            mutation.replace_token(const_token, let_token);
            return Some(JsRuleAction::new(
                            ctx.metadata().action_category(ctx.category(), ctx.group()),
                            ctx.metadata().applicability(),
                             markup! { "Replace "<Emphasis>"const"</Emphasis>" with "<Emphasis>"let"</Emphasis>" if you assign it to a new value." }
                                .to_owned(),
                            mutation,
            ));
        }
        None
    }
}
