use crate::services::semantic::SemanticServices;
use biome_analyze::{context::RuleContext, Rule, RuleDiagnostic};
use biome_analyze::{declare_lint_rule, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::Scope;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{JsSyntaxKind, TextRange};
use biome_rowan::AstNode;
use rustc_hash::FxHashMap;

declare_lint_rule! {
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
        language: "js",
        sources: &[
            RuleSource::Eslint("no-redeclare"),
            RuleSource::EslintTypeScript("no-redeclare"),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

#[derive(Debug)]
pub struct Redeclaration {
    name: Box<str>,
    declaration: TextRange,
    redeclaration: TextRange,
}

impl Rule for NoRedeclare {
    type Query = SemanticServices;
    type State = Redeclaration;
    type Signals = Box<[Redeclaration]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let mut redeclarations = Vec::default();
        for scope in ctx.query().scopes() {
            check_redeclarations_in_single_scope(&scope, &mut redeclarations);
        }
        redeclarations.into_boxed_slice()
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
               "Shouldn't redeclare '"{ name.as_ref() }"'. Consider to delete it or rename it."
            },
        )
        .detail(
            declaration,
            markup! {
               "'"{ name.as_ref() }"' is defined here:"
            },
        );
        Some(diag)
    }
}

fn check_redeclarations_in_single_scope(scope: &Scope, redeclarations: &mut Vec<Redeclaration>) {
    let mut declarations = FxHashMap::<String, (TextRange, AnyJsBindingDeclaration)>::default();
    if scope.syntax().kind() == JsSyntaxKind::JS_FUNCTION_BODY {
        // Handle cases where a variable/type redeclares a parameter or type parameter.
        // For example:
        //
        // ```js
        // function f<T>(a) { type T = number; const a = 0; }
        // ```
        //
        // I previously tried to remove the JsFunctionBody scope
        // to directly add declarations of the function body in the function scope.
        // Unfortunately, this is not a good idea because variables and types outside the function
        // can be referenced in the parameters and type parameters of the function,
        // then shadowed in the function body.
        // Thus, using a distinct scope for the function body and the function makes sense.
        // For example:
        //
        // ```js
        // type U = string;
        // function g() {}
        // function f<T = U>(h = g) { type U = number; function g() {}; }
        // ```
        if let Some(function_scope) = scope.parent() {
            for binding in function_scope.bindings() {
                let id_binding = binding.tree();
                if let Some(decl) = id_binding.declaration() {
                    // Ignore the function itself.
                    if !matches!(decl, AnyJsBindingDeclaration::JsFunctionExpression(_)) {
                        let name = id_binding.to_trimmed_string();
                        declarations.insert(name, (id_binding.syntax().text_trimmed_range(), decl));
                    }
                }
            }
        }
    }
    for binding in scope.bindings() {
        let id_binding = binding.tree();

        // We consider only binding of a declaration
        // This allows to skip function parameters, methods, ...
        if let Some(decl) = id_binding.declaration() {
            let name = id_binding.to_trimmed_string();
            if let Some((first_text_range, first_decl)) = declarations.get(&name) {
                // Do not report:
                // - mergeable declarations.
                //   e.g. a `function` and a `namespace`
                // - when both are parameter-like.
                //   A parameter can override a previous parameter.
                // - when both are type parameter in different declarations.
                //   A type parameter can be redeclared if they are in different declarations.
                if !(first_decl.is_mergeable(&decl)
                    || first_decl.is_parameter_like() && decl.is_parameter_like()
                    || first_decl.is_type_parameter()
                        && decl.is_type_parameter()
                        && first_decl.syntax().parent() != decl.syntax().parent())
                {
                    redeclarations.push(Redeclaration {
                        name: name.into_boxed_str(),
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
