use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsFileSource, TsEnumDeclaration};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow TypeScript enum.
    ///
    /// TypeScript enums are not a type-level extension to JavaScript like type annotations or definitions.
    /// Users may wish to disable non-type-level extensions to use bundlers or compilers that only strip types.
    ///
    /// Const enums are not covered by this rule since `noConstEnum` already handles them.
    /// Enums within the ambient context, including declaration files, are ignores as well.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// enum Foo {
    ///     BAR = 'bar',
    ///     BAZ = 'baz',
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const Foo = {
    ///     BAR: 'bar',
    ///     BAZ: 'baz',
    /// } as const
    /// ```
    ///
    /// ```ts
    /// type Foo = 'bar' | 'baz'
    /// ```
    ///
    /// ```ts
    /// const enum Foo {
    ///     BAR = 'bar',
    ///     BAZ = 'baz',
    /// }
    /// ```
    ///
    ///
    pub NoEnum {
        version: "1.9.0",
        name: "noEnum",
        language: "ts",
        recommended: false,
    }
}

impl Rule for NoEnum {
    type Query = Ast<TsEnumDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let enum_decl = ctx.query();

        let source_type = ctx.source_type::<JsFileSource>().language();
        let is_declaration = source_type.is_definition_file();

        if is_declaration {
            return None;
        }

        let is_const_decl = enum_decl.const_token().is_some();

        if is_const_decl || enum_decl.is_ambient() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Don't use "<Emphasis>"enum"</Emphasis>
                },
            )
            .note(markup! {
                "TypeScript enums are not a type-level extension to JavaScript like type annotations or definitions. Users may wish to disable non-type-level extensions to use bundlers or compilers that only strip types."
            })
            .note(markup! {
                "Use JavaScript objects or TypeScript unions instead."
            }),
        )
    }
}
