use crate::JsRuleAction;
use crate::services::typed::Typed;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{JsVariableDeclaration, JsVariableDeclarator, JsVariableDeclaratorList, T};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_disposables::UseDisposablesOptions;

declare_lint_rule! {
    /// Detects a disposable object assigned to a variable without using or await using syntax.
    ///
    /// Disposable objects, which implements Disposable or AsyncDisposable interface, are intended
    /// to dispose after use. Not disposing them can lead some resource or memory leak depending on
    /// the implementation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=example1.ts
    /// function createDisposable(): Disposable {
    ///   return {
    ///     [Symbol.dispose]() {
    ///       // do something
    ///     },
    ///   };
    /// }
    ///
    /// const disposable = createDisposable();
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=example2.ts
    /// class MyClass implements AsyncDisposable {
    ///   async [Symbol.asyncDispose]() {
    ///     // do something
    ///   }
    /// }
    ///
    /// const instance = new MyClass();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=example3.ts
    /// function createDisposable(): Disposable {
    ///   return {
    ///     [Symbol.dispose]() {
    ///       // do something
    ///     },
    ///   };
    /// }
    ///
    /// using disposable = createDisposable();
    /// ```
    ///
    /// ```ts,file=example4.ts
    /// class MyClass implements AsyncDisposable {
    ///   async [Symbol.asyncDispose]() {
    ///     // do something
    ///   }
    /// }
    ///
    /// await using instance = new MyClass();
    /// ```
    ///
    pub UseDisposables {
        version: "next",
        name: "useDisposables",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Project],
    }
}

impl Rule for UseDisposables {
    type Query = Typed<JsVariableDeclarator>;
    type State = DisposableKind;
    type Signals = Option<Self::State>;
    type Options = UseDisposablesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let decl = ctx.query();
        let initializer = decl.initializer()?;
        let expression = initializer.expression().ok()?;
        let ty = ctx.type_of_expression(&expression);

        // Lookup the parent declaration which possibly has `await` and/or `using` tokens.
        let parent = decl
            .parent::<JsVariableDeclaratorList>()?
            .parent::<JsVariableDeclaration>()?;

        let is_disposed = parent.kind().ok()?.kind() == T![using];
        if ty.is_disposable() && !is_disposed {
            return Some(DisposableKind::Disposable);
        }

        let is_async_disposed = is_disposed && parent.await_token().is_some();
        if ty.is_async_disposable() && !is_async_disposed {
            return Some(DisposableKind::AsyncDisposable);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! { "Disposable object is assigned here but never disposed." },
            )
            .note(match state {
                DisposableKind::Disposable => markup! {
                    "The object implements the "<Emphasis>"Disposable"</Emphasis>" interface, which is intended to be disposed after use with "<Emphasis>"using"</Emphasis>" syntax."
                },
                DisposableKind::AsyncDisposable => markup! {
                    "The object implements the "<Emphasis>"AsyncDisposable"</Emphasis>" interface, which is intended to be disposed after use with "<Emphasis>"await using"</Emphasis>" syntax."
                },
            })
            .note(markup! {
                "Not disposing the object properly can lead some resource or memory leak."
            })
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let decl = ctx
            .query()
            .parent::<JsVariableDeclaratorList>()?
            .parent::<JsVariableDeclaration>()?;

        let mut new_decl = decl
            .clone()
            .with_kind_token(make::token_with_trailing_space(T![using]));

        if let DisposableKind::AsyncDisposable = state {
            new_decl = new_decl.with_await_token(Some(make::token_with_trailing_space(T![await])));
        }

        mutation.replace_node(decl, new_decl);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add the "<Emphasis>"using"</Emphasis>" keyword to dispose the object when leaving the scope." },
            mutation,
        ))
    }
}

pub enum DisposableKind {
    Disposable,
    AsyncDisposable,
}
