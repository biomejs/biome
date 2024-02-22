use crate::semantic_services::SemanticServices;
use biome_analyze::{context::RuleContext, Rule, RuleDiagnostic};
use biome_analyze::{declare_rule, RuleSource};
use biome_console::markup;
use biome_js_semantic::Scope;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::TextRange;
use biome_rowan::AstNode;
use rustc_hash::FxHashMap;

declare_rule! {
    /// Disallow variable, function, class, and type redeclarations in the same scope.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 3;
    /// var a = 10;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let a = 3;
    /// let a = 10;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f() {}
    /// function f() {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class C {
    ///     static {
    ///         var c = 3;
    ///         var c = 10;
    ///     }
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// type Person = { name: string; }
    /// class Person { name: string; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 3;
    /// a = 10;
    /// ```
    ///
    /// ```ts
    /// class Foo {
    ///     bar(a: A);
    ///     bar(a: A, b: B);
    ///     bar(a: A, b: B) {}
    /// }
    /// ```
    pub NoRedeclare {
        version: "1.0.0",
        name: "noRedeclare",
        source: RuleSource::EslintTypeScript("no-redeclare"),
        recommended: true,
    }
}

#[derive(Debug)]
pub struct Redeclaration {
    name: String,
    declaration: TextRange,
    redeclaration: TextRange,
}

impl Rule for NoRedeclare {
    type Query = SemanticServices;
    type State = Redeclaration;
    type Signals = Vec<Redeclaration>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut redeclarations = Vec::default();
        for scope in ctx.query().scopes() {
            check_redeclarations_in_single_scope(&scope, &mut redeclarations);
        }
        redeclarations
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let Redeclaration {
            name,
            declaration,
            redeclaration,
        } = state;
        let diag = RuleDiagnostic::new(
            rule_category!(),
            redeclaration,
            markup! {
               "Shouldn't redeclare '"{ name }"'. Consider to delete it or rename it."
            },
        )
        .detail(
            declaration,
            markup! {
               "'"{ name }"' is defined here:"
            },
        );
        Some(diag)
    }
}

fn check_redeclarations_in_single_scope(scope: &Scope, redeclarations: &mut Vec<Redeclaration>) {
    let mut declarations = FxHashMap::<String, (TextRange, AnyJsBindingDeclaration)>::default();
    for binding in scope.bindings() {
        let id_binding = binding.tree();

        // We consider only binding of a declaration
        // This allows to skip function parameters, methods, ...
        if let Some(decl) = id_binding.declaration() {
            let name = id_binding.text();
            if let Some((first_text_range, first_decl)) = declarations.get(&name) {
                // Do not report:
                // - mergeable declarations.
                //   e.g. a `function` and a `namespace`
                // - when both are parameter-like.
                //   A parameter can override a previous parameter.
                if !(first_decl.is_mergeable(&decl)
                    || first_decl.is_parameter_like() && decl.is_parameter_like())
                {
                    redeclarations.push(Redeclaration {
                        name,
                        declaration: *first_text_range,
                        redeclaration: id_binding.syntax().text_trimmed_range(),
                    })
                }
            } else {
                declarations.insert(name, (id_binding.syntax().text_trimmed_range(), decl));
            }
        }
    }
}
