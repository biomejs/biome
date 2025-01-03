use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsFileSource, JsVariableDeclaration, JsVariableDeclarator};

declare_lint_rule! {
    /// Disallow use of implicit `any` type on variable declarations.
    ///
    /// TypeScript variable declaration without any type annotation and initialization have the `any` type.
    /// The any type in TypeScript is a dangerous “escape hatch” from the type system.
    /// Using any disables many type checking rules and is generally best used only as a last resort or when prototyping code.
    /// TypeScript’s [`--noImplicitAny` compiler option](https://www.typescriptlang.org/tsconfig#noImplicitAny) doesn't report this case.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// var a;
    /// a = 2;
    /// ````
    ///
    /// ```ts,expect_diagnostic
    /// let b;
    /// b = 1
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// var a = 1;
    /// let a:number;
    /// var b: number
    /// var b =10;
    /// ```
    ///
    pub NoImplicitAnyLet {
        version: "1.4.0",
        name: "noImplicitAnyLet",
        language: "ts",
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoImplicitAnyLet {
    type Query = Ast<JsVariableDeclaration>;
    type State = JsVariableDeclarator;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        let node = ctx.query();

        if !source_type.is_typescript() || source_type.is_definition_file() || node.is_const() {
            return None;
        }

        for declarator in node.declarators() {
            let variable = declarator.ok()?;
            let is_initialized = variable.initializer().is_some();
            let is_type_annotated = variable.variable_annotation().is_some();
            if !is_initialized && !is_type_annotated {
                return Some(variable);
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let variable = node
            .id()
            .ok()?
            .as_any_js_binding()?
            .as_js_identifier_binding()?
            .name_token()
            .ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                variable.text_range(),
                markup! {
                    "This variable implicitly has the " <Emphasis>"any"</Emphasis> " type."
                },
            )
            .note(markup! {
                "Variable declarations without type annotation and initialization implicitly have the "<Emphasis>"any"</Emphasis>" type. Declare a type or initialize the variable with some value."
            }),
        )
    }
}
