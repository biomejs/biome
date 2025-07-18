use crate::services::semantic::SemanticServices;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::binding_ext::{AnyJsBindingDeclaration, AnyJsIdentifierBinding};
use biome_rowan::{AstNode, SyntaxNode, TextRange};

declare_lint_rule! {
    /// Enforce the use of variables within the scope they are defined.
    ///
    /// This rule generates warnings when variables are used outside of the block
    /// in which they were defined. This emulates C-style block scope.
    ///
    /// This rule helps newcomers to the language avoid difficult bugs with variable hoisting.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function doIf() {
    ///     if (true) {
    ///         var build = true;
    ///     }
    ///
    ///     console.log(build);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function doFor() {
    ///     for (var x = 1; x < 10; x++) {
    ///         var y = f(x);
    ///     }
    ///     console.log(y);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function doTryCatch() {
    ///     try {
    ///         var build = 1;
    ///     } catch (e) {
    ///         var f = build;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function doIf() {
    ///     var build;
    ///
    ///     if (true) {
    ///         build = true;
    ///     }
    ///
    ///     console.log(build);
    /// }
    /// ```
    ///
    /// ```js
    /// function doFor() {
    ///     for (var x = 1; x < 10; x++) {
    ///         var y = f(x);
    ///         console.log(y);
    ///     }
    /// }
    /// ```
    pub UseBlockScopedVar {
        version: "1.0.0",
        name: "useBlockScopedVar",
        language: "js",
        sources: &[RuleSource::Eslint("block-scoped-var").same()],
        recommended: false,
    }
}

#[derive(Debug)]
pub struct BlockScopeViolation {
    variable_name: String,
    declaration_range: TextRange,
    reference_range: TextRange,
}

impl Rule for UseBlockScopedVar {
    type Query = SemanticServices;
    type State = BlockScopeViolation;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let mut violations = Vec::new();

        for binding in model.all_bindings() {
            let id = binding.tree();

            // Only check regular variables, not types
            if let AnyJsIdentifierBinding::JsIdentifierBinding(_) = id {
                let Some(declaration) = id.declaration() else {
                    continue;
                };

                // Only check var declarations
                if !is_var_declaration(&declaration) {
                    continue;
                }

                let Some(declaration_block) = find_declaration_block(&declaration) else {
                    continue;
                };

                let variable_name = id
                    .name_token()
                    .map(|token| token.text_trimmed().to_string())
                    .unwrap_or_default();

                // Check all references to this variable
                for reference in binding.all_references() {
                    let reference_node = reference.syntax();

                    // Skip if reference is in the same block as declaration
                    if is_in_same_block_scope(reference_node, &declaration_block) {
                        continue;
                    }

                    violations.push(BlockScopeViolation {
                        variable_name: variable_name.clone(),
                        declaration_range: id.range(),
                        reference_range: reference_node.text_trimmed_range(),
                    });
                }
            }
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.reference_range,
                markup! {
                    "Variable "<Emphasis>{state.variable_name}</Emphasis>" used outside of its declaration scope."
                },
            )
            .detail(
                state.declaration_range,
                markup! { "Variable declared here:" },
            )
            .note(
                markup! { "Variables declared with 'var' should be used within the block they are defined to emulate block scope." },
            ),
        )
    }
}

/// Check if the declaration is a var declaration
fn is_var_declaration(declaration: &AnyJsBindingDeclaration) -> bool {
    match declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration.clone())
    {
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => declarator
            .declaration()
            .map(|decl| decl.is_var())
            .unwrap_or(false),
        _ => false,
    }
}

/// Find the block that contains this declaration
fn find_declaration_block(
    declaration: &AnyJsBindingDeclaration,
) -> Option<SyntaxNode<biome_js_syntax::JsLanguage>> {
    let syntax = declaration.syntax();

    // Walk up the ancestors to find the containing block
    for ancestor in syntax.ancestors() {
        match ancestor.kind() {
            // Block statements (if, else, try, catch, finally, etc.)
            biome_js_syntax::JsSyntaxKind::JS_BLOCK_STATEMENT => {
                if let Some(parent) = ancestor.parent() {
                    match parent.kind() {
                        // These create block scopes in C-style languages
                        biome_js_syntax::JsSyntaxKind::JS_IF_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_ELSE_CLAUSE
                        | biome_js_syntax::JsSyntaxKind::JS_FOR_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_FOR_IN_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_FOR_OF_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_WHILE_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_DO_WHILE_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_WITH_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_TRY_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_CATCH_CLAUSE
                        | biome_js_syntax::JsSyntaxKind::JS_FINALLY_CLAUSE
                        | biome_js_syntax::JsSyntaxKind::JS_SWITCH_STATEMENT
                        | biome_js_syntax::JsSyntaxKind::JS_CASE_CLAUSE
                        | biome_js_syntax::JsSyntaxKind::JS_DEFAULT_CLAUSE
                        | biome_js_syntax::JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                            return Some(ancestor);
                        }
                        _ => continue,
                    }
                }
            }
            _ => continue,
        }
    }

    None
}

/// Check if a reference is in the same block scope as the declaration
fn is_in_same_block_scope(
    reference_node: &SyntaxNode<biome_js_syntax::JsLanguage>,
    declaration_block: &SyntaxNode<biome_js_syntax::JsLanguage>,
) -> bool {
    // Walk up from the reference to see if we find the declaration block
    for ancestor in reference_node.ancestors() {
        if ancestor == *declaration_block {
            return true;
        }
    }
    false
}
