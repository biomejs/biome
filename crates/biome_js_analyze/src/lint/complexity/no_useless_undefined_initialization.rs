use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make::js_variable_declarator_list;
use biome_js_syntax::{JsLanguage, JsSyntaxToken, JsVariableDeclarator, JsVariableStatement};
use biome_rowan::{chain_trivia_pieces, SyntaxTriviaPiece};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow initializing variables to `undefined`.
    ///
    /// A variable that is declared and not initialized to any value automatically gets the value of `undefined`.
    /// It’s considered a best practice to avoid initializing variables to `undefined`.
    ///
    /// Please note that any inline comments attached to the initialization value or variable will be moved at the end of the variable declaration on auto-fix.
    /// Please be also aware that this differs from Eslint's behaviour.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = undefined;
    /// ```
    /// ```js,expect_diagnostic
    /// let b = undefined, c = 1, d = 2;
    /// ```
    /// ```js,expect_diagnostic
    /// for (let i = 0; i < 100; i++) {
    /// 	let i = undefined;
    /// }
    /// ```
    /// ```js,expect_diagnostic
    /// let f = /**/undefined/**/ ;
    /// ```
    /// ### Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    /// ```js
    /// class Foo {
    /// 	bar = undefined;
    /// }
    /// ```
    ///
    pub NoUselessUndefinedInitialization {
        version: "1.7.2",
        name: "noUselessUndefinedInitialization",
        language: "js",
        sources: &[RuleSource::Eslint("no-undef-init")],
        fix_kind: FixKind::Safe,
        recommended: false,
    }
}

impl Rule for NoUselessUndefinedInitialization {
    type Query = Ast<JsVariableStatement>;
    type State = (Box<str>, TextRange);
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let statement = ctx.query();

        let mut signals = vec![];
        let Ok(node) = statement.declaration() else {
            return signals.into_boxed_slice();
        };

        let let_or_var_kind = node.is_let() || node.is_var();

        if !let_or_var_kind {
            return signals.into_boxed_slice();
        }

        for declarator in node.declarators() {
            let Ok(decl) = declarator else { continue };

            let Some(initializer) = decl.initializer() else {
                continue;
            };

            let Some(keyword) = initializer
                .expression()
                .ok()
                .and_then(|expression| expression.as_js_reference_identifier())
            else {
                continue;
            };

            if keyword.is_undefined() {
                let decl_range = initializer.range();
                let Some(binding_name) = decl.id().ok().map(|id| id.to_trimmed_string()) else {
                    continue;
                };
                signals.push((binding_name.into(), decl_range));
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.1,
            markup! {
                "It's not necessary to initialize "<Emphasis>{state.0.as_ref()}</Emphasis>" to undefined."
            }).note("A variable that is declared and not initialized to any value automatically gets the value of undefined.")
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let assignment_statement = node.clone();

        let current_declaration_statement = node.clone().declaration().ok()?;
        let declarators = current_declaration_statement.declarators();

        let current_declaration = declarators
            .clone()
            .into_iter()
            .filter_map(|declarator| declarator.ok())
            .find(|decl| {
                decl.id()
                    .ok()
                    .and_then(|id| {
                        id.as_any_js_binding()?
                            .as_js_identifier_binding()?
                            .name_token()
                            .ok()
                    })
                    .is_some_and(|id| id.text_trimmed() == state.0.as_ref())
            })?;

        let current_initializer = current_declaration.initializer()?;

        let eq_token_trivia = current_initializer
            .eq_token()
            .map(|token| token.trailing_trivia())
            .ok()?
            .pieces();

        let expression_trivia = current_initializer
            .expression()
            .ok()?
            .as_js_reference_identifier()
            .map(|reference| reference.value_token())?
            .ok()?
            .trailing_trivia()
            .pieces();

        // Save the separators too
        let separators_syntax = declarators.clone().into_syntax();
        let separators: Vec<JsSyntaxToken> = separators_syntax.tokens().collect();

        let new_declaration = current_declaration.clone().with_initializer(None);
        let new_declarators: Vec<JsVariableDeclarator> = declarators
            .clone()
            .into_iter()
            .filter_map(|decl| decl.ok())
            .map(|decl| {
                if decl == current_declaration {
                    new_declaration.clone()
                } else {
                    decl
                }
            })
            .collect();

        // Recreate the declaration statement with updated declarators
        let new_declaration_statement = current_declaration_statement
            .with_declarators(js_variable_declarator_list(new_declarators, separators));

        let chained_comments: Vec<SyntaxTriviaPiece<JsLanguage>> =
            chain_trivia_pieces(eq_token_trivia, expression_trivia)
                .filter(|trivia| trivia.is_comments())
                .collect();

        // Create the whole statement using updated subtree and append comments to the statement
        let new_node = assignment_statement
            .clone()
            .with_declaration(new_declaration_statement)
            .append_trivia_pieces(chained_comments)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(assignment_statement, new_node);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove undefined initialization." }.to_owned(),
            mutation,
        ))
    }
}
