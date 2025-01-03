use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{AllBindingWriteReferencesIter, Reference, ReferencesExtensions};
use biome_js_syntax::{AnyJsBinding, AnyJsBindingPattern, AnyJsFormalParameter, AnyJsParameter};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow reassigning `function` parameters.
    ///
    /// Assignment to a `function` parameters can be misleading and confusing,
    /// as modifying parameters will also mutate the `arguments` object.
    /// It is often unintended and indicative of a programmer error.
    ///
    /// In contrast to the _ESLint_ rule, this rule cannot be configured to report
    /// assignments to a property of a parameter.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f(param) {
    ///     param = 13;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(param) {
    ///     param++;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function f(param) {
    ///     for (param of arr) {}
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class C {
    ///     constructor(readonly prop: number) {
    ///         prop++
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f(param) {
    ///     let local = param;
    /// }
    /// ```
    ///
    pub NoParameterAssign {
        version: "1.0.0",
        name: "noParameterAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-param-reassign")],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoParameterAssign {
    type Query = Semantic<AnyJsParameter>;
    type State = Reference;
    type Signals = AllBindingWriteReferencesIter;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let param = ctx.query();
        let model = ctx.model();
        if let Some(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding))) =
            binding_of(param)
        {
            return binding.all_writes(model);
        }
        // Empty iterator that conforms to `AllBindingWriteReferencesIter` type.
        std::iter::successors(None, |_| None)
    }

    fn diagnostic(ctx: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        let param = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.syntax().text_trimmed_range(),
                markup! {
                    "Reassigning a "<Emphasis>"function parameter"</Emphasis>" is confusing."
                },
            )
            .detail(
                param.syntax().text_trimmed_range(),
                markup! {
                    "The "<Emphasis>"parameter"</Emphasis>" is declared here:"
                },
            )
            .note(markup! {
                "Use a local variable instead."
            }),
        )
    }
}

fn binding_of(param: &AnyJsParameter) -> Option<AnyJsBindingPattern> {
    match param {
        AnyJsParameter::AnyJsFormalParameter(formal_param) => match &formal_param {
            AnyJsFormalParameter::JsBogusParameter(_) | AnyJsFormalParameter::JsMetavariable(_) => {
                None
            }
            AnyJsFormalParameter::JsFormalParameter(param) => param.binding().ok(),
        },
        AnyJsParameter::JsRestParameter(param) => param.binding().ok(),
        AnyJsParameter::TsThisParameter(_) => None,
    }
}
