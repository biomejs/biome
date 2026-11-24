use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{JsDeclarationKind, Reference};
use biome_js_syntax::JsIdentifierAssignment;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_function_assign::NoFunctionAssignOptions;

declare_lint_rule! {
    /// Disallow reassigning function declarations.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo() { };
    /// foo = bar;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     foo = bar;
    ///  }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo = bar;
    /// function foo() { };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// [foo] = bar;
    /// function foo() { };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// ({ x: foo = 0 } = bar);
    /// function foo() { };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     [foo] = bar;
    ///  }
    /// ```
    /// ```js,expect_diagnostic
    /// (function () {
    ///     ({ x: foo = 0 } = bar);
    ///     function foo() { };
    ///  })();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo() {
    ///     var foo = bar;
    ///  }
    /// ```
    ///
    /// ```js
    /// function foo(foo) {
    ///     foo = bar;
    ///  }
    /// ```
    ///
    /// ```js
    /// function foo() {
    ///     var foo;
    ///     foo = bar;
    ///  }
    /// ```
    ///
    /// ```js
    /// var foo = () => {};
    /// foo = bar;
    /// ```
    ///
    /// ```js
    /// var foo = function() {};
    /// foo = bar;
    /// ```
    ///
    /// ```js
    /// var foo = function() {
    ///     foo = bar;
    ///  };
    /// ```
    ///
    /// ```js
    /// import bar from 'bar';
    /// function foo() {
    ///     var foo = bar;
    /// }
    /// ```
    pub NoFunctionAssign {
        version: "1.0.0",
        name: "noFunctionAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-func-assign").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

pub enum FunctionDeclaration {
    Local {
        range: TextRange,
        all_writes: Box<[Reference]>,
    },
    OtherSnippet,
}

impl Rule for NoFunctionAssign {
    type Query = Semantic<JsIdentifierAssignment>;
    type State = FunctionDeclaration;
    type Signals = Option<Self::State>;
    type Options = NoFunctionAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let assignment = ctx.query();
        let model = ctx.model();
        if let Some(binding) = model.binding(assignment) {
            if binding.declaration_kind() != JsDeclarationKind::Function {
                return None;
            }
            let all_writes: Vec<Reference> = binding.all_writes().collect();
            if all_writes.first()?.syntax().text_trimmed_range()
                != assignment.syntax().text_trimmed_range()
            {
                return None;
            }
            return Some(FunctionDeclaration::Local {
                range: binding.range(),
                all_writes: all_writes.into_boxed_slice(),
            });
        }

        (ctx.get_service::<EmbeddedService>()?
            .svelte_declaration_kind(assignment.range())?
            == JsDeclarationKind::Function)
            .then_some(FunctionDeclaration::OtherSnippet)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (range, all_writes) = match state {
            FunctionDeclaration::Local { range, all_writes } => {
                (*range, Some(all_writes.as_ref()))
            }
            FunctionDeclaration::OtherSnippet => {
                (ctx.query().syntax().text_trimmed_range(), None)
            }
        };
        let mut diag = RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Do not reassign a function declaration."
            },
        );

        let mut hoisted_quantity = 0;
        if let Some(all_writes) = all_writes {
            for reference in all_writes {
                let node = reference.syntax();
                diag = diag.detail(node.text_trimmed_range(), "Reassigned here.");

                hoisted_quantity += i32::from(reference.is_using_hoisted_declaration());
            }
        }

        let diag = if hoisted_quantity > 0 {
            diag.note(
                markup! {"Reassignment happens here because the function declaration is hoisted."},
            )
        } else {
            diag
        };

        let diag = diag.note(markup! {"Use a local variable instead."});

        Some(diag)
    }
}
