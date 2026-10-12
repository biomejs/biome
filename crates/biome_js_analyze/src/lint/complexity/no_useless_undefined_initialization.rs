use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::js_variable_declarator_list;
use biome_js_semantic::{Binding, CanBeImportedExported, Scope};
use biome_js_syntax::{
    AnyJsFunction, JsIdentifierBinding, JsLanguage, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
    JsVariableDeclarator, JsVariableStatement,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rowan::{SyntaxTriviaPiece, chain_trivia_pieces};
use biome_rule_options::no_useless_undefined_initialization::NoUselessUndefinedInitializationOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

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
    /// Exported variables are not flagged because in some frameworks (e.g., Svelte 4),
    /// initializing exported variables to `undefined` is used to declare optional props.
    ///
    /// ```js
    /// export let x = undefined;
    /// ```
    /// ```js
    /// let y = undefined;
    /// export { y };
    /// ```
    ///
    pub NoUselessUndefinedInitialization {
        version: "1.7.2",
        name: "noUselessUndefinedInitialization",
        language: "js",
        sources: &[RuleSource::Eslint("no-undef-init").same()],
        fix_kind: FixKind::Safe,
        recommended: true,
        severity: Severity::Information,
    }
}

impl Rule for NoUselessUndefinedInitialization {
    type Query = Semantic<JsVariableStatement>;
    type State = JsVariableDeclarator;
    type Signals = Box<[Self::State]>;
    type Options = NoUselessUndefinedInitializationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let statement = ctx.query();
        let model = ctx.model();

        let mut signals = vec![];
        let Ok(node) = statement.declaration() else {
            return signals.into_boxed_slice();
        };

        let let_or_var_kind = node.is_let() || node.is_var();

        if !let_or_var_kind {
            return signals.into_boxed_slice();
        }

        let is_var = node.is_var();

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
                let id_binding = decl
                    .id()
                    .ok()
                    .and_then(|id| id.as_any_js_binding()?.as_js_identifier_binding().cloned());

                // Skip if the variable is exported.
                // In frameworks like Svelte 4, exported variables with `undefined`
                // initialization are used to declare optional props.
                let is_exported = id_binding
                    .as_ref()
                    .is_some_and(|binding| binding.is_exported(model));

                if is_exported {
                    continue;
                }

                // For `var`, resetting to undefined inside loops or during redeclarations/reassignments
                // is meaningful and removing `= undefined` would alter runtime behavior.
                if is_var {
                    let Some(binding) = id_binding.as_ref() else {
                        continue;
                    };
                    let semantic_binding = model.as_binding(binding);
                    let hoisted_scope = model
                        .scope_hoisted_to(&semantic_binding.syntax())
                        .unwrap_or_else(|| semantic_binding.scope());

                    if is_var_in_loop(statement.syntax(), &hoisted_scope) {
                        continue;
                    }

                    if is_var_redeclaration_or_reassigned(
                        binding,
                        &semantic_binding,
                        &hoisted_scope,
                    ) {
                        continue;
                    }
                }

                signals.push(decl);
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = state.initializer()?.range();
        let binding_name = state
            .id()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?;
        let binding_name_text = binding_name.text_trimmed();

        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "It's not necessary to initialize "<Emphasis>{binding_name_text}</Emphasis>" to undefined."
            }).note("A variable that is declared and not initialized to any value automatically gets the value of undefined.")
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let assignment_statement = node.clone();

        let current_declaration_statement = node.clone().declaration().ok()?;
        let declarators = current_declaration_statement.declarators();

        let current_declaration = state.clone();

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

fn is_var_in_loop(node: &JsSyntaxNode, hoisted_scope: &Scope) -> bool {
    let hoisted_syntax = hoisted_scope.syntax();
    for ancestor in node.ancestors().skip(1) {
        if ancestor == hoisted_syntax.clone()
            || AnyJsFunction::can_cast(ancestor.kind())
            || matches!(
                ancestor.kind(),
                JsSyntaxKind::JS_CLASS_DECLARATION
                    | JsSyntaxKind::JS_CLASS_EXPRESSION
                    | JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
                    | JsSyntaxKind::JS_METHOD_CLASS_MEMBER
                    | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
                    | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
                    | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
                    | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
                    | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
                    | JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
            )
        {
            break;
        }
        if matches!(
            ancestor.kind(),
            JsSyntaxKind::JS_FOR_STATEMENT
                | JsSyntaxKind::JS_FOR_IN_STATEMENT
                | JsSyntaxKind::JS_FOR_OF_STATEMENT
                | JsSyntaxKind::JS_WHILE_STATEMENT
                | JsSyntaxKind::JS_DO_WHILE_STATEMENT
        ) {
            return true;
        }
    }
    false
}

fn is_var_redeclaration_or_reassigned(
    binding: &JsIdentifierBinding,
    semantic_binding: &Binding,
    hoisted_scope: &Scope,
) -> bool {
    let Ok(name_token) = binding.name_token() else {
        return false;
    };
    let name = name_token.token_text_trimmed();

    // Check if there are multiple bindings with the same name in the hoisted scope
    let mut count = 0;
    for b in hoisted_scope.bindings() {
        let has_same_name = b
            .tree()
            .name_token()
            .is_ok_and(|t| t.token_text_trimmed() == name);
        if has_same_name {
            count += 1;
            if count > 1 {
                return true;
            }
        }
    }

    // If hoisted scope is a function body, check parameter bindings in the function scope
    if hoisted_scope.syntax().kind() == JsSyntaxKind::JS_FUNCTION_BODY
        && let Some(parent_scope) = hoisted_scope.parent()
    {
        let matches_param = parent_scope.bindings().any(|b| {
            b.tree()
                .name_token()
                .is_ok_and(|t| t.token_text_trimmed() == name)
        });
        if matches_param {
            return true;
        }
    }

    // Check if there are write references prior to this declaration in text order
    let binding_start = binding.syntax().text_trimmed_range().start();
    for write in semantic_binding.all_writes() {
        if write.syntax().text_trimmed_range().start() < binding_start {
            return true;
        }
    }

    false
}

