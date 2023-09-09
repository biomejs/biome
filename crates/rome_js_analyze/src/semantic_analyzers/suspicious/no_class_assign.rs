use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use rome_js_semantic::{Reference, ReferencesExtensions};
use rome_js_syntax::AnyJsClass;

use crate::semantic_services::Semantic;

declare_rule! {
    /// Disallow reassigning class members.
    ///
    /// A class declaration creates a variable that we can modify, however, the modification is a mistake in most cases.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {}
    /// A = 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// A = 0;
    /// class A {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class A {
    /// 	b() {
    /// 		A = 0;
    /// 	}
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let A = class A {
    /// 	b() {
    /// 		A = 0;
    /// 		// `let A` is shadowed by the class name.
    /// 	}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let A = class A {}
    /// A = 0; // A is a variable.
    /// ```
    ///
    /// ```js
    /// let A = class {
    ///     b() {
    ///         A = 0; // A is a variable.
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// class A {
    /// 	b(A) {
    /// 		A = 0; // A is a parameter.
    /// 	}
    /// }
    /// ```
    ///
    pub(crate) NoClassAssign {
        version: "1.0.0",
        name: "noClassAssign",
        recommended: true,
    }
}

impl Rule for NoClassAssign {
    type Query = Semantic<AnyJsClass>;
    type State = Reference;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        if let Ok(Some(id)) = node.id() {
            if let Some(id_binding) = id.as_js_identifier_binding() {
                return id_binding.all_writes(model).collect();
            }
        }

        Vec::new()
    }

    fn diagnostic(ctx: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx
            .query()
            .id()
            .ok()??
            .as_js_identifier_binding()?
            .name_token()
            .ok()?;
        let class_name = binding.text_trimmed();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.syntax().text_trimmed_range(),
                markup! {"'"{class_name}"' is a class."},
            )
            .detail(
                binding.text_trimmed_range(),
                markup! {"'"{class_name}"' is defined here."},
            ),
        )
    }
}
