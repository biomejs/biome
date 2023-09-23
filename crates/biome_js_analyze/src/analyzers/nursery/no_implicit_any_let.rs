use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsFileSource, JsVariableDeclaration, JsVariableDeclarator};

declare_rule! {
    /// Restrict use of implicit any type in Typescript.
    ///
    /// Typescript variable declaration without any `type` or `initialization` can cause issue later in the code.
    ///
    ///
    ///
    /// Source: https://www.typescriptlang.org/tsconfig#noImplicitAny
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// var a;
    /// a = 2;
    /// let b;
    /// b = 1
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// var a = 1;
    /// let a:number;
    /// var b: number
    /// var b =10;
    /// ```
    ///
    pub(crate) NoImplicitAnyLet {
        version: "next",
        name: "noImplicitAnyLet",
        recommended: true,
    }
}

impl Rule for NoImplicitAnyLet {
    type Query = Ast<JsVariableDeclaration>;
    type State = JsVariableDeclarator;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        let is_ts_source = source_type.is_typescript();
        let node = ctx.query();
        let is_declaration = source_type.is_definition_file();

        if node.is_const() || is_declaration || !is_ts_source {
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
                    "Variable " <Emphasis>{variable.text()}</Emphasis> " has implicitly " <Emphasis>"any"</Emphasis> " type"
                },
            )
            .note(markup! {
                "Declare type or initialize the variable with some value"
            }),
        )
    }
}
