use crate::{
    services::control_flow::AnyJsControlFlowRoot, services::semantic::Semantic, JsRuleAction,
};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, JsArrowFunctionExpression,
    JsAssignmentExpression, JsExpressionStatement, JsIdentifierBinding, JsIdentifierExpression,
    JsThisExpression, JsVariableDeclaration, JsVariableDeclarator, JsVariableStatement, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

declare_lint_rule! {
    /// Disallow useless `this` aliasing.
    ///
    /// Arrow functions inherits `this` from their enclosing scope;
    /// this makes `this` aliasing useless in this situation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     method() {
    ///         const self = this;
    ///         return () => {
    ///             return self;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class A {
    ///     method() {
    ///         const self = this;
    ///         return function() {
    ///             this.g();
    ///             return self;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    pub NoUselessThisAlias {
        version: "1.0.0",
        name: "noUselessThisAlias",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("no-this-alias")],
        source_kind: RuleSourceKind::Inspired,
        recommended: true,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessThisAlias {
    type Query = Semantic<JsVariableDeclarator>;
    type State = JsIdentifierBinding;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declarator = ctx.query();
        let model = ctx.model();
        let mut is_this_alias = if let Some(initializer) = declarator.initializer() {
            let initializer = initializer.expression().ok()?.omit_parentheses();
            if !JsThisExpression::can_cast(initializer.syntax().kind()) {
                return None;
            }
            true
        } else {
            false
        };
        let Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(id))) =
            declarator.id()
        else {
            // Ignore destructuring
            return None;
        };
        let this_scope = declarator
            .syntax()
            .ancestors()
            .find_map(AnyJsControlFlowRoot::cast)?;
        for write in id.all_writes(model) {
            let assign = JsAssignmentExpression::cast(write.syntax().parent()?)?;
            let assign_right = assign.right().ok()?.omit_parentheses();
            if !JsThisExpression::can_cast(assign_right.syntax().kind()) {
                return None;
            }
            is_this_alias = true;
        }
        // This cehck is useful when the loop is not executed (no write).
        if !is_this_alias {
            return None;
        }
        for reference in id.all_references(model) {
            let current_this_scope = reference
                .syntax()
                .ancestors()
                .filter(|x| !JsArrowFunctionExpression::can_cast(x.kind()))
                .find_map(AnyJsControlFlowRoot::cast)?;
            if this_scope != current_this_scope {
                // The aliasing is required because they have not the same `this` scope.
                return None;
            }
        }
        Some(id)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let declarator = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                declarator.range(),
                markup! {
                    "This aliasing of "<Emphasis>"this"</Emphasis>" is unnecessary."
                },
            )
            .note(markup! {
                "Arrow functions inherits `this` from their enclosing scope."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, id: &Self::State) -> Option<JsRuleAction> {
        let declarator = ctx.query();
        let model = ctx.model();
        let var_decl = declarator
            .syntax()
            .ancestors()
            .find_map(JsVariableDeclaration::cast)?;
        let mut mutation = ctx.root().begin();
        let this_expr = AnyJsExpression::from(make::js_this_expression(make::token(T![this])));
        for read in id.all_reads(model) {
            let syntax = read.syntax();
            let syntax = syntax.parent()?;
            let expr = JsIdentifierExpression::cast(syntax)?;
            mutation.replace_node(expr.into(), this_expr.clone());
        }
        for write in id.all_writes(model) {
            let syntax = write.syntax();
            let syntax = syntax.parent()?;
            let statement = JsExpressionStatement::cast(syntax.parent()?)?;
            mutation.remove_node(statement);
        }
        let var_declarator_list = var_decl.declarators();
        if var_declarator_list.len() == 1 {
            if let Some(statement) = JsVariableStatement::cast(var_decl.syntax().parent()?) {
                if statement.semicolon_token().is_some() {
                    mutation.remove_token(statement.semicolon_token()?);
                }
            }
            mutation.remove_node(var_decl);
        } else {
            let mut deleted_comma = None;
            for (current_declarator, current_comma) in var_declarator_list
                .iter()
                .zip(var_declarator_list.separators())
            {
                deleted_comma = current_comma.ok();
                let current_declarator = current_declarator.ok()?;
                if &current_declarator == declarator {
                    break;
                }
            }
            mutation.remove_node(declarator.clone());
            mutation.remove_token(deleted_comma?);
        }
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>"this"</Emphasis>" instead of an alias."
            }
            .to_owned(),
            mutation,
        ))
    }
}
