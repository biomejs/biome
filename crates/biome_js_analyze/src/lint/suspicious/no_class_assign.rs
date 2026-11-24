use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::JsDeclarationKind;
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{JsIdentifierAssignment, JsIdentifierBinding};
use biome_rowan::AstNode;
use biome_rule_options::no_class_assign::NoClassAssignOptions;

use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;

declare_lint_rule! {
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
    pub NoClassAssign {
        version: "1.0.0",
        name: "noClassAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-class-assign").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

pub enum ClassDeclaration {
    Local(JsIdentifierBinding),
    OtherSnippet,
}

impl Rule for NoClassAssign {
    type Query = Semantic<JsIdentifierAssignment>;
    type State = ClassDeclaration;
    type Signals = Option<Self::State>;
    type Options = NoClassAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assignment = ctx.query();
        if let Some(binding) = ctx.model().binding(assignment) {
            if binding.declaration_kind() != JsDeclarationKind::Class {
                return None;
            }
            let AnyJsIdentifierBinding::JsIdentifierBinding(binding) = binding.tree() else {
                return None;
            };
            return Some(ClassDeclaration::Local(binding));
        }

        (ctx.get_service::<EmbeddedService>()?
            .svelte_declaration_kind(assignment.range())?
            == JsDeclarationKind::Class)
            .then_some(ClassDeclaration::OtherSnippet)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let assignment = ctx.query();
        let assignment_name = assignment.name_token().ok()?;
        let binding = match state {
            ClassDeclaration::Local(binding) => Some(binding.name_token().ok()?),
            ClassDeclaration::OtherSnippet => None,
        };
        let class_name = binding.as_ref().unwrap_or(&assignment_name).text_trimmed();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            assignment.syntax().text_trimmed_range(),
            markup! {"The class '"{class_name}"' is reassigned."},
        );
        if let Some(binding) = binding.as_ref() {
            diagnostic = diagnostic.detail(
                binding.text_trimmed_range(),
                markup! {"'"{class_name}"' is defined here."},
            );
        }
        Some(
            diagnostic
                .note(markup! {
                    "Reassigning a class replaces it. Code that expects the original class, such as "<Emphasis>"new"</Emphasis>" calls and "<Emphasis>"instanceof"</Emphasis>" checks, can then break."
                })
                .note(markup! {
                    "Store the new value in a separate variable, or give the new class a different name."
                }),
        )
    }
}
