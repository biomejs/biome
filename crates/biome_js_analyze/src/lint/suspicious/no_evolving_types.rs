use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsFileSource, JsVariableDeclaration, JsVariableDeclarator};

declare_lint_rule! {
    /// Disallow variables from evolving into `any` type through reassignments.
    ///
    /// In TypeScript, variables without explicit type annotations can evolve their types based on subsequent assignments.
    ///
    /// When  TypeScript's [noImplicitAny](https://www.typescriptlang.org/tsconfig/#noImplicitAny) is disabled,
    /// variables without explicit type annotations have implicitly the type `any`.
    /// Just like the `any` type, evolved `any` types disable many type-checking rules and should be avoided to maintain strong type safety.
    /// This rule prevents such cases by ensuring variables do not evolve into `any` type, encouraging explicit type annotations and controlled type evolutions.
    ///
    /// If you enabled TypeScript's [noImplicitAny](https://www.typescriptlang.org/tsconfig/#noImplicitAny) and want to benefit of evolving types,
    /// then we recommend to disable this rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let a;
    /// ````
    ///
    /// ```ts,expect_diagnostic
    /// const b = [];
    /// ````
    ///
    /// ```ts,expect_diagnostic
    /// let c = null;
    /// ````
    ///
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let a: number;
    /// let b = 1;
    /// var c : string;
    /// var d = "abn";
    /// const e: never[] = [];
    /// const f = [null];
    /// const g = ['1'];
    /// const h = [1];
    /// let workspace: Workspace | null = null;
    /// ```
    ///
    pub NoEvolvingTypes {
        version: "1.6.3",
        name: "noEvolvingTypes",
        language: "ts",
        recommended: false,
    }
}

impl Rule for NoEvolvingTypes {
    type Query = Ast<JsVariableDeclaration>;
    type State = JsVariableDeclarator;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>().language();
        let node = ctx.query();

        if !source_type.is_typescript() || source_type.is_definition_file() {
            return None;
        }

        for declarator in node.declarators() {
            let variable = declarator.ok()?;

            let is_initialized = variable.initializer().is_some();
            let is_type_annotated = variable.variable_annotation().is_some();

            if !is_initialized && !is_type_annotated {
                return Some(variable);
            }

            if is_initialized {
                let initializer = variable.initializer()?;
                let expression = initializer.expression().ok()?;
                match expression {
                    AnyJsExpression::AnyJsLiteralExpression(literal_expr) => {
                        if literal_expr.as_js_null_literal_expression().is_some()
                            && !is_type_annotated
                        {
                            return Some(variable);
                        }
                    }
                    AnyJsExpression::JsArrayExpression(array_expr) => {
                        if array_expr.elements().into_iter().next().is_none() && !is_type_annotated
                        {
                            return Some(variable);
                        }
                    }
                    _ => continue,
                };
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
                variable.text_trimmed_range(),
                markup! {
                    "The type of this variable may evolve implicitly to any type, including the "<Emphasis>"any"</Emphasis>" type."
                },
            )
            .note(markup! {
                "Add an explicit type or initialization to avoid implicit type evolution."
            }),
        )
    }
}
